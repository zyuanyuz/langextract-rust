//! Provider configuration demonstration
//!
//! This example shows how to use the new agnostic provider system
//! with explicit configuration for different providers.

use langextract_rust::{
    extract, ExampleData, Extraction, ExtractConfig,
    ProviderConfig
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🚀 LangExtract Provider Configuration Demo\n");

    // Create some example data
    let examples = vec![
        ExampleData::new(
            "John Smith is 25 years old".to_string(),
            vec![
                Extraction::new("name".to_string(), "John Smith".to_string()),
                Extraction::new("age".to_string(), "25".to_string()),
            ],
        ),
    ];

    let test_text = "Alice Johnson is 30 years old and Bob Wilson is 45";

    println!("Test text: {}\n", test_text);

    // Demo 1: Ollama provider (local)
    println!("📋 1. Ollama Provider Configuration");
    let ollama_config = ProviderConfig::ollama("mistral", None)
        .with_base_url("http://localhost:11434".to_string());
    
    println!("   Provider Type: {}", ollama_config.provider_type);
    println!("   Base URL: {}", ollama_config.base_url);
    println!("   Model: {}", ollama_config.model);
    println!("   API Key Required: {}", ollama_config.api_key.is_some());

    let extract_config = ExtractConfig {
        model_id: "mistral".to_string(),
        model_url: Some("http://localhost:11434".to_string()),
        ..Default::default()
    };

    println!("   Testing extraction...");
    match extract(test_text, Some("Extract names and ages"), &examples, extract_config).await {
        Ok(result) => {
            println!("   ✅ Success: {} extractions", result.extraction_count());
        }
        Err(e) => {
            println!("   ⚠️  Expected failure (Ollama not running): {}", e);
        }
    }
    println!();

    // Demo 2: OpenAI provider (simulated)
    println!("📋 2. OpenAI Provider Configuration");
    let openai_config = ProviderConfig::openai("gpt-4", Some("fake-key".to_string()))
        .with_base_url("https://api.openai.com/v1".to_string());
    
    println!("   Provider Type: {}", openai_config.provider_type);
    println!("   Base URL: {}", openai_config.base_url);
    println!("   Model: {}", openai_config.model);
    println!("   API Key Required: {}", openai_config.api_key.is_some());

    let extract_config = ExtractConfig {
        model_id: "gpt-4".to_string(),
        api_key: Some("fake-key".to_string()),
        ..Default::default()
    };

    println!("   Testing extraction...");
    match extract(test_text, Some("Extract names and ages"), &examples, extract_config).await {
        Ok(result) => {
            println!("   ✅ Success: {} extractions", result.extraction_count());
            if let Some(extractions) = &result.extractions {
                for extraction in extractions {
                    println!("      - {}: {}", extraction.extraction_class, extraction.extraction_text);
                }
            }
        }
        Err(e) => {
            println!("   ⚠️  Expected failure (fake API key): {}", e);
        }
    }
    println!();

    // Demo 3: Custom provider
    println!("📋 3. Custom Provider Configuration");
    let custom_config = ProviderConfig::custom("https://my-custom-api.com/v1", "my-model")
        .with_api_key("custom-key".to_string())
        .with_header("X-Custom-Header".to_string(), "custom-value".to_string());
    
    println!("   Provider Type: {}", custom_config.provider_type);
    println!("   Base URL: {}", custom_config.base_url);
    println!("   Model: {}", custom_config.model);
    println!("   Headers: {:?}", custom_config.headers);

    let extract_config = ExtractConfig {
        model_id: "my-model".to_string(),
        model_url: Some("https://my-custom-api.com/v1".to_string()),
        api_key: Some("custom-key".to_string()),
        ..Default::default()
    };

    println!("   Testing extraction...");
    match extract(test_text, Some("Extract names and ages"), &examples, extract_config).await {
        Ok(result) => {
            println!("   ✅ Success: {} extractions", result.extraction_count());
        }
        Err(e) => {
            println!("   ⚠️  Expected failure (custom endpoint not real): {}", e);
        }
    }
    println!();

    // Demo 4: Show that explicit provider configuration is required
    println!("📋 4. Explicit Provider Configuration Required");
    
    let config = ExtractConfig {
        model_id: "mistral".to_string(),
        ..Default::default()
    };
    
    println!("   Trying to extract without explicit provider configuration...");
    match extract(test_text, Some("Extract names and ages"), &examples, config).await {
        Ok(_) => {
            println!("   ❌ This should not have succeeded!");
        }
        Err(e) => {
            println!("   ✅ Expected failure: Provider configuration is required");
            if e.to_string().contains("Provider configuration is required") {
                println!("   ✅ Correct error message displayed");
            }
        }
    }

    println!("\n🎉 Provider demo complete!");
    println!("\n💡 Key Benefits of Explicit Provider Configuration:");
    println!("   • No magic model-name-based auto-detection");
    println!("   • Explicit provider configuration required");
    println!("   • Support for custom base URLs");
    println!("   • Flexible header and parameter passing");
    println!("   • Easy to extend with new providers");
    println!("   • More predictable and debuggable behavior");

    Ok(())
}
