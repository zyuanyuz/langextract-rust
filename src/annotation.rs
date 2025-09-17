//! Text annotation functionality.

use crate::{
    alignment::TextAligner,
    chunking::{ChunkResult, ResultAggregator, TextChunk, TokenChunk, ChunkIterator},
    data::{AnnotatedDocument, Extraction, FormatType, Document},
    exceptions::LangExtractResult,
    inference::BaseLanguageModel,
    logging::{report_progress, ProgressEvent},
    prompting::PromptTemplateStructured,
    resolver::Resolver,
    tokenizer::Tokenizer,
};
use futures::future::join_all;
use std::collections::HashMap;
use std::time::Instant;

/// Main annotator for processing text through language models
pub struct Annotator {
    language_model: Box<dyn BaseLanguageModel>,
    prompt_template: PromptTemplateStructured,
    #[allow(dead_code)]
    format_type: FormatType,
    #[allow(dead_code)]
    fence_output: bool,
}

impl Annotator {
    /// Create a new annotator
    pub fn new(
        language_model: Box<dyn BaseLanguageModel>,
        prompt_template: PromptTemplateStructured,
        format_type: FormatType,
        fence_output: bool,
    ) -> Self {
        Self {
            language_model,
            prompt_template,
            format_type,
            fence_output,
        }
    }

    /// Annotate text and return annotated document
    pub async fn annotate_text(
        &self,
        text: &str,
        resolver: &Resolver,
        max_char_buffer: usize,
        batch_length: usize,
        additional_context: Option<&str>,
        debug: bool,
        extraction_passes: usize,
        max_workers: usize,
    ) -> LangExtractResult<AnnotatedDocument> {
        // Check if we need to chunk the text
        if text.len() <= max_char_buffer {
            // Text is small enough, process directly
            return self.process_single_text(text, resolver, additional_context, debug).await;
        }

        // Text is too large, use token-based chunking
        if debug {
            report_progress(ProgressEvent::Debug {
                operation: "chunking".to_string(),
                details: format!("Text length ({} chars) exceeds buffer limit ({} chars), using token-based chunking",
                    text.len(), max_char_buffer),
            });
        }

        self.process_token_chunked_text(
            text,
            resolver,
            max_char_buffer,
            batch_length,
            additional_context,
            debug,
            extraction_passes,
            max_workers,
        ).await
    }

    /// Process text that fits within the buffer limit
    async fn process_single_text(
        &self,
        text: &str,
        resolver: &Resolver,
        additional_context: Option<&str>,
        debug: bool,
    ) -> LangExtractResult<AnnotatedDocument> {
        // Build the prompt
        let prompt = self.build_prompt(text, additional_context)?;

        // Report processing started
        report_progress(ProgressEvent::ProcessingStarted {
            text_length: text.len(),
            model: self.language_model.model_id().to_string(),
            provider: self.language_model.provider_name().to_string(),
        });

        if debug {
            let prompt_preview = if prompt.len() > 200 {
                format!("{}... (truncated, total length: {} chars)",
                    &prompt.chars().take(200).collect::<String>(), prompt.len())
            } else {
                prompt.clone()
            };
            report_progress(ProgressEvent::Debug {
                operation: "model_call".to_string(),
                details: format!("Model: {}, Provider: {}, Prompt: {}",
                    self.language_model.model_id(),
                    self.language_model.provider_name(),
                    prompt_preview),
            });
        }

        // Create inference parameters
        let mut kwargs = HashMap::new();
        kwargs.insert("temperature".to_string(), serde_json::json!(1));
        kwargs.insert("max_completion_tokens".to_string(), serde_json::json!(8000));

        // Call the language model
        let results = self.language_model.infer(&[prompt], &kwargs).await?;

        report_progress(ProgressEvent::ModelResponse {
            success: true,
            output_length: results.first()
                .and_then(|batch| batch.first())
                .map(|output| output.text().len()),
        });

        if debug {
            report_progress(ProgressEvent::Debug {
                operation: "model_response".to_string(),
                details: format!("Received {} batches from language model", results.len()),
            });
        }

        // Extract the response
        let mut annotated_doc = AnnotatedDocument::with_extractions(Vec::new(), text.to_string());

        if let Some(batch) = results.first() {
            if let Some(output) = batch.first() {
                let response_text = output.text();

                if debug {
                    report_progress(ProgressEvent::Debug {
                        operation: "model_response".to_string(),
                        details: format!("Raw response from model: {}", response_text),
                    });
                }

                // Extract expected fields from examples for validation
                // 只收集extraction_class的类别到expected_fields中
                let expected_fields: Vec<String> = self.prompt_template.examples
                    .iter()
                    .flat_map(|example| example.extractions.iter())
                    .map(|extraction| extraction.extraction_class.clone())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();

                // Use new validation system with raw data preservation
                report_progress(ProgressEvent::ValidationStarted {
                    raw_output_length: response_text.len(),
                });

                match resolver.validate_and_parse(response_text, &expected_fields) {
                    Ok((mut extractions, validation_result)) => {
                        // Report validation results
                        report_progress(ProgressEvent::ValidationCompleted {
                            extractions_found: extractions.len(),
                            aligned_count: 0, // Will be updated after alignment
                            errors: validation_result.errors.len(),
                            warnings: validation_result.warnings.len(),
                        });

                        if debug {
                            if let Some(raw_file) = &validation_result.raw_output_file {
                                report_progress(ProgressEvent::Debug {
                                    operation: "validation".to_string(),
                                    details: format!("Raw output saved to: {}", raw_file),
                                });
                            }

                            for error in &validation_result.errors {
                                report_progress(ProgressEvent::Debug {
                                    operation: "validation".to_string(),
                                    details: format!("Validation error: {}", error.message),
                                });
                            }
                            for warning in &validation_result.warnings {
                                report_progress(ProgressEvent::Debug {
                                    operation: "validation".to_string(),
                                    details: format!("Validation warning: {}", warning.message),
                                });
                            }
                        }

                        // Align extractions with the source text
                        let aligner = TextAligner::new();
                        let aligned_count = aligner.align_extractions(&mut extractions, text, 0)
                            .unwrap_or(0);

                        annotated_doc.extractions = Some(extractions);

                        // Update validation result with actual aligned count
                        report_progress(ProgressEvent::ValidationCompleted {
                            extractions_found: annotated_doc.extraction_count(),
                            aligned_count,
                            errors: validation_result.errors.len(),
                            warnings: validation_result.warnings.len(),
                        });
                    }
                    Err(e) => {
                        if debug {
                            report_progress(ProgressEvent::Debug {
                                operation: "validation".to_string(),
                                details: format!("Failed to parse response as structured data: {}. Treating as unstructured response", e),
                            });
                        }
                        // If parsing fails, create a single extraction with the raw response
                        let extraction = Extraction::new("raw_response".to_string(), response_text.to_string());
                        annotated_doc.extractions = Some(vec![extraction]);
                    }
                }
            }
        }

        Ok(annotated_doc)
    }

    /// Process large text using chunking
    /// Process text with chunking using token-based strategy
    async fn process_token_chunked_text(
        &self,
        text: &str,
        resolver: &Resolver,
        max_char_buffer: usize,
        batch_length: usize,
        additional_context: Option<&str>,
        debug: bool,
        extraction_passes: usize,
        max_workers: usize,
    ) -> LangExtractResult<AnnotatedDocument> {
        // Create tokenizer and tokenize the text
        let tokenizer = Tokenizer::new()?;
        let tokenized_text = tokenizer.tokenize(text)?;

        // Create document for chunking
        let document = Document {
            document_id: None,
            text: text.to_string(),
            additional_context: None,
        };

        // Create token-based chunk iterator
        let chunk_iter = ChunkIterator::new(&tokenized_text, &tokenizer, max_char_buffer, Some(&document))?;

        // Collect chunks from iterator
        let token_chunks: Result<Vec<TokenChunk>, _> = chunk_iter.collect();
        let token_chunks = token_chunks?;

        // Convert TokenChunks to TextChunks for compatibility with existing pipeline
        let mut text_chunks = Vec::new();
        for (i, token_chunk) in token_chunks.iter().enumerate() {
            let chunk_text = token_chunk.chunk_text(&tokenizer)?;
            let char_interval = token_chunk.char_interval(&tokenizer)?;
            let chunk_len = chunk_text.len();

            let text_chunk = TextChunk {
                id: i,
                text: chunk_text,
                char_offset: char_interval.start_pos.unwrap_or(0),
                char_length: chunk_len,
                document_id: None,
                has_overlap: false,
                overlap_info: None,
            };
            text_chunks.push(text_chunk);
        }

        // Report chunking started
        report_progress(ProgressEvent::ChunkingStarted {
            total_chars: text.len(),
            chunk_count: text_chunks.len(),
            strategy: "token-based".to_string(),
        });

        if debug {
            for (i, chunk) in text_chunks.iter().enumerate() {
                report_progress(ProgressEvent::Debug {
                    operation: "chunking".to_string(),
                    details: format!("Token Chunk {}: {} chars (offset: {})", i, chunk.char_length, chunk.char_offset),
                });
            }
        }

        // Process chunks in parallel batches
        self.process_text_chunks_in_batches(
            text_chunks,
            text,
            resolver,
            batch_length,
            additional_context,
            debug,
            extraction_passes,
            max_workers,
        ).await
    }

    /// Common method to process text chunks in parallel batches
    async fn process_text_chunks_in_batches(
        &self,
        chunks: Vec<TextChunk>,
        original_text: &str,
        resolver: &Resolver,
        batch_length: usize,
        additional_context: Option<&str>,
        debug: bool,
        extraction_passes: usize,
        max_workers: usize,
    ) -> LangExtractResult<AnnotatedDocument> {
        // Process chunks in parallel batches
        let mut chunk_results = Vec::new();
        let effective_workers = std::cmp::min(max_workers, batch_length);
        let total_chunks = chunks.len();
        let mut processed_chunks = 0;

        for (batch_idx, chunk_batch) in chunks.chunks(batch_length).enumerate() {
            // Progress reporting for each batch
            report_progress(ProgressEvent::BatchProgress {
                batch_number: batch_idx + 1,
                total_batches: (chunks.len() + batch_length - 1) / batch_length,
                chunks_processed: processed_chunks,
                total_chunks,
            });

            let batch_futures: Vec<_> = chunk_batch.iter()
                .take(effective_workers)
                .map(|chunk| self.process_chunk(chunk, resolver, additional_context, debug))
                .collect();

            let batch_results = join_all(batch_futures).await;

            for result in batch_results {
                chunk_results.push(result?);
            }

            processed_chunks += chunk_batch.len();

            if debug {
                report_progress(ProgressEvent::Debug {
                    operation: "batch_processing".to_string(),
                    details: format!("Batch {} processing completed ({}/{} chunks done)",
                        batch_idx + 1, processed_chunks, total_chunks),
                });
            }
        }

        // Handle multiple extraction passes
        if extraction_passes > 1 {
            if debug {
                report_progress(ProgressEvent::Debug {
                    operation: "multipass".to_string(),
                    details: format!("Running {} additional extraction passes", extraction_passes - 1),
                });
            }

            // TODO: Implement multi-pass extraction
            // For now, we just use the single pass results
        }

        // Aggregate results
        report_progress(ProgressEvent::AggregationStarted {
            chunk_count: chunks.len(),
        });
        let aggregator = ResultAggregator::new();
        let final_result = aggregator.aggregate_chunk_results(
            chunk_results,
            original_text.to_string(),
            None,
        )?;

        report_progress(ProgressEvent::ProcessingCompleted {
            total_extractions: final_result.extraction_count(),
            processing_time_ms: 0, // We don't track time here, but it's required
        });

        if debug {
            report_progress(ProgressEvent::Debug {
                operation: "aggregation".to_string(),
                details: format!("Aggregated {} total extractions from {} chunks",
                    final_result.extraction_count(), chunks.len()),
            });
        }

        Ok(final_result)
    }



    /// Process a single chunk
    async fn process_chunk(
        &self,
        chunk: &TextChunk,
        resolver: &Resolver,
        additional_context: Option<&str>,
        debug: bool,
    ) -> LangExtractResult<ChunkResult> {
        let start_time = Instant::now();

        match self.process_single_text(&chunk.text, resolver, additional_context, false).await {
            Ok(annotated_doc) => {
                let mut extractions = annotated_doc.extractions.unwrap_or_default();

                // Align extractions with the chunk text
                let aligner = TextAligner::new();
                let aligned_count = aligner.align_chunk_extractions(
                    &mut extractions,
                    &chunk.text,
                    chunk.char_offset,
                ).unwrap_or(0);

                if debug {
                    report_progress(ProgressEvent::Debug {
                        operation: "chunk_processing".to_string(),
                        details: format!("Chunk {} produced {} extractions ({} aligned)",
                            chunk.id, extractions.len(), aligned_count),
                    });
                }

                Ok(ChunkResult::success(
                    chunk.id,
                    extractions,
                    chunk.char_offset,
                    chunk.char_length,
                ).with_processing_time(start_time.elapsed()))
            }
            Err(e) => {
                if debug {
                    report_progress(ProgressEvent::Debug {
                        operation: "chunk_processing".to_string(),
                        details: format!("Chunk {} failed: {}", chunk.id, e),
                    });
                }

                Ok(ChunkResult::failure(
                    chunk.id,
                    chunk.char_offset,
                    chunk.char_length,
                    e.to_string(),
                ).with_processing_time(start_time.elapsed()))
            }
        }
    }

    /// Build the prompt using the new template system
    fn build_prompt(&self, text: &str, additional_context: Option<&str>) -> LangExtractResult<String> {
        // Use the new template system for better prompt generation
        self.prompt_template.render(text, additional_context)
    }

    /// Parse the model response into extractions
    #[allow(dead_code)]
    fn parse_response(&self, response: &str) -> LangExtractResult<Vec<Extraction>> {
        // Try to parse as JSON first
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(response) {
            return self.parse_json_response(&json_value);
        }

        // If that fails, try to extract JSON from the response (in case it's wrapped)
        if let Some(json_start) = response.find('{') {
            if let Some(json_end) = response.rfind('}') {
                let json_str = &response[json_start..=json_end];
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_str) {
                    return self.parse_json_response(&json_value);
                }
            }
        }

        Err(crate::exceptions::LangExtractError::parsing(
            format!("Could not parse response as JSON: {}", response)
        ))
    }

    /// Parse JSON response into extractions
    #[allow(dead_code)]
    fn parse_json_response(&self, json: &serde_json::Value) -> LangExtractResult<Vec<Extraction>> {
        let mut extractions = Vec::new();

        // Handle array at top level: [{"name": "John", "age": "25"}, ...]
        if let Some(array) = json.as_array() {
            for (index, item) in array.iter().enumerate() {
                extractions.extend(self.parse_single_item(item, Some(index))?);
            }
            return Ok(extractions);
        }

        // Handle object with data field: {"data": [{"name": "John"}, ...]}
        if let Some(obj) = json.as_object() {
            // Check for common wrapper fields
            if let Some(data_array) = obj.get("data").and_then(|v| v.as_array()) {
                for (index, item) in data_array.iter().enumerate() {
                    extractions.extend(self.parse_single_item(item, Some(index))?);
                }
                return Ok(extractions);
            }

            if let Some(results_array) = obj.get("results").and_then(|v| v.as_array()) {
                for (index, item) in results_array.iter().enumerate() {
                    extractions.extend(self.parse_single_item(item, Some(index))?);
                }
                return Ok(extractions);
            }

            // Handle flat JSON structure like {"name": "John", "age": "25"}
            extractions.extend(self.parse_single_item(json, None)?);
        }

        Ok(extractions)
    }

    /// Parse a single item (object or primitive) into extractions
    #[allow(dead_code)]
    fn parse_single_item(&self, item: &serde_json::Value, index: Option<usize>) -> LangExtractResult<Vec<Extraction>> {
        let mut extractions = Vec::new();

        if let Some(obj) = item.as_object() {
            for (key, value) in obj {
                // Skip null values
                if value.is_null() {
                    continue;
                }

                let extraction_text = if let Some(text) = value.as_str() {
                    text.to_string()
                } else if let Some(num) = value.as_number() {
                    num.to_string()
                } else if let Some(bool_val) = value.as_bool() {
                    bool_val.to_string()
                } else {
                    // For complex values, serialize as JSON
                    serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
                };

                // Create extraction class name - include index if we're processing an array
                let extraction_class = if let Some(idx) = index {
                    format!("{}_{}", key, idx)
                } else {
                    key.clone()
                };

                extractions.push(Extraction::new(extraction_class, extraction_text));
            }
        }

        Ok(extractions)
    }
}
