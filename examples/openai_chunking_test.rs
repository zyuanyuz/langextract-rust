//! Test OpenAI provider with chunking for large documents
//!
//! This example demonstrates how the OpenAI provider handles large documents
//! using semantic chunking and parallel processing.

use langextract_rust::{extract, ExampleData, Extraction, ExtractConfig, FormatType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 Testing OpenAI Provider with Large Document Chunking\n");

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

    // Create a large test document
    let large_text = create_large_test_document();
    
    println!("📄 Large Document Statistics:");
    println!("   Total Length: {} characters", large_text.len());
    println!("   Word Count: ~{} words", large_text.split_whitespace().count());
    println!();

    // Create extraction configuration for OpenAI with semantic chunking
    let config = ExtractConfig {
        model_id: "gpt-3.5-turbo".to_string(),  // More cost-effective than GPT-4
        api_key: None,  // Will load from .env file
        format_type: FormatType::Json,
        temperature: 0.2,  // Lower temperature for consistency
        use_schema_constraints: false,  // Disable for simplicity
        fence_output: Some(false),  // Let OpenAI return raw JSON
        debug: true,

        // Semantic chunking parameters optimized for OpenAI API
        max_char_buffer: 600,  // Characters per chunk (respects semantic boundaries)
        batch_length: 2,       // Process 2 chunks in parallel (be gentle on API)
        max_workers: 2,        // Use 2 workers to avoid rate limits
        extraction_passes: 1,  // Single pass for cost efficiency
        enable_multipass: false, // Disable multi-pass for this example
        
        ..Default::default()
    };

    println!("⚙️  Semantic Chunking Configuration:");
    println!("   Model: {}", config.model_id);
    println!("   Max chars per buffer: {} (respects semantic boundaries)", config.max_char_buffer);
    println!("   Batch length: {}", config.batch_length);
    println!("   Max workers: {}", config.max_workers);
    println!("   Multi-pass enabled: {}", config.enable_multipass);
    println!();

    println!("🔄 Starting extraction with OpenAI and semantic chunking...");
    println!("   Using AI-powered content understanding");
    println!("   Chunks will respect semantic structure");

    // Perform extraction with chunking
    match extract(
        &large_text,
        Some("Extract person names, ages, job titles, and locations from the text. Return the results as JSON."),
        &examples,
        config,
    ).await {
        Ok(result) => {
            println!("✅ Extraction successful!");
            println!("Found {} total extractions\n", result.extraction_count());
            
            if let Some(extractions) = &result.extractions {
                // Group extractions by type
                let mut by_type = std::collections::HashMap::new();
                for extraction in extractions {
                    by_type.entry(extraction.extraction_class.clone())
                        .or_insert_with(Vec::new)
                        .push(extraction);
                }

                println!("📊 Extraction Summary by Type:");
                for (extraction_type, items) in &by_type {
                    println!("   {}: {} instances", extraction_type, items.len());
                }
                println!();

                println!("🎯 Sample Extractions:");
                for (i, extraction) in extractions.iter().take(10).enumerate() {
                    println!("{}. [{}] {}", 
                        i + 1, 
                        extraction.extraction_class, 
                        extraction.extraction_text
                    );
                }
                
                if extractions.len() > 10 {
                    println!("   ... and {} more", extractions.len() - 10);
                }
            }
        }
        Err(e) => {
            println!("❌ Extraction failed: {}", e);
            println!("\nPossible issues:");
            println!("1. API key not found in .env file");
            println!("2. Invalid API key or insufficient credits");
            println!("3. Network connectivity issues");
            println!("4. Rate limiting (try reducing batch_length or max_workers)");
        }
    }

    Ok(())
}

/// Create a moderately large test document with multiple people
fn create_large_test_document() -> String {
    let people_data = vec![
        ("Alice Johnson", "28", "data scientist", "Google in Mountain View"),
        ("Bob Rodriguez", "35", "mechanical engineer", "Tesla in Austin, Texas"),
        ("Professor Maria Garcia", "55", "computer science professor", "MIT in Cambridge"),
        ("David Chen", "42", "software architect", "Microsoft in Seattle"),
        ("Emily Watson", "31", "product manager", "Apple in Cupertino"),
        ("Dr. James Wilson", "48", "research scientist", "Stanford Research Institute"),
        ("Lisa Kim", "29", "UX designer", "Meta in Menlo Park"),
        ("Michael Brown", "38", "data engineer", "Netflix in Los Gatos"),
    ];

    let mut document = String::new();
    
    document.push_str("# Technology Industry Report\n\n");
    document.push_str("This report analyzes key personnel across major technology companies. ");
    document.push_str("The data covers professionals from various disciplines including software engineering, data science, and product management.\n\n");

    for (i, (name, age, job, location)) in people_data.iter().enumerate() {
        if i % 2 == 0 {
            document.push_str("## Section ");
            document.push_str(&((i / 2) + 1).to_string());
            document.push_str("\n\n");
        }

        document.push_str(&format!("{} is a {}-year-old {} working at {}. ", name, age, job, location));
        document.push_str("They have made significant contributions to their field and are recognized by their peers. ");
        
        if i % 2 == 0 {
            document.push_str("Their expertise has been instrumental in driving innovation. ");
        } else {
            document.push_str("They are known for their collaborative approach and technical leadership. ");
        }

        if i % 2 == 1 {
            document.push_str("\n\n");
        }
    }

    document.push_str("## Conclusion\n\n");
    document.push_str("The technology industry continues to attract top talent from diverse backgrounds. ");
    document.push_str("These professionals represent innovation in AI, cloud computing, and software development. ");
    document.push_str("Their work shapes the future of technology.");

    document
}
