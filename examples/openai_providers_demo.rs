//! Demo for different OpenAI-compatible providers
//!
//! This example shows how to configure the library for:
//! - Standard OpenAI API
//! - Azure OpenAI
//! - Other OpenAI-compatible services (OpenRouter, Together AI, etc.)

use langextract_rust::{
    extract, ExampleData, Extraction, ExtractConfig,
    ProviderConfig, 
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("🚀 OpenAI-Compatible Providers Demo\n");

    // Create example data
    let examples = vec![
        ExampleData::new(
            "John Smith is 25 years old and works as an engineer".to_string(),
            vec![
                Extraction::new("name".to_string(), "John Smith".to_string()),
                Extraction::new("age".to_string(), "25".to_string()),
                Extraction::new("job".to_string(), "engineer".to_string()),
            ],
        ),
    ];

    let test_text = "Alice Johnson is 30 years old and works as a data scientist";

    println!("Test text: {}\n", test_text);

    // Demo 1: Standard OpenAI
    println!("📋 1. Standard OpenAI Configuration");
    let openai_config = ProviderConfig::openai("gpt-3.5-turbo", None);
    
    println!("   Provider Type: {}", openai_config.provider_type);
    println!("   Base URL: {}", openai_config.base_url);
    println!("   Model: {}", openai_config.model);
    println!("   Headers: {:?}", openai_config.headers);
    
    let extract_config = ExtractConfig {
        model_id: "gpt-3.5-turbo".to_string(),
        api_key: None, // Will load from .env
        ..Default::default()
    };

    println!("   Testing extraction...");
    match extract(test_text, Some("Extract names, ages, and jobs"), &examples, extract_config).await {
        Ok(result) => {
            println!("   ✅ Success: {} extractions", result.extraction_count());
            if let Some(extractions) = &result.extractions {
                for extraction in extractions.iter().take(3) {
                    println!("      - {}: {}", extraction.extraction_class, extraction.extraction_text);
                }
            }
        }
        Err(e) => {
            println!("   ⚠️  Error (expected without API key): {}", e);
        }
    }
    println!();

    // Demo 2: Azure OpenAI
    println!("📋 2. Azure OpenAI Configuration");
    let azure_config = ProviderConfig::azure_openai(
        "my-resource",      // Azure resource name
        "gpt-35-turbo",     // Deployment name
        Some("2024-02-15-preview"), // API version
        Some("fake-key".to_string())
    );
    
    println!("   Provider Type: {}", azure_config.provider_type);
    println!("   Base URL: {}", azure_config.base_url);
    println!("   Model: {}", azure_config.model);
    println!("   Headers: {:?}", azure_config.headers);
    println!("   (Note: This is a demo configuration, not tested)");
    println!();

    // Demo 3: OpenRouter (OpenAI-compatible)
    println!("📋 3. OpenRouter Configuration");
    let openrouter_config = ProviderConfig::openai_compatible(
        "https://openrouter.ai/api/v1",
        "anthropic/claude-3-haiku",
        Some("fake-key".to_string())
    );
    
    println!("   Provider Type: {}", openrouter_config.provider_type);
    println!("   Base URL: {}", openrouter_config.base_url);
    println!("   Model: {}", openrouter_config.model);
    println!("   (Note: This is a demo configuration, not tested)");
    println!();

    // Demo 4: Together AI (OpenAI-compatible)
    println!("📋 4. Together AI Configuration");
    let together_config = ProviderConfig::openai_compatible(
        "https://api.together.xyz/v1",
        "meta-llama/Llama-2-7b-chat-hf",
        Some("fake-key".to_string())
    );
    
    println!("   Provider Type: {}", together_config.provider_type);
    println!("   Base URL: {}", together_config.base_url);
    println!("   Model: {}", together_config.model);
    println!("   (Note: This is a demo configuration, not tested)");
    println!();

    // Demo 5: Custom Headers (for special authentication)
    println!("📋 5. Custom Headers Example");
    let mut custom_config = ProviderConfig::openai_compatible(
        "https://api.custom-provider.com/v1",
        "custom-model",
        Some("api-key".to_string())
    );
    
    // Add custom headers
    custom_config = custom_config
        .with_header("Authorization".to_string(), "Bearer custom-token".to_string())
        .with_header("X-Organization".to_string(), "my-org".to_string());
    
    println!("   Provider Type: {}", custom_config.provider_type);
    println!("   Base URL: {}", custom_config.base_url);
    println!("   Model: {}", custom_config.model);
    println!("   Headers: {:?}", custom_config.headers);
    println!();

    println!("🎉 Provider configuration demo complete!");
    println!("\n💡 Key Benefits:");
    println!("   • Support for all major OpenAI-compatible services");
    println!("   • Easy Azure OpenAI integration");
    println!("   • Flexible header and authentication options");
    println!("   • Consistent API across all providers");
    println!("   • Same chunking and processing features for all providers");

    Ok(())
}
