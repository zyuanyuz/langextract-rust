//! Text chunking functionality for processing large documents.
//!
//! This module provides comprehensive text chunking capabilities to handle
//! documents that exceed the language model's context window. It supports
//! multiple chunking strategies and overlap management to ensure no information
//! is lost during processing.

use crate::{
    data::{AnnotatedDocument, CharInterval, Document, Extraction},
    exceptions::LangExtractResult,
};
use regex::Regex;

/// Different strategies for chunking text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkingStrategy {
    /// Fixed character-based chunking
    FixedSize,
    /// Split at sentence boundaries
    Sentence,
    /// Split at paragraph boundaries
    Paragraph,
    /// Adaptive chunking based on content structure
    Adaptive,
}

/// A chunk of text with metadata
#[derive(Debug, Clone)]
pub struct TextChunk {
    /// The chunk ID
    pub id: usize,
    /// Text content of the chunk
    pub text: String,
    /// Character offset from the beginning of the original document
    pub char_offset: usize,
    /// Length of the chunk in characters
    pub char_length: usize,
    /// Original document this chunk belongs to
    pub document_id: Option<String>,
    /// Whether this chunk overlaps with adjacent chunks
    pub has_overlap: bool,
    /// Overlap information (start and end overlap lengths)
    pub overlap_info: Option<(usize, usize)>,
}

impl TextChunk {
    /// Create a new text chunk
    pub fn new(
        id: usize,
        text: String,
        char_offset: usize,
        document_id: Option<String>,
    ) -> Self {
        let char_length = text.len();
        Self {
            id,
            text,
            char_offset,
            char_length,
            document_id,
            has_overlap: false,
            overlap_info: None,
        }
    }

    /// Create a chunk with overlap information
    pub fn with_overlap(
        id: usize,
        text: String,
        char_offset: usize,
        document_id: Option<String>,
        overlap_start: usize,
        overlap_end: usize,
    ) -> Self {
        let char_length = text.len();
        Self {
            id,
            text,
            char_offset,
            char_length,
            document_id,
            has_overlap: overlap_start > 0 || overlap_end > 0,
            overlap_info: Some((overlap_start, overlap_end)),
        }
    }

    /// Get the character interval for this chunk in the original document
    pub fn char_interval(&self) -> CharInterval {
        CharInterval::new(
            Some(self.char_offset),
            Some(self.char_offset + self.char_length),
        )
    }

    /// Get the core text without overlaps
    pub fn core_text(&self) -> &str {
        if let Some((start_overlap, end_overlap)) = self.overlap_info {
            let start = start_overlap;
            let end = self.text.len().saturating_sub(end_overlap);
            &self.text[start..end]
        } else {
            &self.text
        }
    }
}

/// Configuration for text chunking
#[derive(Debug, Clone)]
pub struct ChunkingConfig {
    /// Maximum characters per chunk
    pub max_chunk_size: usize,
    /// Overlap size in characters
    pub overlap_size: usize,
    /// Chunking strategy to use
    pub strategy: ChunkingStrategy,
    /// Minimum chunk size (chunks smaller than this will be merged)
    pub min_chunk_size: usize,
    /// Whether to respect paragraph boundaries
    pub respect_paragraphs: bool,
    /// Whether to respect sentence boundaries
    pub respect_sentences: bool,
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            max_chunk_size: 2000,
            overlap_size: 200,
            strategy: ChunkingStrategy::Adaptive,
            min_chunk_size: 100,
            respect_paragraphs: true,
            respect_sentences: true,
        }
    }
}

/// Text chunker for processing large documents
pub struct TextChunker {
    config: ChunkingConfig,
    sentence_regex: Regex,
    paragraph_regex: Regex,
}

impl TextChunker {
    /// Create a new text chunker with default configuration
    pub fn new() -> Self {
        Self::with_config(ChunkingConfig::default())
    }

    /// Create a new text chunker with custom configuration
    pub fn with_config(config: ChunkingConfig) -> Self {
        // Regex for sentence boundaries (basic implementation)
        let sentence_regex = Regex::new(r"[.!?]+\s+").unwrap();
        
        // Regex for paragraph boundaries
        let paragraph_regex = Regex::new(r"\n\s*\n").unwrap();

        Self {
            config,
            sentence_regex,
            paragraph_regex,
        }
    }

    /// Chunk a document into smaller pieces
    pub fn chunk_document(&self, document: &Document) -> LangExtractResult<Vec<TextChunk>> {
        self.chunk_text(&document.text, document.document_id.clone())
    }

    /// Chunk text into smaller pieces
    pub fn chunk_text(&self, text: &str, document_id: Option<String>) -> LangExtractResult<Vec<TextChunk>> {
        if text.len() <= self.config.max_chunk_size {
            // Text is small enough, return as single chunk
            return Ok(vec![TextChunk::new(0, text.to_string(), 0, document_id)]);
        }

        match self.config.strategy {
            ChunkingStrategy::FixedSize => self.chunk_fixed_size(text, document_id),
            ChunkingStrategy::Sentence => self.chunk_by_sentences(text, document_id),
            ChunkingStrategy::Paragraph => self.chunk_by_paragraphs(text, document_id),
            ChunkingStrategy::Adaptive => self.chunk_adaptive(text, document_id),
        }
    }

    /// Fixed-size chunking with overlap
    fn chunk_fixed_size(&self, text: &str, document_id: Option<String>) -> LangExtractResult<Vec<TextChunk>> {
        let mut chunks = Vec::new();
        let mut chunk_id = 0;
        let mut current_pos = 0;

        while current_pos < text.len() {
            let chunk_end = std::cmp::min(
                current_pos + self.config.max_chunk_size,
                text.len()
            );

            let chunk_text = text[current_pos..chunk_end].to_string();
            
            let overlap_start = if chunk_id > 0 { self.config.overlap_size } else { 0 };
            let overlap_end = if chunk_end < text.len() { self.config.overlap_size } else { 0 };

            let chunk = TextChunk::with_overlap(
                chunk_id,
                chunk_text,
                current_pos,
                document_id.clone(),
                overlap_start,
                overlap_end,
            );

            chunks.push(chunk);
            chunk_id += 1;

            // Move forward, accounting for overlap
            let step_size = self.config.max_chunk_size.saturating_sub(self.config.overlap_size);
            current_pos += step_size;
        }

        Ok(chunks)
    }

    /// Chunk by sentence boundaries
    fn chunk_by_sentences(&self, text: &str, document_id: Option<String>) -> LangExtractResult<Vec<TextChunk>> {
        let sentence_boundaries = self.find_sentence_boundaries(text);
        self.chunk_by_boundaries(text, &sentence_boundaries, document_id)
    }

    /// Chunk by paragraph boundaries  
    fn chunk_by_paragraphs(&self, text: &str, document_id: Option<String>) -> LangExtractResult<Vec<TextChunk>> {
        let paragraph_boundaries = self.find_paragraph_boundaries(text);
        self.chunk_by_boundaries(text, &paragraph_boundaries, document_id)
    }

    /// Adaptive chunking that respects natural boundaries
    fn chunk_adaptive(&self, text: &str, document_id: Option<String>) -> LangExtractResult<Vec<TextChunk>> {
        // First try paragraph boundaries
        let paragraph_boundaries = self.find_paragraph_boundaries(text);
        if !paragraph_boundaries.is_empty() && self.config.respect_paragraphs {
            if let Ok(chunks) = self.chunk_by_boundaries(text, &paragraph_boundaries, document_id.clone()) {
                // Check if any chunks are too large
                let oversized_chunks: Vec<_> = chunks.iter()
                    .filter(|c| c.char_length > self.config.max_chunk_size)
                    .collect();
                
                if oversized_chunks.is_empty() {
                    return Ok(chunks);
                }
            }
        }

        // Fall back to sentence boundaries
        if self.config.respect_sentences {
            let sentence_boundaries = self.find_sentence_boundaries(text);
            if let Ok(chunks) = self.chunk_by_boundaries(text, &sentence_boundaries, document_id.clone()) {
                let oversized_chunks: Vec<_> = chunks.iter()
                    .filter(|c| c.char_length > self.config.max_chunk_size)
                    .collect();
                
                if oversized_chunks.is_empty() {
                    return Ok(chunks);
                }
            }
        }

        // Final fallback to fixed-size chunking
        self.chunk_fixed_size(text, document_id)
    }

    /// Find sentence boundaries in text
    fn find_sentence_boundaries(&self, text: &str) -> Vec<usize> {
        let mut boundaries = vec![0]; // Start of text
        
        for mat in self.sentence_regex.find_iter(text) {
            boundaries.push(mat.end());
        }
        
        if boundaries.last() != Some(&text.len()) {
            boundaries.push(text.len()); // End of text
        }
        
        boundaries
    }

    /// Find paragraph boundaries in text
    fn find_paragraph_boundaries(&self, text: &str) -> Vec<usize> {
        let mut boundaries = vec![0]; // Start of text
        
        for mat in self.paragraph_regex.find_iter(text) {
            boundaries.push(mat.end());
        }
        
        if boundaries.last() != Some(&text.len()) {
            boundaries.push(text.len()); // End of text
        }
        
        boundaries
    }

    /// Chunk text based on provided boundaries
    fn chunk_by_boundaries(
        &self,
        text: &str,
        boundaries: &[usize],
        document_id: Option<String>,
    ) -> LangExtractResult<Vec<TextChunk>> {
        let mut chunks = Vec::new();
        let mut chunk_id = 0;
        let mut current_start = 0;

        for &boundary in boundaries.iter().skip(1) {
            let potential_chunk_size = boundary - current_start;
            
            // If the potential chunk is within size limits, use it
            if potential_chunk_size <= self.config.max_chunk_size {
                if potential_chunk_size >= self.config.min_chunk_size || chunks.is_empty() {
                    let chunk_text = text[current_start..boundary].to_string();
                    let chunk = TextChunk::new(chunk_id, chunk_text, current_start, document_id.clone());
                    chunks.push(chunk);
                    chunk_id += 1;
                    current_start = boundary;
                }
            } else {
                // Chunk is too large, need to split it further
                // For now, fall back to fixed-size chunking for this section
                let section = &text[current_start..boundary];
                let mut section_chunks = self.chunk_fixed_size(section, document_id.clone())?;
                
                // Adjust offsets
                for chunk in &mut section_chunks {
                    chunk.id = chunk_id;
                    chunk.char_offset += current_start;
                    chunk_id += 1;
                }
                
                chunks.extend(section_chunks);
                current_start = boundary;
            }
        }

        if chunks.is_empty() {
            // Fallback: create a single chunk with the entire text
            chunks.push(TextChunk::new(0, text.to_string(), 0, document_id));
        }

        Ok(chunks)
    }

    /// Get chunking configuration
    pub fn config(&self) -> &ChunkingConfig {
        &self.config
    }
}

impl Default for TextChunker {
    fn default() -> Self {
        Self::new()
    }
}

/// Result aggregator for combining extractions from multiple chunks
pub struct ResultAggregator {
    /// Similarity threshold for duplicate detection
    similarity_threshold: f32,
    /// Whether to merge overlapping extractions
    merge_overlaps: bool,
}

impl ResultAggregator {
    /// Create a new result aggregator
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.8,
            merge_overlaps: true,
        }
    }

    /// Create a result aggregator with custom settings
    pub fn with_settings(similarity_threshold: f32, merge_overlaps: bool) -> Self {
        Self {
            similarity_threshold,
            merge_overlaps,
        }
    }

    /// Aggregate results from multiple chunks into a single annotated document
    pub fn aggregate_chunk_results(
        &self,
        chunk_results: Vec<ChunkResult>,
        original_text: String,
        document_id: Option<String>,
    ) -> LangExtractResult<AnnotatedDocument> {
        let mut all_extractions = Vec::new();

        // Collect all extractions from chunks
        for chunk_result in chunk_results {
            if let Some(extractions) = chunk_result.extractions {
                // Character positions should already be adjusted by the alignment process
                // during chunk processing, so we don't need to add the offset again here
                all_extractions.extend(extractions);
            }
        }

        // Deduplicate and merge overlapping extractions
        let deduplicated_extractions = if self.merge_overlaps {
            self.deduplicate_extractions(all_extractions)?
        } else {
            all_extractions
        };

        // Create the aggregated document
        let mut annotated_doc = AnnotatedDocument::with_extractions(deduplicated_extractions, original_text);
        annotated_doc.document_id = document_id;

        Ok(annotated_doc)
    }

    /// Remove duplicate extractions based on similarity
    fn deduplicate_extractions(&self, extractions: Vec<Extraction>) -> LangExtractResult<Vec<Extraction>> {
        let mut unique_extractions = Vec::new();
        
        for extraction in extractions {
            let mut is_duplicate = false;
            
            // Check against existing extractions
            for existing in &unique_extractions {
                if self.are_similar_extractions(&extraction, existing) {
                    is_duplicate = true;
                    break;
                }
            }
            
            if !is_duplicate {
                unique_extractions.push(extraction);
            }
        }

        Ok(unique_extractions)
    }

    /// Check if two extractions are similar enough to be considered duplicates
    fn are_similar_extractions(&self, e1: &Extraction, e2: &Extraction) -> bool {
        // Same extraction class and similar text
        if e1.extraction_class == e2.extraction_class {
            let similarity = self.text_similarity(&e1.extraction_text, &e2.extraction_text);
            return similarity >= self.similarity_threshold;
        }

        // Check for overlapping character positions
        if let (Some(interval1), Some(interval2)) = (&e1.char_interval, &e2.char_interval) {
            if interval1.overlaps_with(interval2) {
                let similarity = self.text_similarity(&e1.extraction_text, &e2.extraction_text);
                return similarity >= self.similarity_threshold;
            }
        }

        false
    }

    /// Calculate simple text similarity (Jaccard similarity on words)
    fn text_similarity(&self, text1: &str, text2: &str) -> f32 {
        if text1 == text2 {
            return 1.0;
        }

        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();

        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }
}

impl Default for ResultAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result from processing a single chunk
#[derive(Debug, Clone)]
pub struct ChunkResult {
    /// ID of the chunk that was processed
    pub chunk_id: usize,
    /// Extractions found in this chunk
    pub extractions: Option<Vec<Extraction>>,
    /// Character offset of this chunk in the original document
    pub char_offset: usize,
    /// Length of the chunk
    pub char_length: usize,
    /// Whether processing was successful
    pub success: bool,
    /// Error message if processing failed
    pub error: Option<String>,
    /// Processing time for this chunk
    pub processing_time: Option<std::time::Duration>,
}

impl ChunkResult {
    /// Create a successful chunk result
    pub fn success(
        chunk_id: usize,
        extractions: Vec<Extraction>,
        char_offset: usize,
        char_length: usize,
    ) -> Self {
        Self {
            chunk_id,
            extractions: Some(extractions),
            char_offset,
            char_length,
            success: true,
            error: None,
            processing_time: None,
        }
    }

    /// Create a failed chunk result
    pub fn failure(
        chunk_id: usize,
        char_offset: usize,
        char_length: usize,
        error: String,
    ) -> Self {
        Self {
            chunk_id,
            extractions: None,
            char_offset,
            char_length,
            success: false,
            error: Some(error),
            processing_time: None,
        }
    }

    /// Set processing time
    pub fn with_processing_time(mut self, duration: std::time::Duration) -> Self {
        self.processing_time = Some(duration);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_size_chunking() {
        let chunker = TextChunker::with_config(ChunkingConfig {
            max_chunk_size: 20,
            overlap_size: 5,
            strategy: ChunkingStrategy::FixedSize,
            ..Default::default()
        });

        let text = "This is a test document with some text that needs to be chunked into smaller pieces.";
        let chunks = chunker.chunk_text(text, None).unwrap();

        assert!(chunks.len() > 1);
        for chunk in &chunks {
            assert!(chunk.char_length <= 20);
        }
    }

    #[test]
    fn test_sentence_chunking() {
        let chunker = TextChunker::with_config(ChunkingConfig {
            max_chunk_size: 50,
            strategy: ChunkingStrategy::Sentence,
            ..Default::default()
        });

        let text = "First sentence. Second sentence! Third sentence? Fourth sentence.";
        let chunks = chunker.chunk_text(text, None).unwrap();

        // Should have multiple chunks based on sentences
        assert!(chunks.len() > 0);
        for chunk in &chunks {
            println!("Chunk: '{}'", chunk.text);
        }
    }

    #[test]
    fn test_small_text_no_chunking() {
        let chunker = TextChunker::new();
        let text = "Short text.";
        let chunks = chunker.chunk_text(text, None).unwrap();

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, text);
    }

    #[test]
    fn test_chunk_char_interval() {
        let chunk = TextChunk::new(0, "test".to_string(), 10, None);
        let interval = chunk.char_interval();
        
        assert_eq!(interval.start_pos, Some(10));
        assert_eq!(interval.end_pos, Some(14));
    }

    #[test]
    fn test_chunk_with_overlap() {
        let chunk = TextChunk::with_overlap(
            0,
            "overlap test text".to_string(),
            0,
            None,
            3,
            4,
        );

        assert!(chunk.has_overlap);
        assert_eq!(chunk.overlap_info, Some((3, 4)));
        assert_eq!(chunk.core_text(), "rlap test ");
    }
}
