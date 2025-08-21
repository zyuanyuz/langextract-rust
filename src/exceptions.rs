//! Error types and result definitions for LangExtract.
//!
//! This module provides a comprehensive error handling system for the langextract
//! library, with specific error types for different failure modes.

use thiserror::Error;

/// Result type alias for LangExtract operations
pub type LangExtractResult<T> = Result<T, LangExtractError>;

/// Base error type for all LangExtract operations
///
/// This enum covers all possible error conditions that can occur
/// during text extraction and processing.
#[derive(Error, Debug)]
pub enum LangExtractError {
    /// Configuration-related errors (missing API keys, invalid model IDs, etc.)
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Runtime inference errors (API failures, network issues, etc.)
    #[error("Inference error: {message}")]
    InferenceError {
        message: String,
        provider: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Invalid input provided to the library
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Network-related errors (URL download failures, etc.)
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// JSON/YAML parsing errors
    #[error("Parsing error: {0}")]
    ParsingError(String),

    /// I/O errors (file operations, etc.)
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Text processing errors (chunking, alignment, etc.)
    #[error("Processing error: {0}")]
    ProcessingError(String),

    /// Tokenization errors
    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    /// Chunking errors
    #[error("Chunking error: {0}")]
    ChunkingError(String),

    /// Visualization errors
    #[error("Visualization error: {0}")]
    VisualizationError(String),

    /// Generic error for unexpected conditions
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl LangExtractError {
    /// Create a new configuration error
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::ConfigurationError(message.into())
    }

    /// Create a new inference error with provider information
    pub fn inference<S: Into<String>>(
        message: S,
        provider: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::InferenceError {
            message: message.into(),
            provider,
            source,
        }
    }

    /// Create a new inference error without provider information
    pub fn inference_simple<S: Into<String>>(message: S) -> Self {
        Self::InferenceError {
            message: message.into(),
            provider: None,
            source: None,
        }
    }

    /// Create a new invalid input error
    pub fn invalid_input<S: Into<String>>(message: S) -> Self {
        Self::InvalidInput(message.into())
    }

    /// Create a new parsing error
    pub fn parsing<S: Into<String>>(message: S) -> Self {
        Self::ParsingError(message.into())
    }

    /// Create a new serialization error
    pub fn serialization<S: Into<String>>(message: S) -> Self {
        Self::SerializationError(message.into())
    }

    /// Create a new processing error
    pub fn processing<S: Into<String>>(message: S) -> Self {
        Self::ProcessingError(message.into())
    }

    /// Create a new tokenization error
    pub fn tokenization<S: Into<String>>(message: S) -> Self {
        Self::TokenizationError(message.into())
    }

    /// Create a new chunking error
    pub fn chunking<S: Into<String>>(message: S) -> Self {
        Self::ChunkingError(message.into())
    }

    /// Create a new visualization error
    pub fn visualization<S: Into<String>>(message: S) -> Self {
        Self::VisualizationError(message.into())
    }

    /// Create a new unexpected error
    pub fn unexpected<S: Into<String>>(message: S) -> Self {
        Self::UnexpectedError(message.into())
    }

    /// Get the provider name if this is an inference error
    pub fn provider(&self) -> Option<&str> {
        match self {
            Self::InferenceError { provider, .. } => provider.as_deref(),
            _ => None,
        }
    }

    /// Check if this error is related to configuration
    pub fn is_configuration_error(&self) -> bool {
        matches!(self, Self::ConfigurationError(_))
    }

    /// Check if this error is related to inference
    pub fn is_inference_error(&self) -> bool {
        matches!(self, Self::InferenceError { .. })
    }

    /// Check if this error is related to network issues
    pub fn is_network_error(&self) -> bool {
        matches!(self, Self::NetworkError(_))
    }

    /// Check if this error is related to parsing
    pub fn is_parsing_error(&self) -> bool {
        matches!(self, Self::ParsingError(_))
    }
}

// Convert from serde JSON errors
impl From<serde_json::Error> for LangExtractError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(format!("JSON error: {}", err))
    }
}

// Convert from serde YAML errors
impl From<serde_yaml::Error> for LangExtractError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerializationError(format!("YAML error: {}", err))
    }
}

// Convert from environment variable errors
impl From<dotenvy::Error> for LangExtractError {
    fn from(err: dotenvy::Error) -> Self {
        Self::ConfigurationError(format!("Environment variable error: {}", err))
    }
}

/// Specialized error for inference operations
#[derive(Error, Debug)]
pub enum InferenceError {
    /// No scored outputs available from the language model
    #[error("No outputs available: {message}")]
    NoOutputsAvailable { message: String },

    /// API rate limit exceeded
    #[error("Rate limit exceeded for provider {provider}")]
    RateLimitExceeded { provider: String },

    /// Invalid model configuration
    #[error("Invalid model configuration: {message}")]
    InvalidConfiguration { message: String },

    /// Model not found or unsupported
    #[error("Model not found: {model_id}")]
    ModelNotFound { model_id: String },

    /// Authentication failed
    #[error("Authentication failed for provider {provider}: {message}")]
    AuthenticationFailed { provider: String, message: String },

    /// Quota exceeded
    #[error("Quota exceeded for provider {provider}")]
    QuotaExceeded { provider: String },

    /// Service temporarily unavailable
    #[error("Service unavailable for provider {provider}")]
    ServiceUnavailable { provider: String },

    /// Generic inference failure
    #[error("Inference failed: {message}")]
    InferenceFailed { message: String },
}

impl From<InferenceError> for LangExtractError {
    fn from(err: InferenceError) -> Self {
        let provider = match &err {
            InferenceError::RateLimitExceeded { provider } => Some(provider.clone()),
            InferenceError::AuthenticationFailed { provider, .. } => Some(provider.clone()),
            InferenceError::QuotaExceeded { provider } => Some(provider.clone()),
            InferenceError::ServiceUnavailable { provider } => Some(provider.clone()),
            _ => None,
        };

        Self::InferenceError {
            message: err.to_string(),
            provider,
            source: Some(Box::new(err)),
        }
    }
}

/// Specialized error for resolver operations
#[derive(Error, Debug)]
pub enum ResolverError {
    /// Failed to parse model output
    #[error("Failed to parse output: {message}")]
    ParseError { message: String },

    /// Invalid output format
    #[error("Invalid output format: expected {expected}, got {actual}")]
    InvalidFormat { expected: String, actual: String },

    /// Missing required fields in output
    #[error("Missing required fields: {fields:?}")]
    MissingFields { fields: Vec<String> },

    /// Schema validation failed
    #[error("Schema validation failed: {message}")]
    SchemaValidationFailed { message: String },
}

impl From<ResolverError> for LangExtractError {
    fn from(err: ResolverError) -> Self {
        Self::ParsingError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let config_err = LangExtractError::configuration("Missing API key");
        assert!(config_err.is_configuration_error());
        assert!(!config_err.is_inference_error());

        let inference_err = LangExtractError::inference(
            "Model failed",
            Some("gemini".to_string()),
            None,
        );
        assert!(inference_err.is_inference_error());
        assert_eq!(inference_err.provider(), Some("gemini"));

        let parsing_err = LangExtractError::parsing("Invalid JSON");
        assert!(parsing_err.is_parsing_error());
    }

    #[test]
    fn test_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json");
        assert!(json_error.is_err());
        
        let lang_error: LangExtractError = json_error.unwrap_err().into();
        assert!(lang_error.is_parsing_error() || matches!(lang_error, LangExtractError::SerializationError(_)));
    }

    #[test]
    fn test_inference_error_conversion() {
        let inference_err = InferenceError::ModelNotFound {
            model_id: "unknown-model".to_string(),
        };
        
        let lang_err: LangExtractError = inference_err.into();
        assert!(lang_err.is_inference_error());
    }

    #[test]
    fn test_error_display() {
        let error = LangExtractError::configuration("Test error message");
        let display = format!("{}", error);
        assert!(display.contains("Configuration error"));
        assert!(display.contains("Test error message"));
    }

    #[test]
    fn test_resolver_error_conversion() {
        let resolver_err = ResolverError::ParseError {
            message: "Invalid JSON structure".to_string(),
        };
        
        let lang_err: LangExtractError = resolver_err.into();
        assert!(lang_err.is_parsing_error());
    }
}
