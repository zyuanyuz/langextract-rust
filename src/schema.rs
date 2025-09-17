//! Schema definitions and abstractions for structured prompt outputs.

use crate::{data::ExampleData, exceptions::LangExtractResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shared key for extraction arrays in JSON/YAML
pub const EXTRACTIONS_KEY: &str = "extractions";

/// Attributes for
pub const ATTRIBUTES_SUFFIX:&str = "_attributes";

/// Enumeration of constraint types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintType {
    None,
}

/// Represents a constraint for model output decoding
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constraint {
    /// The type of constraint applied
    pub constraint_type: ConstraintType,
}

impl Default for Constraint {
    fn default() -> Self {
        Self {
            constraint_type: ConstraintType::None,
        }
    }
}

impl Constraint {
    /// Create a new constraint with no restrictions
    pub fn none() -> Self {
        Self::default()
    }
}

/// Abstract base trait for generating structured constraints from examples
pub trait BaseSchema: Send + Sync {
    /// Factory method to build a schema instance from example data
    fn from_examples(
        examples_data: &[ExampleData],
        attribute_suffix: &str,
    ) -> LangExtractResult<Box<dyn BaseSchema>>
    where
        Self: Sized;

    /// Convert schema to provider-specific configuration
    ///
    /// Returns a dictionary of provider kwargs (e.g., response_schema for Gemini).
    /// Should be a pure data mapping with no side effects.
    fn to_provider_config(&self) -> HashMap<String, serde_json::Value>;

    /// Whether the provider emits valid output without needing Markdown fences
    ///
    /// Returns true when the provider will emit syntactically valid JSON (or other
    /// machine-parseable format) without needing Markdown fences. This says
    /// nothing about attribute-level schema enforcement.
    fn supports_strict_mode(&self) -> bool;

    /// Hook to update schema state based on provider kwargs
    ///
    /// This allows schemas to adjust their behavior based on caller overrides.
    /// For example, FormatModeSchema uses this to sync its format when the caller
    /// overrides it, ensuring supports_strict_mode stays accurate.
    fn sync_with_provider_kwargs(&mut self, kwargs: &HashMap<String, serde_json::Value>) {
        // Default implementation does nothing
        let _ = kwargs;
    }

    /// Clone this schema instance
    fn clone_box(&self) -> Box<dyn BaseSchema>;
}

/// Generic schema for providers that support format modes (JSON/YAML)
///
/// This schema doesn't enforce structure, only output format. Useful for
/// providers that can guarantee syntactically valid JSON or YAML but don't
/// support field-level constraints.
#[derive(Debug, Clone)]
pub struct FormatModeSchema {
    format: String,
}

impl FormatModeSchema {
    /// Initialize with a format mode
    pub fn new(format_mode: &str) -> Self {
        Self {
            format: format_mode.to_string(),
        }
    }

    /// Get the current format
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Set the format
    pub fn set_format(&mut self, format: String) {
        self.format = format;
    }
}

impl BaseSchema for FormatModeSchema {
    fn from_examples(
        _examples_data: &[ExampleData],
        _attribute_suffix: &str,
    ) -> LangExtractResult<Box<dyn BaseSchema>> {
        // Since format mode doesn't use examples for constraints,
        // this simply returns a JSON-mode instance
        Ok(Box::new(Self::new("json")))
    }

    fn to_provider_config(&self) -> HashMap<String, serde_json::Value> {
        let mut config = HashMap::new();
        config.insert("format".to_string(), serde_json::json!(self.format));
        config
    }

    fn supports_strict_mode(&self) -> bool {
        // JSON guarantees valid syntax, others may not
        self.format == "json"
    }

    fn sync_with_provider_kwargs(&mut self, kwargs: &HashMap<String, serde_json::Value>) {
        if let Some(format_value) = kwargs.get("format") {
            if let Some(format_str) = format_value.as_str() {
                self.format = format_str.to_string();
            }
        }
    }

    fn clone_box(&self) -> Box<dyn BaseSchema> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{Extraction, ExampleData};

    #[test]
    fn test_constraint_creation() {
        let constraint = Constraint::none();
        assert_eq!(constraint.constraint_type, ConstraintType::None);

        let default_constraint = Constraint::default();
        assert_eq!(default_constraint.constraint_type, ConstraintType::None);
    }

    #[test]
    fn test_format_mode_schema() {
        let mut schema = FormatModeSchema::new("json");
        assert_eq!(schema.format(), "json");
        assert!(schema.supports_strict_mode());

        schema.set_format("yaml".to_string());
        assert_eq!(schema.format(), "yaml");
        assert!(!schema.supports_strict_mode());
    }

    #[test]
    fn test_format_mode_schema_provider_config() {
        let schema = FormatModeSchema::new("json");
        let config = schema.to_provider_config();
        assert_eq!(config.get("format"), Some(&serde_json::json!("json")));
    }

    #[test]
    fn test_format_mode_schema_sync() {
        let mut schema = FormatModeSchema::new("json");
        
        let mut kwargs = HashMap::new();
        kwargs.insert("format".to_string(), serde_json::json!("yaml"));
        
        schema.sync_with_provider_kwargs(&kwargs);
        assert_eq!(schema.format(), "yaml");
        assert!(!schema.supports_strict_mode());
    }

    #[test]
    fn test_format_mode_schema_from_examples() {
        let examples = vec![ExampleData::new(
            "Test text".to_string(),
            vec![Extraction::new("test".to_string(), "value".to_string())],
        )];

        let schema = FormatModeSchema::from_examples(&examples, ATTRIBUTES_SUFFIX).unwrap();
        assert!(schema.supports_strict_mode()); // Should default to JSON
    }

    #[test]
    fn test_constraint_serialization() {
        let constraint = Constraint::none();
        let json = serde_json::to_string(&constraint).unwrap();
        let deserialized: Constraint = serde_json::from_str(&json).unwrap();
        assert_eq!(constraint, deserialized);
    }
}
