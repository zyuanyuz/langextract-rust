use langextract_rust::{
    data::{ExampleData, FormatType, Extraction},
    providers::ProviderConfig,
    extract, ExtractConfig,
};
use std::env;
use tokio;

// Simple examples for debugging alignment
fn create_simple_examples() -> Vec<ExampleData> {
    vec![
        ExampleData::new(
            "Apple MacBook Pro costs $3,999.00 with model number MBP-001".to_string(),
            vec![
                Extraction::new("product_name".to_string(), "Apple MacBook Pro".to_string()),
                Extraction::new("price".to_string(), "$3,999.00".to_string()),
                Extraction::new("model".to_string(), "MBP-001".to_string()),
            ],
        ),
    ]
}

async fn debug_alignment_test() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debug Alignment Test");
    println!("=======================");

    // Use just the first few lines of the product catalog for debugging
    let test_text = r#"ELECTRONICS DEPARTMENT - CATALOG 2024

Apple MacBook Pro 16-inch M3 Max - Model SKU: MBP-M3-16-SLV-2TB
Product Code: APPLE-2024-001, UPC: 194253715726, GTIN: 00194253715726
Advanced M3 Max chip with 16-core CPU and 40-core GPU for professional workflows
Starting at $3,999.00 (MSRP). Warranty Code: APL-WAR-24M-001
Premium aluminum construction with Liquid Retina XDR display

Samsung 85" Neo QLED 8K Smart TV - Model QN85QN900C
SKU: SAM-TV-85-8K-001, Product ID: PID-SAMSUNG-TV-001
Item Number: ITM-789456123, Barcode: 887276661234
Crystal UHD 8K resolution with Quantum HDR 64X technology
Price: $4,299.99, Was: $5,499.99 (Sale ID: SALE-2024-WINTER-001)"#;

    println!("📄 Test text length: {} characters", test_text.len());
    println!("📝 Test text preview:\n{}\n", &test_text[..200.min(test_text.len())]);

    // Try to find a provider
    let provider_config = if let Ok(openai_key) = env::var("OPENAI_API_KEY") {
        ProviderConfig::openai("gpt-4o-mini", Some(openai_key))
    } else {
        println!("⚠️  No OpenAI key found, using mock extraction for alignment debug");
        return debug_manual_alignment(test_text);
    };

    let extract_config = ExtractConfig {
        model_id: provider_config.model.clone(),
        api_key: provider_config.api_key.clone(),
        format_type: FormatType::Json,
        max_char_buffer: 2000,
        temperature: 0.1,
        fence_output: None,
        use_schema_constraints: false,
        batch_length: 1,
        max_workers: 1,
        additional_context: Some("Extract product names, prices, and model numbers".to_string()),
        resolver_params: std::collections::HashMap::new(),
        language_model_params: {
            let mut params = std::collections::HashMap::new();
            params.insert("provider_config".to_string(), serde_json::to_value(&provider_config)?);
            params
        },
        debug: true,
        model_url: Some(provider_config.base_url.clone()),
        extraction_passes: 1,
        enable_multipass: false,
        multipass_min_extractions: 2,
        multipass_quality_threshold: 0.8,
    };

    let examples = create_simple_examples();

    println!("🔄 Running extraction...");
    match extract(
        test_text,
        Some("Extract product information from this catalog excerpt"),
        &examples,
        extract_config,
    ).await {
        Ok(annotated_document) => {
            println!("✅ Extraction completed");
            
            if let Some(extractions) = &annotated_document.extractions {
                println!("\n🔍 Alignment Analysis:");
                println!("{:<30} {:<20} {:<15} {:<15} {:<10}", "Extraction", "Text", "Start", "End", "Status");
                println!("{}", "-".repeat(90));

                for extraction in extractions {
                    let start_pos = extraction.char_interval.as_ref()
                        .and_then(|i| i.start_pos)
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "None".to_string());
                    
                    let end_pos = extraction.char_interval.as_ref()
                        .and_then(|i| i.end_pos)
                        .map(|p| p.to_string())
                        .unwrap_or_else(|| "None".to_string());

                    let status = extraction.alignment_status.as_ref()
                        .map(|s| format!("{:?}", s))
                        .unwrap_or_else(|| "None".to_string());

                    println!("{:<30} {:<20} {:<15} {:<15} {:<10}", 
                        extraction.extraction_class,
                        &extraction.extraction_text[..extraction.extraction_text.len().min(20)],
                        start_pos,
                        end_pos,
                        status
                    );

                    // Show what text is actually at that position
                    if let Some(interval) = &extraction.char_interval {
                        if let (Some(start), Some(end)) = (interval.start_pos, interval.end_pos) {
                            if start < test_text.len() && end <= test_text.len() && start < end {
                                let actual_text = &test_text[start..end];
                                println!("    → Actual text at position: '{}'", actual_text);
                                
                                if actual_text != extraction.extraction_text {
                                    println!("    ⚠️  MISMATCH! Expected: '{}'", extraction.extraction_text);
                                }
                            } else {
                                println!("    ❌ Invalid character positions: start={}, end={}, text_len={}", 
                                    start, end, test_text.len());
                            }
                        }
                    }
                }

                // Test manual alignment for comparison
                println!("\n🔧 Manual Alignment Test:");
                for extraction in extractions {
                    if let Some(pos) = test_text.find(&extraction.extraction_text) {
                        println!("Found '{}' at position {} (manual search)", 
                            extraction.extraction_text, pos);
                    } else {
                        // Try case-insensitive
                        if let Some(pos) = test_text.to_lowercase().find(&extraction.extraction_text.to_lowercase()) {
                            println!("Found '{}' at position {} (case-insensitive manual search)", 
                                extraction.extraction_text, pos);
                        } else {
                            println!("❌ Could not find '{}' in text (manual search)", extraction.extraction_text);
                        }
                    }
                }
            } else {
                println!("❌ No extractions found");
            }
        }
        Err(e) => {
            println!("❌ Extraction failed: {}", e);
        }
    }

    Ok(())
}

fn debug_manual_alignment(test_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Manual Alignment Debug (no LLM)");
    
    // Create some mock extractions that should be in the text
    let mock_extractions = vec![
        "Apple MacBook Pro 16-inch M3 Max",
        "$3,999.00",
        "MBP-M3-16-SLV-2TB",
        "Samsung 85\" Neo QLED 8K Smart TV",
        "$4,299.99",
        "QN85QN900C",
    ];

    println!("\n🔍 Testing manual alignment:");
    for extraction_text in mock_extractions {
        if let Some(pos) = test_text.find(extraction_text) {
            let end_pos = pos + extraction_text.len();
            println!("✅ Found '{}' at {}-{}", extraction_text, pos, end_pos);
            
            // Verify the text at that position
            let actual = &test_text[pos..end_pos];
            if actual == extraction_text {
                println!("   ✅ Position correct: '{}'", actual);
            } else {
                println!("   ❌ Position incorrect: '{}'", actual);
            }
        } else {
            println!("❌ Not found: '{}'", extraction_text);
            
            // Try to find partial matches
            let words: Vec<&str> = extraction_text.split_whitespace().collect();
            if words.len() > 1 {
                for word in &words {
                    if let Some(pos) = test_text.find(word) {
                        println!("   🔍 Found word '{}' at position {}", word, pos);
                    }
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    debug_alignment_test().await
}
