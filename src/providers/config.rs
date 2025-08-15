//! Provider configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Provider type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    /// OpenAI-compatible API (OpenAI, Azure OpenAI, etc.)
    OpenAI,
    /// Ollama local server
    Ollama,
    /// Custom HTTP API
    Custom,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::OpenAI => write!(f, "openai"),
            ProviderType::Ollama => write!(f, "ollama"),
            ProviderType::Custom => write!(f, "custom"),
        }
    }
}

impl std::str::FromStr for ProviderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(ProviderType::OpenAI),
            "ollama" => Ok(ProviderType::Ollama),
            "custom" => Ok(ProviderType::Custom),
            _ => Err(format!("Unknown provider type: {}", s)),
        }
    }
}

/// Universal provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Type of provider
    pub provider_type: ProviderType,
    /// Base URL for the API
    pub base_url: String,
    /// Model identifier
    pub model: String,
    /// API key (if required)
    pub api_key: Option<String>,
    /// Additional headers
    pub headers: HashMap<String, String>,
    /// Provider-specific parameters
    pub extra_params: HashMap<String, serde_json::Value>,
}

impl ProviderConfig {
    /// Create a new OpenAI provider config
    pub fn openai(model: &str, api_key: Option<String>) -> Self {
        Self {
            provider_type: ProviderType::OpenAI,
            base_url: "https://api.openai.com/v1".to_string(),
            model: model.to_string(),
            api_key,
            headers: HashMap::new(),
            extra_params: HashMap::new(),
        }
    }

    /// Create an Azure OpenAI provider config
    pub fn azure_openai(
        resource_name: &str,
        deployment_name: &str,
        api_version: Option<&str>,
        api_key: Option<String>,
    ) -> Self {
        let api_version = api_version.unwrap_or("2024-02-15-preview");
        let base_url = format!("https://{}.openai.azure.com/openai/deployments/{}", 
            resource_name, deployment_name);
        
        let mut config = Self {
            provider_type: ProviderType::OpenAI,
            base_url,
            model: deployment_name.to_string(),
            api_key,
            headers: HashMap::new(),
            extra_params: HashMap::new(),
        };
        
        // Add Azure-specific headers
        config.headers.insert("api-version".to_string(), api_version.to_string());
        config
    }

    /// Create an OpenAI-compatible provider config (e.g., for OpenRouter, Together AI, etc.)
    pub fn openai_compatible(base_url: &str, model: &str, api_key: Option<String>) -> Self {
        Self {
            provider_type: ProviderType::OpenAI,
            base_url: base_url.to_string(),
            model: model.to_string(),
            api_key,
            headers: HashMap::new(),
            extra_params: HashMap::new(),
        }
    }

    /// Create a new Ollama provider config
    pub fn ollama(model: &str, base_url: Option<String>) -> Self {
        Self {
            provider_type: ProviderType::Ollama,
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
            model: model.to_string(),
            api_key: None,
            headers: HashMap::new(),
            extra_params: HashMap::new(),
        }
    }

    /// Create a custom provider config
    pub fn custom(base_url: &str, model: &str) -> Self {
        Self {
            provider_type: ProviderType::Custom,
            base_url: base_url.to_string(),
            model: model.to_string(),
            api_key: None,
            headers: HashMap::new(),
            extra_params: HashMap::new(),
        }
    }

    /// Set API key
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Add a header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Add an extra parameter
    pub fn with_extra_param(mut self, key: String, value: serde_json::Value) -> Self {
        self.extra_params.insert(key, value);
        self
    }

    /// Set base URL
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_conversion() {
        assert_eq!("openai".parse::<ProviderType>().unwrap(), ProviderType::OpenAI);
        assert_eq!("ollama".parse::<ProviderType>().unwrap(), ProviderType::Ollama);
        assert_eq!("OPENAI".parse::<ProviderType>().unwrap(), ProviderType::OpenAI);
        
        assert!(matches!("unknown".parse::<ProviderType>(), Err(_)));
        
        assert_eq!(ProviderType::OpenAI.to_string(), "openai");
        assert_eq!(ProviderType::Ollama.to_string(), "ollama");
    }

    #[test]
    fn test_provider_config_creation() {
        let openai_config = ProviderConfig::openai("gpt-4", Some("test-key".to_string()));
        assert_eq!(openai_config.provider_type, ProviderType::OpenAI);
        assert_eq!(openai_config.model, "gpt-4");
        assert_eq!(openai_config.api_key, Some("test-key".to_string()));
        assert_eq!(openai_config.base_url, "https://api.openai.com/v1");

        let ollama_config = ProviderConfig::ollama("mistral", None);
        assert_eq!(ollama_config.provider_type, ProviderType::Ollama);
        assert_eq!(ollama_config.model, "mistral");
        assert_eq!(ollama_config.base_url, "http://localhost:11434");
        assert!(ollama_config.api_key.is_none());
    }

    #[test]
    fn test_provider_config_builders() {
        let config = ProviderConfig::custom("https://api.example.com", "custom-model")
            .with_api_key("test-key".to_string())
            .with_header("Custom-Header".to_string(), "value".to_string())
            .with_extra_param("param1".to_string(), serde_json::json!("value1"));

        assert_eq!(config.provider_type, ProviderType::Custom);
        assert_eq!(config.api_key, Some("test-key".to_string()));
        assert_eq!(config.headers.get("Custom-Header"), Some(&"value".to_string()));
        assert_eq!(config.extra_params.get("param1"), Some(&serde_json::json!("value1")));
    }

    #[test]
    fn test_serialization() {
        let config = ProviderConfig::ollama("mistral", None);
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ProviderConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.provider_type, deserialized.provider_type);
        assert_eq!(config.model, deserialized.model);
    }
}
