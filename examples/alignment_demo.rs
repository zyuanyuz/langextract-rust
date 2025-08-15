use langextract_rust::{
    data::{Document, FormatType, ExampleData, Extraction},
    extract, ExtractConfig,
    providers::{ProviderConfig, ProviderType},
    alignment::TextAligner,
    visualization::visualize,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenvy::dotenv().ok();

    println!("🔍 Character Alignment Demo");
    println!("============================\n");

    // Sample text with named entities we want to extract
    let source_text = r#"
John Smith is a software engineer at TechCorp Inc., located in San Francisco, California. 
He graduated from Stanford University in 2018 with a degree in Computer Science. 
John can be reached at john.smith@techcorp.com or at his office phone (555) 123-4567.
His colleague, Sarah Johnson, works in the marketing department and can be contacted at sarah.j@techcorp.com.
"#.trim();

    println!("📖 Source Text:");
    println!("{}\n", source_text);

    // Create example data for named entities
    let examples = vec![
        ExampleData::new(
            "John is a software engineer at Microsoft in Seattle".to_string(),
            vec![
                Extraction::new("person".to_string(), "John".to_string()),
                Extraction::new("organization".to_string(), "Microsoft".to_string()),
                Extraction::new("location".to_string(), "Seattle".to_string()),
            ]
        ),
        ExampleData::new(
            "Contact Dr. Smith at smith@university.edu or call (555) 987-6543".to_string(),
            vec![
                Extraction::new("person".to_string(), "Dr. Smith".to_string()),
                Extraction::new("email".to_string(), "smith@university.edu".to_string()),
                Extraction::new("phone".to_string(), "(555) 987-6543".to_string()),
            ]
        ),
    ];

    // Configure for Ollama (since it's local and reliable for testing)
    let config = ExtractConfig {
        model_id: "mistral".to_string(),
        api_key: None,
        format_type: FormatType::Json,
        max_char_buffer: 1000, // Small buffer to test chunking if needed
        temperature: 0.1,
        fence_output: Some(true),
        use_schema_constraints: false,
        batch_length: 1,
        max_workers: 1,
        additional_context: None,
        resolver_params: HashMap::new(),
        language_model_params: HashMap::new(),
        debug: true,
        extraction_passes: 1,
        model_url: Some("http://localhost:11434".to_string()),
        enable_multipass: false,
        multipass_min_extractions: 1,
        multipass_quality_threshold: 0.3,
    };

    println!("🔮 Extracting entities with character alignment...\n");

    // Perform extraction
    match extract(source_text, None, &examples, config).await {
        Ok(result) => {
            println!("✅ Extraction completed successfully!\n");

            if let Some(extractions) = &result.extractions {
                println!("📊 Alignment Results:");
                println!("--------------------");
                
                // Display each extraction with its position
                for (i, extraction) in extractions.iter().enumerate() {
                    println!("{}. [{}] \"{}\"", 
                        i + 1, 
                        extraction.extraction_class, 
                        extraction.extraction_text
                    );
                    
                    match (&extraction.char_interval, &extraction.alignment_status) {
                        (Some(interval), Some(status)) => {
                            if let (Some(start), Some(end)) = (interval.start_pos, interval.end_pos) {
                                let aligned_text = &source_text[start..end];
                                println!("   📍 Position: {}..{} (status: {:?})", start, end, status);
                                println!("   🎯 Aligned text: \"{}\"", aligned_text);
                                
                                // Show context around the match
                                let context_start = start.saturating_sub(20);
                                let context_end = std::cmp::min(end + 20, source_text.len());
                                let context = &source_text[context_start..context_end];
                                let highlight_start = start - context_start;
                                let highlight_end = end - context_start;
                                
                                println!("   📖 Context: \"{}[{}]{}\"",
                                    &context[..highlight_start],
                                    &context[highlight_start..highlight_end],
                                    &context[highlight_end..]
                                );
                            }
                        },
                        _ => {
                            println!("   ❌ Not aligned to source text");
                        }
                    }
                    println!();
                }

                // Show alignment statistics
                let aligner = TextAligner::new();
                let stats = aligner.get_alignment_stats(extractions);
                
                println!("📈 Alignment Statistics:");
                println!("   Total extractions: {}", stats.total);
                println!("   Exact matches: {}", stats.exact);
                println!("   Fuzzy matches: {}", stats.fuzzy);
                println!("   Lesser matches: {}", stats.lesser);
                println!("   Greater matches: {}", stats.greater);
                println!("   Unaligned: {}", stats.unaligned);
                println!("   Success rate: {:.1}%", stats.success_rate() * 100.0);
                println!("   Exact match rate: {:.1}%", stats.exact_match_rate() * 100.0);
                println!();
            }

            // Show visualization
            println!("🎨 Text Visualization:");
            println!("=====================");
            visualize(&result, true);
        },
        Err(e) => {
            eprintln!("❌ Error during extraction: {}", e);
            
            // Test alignment manually with sample data
            println!("\n🧪 Testing alignment with sample extractions...");
            test_manual_alignment(source_text).await?;
        }
    }

    Ok(())
}

async fn test_manual_alignment(source_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    use langextract_rust::data::Extraction;
    
    // Create some sample extractions to test alignment
    let mut test_extractions = vec![
        Extraction::new("person".to_string(), "John Smith".to_string()),
        Extraction::new("organization".to_string(), "TechCorp Inc.".to_string()),
        Extraction::new("location".to_string(), "San Francisco".to_string()),
        Extraction::new("email".to_string(), "john.smith@techcorp.com".to_string()),
        Extraction::new("person".to_string(), "Sarah Johnson".to_string()),
        // Test fuzzy matching
        Extraction::new("university".to_string(), "Stanford".to_string()),
        // Test no match
        Extraction::new("person".to_string(), "Bob Wilson".to_string()),
    ];

    let aligner = TextAligner::new();
    let aligned_count = aligner.align_extractions(&mut test_extractions, source_text, 0)?;
    
    println!("Aligned {} out of {} test extractions", aligned_count, test_extractions.len());
    
    for extraction in &test_extractions {
        println!("\"{}\" -> {:?} at {:?}", 
            extraction.extraction_text,
            extraction.alignment_status,
            extraction.char_interval
        );
    }

    Ok(())
}
