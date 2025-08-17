//! Factory for creating language model instances.

use crate::{
    data::ExampleData,
    exceptions::{LangExtractError, LangExtractResult},
    inference::BaseLanguageModel,
    providers::{create_provider, ProviderConfig},
    ExtractConfig,
};

#[cfg(test)]
use crate::providers::ProviderType;

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
    // Check if provider configuration is already specified in language_model_params
    if let Some(provider_config_value) = config.language_model_params.get("provider_config") {
        if let Ok(provider_config) = serde_json::from_value::<ProviderConfig>(provider_config_value.clone()) {
            return Ok(provider_config);
        }
    }
    
    // Provider configuration is required - no auto-detection
    Err(LangExtractError::configuration(
        "Provider configuration is required. Please specify a provider either:\n\
         1. Via CLI: --provider <openai|ollama|custom>\n\
         2. Via config: Set language_model_params['provider_config']\n\
         3. Via ProviderConfig in code\n\n\
         Auto-detection based on model names has been removed for explicit configuration."
    ))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explicit_provider_required() {
        let config = ExtractConfig {
            model_id: "mistral".to_string(),
            api_key: None,
            ..Default::default()
        };
        
        let result = create_provider_config(&config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Provider configuration is required"));
    }

    #[test]
    fn test_create_provider_config_with_explicit_config() {
        let provider_config = ProviderConfig::ollama("mistral", Some("http://localhost:11434".to_string()));
        
        let mut config = ExtractConfig {
            model_id: "mistral".to_string(),
            api_key: None,
            ..Default::default()
        };
        
        // Set explicit provider config
        config.language_model_params.insert(
            "provider_config".to_string(),
            serde_json::to_value(&provider_config).unwrap()
        );
        
        let result_config = create_provider_config(&config).unwrap();
        assert_eq!(result_config.provider_type, ProviderType::Ollama);
        assert_eq!(result_config.model, "mistral");
        assert_eq!(result_config.base_url, "http://localhost:11434");
    }
}
