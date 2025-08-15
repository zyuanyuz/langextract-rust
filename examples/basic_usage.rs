//! Basic usage example for langextract-rust
//!
//! This example demonstrates how to use the langextract library
//! to extract structured information from text.

use langextract::{extract, ExampleData, Extraction, ExtractConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

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
        ExampleData::new(
            "Dr. Sarah Johnson, 35, is a cardiologist at Mayo Clinic".to_string(),
            vec![
                Extraction::new("person".to_string(), "Dr. Sarah Johnson".to_string()),
                Extraction::new("age".to_string(), "35".to_string()),
                Extraction::new("profession".to_string(), "cardiologist".to_string()),
                Extraction::new("workplace".to_string(), "Mayo Clinic".to_string()),
            ],
        ),
    ];

    // Create extraction configuration
    let config = ExtractConfig {
        model_id: "gemini-2.5-flash".to_string(),
        // API key can be set via environment variable GEMINI_API_KEY
        api_key: None,
        temperature: 0.3,  // Lower temperature for more consistent results
        debug: true,
        ..Default::default()
    };

    // Text to extract information from
    let text = "Alice Smith is 28 years old and works as a data scientist at Google. \
               Bob Wilson, 42, is a mechanical engineer working at Tesla. \
               Professor Maria Garcia, age 55, teaches computer science at Stanford University.";

    println!("Text to process:");
    println!("{}\n", text);

    // Note: This will fail until we implement actual providers
    // For now, it demonstrates the API structure
    match extract(
        text,
        Some("Extract person names, ages, professions, and workplaces from the text"),
        &examples,
        config,
    ).await {
        Ok(result) => {
            println!("Extraction successful!");
            println!("Found {} extractions", result.extraction_count());
            
            if let Some(extractions) = &result.extractions {
                for extraction in extractions {
                    println!(
                        "Class: {}, Text: {}, Interval: {:?}",
                        extraction.extraction_class,
                        extraction.extraction_text,
                        extraction.char_interval
                    );
                }
            }
        }
        Err(e) => {
            println!("Extraction failed (expected in current implementation): {}", e);
            println!("This is normal - provider implementations are not yet complete.");
        }
    }

    Ok(())
}
