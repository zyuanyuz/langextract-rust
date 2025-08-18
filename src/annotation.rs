//! Text annotation functionality.

use crate::{
    alignment::TextAligner,
    chunking::{ChunkResult, ChunkingConfig, ResultAggregator, TextChunk, TextChunker},
    data::{AnnotatedDocument, Extraction, FormatType},
    exceptions::LangExtractResult,
    inference::BaseLanguageModel,
    prompting::PromptTemplateStructured,
    resolver::Resolver,
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

        // Text is too large, use chunking
        if debug {
            println!("🔧 DEBUG: Text length ({} chars) exceeds buffer limit ({} chars), using chunking", 
                text.len(), max_char_buffer);
        }

        self.process_chunked_text(
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
        
        // Always show language model call progress
        println!("🤖 Calling {} model: {} ({} chars input)", 
            self.language_model.provider_name(),
            self.language_model.model_id(),
            text.len());
        
        if debug {
            println!("🔧 DEBUG: Calling language model with prompt:");
            println!("   Model: {}", self.language_model.model_id());
            println!("   Provider: {}", self.language_model.provider_name());
            println!("   Prompt: {}", &prompt.chars().take(200).collect::<String>());
            if prompt.len() > 200 {
                println!("   ... (truncated, total length: {} chars)", prompt.len());
            }
        }

        // Create inference parameters
        let mut kwargs = HashMap::new();
        kwargs.insert("temperature".to_string(), serde_json::json!(0.2));
        kwargs.insert("max_tokens".to_string(), serde_json::json!(1000));

        // Call the language model
        let results = self.language_model.infer(&[prompt], &kwargs).await?;
        
        println!("📥 Received response from language model");
        if debug {
            println!("🔧 DEBUG: Received {} batches from language model", results.len());
        }

        // Extract the response
        let mut annotated_doc = AnnotatedDocument::with_extractions(Vec::new(), text.to_string());
        
        if let Some(batch) = results.first() {
            if let Some(output) = batch.first() {
                let response_text = output.text();
                
                if debug {
                    println!("🔧 DEBUG: Raw response from model:");
                    println!("   {}", response_text);
                }

                // Extract expected fields from examples for validation
                let expected_fields: Vec<String> = self.prompt_template.examples
                    .iter()
                    .flat_map(|example| example.extractions.iter())
                    .map(|extraction| extraction.extraction_class.clone())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();

                // Use new validation system with raw data preservation
                match resolver.validate_and_parse(response_text, &expected_fields) {
                    Ok((mut extractions, validation_result)) => {
                        // Report validation results
                        if debug {
                            println!("🔧 DEBUG: Validation result: valid={}, errors={}, warnings={}", 
                                validation_result.is_valid, 
                                validation_result.errors.len(),
                                validation_result.warnings.len());
                            
                            if let Some(raw_file) = &validation_result.raw_output_file {
                                println!("🔧 DEBUG: Raw output saved to: {}", raw_file);
                            }

                            for error in &validation_result.errors {
                                println!("🔧 DEBUG: Validation error: {}", error.message);
                            }
                            for warning in &validation_result.warnings {
                                println!("🔧 DEBUG: Validation warning: {}", warning.message);
                            }
                        }

                        // Align extractions with the source text
                        let aligner = TextAligner::new();
                        let aligned_count = aligner.align_extractions(&mut extractions, text, 0)
                            .unwrap_or(0);
                        
                        annotated_doc.extractions = Some(extractions);
                        if debug {
                            println!("🔧 DEBUG: Successfully parsed {} extractions ({} aligned)", 
                                   annotated_doc.extraction_count(), aligned_count);
                        }
                    }
                    Err(e) => {
                        if debug {
                            println!("🔧 DEBUG: Failed to parse response as structured data: {}", e);
                            println!("   Treating as unstructured response");
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
    async fn process_chunked_text(
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
        // Create chunker with appropriate configuration
        let chunking_config = ChunkingConfig {
            max_chunk_size: max_char_buffer,
            overlap_size: max_char_buffer / 10, // 10% overlap
            ..Default::default()
        };
        let chunker = TextChunker::with_config(chunking_config);

        // Chunk the text
        let chunks = chunker.chunk_text(text, None)?;
        
        // Always show chunking progress for user feedback
        println!("📄 Processing document with {} chunks ({} chars total)", chunks.len(), text.len());
        if debug {
            for (i, chunk) in chunks.iter().enumerate() {
                println!("   Chunk {}: {} chars (offset: {})", i, chunk.char_length, chunk.char_offset);
            }
        }

        // Process chunks in parallel batches
        let mut chunk_results = Vec::new();
        let effective_workers = std::cmp::min(max_workers, batch_length);
        let total_chunks = chunks.len();
        let mut processed_chunks = 0;

        for (batch_idx, chunk_batch) in chunks.chunks(batch_length).enumerate() {
            // Progress reporting for each batch
            println!("🔄 Processing batch {} ({}/{} chunks processed)", 
                batch_idx + 1, processed_chunks, total_chunks);

            let batch_futures: Vec<_> = chunk_batch.iter()
                .take(effective_workers)
                .map(|chunk| self.process_chunk(chunk, resolver, additional_context, debug))
                .collect();

            let batch_results = join_all(batch_futures).await;
            
            for result in batch_results {
                chunk_results.push(result?);
            }

            processed_chunks += chunk_batch.len();
            println!("✅ Completed batch {} ({}/{} chunks done)", 
                batch_idx + 1, processed_chunks, total_chunks);
            
            if debug {
                println!("🔧 DEBUG: Batch {} processing details completed", batch_idx + 1);
            }
        }

        // Handle multiple extraction passes
        if extraction_passes > 1 {
            if debug {
                println!("🔧 DEBUG: Running {} additional extraction passes", extraction_passes - 1);
            }
            
            // TODO: Implement multi-pass extraction
            // For now, we just use the single pass results
        }

        // Aggregate results
        println!("🔄 Aggregating results from {} chunks...", chunks.len());
        let aggregator = ResultAggregator::new();
        let final_result = aggregator.aggregate_chunk_results(
            chunk_results,
            text.to_string(),
            None,
        )?;

        println!("🎯 Extraction complete! Found {} total extractions", 
            final_result.extraction_count());
        
        if debug {
            println!("🔧 DEBUG: Aggregated {} total extractions from {} chunks", 
                final_result.extraction_count(), chunks.len());
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
                    println!("🔧 DEBUG: Chunk {} produced {} extractions ({} aligned)", 
                        chunk.id, extractions.len(), aligned_count);
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
                    println!("🔧 DEBUG: Chunk {} failed: {}", chunk.id, e);
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
