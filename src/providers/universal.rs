//! Universal provider implementation.

use super::config::{ProviderConfig, ProviderType};
use crate::{
    data::FormatType,
    exceptions::{LangExtractError, LangExtractResult},
    inference::{BaseLanguageModel, ScoredOutput},
    schema::BaseSchema,
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Universal language model provider
pub struct UniversalProvider {
    config: ProviderConfig,
    format_type: FormatType,
    client: reqwest::Client,
    #[cfg(feature = "openai")]
    openai_client: Option<async_openai::Client<async_openai::config::OpenAIConfig>>,
    schema: Option<Box<dyn BaseSchema>>,
    fence_output_override: Option<bool>,
}

impl UniversalProvider {
    /// Create a new universal provider
    pub fn new(config: ProviderConfig) -> LangExtractResult<Self> {
        let client = reqwest::Client::new();

        #[cfg(feature = "openai")]
        let openai_client = if config.provider_type == ProviderType::OpenAI {
            if let Some(api_key) = &config.api_key {
                let openai_config = async_openai::config::OpenAIConfig::new()
                    .with_api_key(api_key)
                    .with_api_base(&config.base_url);
                Some(async_openai::Client::with_config(openai_config))
            } else {
                return Err(LangExtractError::configuration(
                    "API key is required for OpenAI provider",
                ));
            }
        } else {
            None
        };

        #[cfg(not(feature = "openai"))]
        let openai_client = None;

        Ok(Self {
            config,
            format_type: FormatType::Json,
            client,
            openai_client,
            schema: None,
            fence_output_override: None,
        })
    }

    /// Inference implementation for OpenAI-compatible APIs
    #[cfg(feature = "openai")]
    async fn infer_openai(
        &self,
        batch_prompts: &[String],
        kwargs: &HashMap<String, serde_json::Value>,
    ) -> LangExtractResult<Vec<Vec<ScoredOutput>>> {
        use async_openai::types::{
            ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
            ChatCompletionRequestSystemMessageContent, CreateChatCompletionRequest,
        };

        let client = self.openai_client.as_ref().ok_or_else(|| {
            LangExtractError::configuration("OpenAI client not initialized")
        })?;

        let mut results = Vec::new();

        for prompt in batch_prompts {
            // Create system message for format instructions
            let system_message = match self.format_type {
                FormatType::Json => "You are a helpful assistant that responds in JSON format. Always return valid JSON that matches the expected structure from the examples.",
                FormatType::Yaml => "You are a helpful assistant that responds in YAML format. Always return valid YAML that matches the expected structure from the examples.",
            };

            // Create messages for the chat completion
            let messages = vec![
                ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage {
                    content: ChatCompletionRequestSystemMessageContent::Text(system_message.to_string()),
                    name: None,
                }),
                ChatCompletionRequestMessage::User(async_openai::types::ChatCompletionRequestUserMessage {
                    content: async_openai::types::ChatCompletionRequestUserMessageContent::Text(prompt.clone()),
                    name: None,
                }),
            ];

            // Build the request
            let mut request = CreateChatCompletionRequest {
                model: self.config.model.clone(),
                messages,
                temperature: None,
                max_tokens: None,
                ..Default::default()
            };

            // Apply parameters from kwargs
            if let Some(temp) = kwargs.get("temperature") {
                if let Some(temp_f64) = temp.as_f64() {
                    request.temperature = Some(temp_f64 as f32);
                }
            }
            
            if let Some(max_tokens) = kwargs.get("max_tokens") {
                if let Some(max_tokens_u64) = max_tokens.as_u64() {
                    request.max_tokens = Some(max_tokens_u64 as u32);
                }
            }

            // Make the API call - using the correct async-openai API
            let response = client.chat().create(request).await.map_err(|e| {
                LangExtractError::inference_simple(format!("OpenAI API error: {}", e))
            })?;

            // Extract the response content
            let content = response
                .choices
                .get(0)
                .and_then(|choice| choice.message.content.as_ref())
                .ok_or_else(|| {
                    LangExtractError::parsing("No content in OpenAI response")
                })?;

            results.push(vec![ScoredOutput::from_text(content.clone())]);
        }

        Ok(results)
    }

    /// Inference implementation for Ollama
    async fn infer_ollama(
        &self,
        batch_prompts: &[String],
        kwargs: &HashMap<String, serde_json::Value>,
    ) -> LangExtractResult<Vec<Vec<ScoredOutput>>> {
        let mut results = Vec::new();

        for prompt in batch_prompts {
            let mut request_body = serde_json::json!({
                "model": self.config.model,
                "prompt": prompt,
                "stream": false,
            });

            // Set format for JSON output if needed
            if self.format_type == FormatType::Json {
                request_body["format"] = serde_json::json!("json");
            }

            // Apply parameters from kwargs
            if let Some(options) = request_body.get_mut("options") {
                if let Some(temp) = kwargs.get("temperature") {
                    options["temperature"] = temp.clone();
                }
                if let Some(max_tokens) = kwargs.get("max_tokens") {
                    options["num_predict"] = max_tokens.clone();
                }
            } else {
                let mut options = serde_json::Map::new();
                if let Some(temp) = kwargs.get("temperature") {
                    options.insert("temperature".to_string(), temp.clone());
                }
                if let Some(max_tokens) = kwargs.get("max_tokens") {
                    options.insert("num_predict".to_string(), max_tokens.clone());
                }
                if !options.is_empty() {
                    request_body["options"] = serde_json::Value::Object(options);
                }
            }

            let url = format!("{}/api/generate", self.config.base_url);
            
            let mut request = self.client.post(&url).json(&request_body);

            // Add headers
            for (key, value) in &self.config.headers {
                request = request.header(key, value);
            }

            let response = request.send().await.map_err(|e| {
                LangExtractError::NetworkError(e)
            })?;

            if !response.status().is_success() {
                return Err(LangExtractError::inference_simple(format!(
                    "Ollama API error: HTTP {}",
                    response.status()
                )));
            }

            let response_body: serde_json::Value = response.json().await.map_err(|e| {
                LangExtractError::parsing(format!("Failed to parse Ollama response: {}", e))
            })?;

            let content = response_body
                .get("response")
                .and_then(|r| r.as_str())
                .ok_or_else(|| {
                    LangExtractError::parsing("Missing 'response' field in Ollama response")
                })?;

            results.push(vec![ScoredOutput::from_text(content.to_string())]);
        }

        Ok(results)
    }
}

#[async_trait]
impl BaseLanguageModel for UniversalProvider {
    fn get_schema_class(&self) -> Option<Box<dyn BaseSchema>> {
        // Return a format mode schema for now
        crate::schema::FormatModeSchema::from_examples(&[], "_attributes").ok()
    }

    fn apply_schema(&mut self, schema: Option<Box<dyn BaseSchema>>) {
        self.schema = schema;
    }

    fn set_fence_output(&mut self, fence_output: Option<bool>) {
        self.fence_output_override = fence_output;
    }

    fn requires_fence_output(&self) -> bool {
        if let Some(override_val) = self.fence_output_override {
            return override_val;
        }

        // OpenAI with JSON mode doesn't need fences, Ollama might
        match self.config.provider_type {
            ProviderType::OpenAI if self.schema.is_some() => false,
            _ => true,
        }
    }

    async fn infer(
        &self,
        batch_prompts: &[String],
        kwargs: &HashMap<String, serde_json::Value>,
    ) -> LangExtractResult<Vec<Vec<ScoredOutput>>> {
        match self.config.provider_type {
            #[cfg(feature = "openai")]
            ProviderType::OpenAI => self.infer_openai(batch_prompts, kwargs).await,
            ProviderType::Ollama => self.infer_ollama(batch_prompts, kwargs).await,
            ProviderType::Custom => {
                Err(LangExtractError::configuration(
                    "Custom provider inference not yet implemented"
                ))
            }
            #[cfg(not(feature = "openai"))]
            ProviderType::OpenAI => {
                Err(LangExtractError::configuration(
                    "OpenAI feature not enabled. Enable with --features openai"
                ))
            }
        }
    }

    fn format_type(&self) -> FormatType {
        self.format_type
    }

    fn model_id(&self) -> &str {
        &self.config.model
    }

    fn provider_name(&self) -> &str {
        match self.config.provider_type {
            ProviderType::OpenAI => "openai",
            ProviderType::Ollama => "ollama",
            ProviderType::Custom => "custom",
        }
    }
}
