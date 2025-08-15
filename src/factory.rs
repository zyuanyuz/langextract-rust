//! Factory for creating language model instances.

use crate::{
    data::ExampleData,
    exceptions::{LangExtractError, LangExtractResult},
    inference::BaseLanguageModel,
    providers::{create_provider, ProviderConfig, ProviderType},
    ExtractConfig,
};
use std::env;

/// Create a language model based on configuration
pub async fn create_model(
    config: &ExtractConfig,
    examples: Option<&[ExampleData]>,
) -> LangExtractResult<Box<dyn BaseLanguageModel>> {
    // Determine provider type and configuration from the ExtractConfig
    let provider_config = create_provider_config(config)?;
    
    // Create the provider
    let mut provider = create_provider(provider_config)?;
    
    // Apply schema if examples are provided
    if let Some(example_data) = examples {
        if config.use_schema_constraints && !example_data.is_empty() {
            if let Some(schema_class) = provider.get_schema_class() {
                // For now, we'll use a basic schema
                provider.apply_schema(Some(schema_class));
            }
        }
    }
    
    // Set fence output preference
    provider.set_fence_output(config.fence_output);
    
    Ok(Box::new(provider))
}

/// Create provider configuration from ExtractConfig
fn create_provider_config(config: &ExtractConfig) -> LangExtractResult<ProviderConfig> {
    // Try to determine provider type from model_id
    let provider_type = determine_provider_type(&config.model_id)?;
    
    match provider_type {
        ProviderType::OpenAI => {
            let api_key = config.api_key.clone()
                .or_else(|| env::var("OPENAI_API_KEY").ok())
                .or_else(|| env::var("LANGEXTRACT_API_KEY").ok());
            
            let mut provider_config = ProviderConfig::openai(&config.model_id, api_key);
            
            if let Some(url) = &config.model_url {
                provider_config = provider_config.with_base_url(url.clone());
            }
            
            Ok(provider_config)
        }
        ProviderType::Ollama => {
            let base_url = config.model_url.clone()
                .or_else(|| env::var("OLLAMA_BASE_URL").ok())
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            
            Ok(ProviderConfig::ollama(&config.model_id, Some(base_url)))
        }
        ProviderType::Custom => {
            let base_url = config.model_url.as_ref()
                .ok_or_else(|| LangExtractError::configuration(
                    "model_url is required for custom providers"
                ))?;
            
            let mut provider_config = ProviderConfig::custom(base_url, &config.model_id);
            
            if let Some(api_key) = &config.api_key {
                provider_config = provider_config.with_api_key(api_key.clone());
            }
            
            Ok(provider_config)
        }
    }
}

/// Determine provider type from model ID
fn determine_provider_type(model_id: &str) -> LangExtractResult<ProviderType> {
    let model_lower = model_id.to_lowercase();
    
    if model_lower.contains("gpt") || model_lower.contains("openai") {
        Ok(ProviderType::OpenAI)
    } else if model_lower.contains("mistral") || model_lower.contains("llama") || 
             model_lower.contains("ollama") || model_lower.contains("codellama") {
        Ok(ProviderType::Ollama)
    } else {
        // Default to custom for unknown models
        Ok(ProviderType::Custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_provider_type() {
        assert_eq!(determine_provider_type("gpt-4").unwrap(), ProviderType::OpenAI);
        assert_eq!(determine_provider_type("gpt-3.5-turbo").unwrap(), ProviderType::OpenAI);
        assert_eq!(determine_provider_type("mistral").unwrap(), ProviderType::Ollama);
        assert_eq!(determine_provider_type("llama2").unwrap(), ProviderType::Ollama);
        assert_eq!(determine_provider_type("codellama").unwrap(), ProviderType::Ollama);
        assert_eq!(determine_provider_type("custom-model").unwrap(), ProviderType::Custom);
    }

    #[test]
    fn test_create_provider_config() {
        let config = ExtractConfig {
            model_id: "mistral".to_string(),
            api_key: None,
            model_url: Some("http://localhost:11434".to_string()),
            ..Default::default()
        };
        
        let provider_config = create_provider_config(&config).unwrap();
        assert_eq!(provider_config.provider_type, ProviderType::Ollama);
        assert_eq!(provider_config.model, "mistral");
        assert_eq!(provider_config.base_url, "http://localhost:11434");
    }
}
