//! Language model inference abstractions and implementations.
//!
//! This module provides the core abstraction for language model inference,
//! including the base trait that all providers must implement.

use crate::{data::FormatType, exceptions::LangExtractResult, schema::BaseSchema};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A scored output from a language model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoredOutput {
    /// Confidence score for this output (if available)
    pub score: Option<f32>,
    /// The generated text output
    pub output: Option<String>,
}

impl ScoredOutput {
    /// Create a new scored output
    pub fn new(output: String, score: Option<f32>) -> Self {
        Self {
            output: Some(output),
            score,
        }
    }

    /// Create a scored output with just text (no score)
    pub fn from_text(output: String) -> Self {
        Self {
            output: Some(output),
            score: None,
        }
    }

    /// Get the output text, returning empty string if None
    pub fn text(&self) -> &str {
        self.output.as_deref().unwrap_or("")
    }

    /// Check if this output has a score
    pub fn has_score(&self) -> bool {
        self.score.is_some()
    }
}

impl fmt::Display for ScoredOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let score_str = match self.score {
            Some(score) => format!("{:.2}", score),
            None => "-".to_string(),
        };

        match &self.output {
            Some(output) => {
                writeln!(f, "Score: {}", score_str)?;
                writeln!(f, "Output:")?;
                for line in output.lines() {
                    writeln!(f, "  {}", line)?;
                }
                Ok(())
            }
            None => write!(f, "Score: {}\nOutput: None", score_str),
        }
    }
}

/// Abstract base trait for language model inference
///
/// All language model providers must implement this trait to be compatible
/// with the langextract framework.
#[async_trait]
pub trait BaseLanguageModel: Send + Sync {
    /// Get the schema class this provider supports
    fn get_schema_class(&self) -> Option<Box<dyn BaseSchema>> {
        None
    }

    /// Apply a schema instance to this provider
    fn apply_schema(&mut self, _schema: Option<Box<dyn BaseSchema>>) {
        // Default implementation does nothing
    }

    /// Set explicit fence output preference
    fn set_fence_output(&mut self, _fence_output: Option<bool>) {
        // Default implementation does nothing
    }

    /// Whether this model requires fence output for parsing
    fn requires_fence_output(&self) -> bool {
        true // Conservative default
    }

    /// Perform inference on a batch of prompts
    ///
    /// # Arguments
    ///
    /// * `batch_prompts` - Batch of input prompts for inference
    /// * `kwargs` - Additional inference parameters (temperature, max_tokens, etc.)
    ///
    /// # Returns
    ///
    /// A vector of batches, where each batch contains scored outputs for one prompt
    async fn infer(
        &self,
        batch_prompts: &[String],
        kwargs: &std::collections::HashMap<String, serde_json::Value>,
    ) -> LangExtractResult<Vec<Vec<ScoredOutput>>>;

    /// Convenience method for single prompt inference
    async fn infer_single(
        &self,
        prompt: &str,
        kwargs: &std::collections::HashMap<String, serde_json::Value>,
    ) -> LangExtractResult<Vec<ScoredOutput>> {
        let results = self.infer(&[prompt.to_string()], kwargs).await?;
        Ok(results.into_iter().next().unwrap_or_default())
    }

    /// Parse model output as JSON or YAML
    ///
    /// This expects raw JSON/YAML without code fences.
    /// Code fence extraction is handled by the resolver.
    fn parse_output(&self, output: &str) -> LangExtractResult<serde_json::Value> {
        // Default implementation tries JSON first, then YAML
        match serde_json::from_str(output) {
            Ok(value) => Ok(value),
            Err(_) => {
                // Try YAML if JSON fails
                match serde_yaml::from_str::<serde_yaml::Value>(output) {
                    Ok(value) => {
                        // Convert YAML value to JSON value for consistency
                        let json_str = serde_json::to_string(&value)?;
                        Ok(serde_json::from_str(&json_str)?)
                    }
                    Err(e) => Err(crate::exceptions::LangExtractError::parsing(format!(
                        "Failed to parse output as JSON or YAML: {}",
                        e
                    ))),
                }
            }
        }
    }

    /// Get the format type this model uses
    fn format_type(&self) -> FormatType {
        FormatType::Json // Default to JSON
    }

    /// Get the model ID/name
    fn model_id(&self) -> &str;

    /// Get the provider name
    fn provider_name(&self) -> &str;

    /// Get supported model IDs for this provider
    fn supported_models() -> Vec<&'static str>
    where
        Self: Sized,
    {
        vec![]
    }

    /// Check if this provider supports a given model ID
    fn supports_model(model_id: &str) -> bool
    where
        Self: Sized,
    {
        Self::supported_models()
            .iter()
            .any(|&supported| model_id.contains(supported))
    }
}

/// Error type for inference operations that don't produce any outputs
#[derive(Debug, thiserror::Error)]
#[error("No scored outputs available from the language model: {message}")]
pub struct InferenceOutputError {
    pub message: String,
}

impl InferenceOutputError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

/// Inference configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Sampling temperature (0.0 to 1.0)
    pub temperature: f32,
    /// Maximum number of tokens to generate
    pub max_tokens: Option<usize>,
    /// Number of candidate outputs to generate
    pub num_candidates: usize,
    /// Stop sequences to halt generation
    pub stop_sequences: Vec<String>,
    /// Additional provider-specific parameters
    pub extra_params: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            temperature: 0.5,
            max_tokens: None,
            num_candidates: 1,
            stop_sequences: vec![],
            extra_params: std::collections::HashMap::new(),
        }
    }
}

impl InferenceConfig {
    /// Create a new inference config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 1.0);
        self
    }

    /// Set the maximum number of tokens
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set the number of candidate outputs
    pub fn with_num_candidates(mut self, num_candidates: usize) -> Self {
        self.num_candidates = num_candidates.max(1);
        self
    }

    /// Add a stop sequence
    pub fn with_stop_sequence(mut self, stop_sequence: String) -> Self {
        self.stop_sequences.push(stop_sequence);
        self
    }

    /// Add an extra parameter
    pub fn with_extra_param(mut self, key: String, value: serde_json::Value) -> Self {
        self.extra_params.insert(key, value);
        self
    }

    /// Convert to a HashMap for passing to inference methods
    pub fn to_hashmap(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let mut map = std::collections::HashMap::new();
        map.insert("temperature".to_string(), serde_json::json!(self.temperature));
        
        if let Some(max_tokens) = self.max_tokens {
            map.insert("max_tokens".to_string(), serde_json::json!(max_tokens));
        }
        
        map.insert("num_candidates".to_string(), serde_json::json!(self.num_candidates));
        
        if !self.stop_sequences.is_empty() {
            map.insert("stop_sequences".to_string(), serde_json::json!(self.stop_sequences));
        }

        // Add extra parameters
        for (key, value) in &self.extra_params {
            map.insert(key.clone(), value.clone());
        }

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scored_output_creation() {
        let output = ScoredOutput::new("Hello world".to_string(), Some(0.9));
        assert_eq!(output.text(), "Hello world");
        assert!(output.has_score());
        assert_eq!(output.score, Some(0.9));

        let output_no_score = ScoredOutput::from_text("Hello world".to_string());
        assert_eq!(output_no_score.text(), "Hello world");
        assert!(!output_no_score.has_score());
    }

    #[test]
    fn test_scored_output_display() {
        let output = ScoredOutput::new("Hello\nworld".to_string(), Some(0.85));
        let display = format!("{}", output);
        assert!(display.contains("Score: 0.85"));
        assert!(display.contains("  Hello"));
        assert!(display.contains("  world"));

        let output_no_score = ScoredOutput::from_text("Test".to_string());
        let display = format!("{}", output_no_score);
        assert!(display.contains("Score: -"));
    }

    #[test]
    fn test_inference_config() {
        let config = InferenceConfig::new()
            .with_temperature(0.7)
            .with_max_tokens(100)
            .with_num_candidates(3)
            .with_stop_sequence("END".to_string())
            .with_extra_param("custom_param".to_string(), serde_json::json!("value"));

        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_tokens, Some(100));
        assert_eq!(config.num_candidates, 3);
        assert_eq!(config.stop_sequences, vec!["END"]);

        let hashmap = config.to_hashmap();
        assert_eq!(hashmap.get("temperature"), Some(&serde_json::json!(0.7f32)));
        assert_eq!(hashmap.get("max_tokens"), Some(&serde_json::json!(100)));
        assert_eq!(hashmap.get("custom_param"), Some(&serde_json::json!("value")));
    }

    #[test]
    fn test_temperature_clamping() {
        let config = InferenceConfig::new().with_temperature(1.5);
        assert_eq!(config.temperature, 1.0);

        let config = InferenceConfig::new().with_temperature(-0.5);
        assert_eq!(config.temperature, 0.0);
    }

    #[test]
    fn test_serialization() {
        let output = ScoredOutput::new("test".to_string(), Some(0.5));
        let json = serde_json::to_string(&output).unwrap();
        let deserialized: ScoredOutput = serde_json::from_str(&json).unwrap();
        assert_eq!(output, deserialized);

        let config = InferenceConfig::new().with_temperature(0.8);
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: InferenceConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.temperature, deserialized.temperature);
    }
}
