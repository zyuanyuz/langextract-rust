//! Core data types for the annotation pipeline.
//!
//! This module defines the fundamental data structures used throughout the langextract
//! library, including documents, extractions, and configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Status indicating how well an extraction aligns with the source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlignmentStatus {
    /// Extraction matches the source text exactly
    MatchExact,
    /// Extraction text is longer than the source span
    MatchGreater,
    /// Extraction text is shorter than the source span
    MatchLesser,
    /// Extraction text approximately matches but with differences
    MatchFuzzy,
}

/// Represents a character interval in text
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharInterval {
    /// Starting position of the interval (inclusive)
    pub start_pos: Option<usize>,
    /// Ending position of the interval (exclusive)
    pub end_pos: Option<usize>,
}

impl CharInterval {
    /// Create a new character interval
    pub fn new(start_pos: Option<usize>, end_pos: Option<usize>) -> Self {
        Self { start_pos, end_pos }
    }

    /// Check if this interval overlaps with another
    pub fn overlaps_with(&self, other: &CharInterval) -> bool {
        match (self.start_pos, self.end_pos, other.start_pos, other.end_pos) {
            (Some(s1), Some(e1), Some(s2), Some(e2)) => {
                // Two intervals overlap if one starts before the other ends
                s1 < e2 && s2 < e1
            }
            _ => false, // If any position is None, consider no overlap
        }
    }

    /// Get the length of the interval
    pub fn length(&self) -> Option<usize> {
        match (self.start_pos, self.end_pos) {
            (Some(start), Some(end)) if end >= start => Some(end - start),
            _ => None,
        }
    }
}

/// Token interval information (placeholder for future tokenizer integration)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenInterval {
    /// Starting token index
    pub start_token: Option<usize>,
    /// Ending token index
    pub end_token: Option<usize>,
}

impl TokenInterval {
    /// Create a new token interval
    pub fn new(start_token: Option<usize>, end_token: Option<usize>) -> Self {
        Self {
            start_token,
            end_token,
        }
    }
}

/// Represents an extraction extracted from text
///
/// This struct encapsulates an extraction's characteristics and its position
/// within the source text. It can represent diverse information for NLP
/// information extraction tasks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extraction {
    /// The class or type of the extraction
    pub extraction_class: String,
    /// The extracted text content
    pub extraction_text: String,
    /// Character position in the original text
    pub char_interval: Option<CharInterval>,
    /// How well this extraction aligns with the source
    pub alignment_status: Option<AlignmentStatus>,
    /// Index of this extraction in the list
    pub extraction_index: Option<usize>,
    /// Group index for related extractions
    pub group_index: Option<usize>,
    /// Human-readable description
    pub description: Option<String>,
    /// Additional attributes as key-value pairs
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    /// Token position information
    #[serde(skip)]
    pub token_interval: Option<TokenInterval>,
}

impl Extraction {
    /// Create a new extraction with just the class and text
    pub fn new(extraction_class: String, extraction_text: String) -> Self {
        Self {
            extraction_class,
            extraction_text,
            char_interval: None,
            alignment_status: None,
            extraction_index: None,
            group_index: None,
            description: None,
            attributes: None,
            token_interval: None,
        }
    }
}

impl Default for Extraction {
    fn default() -> Self {
        Self {
            extraction_class: String::new(),
            extraction_text: String::new(),
            char_interval: None,
            alignment_status: None,
            extraction_index: None,
            group_index: None,
            description: None,
            attributes: None,
            token_interval: None,
        }
    }
}

impl Extraction {
    /// Create a new extraction with character interval
    pub fn with_char_interval(
        extraction_class: String,
        extraction_text: String,
        char_interval: CharInterval,
    ) -> Self {
        Self {
            extraction_class,
            extraction_text,
            char_interval: Some(char_interval),
            alignment_status: None,
            extraction_index: None,
            group_index: None,
            description: None,
            attributes: None,
            token_interval: None,
        }
    }

    /// Set the character interval for this extraction
    pub fn set_char_interval(&mut self, interval: CharInterval) {
        self.char_interval = Some(interval);
    }

    /// Set an attribute value
    pub fn set_attribute(&mut self, key: String, value: serde_json::Value) {
        if self.attributes.is_none() {
            self.attributes = Some(HashMap::new());
        }
        if let Some(attrs) = &mut self.attributes {
            attrs.insert(key, value);
        }
    }

    /// Get an attribute value
    pub fn get_attribute(&self, key: &str) -> Option<&serde_json::Value> {
        self.attributes.as_ref()?.get(key)
    }

    /// Check if this extraction overlaps with another based on character intervals
    pub fn overlaps_with(&self, other: &Extraction) -> bool {
        match (&self.char_interval, &other.char_interval) {
            (Some(interval1), Some(interval2)) => interval1.overlaps_with(interval2),
            _ => false,
        }
    }
}

/// Document class for input text
///
/// Represents a single document to be processed by the annotation pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    /// Raw text content of the document
    pub text: String,
    /// Optional additional context to supplement prompt instructions
    pub additional_context: Option<String>,
    /// Unique identifier for the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
}

impl Document {
    /// Create a new document with the given text
    pub fn new(text: String) -> Self {
        Self {
            text,
            additional_context: None,
            document_id: None,
        }
    }

    /// Create a new document with text and additional context
    pub fn with_context(text: String, additional_context: String) -> Self {
        Self {
            text,
            additional_context: Some(additional_context),
            document_id: None,
        }
    }

    /// Get or generate a document ID
    pub fn get_document_id(&mut self) -> String {
        if let Some(id) = &self.document_id {
            id.clone()
        } else {
            let id = format!("doc_{}", Uuid::new_v4().simple().to_string()[..8].to_string());
            self.document_id = Some(id.clone());
            id
        }
    }

    /// Set a specific document ID
    pub fn set_document_id(&mut self, id: String) {
        self.document_id = Some(id);
    }
}

/// Annotated document with extractions
///
/// Represents the result of processing a document through the annotation pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnotatedDocument {
    /// Unique identifier for the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// List of extractions found in the document
    pub extractions: Option<Vec<Extraction>>,
    /// Original text content
    pub text: Option<String>,
}

impl AnnotatedDocument {
    /// Create a new annotated document
    pub fn new() -> Self {
        Self {
            document_id: None,
            extractions: None,
            text: None,
        }
    }

    /// Create an annotated document with extractions and text
    pub fn with_extractions(extractions: Vec<Extraction>, text: String) -> Self {
        Self {
            document_id: None,
            extractions: Some(extractions),
            text: Some(text),
        }
    }

    /// Get or generate a document ID
    pub fn get_document_id(&mut self) -> String {
        if let Some(id) = &self.document_id {
            id.clone()
        } else {
            let id = format!("doc_{}", Uuid::new_v4().simple().to_string()[..8].to_string());
            self.document_id = Some(id.clone());
            id
        }
    }

    /// Set the document ID
    pub fn set_document_id(&mut self, id: String) {
        self.document_id = Some(id);
    }

    /// Add an extraction to this document
    pub fn add_extraction(&mut self, extraction: Extraction) {
        if self.extractions.is_none() {
            self.extractions = Some(Vec::new());
        }
        if let Some(extractions) = &mut self.extractions {
            extractions.push(extraction);
        }
    }

    /// Get the number of extractions
    pub fn extraction_count(&self) -> usize {
        self.extractions.as_ref().map_or(0, |e| e.len())
    }

    /// Get extractions of a specific class
    pub fn extractions_by_class(&self, class_name: &str) -> Vec<&Extraction> {
        self.extractions
            .as_ref()
            .map_or(Vec::new(), |extractions| {
                extractions
                    .iter()
                    .filter(|e| e.extraction_class == class_name)
                    .collect()
            })
    }
}

impl Default for AnnotatedDocument {
    fn default() -> Self {
        Self::new()
    }
}

/// Enumeration of supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FormatType {
    /// JSON output format
    Json,
    /// YAML output format
    Yaml,
}

impl std::fmt::Display for FormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatType::Json => write!(f, "json"),
            FormatType::Yaml => write!(f, "yaml"),
        }
    }
}

impl std::str::FromStr for FormatType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(FormatType::Json),
            "yaml" => Ok(FormatType::Yaml),
            _ => Err(format!("Invalid format type: {}", s)),
        }
    }
}

/// Example data for training/prompting
///
/// Represents a single training example that shows the model how to extract
/// information from text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExampleData {
    /// The raw input text (sentence, paragraph, etc.)
    pub text: String,
    /// List of extractions that should be found in this text
    pub extractions: Vec<Extraction>,
}

impl ExampleData {
    /// Create a new example with text and extractions
    pub fn new(text: String, extractions: Vec<Extraction>) -> Self {
        Self { text, extractions }
    }

    /// Create an example with just text (no extractions)
    pub fn with_text(text: String) -> Self {
        Self {
            text,
            extractions: Vec::new(),
        }
    }

    /// Add an extraction to this example
    pub fn add_extraction(&mut self, extraction: Extraction) {
        self.extractions.push(extraction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_char_interval_overlap() {
        let interval1 = CharInterval::new(Some(0), Some(5));
        let interval2 = CharInterval::new(Some(3), Some(8));
        let interval3 = CharInterval::new(Some(10), Some(15));

        assert!(interval1.overlaps_with(&interval2));
        assert!(interval2.overlaps_with(&interval1));
        assert!(!interval1.overlaps_with(&interval3));
        assert!(!interval3.overlaps_with(&interval1));
    }

    #[test]
    fn test_char_interval_length() {
        let interval = CharInterval::new(Some(5), Some(10));
        assert_eq!(interval.length(), Some(5));

        let interval_none = CharInterval::new(None, Some(10));
        assert_eq!(interval_none.length(), None);
    }

    #[test]
    fn test_extraction_creation() {
        let extraction = Extraction::new("person".to_string(), "John Doe".to_string());
        assert_eq!(extraction.extraction_class, "person");
        assert_eq!(extraction.extraction_text, "John Doe");
        assert!(extraction.char_interval.is_none());
    }

    #[test]
    fn test_extraction_attributes() {
        let mut extraction = Extraction::new("person".to_string(), "John Doe".to_string());
        extraction.set_attribute("age".to_string(), json!(30));
        extraction.set_attribute("city".to_string(), json!("New York"));

        assert_eq!(extraction.get_attribute("age"), Some(&json!(30)));
        assert_eq!(extraction.get_attribute("city"), Some(&json!("New York")));
        assert_eq!(extraction.get_attribute("nonexistent"), None);
    }

    #[test]
    fn test_extraction_overlap() {
        let mut extraction1 = Extraction::new("person".to_string(), "John".to_string());
        extraction1.set_char_interval(CharInterval::new(Some(0), Some(4)));

        let mut extraction2 = Extraction::new("name".to_string(), "John Doe".to_string());
        extraction2.set_char_interval(CharInterval::new(Some(2), Some(8)));

        let mut extraction3 = Extraction::new("city".to_string(), "Boston".to_string());
        extraction3.set_char_interval(CharInterval::new(Some(10), Some(16)));

        assert!(extraction1.overlaps_with(&extraction2));
        assert!(!extraction1.overlaps_with(&extraction3));
    }

    #[test]
    fn test_document_id_generation() {
        let mut doc = Document::new("Test text".to_string());
        let id1 = doc.get_document_id();
        let id2 = doc.get_document_id();

        assert_eq!(id1, id2); // Should be same ID when called multiple times
        assert!(id1.starts_with("doc_"));
        assert_eq!(id1.len(), 12); // "doc_" + 8 hex chars
    }

    #[test]
    fn test_annotated_document_operations() {
        let mut doc = AnnotatedDocument::new();
        assert_eq!(doc.extraction_count(), 0);

        let extraction1 = Extraction::new("person".to_string(), "Alice".to_string());
        let extraction2 = Extraction::new("person".to_string(), "Bob".to_string());
        let extraction3 = Extraction::new("location".to_string(), "Paris".to_string());

        doc.add_extraction(extraction1);
        doc.add_extraction(extraction2);
        doc.add_extraction(extraction3);

        assert_eq!(doc.extraction_count(), 3);

        let person_extractions = doc.extractions_by_class("person");
        assert_eq!(person_extractions.len(), 2);

        let location_extractions = doc.extractions_by_class("location");
        assert_eq!(location_extractions.len(), 1);
    }

    #[test]
    fn test_format_type_conversion() {
        assert_eq!("json".parse::<FormatType>().unwrap(), FormatType::Json);
        assert_eq!("yaml".parse::<FormatType>().unwrap(), FormatType::Yaml);
        assert_eq!("JSON".parse::<FormatType>().unwrap(), FormatType::Json);

        assert!(matches!("xml".parse::<FormatType>(), Err(_)));

        assert_eq!(FormatType::Json.to_string(), "json");
        assert_eq!(FormatType::Yaml.to_string(), "yaml");
    }

    #[test]
    fn test_example_data() {
        let mut example = ExampleData::with_text("John is 30 years old".to_string());
        assert_eq!(example.extractions.len(), 0);

        example.add_extraction(Extraction::new("person".to_string(), "John".to_string()));
        example.add_extraction(Extraction::new("age".to_string(), "30".to_string()));

        assert_eq!(example.extractions.len(), 2);
    }

    #[test]
    fn test_serialization() {
        let extraction = Extraction::new("person".to_string(), "John Doe".to_string());
        let json_str = serde_json::to_string(&extraction).unwrap();
        let deserialized: Extraction = serde_json::from_str(&json_str).unwrap();
        assert_eq!(extraction, deserialized);

        let doc = Document::new("Test text".to_string());
        let json_str = serde_json::to_string(&doc).unwrap();
        let deserialized: Document = serde_json::from_str(&json_str).unwrap();
        assert_eq!(doc, deserialized);
    }
}
