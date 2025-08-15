//! Test example for text chunking with large documents
//!
//! This example demonstrates how the langextract library handles large documents
//! by automatically chunking them and processing them in parallel.

use langextract::{extract, ExampleData, Extraction, ExtractConfig, FormatType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 Testing LangExtract with Large Document Chunking\n");

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

    // Create a large test document with lots of people
    let large_text = create_large_test_document();
    
    println!("📄 Large Document Statistics:");
    println!("   Total Length: {} characters", large_text.len());
    println!("   Word Count: ~{} words", large_text.split_whitespace().count());
    println!("   Paragraph Count: {}", large_text.split("\n\n").count());
    println!();

    // Create extraction configuration for chunking
    let config = ExtractConfig {
        model_id: "mistral".to_string(),  // This will auto-detect as Ollama provider
        api_key: None,  // Not needed for Ollama
        model_url: Some("http://localhost:11434".to_string()),  // Default Ollama port
        format_type: FormatType::Json,
        temperature: 0.2,  // Lower temperature for consistency
        use_schema_constraints: false,  // Disable for simplicity
        fence_output: Some(false),  // Let Ollama return raw JSON
        debug: true,
        
        // Chunking parameters
        max_char_buffer: 800,  // Small buffer to force chunking
        batch_length: 3,       // Process 3 chunks in parallel
        max_workers: 2,        // Use 2 workers
        extraction_passes: 1,
        
        ..Default::default()
    };

    println!("⚙️  Chunking Configuration:");
    println!("   Max chars per chunk: {}", config.max_char_buffer);
    println!("   Batch length: {}", config.batch_length);
    println!("   Max workers: {}", config.max_workers);
    println!("   Extraction passes: {}", config.extraction_passes);
    println!();

    println!("🔄 Starting extraction with automatic chunking...");

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
                println!();
            }

            // Test visualization with chunked results
            match langextract::visualize(&result, false) {
                Ok(viz) => {
                    println!("📄 Visualization (truncated):");
                    let lines: Vec<&str> = viz.lines().take(20).collect();
                    println!("{}", lines.join("\n"));
                    if viz.lines().count() > 20 {
                        println!("   ... (truncated for display)");
                    }
                }
                Err(e) => {
                    println!("⚠️  Visualization failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Extraction failed: {}", e);
            println!("\nPossible issues:");
            println!("1. Ollama is not running (start with: ollama serve)");
            println!("2. Mistral model is not installed (install with: ollama pull mistral)");
            println!("3. Ollama is running on a different port");
            
            if e.is_network_error() {
                println!("\nThis appears to be a network error. Please check:");
                println!("- Ollama server is running: curl http://localhost:11434/api/tags");
                println!("- Mistral model is available in the tags list");
            }
        }
    }

    Ok(())
}

/// Create a large test document with multiple people and organizations
fn create_large_test_document() -> String {
    let people_data = vec![
        ("Alice Johnson", "28", "data scientist", "Google in Mountain View"),
        ("Bob Rodriguez", "35", "mechanical engineer", "Tesla in Austin, Texas"),
        ("Professor Maria Garcia", "55", "computer science professor", "MIT in Cambridge"),
        ("David Chen", "42", "software architect", "Microsoft in Seattle"),
        ("Emily Watson", "31", "product manager", "Apple in Cupertino"),
        ("Dr. James Wilson", "48", "research scientist", "Stanford Research Institute"),
        ("Lisa Kim", "29", "UX designer", "Facebook in Menlo Park"),
        ("Michael Brown", "38", "data engineer", "Netflix in Los Gatos"),
        ("Sarah Davis", "33", "marketing director", "Salesforce in San Francisco"),
        ("Professor Robert Taylor", "52", "physics professor", "UC Berkeley"),
        ("Jennifer White", "27", "software engineer", "Uber in San Francisco"),
        ("Dr. Kevin Lee", "44", "biomedical researcher", "UCSF Medical Center"),
        ("Amanda Thompson", "36", "financial analyst", "Goldman Sachs in New York"),
        ("Mark Anderson", "41", "cybersecurity specialist", "Palantir in Palo Alto"),
        ("Dr. Rachel Green", "39", "psychiatrist", "Stanford Medical Center"),
    ];

    let mut document = String::new();
    
    document.push_str("# Technology Industry Personnel Report\n\n");
    document.push_str("This comprehensive report analyzes key personnel across major technology companies and academic institutions in Silicon Valley and beyond. ");
    document.push_str("The data has been compiled from public sources, company announcements, and academic publications. ");
    document.push_str("Our analysis covers professionals from various disciplines including software engineering, data science, product management, and academic research.\n\n");

    for (i, (name, age, job, location)) in people_data.iter().enumerate() {
        if i % 3 == 0 {
            document.push_str("## Section ");
            document.push_str(&((i / 3) + 1).to_string());
            document.push_str(": Industry Analysis\n\n");
        }

        // Vary the sentence structure to make it more interesting
        match i % 4 {
            0 => {
                document.push_str(&format!("{} is a {}-year-old {} working at {}. ", name, age, job, location));
                document.push_str("They have made significant contributions to their field and are well-regarded by their peers. ");
            },
            1 => {
                document.push_str(&format!("At {}, we find {}, age {}, who serves as a {}. ", location, name, age, job));
                document.push_str("Their expertise has been instrumental in driving innovation and growth. ");
            },
            2 => {
                document.push_str(&format!("{}, who is {} years old, holds the position of {} at {}. ", name, age, job, location));
                document.push_str("This role involves leading strategic initiatives and mentoring junior staff. ");
            },
            _ => {
                document.push_str(&format!("Working as a {} at {}, {} ({} years old) has established themselves as a leader in the industry. ", job, location, name, age));
                document.push_str("Their work has been published in numerous journals and conferences. ");
            }
        }

        if i % 2 == 0 {
            document.push_str("The company benefits greatly from their technical skills and leadership abilities. ");
        } else {
            document.push_str("They are known for their collaborative approach and innovative thinking. ");
        }

        if i % 3 == 2 {
            document.push_str("\n\n");
        }
    }

    document.push_str("## Conclusion\n\n");
    document.push_str("The technology industry continues to attract top talent from diverse backgrounds and disciplines. ");
    document.push_str("These professionals represent the cutting edge of innovation and research, driving progress in fields ");
    document.push_str("ranging from artificial intelligence and machine learning to biotechnology and sustainable energy. ");
    document.push_str("Their combined efforts are shaping the future of technology and its impact on society.\n\n");

    document.push_str("This report demonstrates the geographic concentration of talent in key technology hubs, ");
    document.push_str("particularly in the San Francisco Bay Area, but also shows the growing presence of tech companies ");
    document.push_str("in other major metropolitan areas such as Austin, Seattle, and Boston. The diversity of roles ");
    document.push_str("and expertise levels highlighted in this analysis reflects the complex, interdisciplinary nature ");
    document.push_str("of modern technology development and deployment.");

    document
}
