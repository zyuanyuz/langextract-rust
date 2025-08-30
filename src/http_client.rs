//! HTTP client utilities for LangExtract providers.
//!
//! This module provides common HTTP functionality to reduce duplication
//! across different provider implementations.

use crate::{
    exceptions::{LangExtractError, LangExtractResult},
    logging::{report_progress, ProgressEvent},
};
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::Duration;

/// Configuration for HTTP requests
#[derive(Debug, Clone)]
pub struct HttpConfig {
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum number of retries
    pub max_retries: usize,
    /// Base delay between retries in seconds
    pub base_delay_seconds: u64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
    /// Custom headers to include in requests
    pub headers: HashMap<String, String>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 120,
            max_retries: 3,
            base_delay_seconds: 30,
            exponential_backoff: true,
            headers: HashMap::new(),
        }
    }
}

/// HTTP client with retry logic and progress reporting
pub struct HttpClient {
    client: reqwest::Client,
    config: HttpConfig,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration
    pub fn new() -> Self {
        Self::with_config(HttpConfig::default())
    }

    /// Create a new HTTP client with custom configuration
    pub fn with_config(config: HttpConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { client, config }
    }

    /// POST request with JSON body and retry logic
    pub async fn post_json_with_retry<T>(
        &self,
        url: &str,
        body: &T,
        operation_name: &str,
    ) -> LangExtractResult<Value>
    where
        T: serde::Serialize,
    {
        self.retry_with_backoff(
            || async {
                self.post_json_single(url, body).await
            },
            operation_name,
        ).await
    }

    /// Single POST request with JSON body
    async fn post_json_single<T>(&self, url: &str, body: &T) -> LangExtractResult<Value>
    where
        T: serde::Serialize,
    {
        let mut request = self.client.post(url).json(body);

        // Add custom headers
        for (key, value) in &self.config.headers {
            request = request.header(key, value);
        }

        let response = request.send().await.map_err(|e| {
            report_progress(ProgressEvent::Error {
                operation: "HTTP request".to_string(),
                error: format!("Request failed: {}", e),
            });
            LangExtractError::NetworkError(e)
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let status_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            report_progress(ProgressEvent::Error {
                operation: "HTTP response".to_string(),
                error: format!("HTTP {} - {}", status, status_text),
            });
            
            return Err(LangExtractError::inference_simple(format!(
                "HTTP error {}: {}",
                status,
                status_text
            )));
        }

        let response_body: Value = response.json().await.map_err(|e| {
            report_progress(ProgressEvent::Error {
                operation: "JSON parsing".to_string(),
                error: format!("Failed to parse response: {}", e),
            });
            LangExtractError::parsing(format!("Failed to parse JSON response: {}", e))
        })?;

        Ok(response_body)
    }

    /// Retry helper function with exponential backoff
    async fn retry_with_backoff<T, F, Fut>(
        &self,
        mut operation: F,
        operation_name: &str,
    ) -> LangExtractResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = LangExtractResult<T>>,
    {
        let max_retries = self.config.max_retries;
        let base_delay = Duration::from_secs(self.config.base_delay_seconds);

        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_retries {
                        // Last attempt failed, return the error
                        return Err(LangExtractError::inference_simple(
                            format!("{} failed after {} attempts. Last error: {}", 
                                operation_name, max_retries + 1, e)
                        ));
                    }

                    // Calculate delay for next attempt
                    let delay = if self.config.exponential_backoff {
                        base_delay * (attempt + 1) as u32
                    } else {
                        base_delay
                    };

                    report_progress(ProgressEvent::RetryAttempt {
                        operation: operation_name.to_string(),
                        attempt: attempt + 1,
                        max_attempts: max_retries + 1,
                        delay_seconds: delay.as_secs(),
                    });

                    // Sleep before retry
                    tokio::time::sleep(delay).await;
                }
            }
        }

        unreachable!("Should have returned from the loop")
    }

    /// Add a header to all requests
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.config.headers.insert(key, value);
        self
    }

    /// Set authentication header
    pub fn with_auth_header(self, auth_type: &str, token: &str) -> Self {
        self.with_header("Authorization".to_string(), format!("{} {}", auth_type, token))
    }

    /// Set bearer token authentication
    pub fn with_bearer_token(self, token: &str) -> Self {
        self.with_auth_header("Bearer", token)
    }

    /// Set API key header
    pub fn with_api_key(self, key: &str) -> Self {
        self.with_header("X-API-Key".to_string(), key.to_string())
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Provider-specific HTTP client builders
impl HttpClient {
    /// Create HTTP client configured for OpenAI
    pub fn for_openai(api_key: &str) -> Self {
        Self::new()
            .with_bearer_token(api_key)
            .with_header("Content-Type".to_string(), "application/json".to_string())
    }

    /// Create HTTP client configured for Ollama
    pub fn for_ollama() -> Self {
        Self::with_config(HttpConfig {
            timeout_seconds: 300, // Longer timeout for local inference
            max_retries: 2,       // Fewer retries for local
            base_delay_seconds: 5, // Shorter delays for local
            ..Default::default()
        })
        .with_header("Content-Type".to_string(), "application/json".to_string())
    }

    /// Create HTTP client for custom providers
    pub fn for_custom_provider(api_key: Option<&str>) -> Self {
        let mut client = Self::with_config(HttpConfig {
            timeout_seconds: 180,
            max_retries: 3,
            base_delay_seconds: 15,
            ..Default::default()
        })
        .with_header("Content-Type".to_string(), "application/json".to_string());

        if let Some(key) = api_key {
            client = client.with_bearer_token(key);
        }

        client
    }
}

/// Common request/response utilities
pub struct RequestBuilder;

impl RequestBuilder {
    /// Build OpenAI-compatible chat completion request
    pub fn openai_chat_completion(
        model: &str,
        messages: Vec<serde_json::Value>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> serde_json::Value {
        let mut request = serde_json::json!({
            "model": model,
            "messages": messages,
        });

        if let Some(temp) = temperature {
            request["temperature"] = serde_json::json!(temp);
        }

        if let Some(tokens) = max_tokens {
            request["max_tokens"] = serde_json::json!(tokens);
        }

        request
    }

    /// Build Ollama generate request
    pub fn ollama_generate(
        model: &str,
        prompt: &str,
        temperature: Option<f32>,
        options: Option<&serde_json::Value>,
    ) -> serde_json::Value {
        let mut request = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
        });

        if let Some(temp) = temperature {
            request["options"] = serde_json::json!({
                "temperature": temp
            });
        }

        if let Some(opts) = options {
            if let Some(existing_opts) = request.get_mut("options") {
                // Merge options
                if let (Some(existing_map), Some(new_map)) = (existing_opts.as_object_mut(), opts.as_object()) {
                    for (key, value) in new_map {
                        existing_map.insert(key.clone(), value.clone());
                    }
                }
            } else {
                request["options"] = opts.clone();
            }
        }

        request
    }

    /// Create OpenAI system message
    pub fn openai_system_message(content: &str) -> serde_json::Value {
        serde_json::json!({
            "role": "system",
            "content": content
        })
    }

    /// Create OpenAI user message
    pub fn openai_user_message(content: &str) -> serde_json::Value {
        serde_json::json!({
            "role": "user",
            "content": content
        })
    }
}

/// Response parser utilities
pub struct ResponseParser;

impl ResponseParser {
    /// Extract text content from OpenAI response
    pub fn openai_response_text(response: &Value) -> LangExtractResult<String> {
        response
            .get("choices")
            .and_then(|choices| choices.as_array())
            .and_then(|arr| arr.first())
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| LangExtractError::parsing("Invalid OpenAI response format"))
    }

    /// Extract text content from Ollama response
    pub fn ollama_response_text(response: &Value) -> LangExtractResult<String> {
        response
            .get("response")
            .and_then(|r| r.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| LangExtractError::parsing("Missing 'response' field in Ollama response"))
    }

    /// Generic response text extractor that tries common fields
    pub fn generic_response_text(response: &Value) -> LangExtractResult<String> {
        // Try common response field names
        let common_fields = ["response", "text", "content", "output", "result"];
        
        for field in &common_fields {
            if let Some(text) = response.get(field).and_then(|v| v.as_str()) {
                return Ok(text.to_string());
            }
        }

        // Try nested structures
        if let Some(data) = response.get("data") {
            return Self::generic_response_text(data);
        }

        Err(LangExtractError::parsing("Could not extract text from response"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_config_default() {
        let config = HttpConfig::default();
        assert_eq!(config.timeout_seconds, 120);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.base_delay_seconds, 30);
        assert!(config.exponential_backoff);
    }

    #[test]
    fn test_request_builder_openai() {
        let messages = vec![
            RequestBuilder::openai_system_message("You are helpful"),
            RequestBuilder::openai_user_message("Hello"),
        ];
        
        let request = RequestBuilder::openai_chat_completion(
            "gpt-4",
            messages,
            Some(0.7),
            Some(100),
        );

        assert_eq!(request["model"], "gpt-4");
        assert_eq!(request["temperature"], 0.7);
        assert_eq!(request["max_tokens"], 100);
        assert!(request["messages"].is_array());
    }

    #[test]
    fn test_request_builder_ollama() {
        let request = RequestBuilder::ollama_generate(
            "mistral",
            "Hello world",
            Some(0.5),
            None,
        );

        assert_eq!(request["model"], "mistral");
        assert_eq!(request["prompt"], "Hello world");
        assert_eq!(request["stream"], false);
        assert_eq!(request["options"]["temperature"], 0.5);
    }

    #[test]
    fn test_response_parser_openai() {
        let response = serde_json::json!({
            "choices": [{
                "message": {
                    "content": "Hello, world!"
                }
            }]
        });

        let text = ResponseParser::openai_response_text(&response).unwrap();
        assert_eq!(text, "Hello, world!");
    }

    #[test]
    fn test_response_parser_ollama() {
        let response = serde_json::json!({
            "response": "Hello from Ollama!"
        });

        let text = ResponseParser::ollama_response_text(&response).unwrap();
        assert_eq!(text, "Hello from Ollama!");
    }

    #[test]
    fn test_response_parser_generic() {
        let response1 = serde_json::json!({
            "text": "Generic response"
        });

        let response2 = serde_json::json!({
            "data": {
                "content": "Nested response"
            }
        });

        assert_eq!(ResponseParser::generic_response_text(&response1).unwrap(), "Generic response");
        assert_eq!(ResponseParser::generic_response_text(&response2).unwrap(), "Nested response");
    }
}
