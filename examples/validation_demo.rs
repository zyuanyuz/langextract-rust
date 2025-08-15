use langextract_rust::{
    data::{ExampleData, Extraction, FormatType},
    extract, ExtractConfig,
    resolver::{ValidationConfig, ValidationResult},
    visualization::visualize,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenvy::dotenv().ok();

    println!("🛡️  Advanced Validation System Demo");
    println!("=====================================\n");

    // Sample text that might produce various validation scenarios
    let source_text = r#"
Dr. Sarah Johnson, a renowned researcher at MIT, published her findings on quantum computing.
She can be reached at s.johnson@mit.edu or by phone at (617) 555-0123.
The research was funded by a $2.5 million grant from the NSF.
Her colleague, Prof. Michael Chen from Stanford University, also contributed to the work.
You can contact him at m.chen@stanford.edu.
"#.trim();

    println!("📖 Source Text:");
    println!("{}\n", source_text);

    // Create example data for comprehensive entity extraction
    let examples = vec![
        ExampleData::new(
            "Dr. John Smith from Harvard University received $1M in funding. Contact: j.smith@harvard.edu or (617) 555-9876.".to_string(),
            vec![
                Extraction::new("person".to_string(), "Dr. John Smith".to_string()),
                Extraction::new("organization".to_string(), "Harvard University".to_string()),
                Extraction::new("funding_amount".to_string(), "$1M".to_string()),
                Extraction::new("email".to_string(), "j.smith@harvard.edu".to_string()),
                Extraction::new("phone".to_string(), "(617) 555-9876".to_string()),
            ]
        ),
        ExampleData::new(
            "Prof. Jane Doe works at UC Berkeley. She studies AI and can be contacted at jane.doe@berkeley.edu.".to_string(),
            vec![
                Extraction::new("person".to_string(), "Prof. Jane Doe".to_string()),
                Extraction::new("organization".to_string(), "UC Berkeley".to_string()),
                Extraction::new("research_area".to_string(), "AI".to_string()),
                Extraction::new("email".to_string(), "jane.doe@berkeley.edu".to_string()),
            ]
        ),
    ];

    println!("🔧 Configuration:");
    
    // Test 1: Basic extraction with validation enabled
    println!("\n1️⃣ Basic Extraction with Validation:");
    let basic_config = ExtractConfig {
        model_id: "mistral".to_string(),
        api_key: None,
        format_type: FormatType::Json,
        max_char_buffer: 2000,
        temperature: 0.1,
        fence_output: Some(true),
        use_schema_constraints: false,
        batch_length: 1,
        max_workers: 1,
        additional_context: Some("Extract all person names, organizations, funding amounts, emails, and phone numbers.".to_string()),
        resolver_params: HashMap::new(),
        language_model_params: HashMap::new(),
        debug: true, // This enables raw output saving
        model_url: Some("http://localhost:11434".to_string()),
        extraction_passes: 1,
        enable_multipass: false,
        multipass_min_extractions: 1,
        multipass_quality_threshold: 0.3,
    };

    match extract(source_text, None, &examples, basic_config).await {
        Ok(result) => {
            println!("✅ Extraction completed successfully!");
            println!("   Found {} extractions", result.extraction_count());
            
            if let Some(extractions) = &result.extractions {
                let mut by_class: HashMap<String, Vec<&Extraction>> = HashMap::new();
                for extraction in extractions {
                    by_class.entry(extraction.extraction_class.clone())
                        .or_default()
                        .push(extraction);
                }
                
                println!("   📊 Breakdown by type:");
                for (class, extractions) in by_class {
                    println!("   📋 {}: {} found", class, extractions.len());
                    for extraction in extractions.iter().take(2) { // Show first 2
                        if let Some(interval) = &extraction.char_interval {
                            if let (Some(start), Some(end)) = (interval.start_pos, interval.end_pos) {
                                println!("      - \"{}\" ({}..{}, {:?})", 
                                    extraction.extraction_text, start, end, 
                                    extraction.alignment_status);
                            }
                        } else {
                            println!("      - \"{}\" (not aligned)", extraction.extraction_text);
                        }
                    }
                    if extractions.len() > 2 {
                        println!("      ... and {} more", extractions.len() - 2);
                    }
                }
            }

            println!("\n🎨 Text Visualization:");
            println!("=====================");
            match visualize(&result, true) {
                Ok(visualization) => println!("{}", visualization),
                Err(e) => println!("❌ Visualization failed: {}", e),
            }
        },
        Err(e) => {
            println!("❌ Extraction failed: {}", e);
        }
    }

    println!("\n{}", "=".repeat(60));

    // Test 2: Demonstrate validation with malformed input
    println!("\n2️⃣ Validation Testing with Simulated Malformed Response:");
    println!("(This would normally be triggered by actual model output)");
    
    use langextract_rust::resolver::Resolver;
    
    // Create a resolver to test validation directly
    let test_config = ExtractConfig::default();
    let resolver = Resolver::new(&test_config, true)?;
    
    // Test with valid JSON
    println!("\n🔍 Testing with valid JSON:");
    let valid_json = r#"{"person": "Dr. Sarah Johnson", "organization": "MIT", "email": "s.johnson@mit.edu"}"#;
    let expected_fields = vec!["person".to_string(), "organization".to_string(), "email".to_string()];
    
    match resolver.validate_and_parse(valid_json, &expected_fields) {
        Ok((extractions, validation_result)) => {
            println!("✅ Valid JSON parsed successfully:");
            println!("   - {} extractions found", extractions.len());
            println!("   - Validation passed: {}", validation_result.is_valid);
            println!("   - Errors: {}", validation_result.errors.len());
            println!("   - Warnings: {}", validation_result.warnings.len());
            if let Some(raw_file) = &validation_result.raw_output_file {
                println!("   - Raw output saved to: {}", raw_file);
            }
        }
        Err(e) => println!("❌ Failed: {}", e),
    }

    // Test with invalid JSON
    println!("\n🔍 Testing with invalid JSON:");
    let invalid_json = r#"{"person": "Dr. Sarah Johnson", "organization": "MIT" // Missing closing brace"#;
    
    match resolver.validate_and_parse(invalid_json, &expected_fields) {
        Ok((extractions, validation_result)) => {
            println!("⚠️  Parsed despite issues:");
            println!("   - {} extractions found", extractions.len());
            println!("   - Validation passed: {}", validation_result.is_valid);
        }
        Err(e) => {
            println!("❌ Parse failed (expected): {}", e);
            println!("   Raw output was still saved for debugging");
        }
    }

    println!("\n🎯 Key Validation Features Demonstrated:");
    println!("=========================================");
    println!("✅ Raw output preservation - All model responses saved to files");
    println!("✅ Validation reporting - Detailed errors and warnings");
    println!("✅ Graceful error handling - System continues even with parse failures");
    println!("✅ Character alignment - Extractions mapped to source positions");
    println!("✅ Debug information - Comprehensive logging for troubleshooting");
    
    println!("\n📁 Check the './raw_outputs' directory for saved model responses!");
    println!("   These files contain the complete raw output for debugging/recovery.");

    Ok(())
}
