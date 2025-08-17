//! Test example for Ollama provider with mistral model
//!
//! This example demonstrates how to use the langextract library
//! with a local Ollama instance running the mistral model.
//!
//! Prerequisites:
//! 1. Install Ollama: https://ollama.ai/
//! 2. Run: ollama pull mistral
//! 3. Start Ollama server (usually runs automatically)

use langextract_rust::{extract, ExampleData, Extraction, ExtractConfig, FormatType, ProviderConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("Testing LangExtract with Ollama/Mistral...\n");

    // Create example data to guide the extraction
    let examples = vec![
        ExampleData::new(
            "John Doe is 30 years old and works as a software engineer in San Francisco".to_string(),
            vec![
                Extraction::new("person_name".to_string(), "John Doe".to_string()),
                Extraction::new("age".to_string(), "30".to_string()),
                Extraction::new("job_title".to_string(), "software engineer".to_string()),
                Extraction::new("location".to_string(), "San Francisco".to_string()),
            ],
        ),
        ExampleData::new(
            "Dr. Sarah Wilson, 45, is a cardiologist at Stanford Hospital".to_string(),
            vec![
                Extraction::new("person_name".to_string(), "Dr. Sarah Wilson".to_string()),
                Extraction::new("age".to_string(), "45".to_string()),
                Extraction::new("job_title".to_string(), "cardiologist".to_string()),
                Extraction::new("location".to_string(), "Stanford Hospital".to_string()),
            ],
        ),
    ];

    // Create provider configuration for Ollama
    let provider_config = ProviderConfig::ollama("mistral", Some("http://localhost:11434".to_string()));
    
    // Create extraction configuration for Ollama/Mistral
    let mut config = ExtractConfig {
        model_id: "mistral".to_string(),
        api_key: None,  // Not needed for Ollama
        model_url: Some("http://localhost:11434".to_string()),  // Default Ollama port
        format_type: FormatType::Json,
        temperature: 0.2,  // Lower temperature for more consistent results
        use_schema_constraints: false,  // Disable for initial testing
        fence_output: Some(false),  // Let Ollama return raw JSON
        debug: true,
        extraction_passes: 1,
        max_char_buffer: 2000,
        batch_length: 5,
        max_workers: 2,
        ..Default::default()
    };

    // Set explicit provider configuration
    config.language_model_params.insert(
        "provider_config".to_string(),
        serde_json::to_value(&provider_config)?,
    );

    // Test text to extract information from
    let test_text = "Alice Johnson is a 28-year-old data scientist working at Google in Mountain View. \
                    Bob Rodriguez, age 35, is a mechanical engineer at Tesla in Austin, Texas. \
                    Professor Maria Garcia teaches computer science at MIT in Cambridge.";

    println!("Input text:");
    println!("{}\n", test_text);

    println!("Examples provided to guide extraction:");
    for (i, example) in examples.iter().enumerate() {
        println!("Example {}:", i + 1);
        println!("  Text: {}", example.text);
        println!("  Extractions:");
        for extraction in &example.extractions {
            println!("    - {}: {}", extraction.extraction_class, extraction.extraction_text);
        }
        println!();
    }

    println!("Attempting extraction with Ollama/Mistral...");

    // Perform extraction
    match extract(
        test_text,
        Some("Extract person names, ages, job titles, and locations from the text. Return the results as JSON."),
        &examples,
        config,
    ).await {
        Ok(result) => {
            println!("✅ Extraction successful!");
            println!("Found {} extractions\n", result.extraction_count());
            
            if let Some(extractions) = &result.extractions {
                println!("Extracted information:");
                for (i, extraction) in extractions.iter().enumerate() {
                    println!("{}. Class: {}", i + 1, extraction.extraction_class);
                    println!("   Text: {}", extraction.extraction_text);
                    if let Some(description) = &extraction.description {
                        println!("   Description: {}", description);
                    }
                    if let Some(interval) = &extraction.char_interval {
                        println!("   Position: {:?}", interval);
                    }
                    println!();
                }
            } else {
                println!("No extractions found in the result.");
            }

            // Test visualization
            match langextract_rust::visualize(&result, true) {
                Ok(viz) => {
                    println!("Visualization:");
                    println!("{}", viz);
                }
                Err(e) => {
                    println!("Visualization failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Extraction failed: {}", e);
            println!("\nPossible issues:");
            println!("1. Ollama is not running (start with: ollama serve)");
            println!("2. Mistral model is not installed (install with: ollama pull mistral)");
            println!("3. Ollama is running on a different port");
            println!("4. Network connectivity issues");
            
            if e.is_network_error() {
                println!("\nThis appears to be a network error. Please check:");
                println!("- Ollama server is running: curl http://localhost:11434/api/tags");
                println!("- Mistral model is available in the tags list");
            }
        }
    }

    Ok(())
}
