//! Test OpenAI provider integration
//!
//! This example demonstrates how to use the OpenAI provider
//! with the langextract library.

use langextract_rust::{extract, ExampleData, Extraction, ExtractConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 Testing OpenAI Provider Integration\n");

    // Create example data to guide the extraction
    let examples = vec![
        ExampleData::new(
            "John Doe is 30 years old and works as a software engineer".to_string(),
            vec![
                Extraction::new("person".to_string(), "John Doe".to_string()),
                Extraction::new("age".to_string(), "30".to_string()),
                Extraction::new("profession".to_string(), "software engineer".to_string()),
            ],
        ),
    ];

    // Create extraction configuration for OpenAI with chunking support
    let provider_config = langextract_rust::ProviderConfig::openai("gpt-3.5-turbo", None);
    
    let mut config = ExtractConfig {
        model_id: "gpt-3.5-turbo".to_string(), // Use a more affordable model for testing
        // API key will be loaded from environment variable OPENAI_API_KEY
        api_key: None,
        temperature: 0.3,  // Lower temperature for more consistent results
        debug: true,
        
        // Token-based chunking configuration (text is small but shows capabilities)
        max_char_buffer: 1000,  // Characters per chunk (respects sentence boundaries)
        batch_length: 5,        // Process chunks in batches
        max_workers: 3,         // Concurrent API calls
        extraction_passes: 1,   // Single pass for simple example
        enable_multipass: false, // Can be enabled for complex documents
        
        ..Default::default()
    };
    
    // Set provider configuration
    config.language_model_params.insert(
        "provider_config".to_string(),
        serde_json::to_value(&provider_config)?,
    );

    // Text to extract information from
    let text = "Alice Smith is 28 years old and works as a data scientist at Google.";

    println!("Text to process: {}\n", text);
    println!("Model: {}", config.model_id);
    println!("Temperature: {}", config.temperature);

    println!("\nAttempting extraction with OpenAI...");

    match extract(
        text,
        Some("Extract person names, ages, and professions from the text"),
        &examples,
        config,
    ).await {
        Ok(result) => {
            println!("✅ Extraction successful!");
            println!("Found {} extractions", result.extraction_count());
            
            if let Some(extractions) = &result.extractions {
                for extraction in extractions {
                    println!(
                        "  - Class: {}, Text: '{}', Interval: {:?}",
                        extraction.extraction_class,
                        extraction.extraction_text,
                        extraction.char_interval
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Extraction failed: {}", e);
            println!("Error type: {:?}", e);
            
            // Print helpful debugging information
            println!("\n🔍 Debugging Information:");
            println!("- Check that OPENAI_API_KEY is set in your .env file");
            println!("- Verify your API key is valid and has sufficient credits");
            println!("- Ensure you have internet connectivity");
            
            return Err(e.into());
        }
    }

    Ok(())
}
