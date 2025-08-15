use langextract_rust::{
    data::{ExampleData, FormatType, Extraction},
    providers::ProviderConfig,
    visualization::{export_document, ExportConfig, ExportFormat},
    extract, ExtractConfig,
};
use std::env;
use tokio;

// Product catalog examples to guide the model
fn create_product_examples() -> Vec<ExampleData> {
    vec![
        ExampleData::new(
            "Apple iPhone 15 Pro - Model A3101, UPC: 194253123456, Starting at $999.00 (MSRP). Premium titanium construction with A17 Pro chip. SKU: APPLE-IP15-PRO-001, Product Code: APPLE-2024-002".to_string(),
            vec![
                Extraction::new("product_name".to_string(), "Apple iPhone 15 Pro".to_string()),
                Extraction::new("model".to_string(), "A3101".to_string()),
                Extraction::new("sku".to_string(), "APPLE-IP15-PRO-001".to_string()),
                Extraction::new("product_code".to_string(), "APPLE-2024-002".to_string()),
                Extraction::new("upc".to_string(), "194253123456".to_string()),
                Extraction::new("price".to_string(), "$999.00".to_string()),
                Extraction::new("material".to_string(), "titanium".to_string()),
                Extraction::new("chip".to_string(), "A17 Pro".to_string()),
            ],
        ),
        ExampleData::new(
            "Samsung 65\" QLED 4K Smart TV - Model QN65Q70C, SKU: SAM-TV-65-4K-001, Price: $1,299.99, Was: $1,599.99 (Sale ID: SALE-2024-SPRING-001). Crystal UHD 4K resolution with Quantum HDR technology. Currently in stock.".to_string(),
            vec![
                Extraction::new("product_name".to_string(), "Samsung 65\" QLED 4K Smart TV".to_string()),
                Extraction::new("model".to_string(), "QN65Q70C".to_string()),
                Extraction::new("sku".to_string(), "SAM-TV-65-4K-001".to_string()),
                Extraction::new("price".to_string(), "$1,299.99".to_string()),
                Extraction::new("original_price".to_string(), "$1,599.99".to_string()),
                Extraction::new("sale_id".to_string(), "SALE-2024-SPRING-001".to_string()),
                Extraction::new("resolution".to_string(), "4K".to_string()),
                Extraction::new("technology".to_string(), "Quantum HDR".to_string()),
                Extraction::new("availability".to_string(), "in stock".to_string()),
            ],
        ),
        ExampleData::new(
            "Pfizer Lipitor (Atorvastatin) 20mg Tablets, NDC: 0071-0155-23, Generic Name: Atorvastatin Calcium, Package Size: 90 tablets, Lot Number: LIP24A001, Expiration: 06/2026".to_string(),
            vec![
                Extraction::new("product_name".to_string(), "Pfizer Lipitor".to_string()),
                Extraction::new("generic_name".to_string(), "Atorvastatin".to_string()),
                Extraction::new("strength".to_string(), "20mg".to_string()),
                Extraction::new("ndc".to_string(), "0071-0155-23".to_string()),
                Extraction::new("package_size".to_string(), "90 tablets".to_string()),
                Extraction::new("lot_number".to_string(), "LIP24A001".to_string()),
                Extraction::new("expiration_date".to_string(), "06/2026".to_string()),
            ],
        ),
        ExampleData::new(
            "DeWalt 20V MAX Cordless Drill/Driver Kit, Model: DCD771C2, UPC: 885911234567, Price: $149.00, Contractor Price: $129.00, Warranty: 3 years".to_string(),
            vec![
                Extraction::new("product_name".to_string(), "DeWalt 20V MAX Cordless Drill/Driver Kit".to_string()),
                Extraction::new("model".to_string(), "DCD771C2".to_string()),
                Extraction::new("upc".to_string(), "885911234567".to_string()),
                Extraction::new("price".to_string(), "$149.00".to_string()),
                Extraction::new("contractor_price".to_string(), "$129.00".to_string()),
                Extraction::new("warranty".to_string(), "3 years".to_string()),
            ],
        ),
        ExampleData::new(
            "Nike Air Max 270 - Men's Running Shoes, Style Code: AH8050-001, Size Range: 7-15, Price: $150.00, Color: Black/White, Release Date: 03/26/2018".to_string(),
            vec![
                Extraction::new("product_name".to_string(), "Nike Air Max 270".to_string()),
                Extraction::new("category".to_string(), "Men's Running Shoes".to_string()),
                Extraction::new("style_code".to_string(), "AH8050-001".to_string()),
                Extraction::new("size_range".to_string(), "7-15".to_string()),
                Extraction::new("price".to_string(), "$150.00".to_string()),
                Extraction::new("color".to_string(), "Black/White".to_string()),
                Extraction::new("release_date".to_string(), "03/26/2018".to_string()),
            ],
        ),
    ]
}

async fn test_product_extraction(provider_config: ProviderConfig, provider_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🛍️  Testing Product Catalog Extraction with {}", provider_name);
    println!("{}", "=".repeat(50));

    // Read the product catalog file
    let product_text = std::fs::read_to_string("sample_product_text.txt")?;
    println!("📄 Loaded product catalog: {} characters", product_text.len());

    let extract_config = ExtractConfig {
        model_id: provider_config.model.clone(),
        api_key: provider_config.api_key.clone(),
        format_type: FormatType::Json,
        max_char_buffer: 8000, // Larger buffer for product catalog
        temperature: 0.3, // Lower temperature for more consistent extraction
        fence_output: None,
        use_schema_constraints: false,
        batch_length: 6, // Optimized batch size for better throughput
        max_workers: 6, // Increased parallel workers for faster processing
        additional_context: Some("Extract detailed product information including names, models, SKUs, prices, codes, and specifications from this product catalog. Focus on identifying specific product identifiers, pricing, and technical details.".to_string()),
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
        multipass_min_extractions: 5,
        multipass_quality_threshold: 0.8,
    };

    let examples = create_product_examples();
    let start_time = std::time::Instant::now();

    println!("🔄 Starting extraction...");
    
    match extract(
        &product_text,
        Some("Extract comprehensive product information from this electronics and retail catalog"),
        &examples,
        extract_config,
    ).await {
        Ok(annotated_document) => {
            let processing_time = start_time.elapsed();
            let extraction_count = annotated_document.extraction_count();

            println!("✅ Extraction completed in {:?}", processing_time);
            println!("📊 Found {} total extractions", extraction_count);

            // Analyze extraction types
            if let Some(extractions) = &annotated_document.extractions {
                let mut category_counts = std::collections::HashMap::new();
                for extraction in extractions {
                    *category_counts.entry(&extraction.extraction_class).or_insert(0) += 1;
                }

                println!("\n📈 Extraction Breakdown:");
                for (category, count) in &category_counts {
                    println!("   • {}: {} items", category, count);
                }

                // Show some example extractions
                println!("\n🔍 Sample Extractions:");
                let mut shown = 0;
                for extraction in extractions.iter().take(15) {
                    println!("   [{}] {}", extraction.extraction_class, extraction.extraction_text);
                    shown += 1;
                }
                if extractions.len() > shown {
                    println!("   ... and {} more", extractions.len() - shown);
                }
            }

            // Generate comprehensive reports
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let base_filename = format!("product_catalog_{}_{}", provider_name.to_lowercase().replace(" ", "_"), timestamp);

            // Generate HTML report with product focus
            let html_config = ExportConfig {
                format: ExportFormat::Html,
                title: Some(format!("Product Catalog Analysis - {} ({} extractions)", provider_name, extraction_count)),
                highlight_extractions: true,
                show_char_intervals: false, // Skip intervals for cleaner product view
                include_statistics: true,
                custom_css: Some(r#"
                    .product-highlight { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
                    .extraction-class { 
                        font-size: 0.9em; 
                        text-transform: uppercase; 
                        letter-spacing: 0.5px; 
                    }
                    .price-highlight { background: #10b981; color: white; }
                    .code-highlight { background: #f59e0b; color: white; }
                "#.to_string()),
                ..Default::default()
            };

            if let Ok(html_output) = export_document(&annotated_document, &html_config) {
                let html_file = format!("{}.html", base_filename);
                std::fs::write(&html_file, html_output)?;
                println!("\n📄 Interactive HTML report: {}", html_file);
            }

            // Generate structured JSON for analysis
            let json_config = ExportConfig {
                format: ExportFormat::Json,
                show_char_intervals: true,
                include_text: false, // Skip full text for cleaner JSON
                include_statistics: true,
                ..Default::default()
            };

            if let Ok(json_output) = export_document(&annotated_document, &json_config) {
                let json_file = format!("{}.json", base_filename);
                std::fs::write(&json_file, json_output)?;
                println!("🔧 Structured JSON data: {}", json_file);
            }

            // Generate CSV for product analysis
            let csv_config = ExportConfig {
                format: ExportFormat::Csv,
                show_char_intervals: false,
                ..Default::default()
            };

            if let Ok(csv_output) = export_document(&annotated_document, &csv_config) {
                let csv_file = format!("{}.csv", base_filename);
                std::fs::write(&csv_file, csv_output)?;
                println!("📊 Product data CSV: {}", csv_file);
            }

            // Product-specific analysis
            if let Some(extractions) = &annotated_document.extractions {
                println!("\n💰 Price Analysis:");
                let prices: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "price" || e.extraction_class == "original_price")
                    .collect();
                println!("   Found {} price entries", prices.len());

                println!("\n🏷️  Product Code Analysis:");
                let codes: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class.contains("code") || e.extraction_class.contains("sku") || e.extraction_class.contains("upc"))
                    .collect();
                println!("   Found {} product identifiers", codes.len());

                println!("\n📦 Product Category Distribution:");
                let products: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "product_name")
                    .collect();
                println!("   Identified {} distinct products", products.len());
            }

            println!("\n🎉 Product catalog analysis complete!");
            println!("💡 Tips:");
            println!("   • Open the .html file to see highlighted products in context");
            println!("   • Use the .csv file for spreadsheet analysis and filtering");
            println!("   • Check the .json file for programmatic data processing");

        }
        Err(e) => {
            println!("❌ Extraction failed: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

async fn create_provider_config() -> Option<(ProviderConfig, String)> {
    // Try OpenAI first
    if let Ok(openai_key) = env::var("OPENAI_API_KEY") {
        return Some((
            ProviderConfig::openai("gpt-4o-mini", Some(openai_key)),
            "OpenAI GPT-4o-mini".to_string(),
        ));
    }

    // Try Ollama
    let ollama_base_url = env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".to_string());
    
    // Quick check if Ollama is available
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
    
    if client.get(&format!("{}/api/tags", ollama_base_url)).send().await.is_ok() {
        return Some((
            ProviderConfig::ollama("mistral", Some(ollama_base_url)),
            "Ollama Mistral".to_string(),
        ));
    }

    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛍️  LangExtract Product Catalog Extraction Test");
    println!("🎯 Specialized test for extracting structured product data");
    println!("{}", "=".repeat(60));

    // Check if the product file exists
    if !std::path::Path::new("sample_product_text.txt").exists() {
        println!("❌ Error: sample_product_text.txt not found!");
        println!("   Please ensure the product catalog file is in the current directory.");
        return Ok(());
    }

    // Try to find an available provider
    if let Some((config, provider_name)) = create_provider_config().await {
        println!("✅ Using provider: {}", provider_name);
        println!("🔧 Model: {}", config.model);
        
        test_product_extraction(config, &provider_name).await?;
    } else {
        println!("❌ No language model providers available!");
        println!("\nTo run this test, please configure a provider:");
        println!("  • OpenAI: export OPENAI_API_KEY=your_openai_key");
        println!("  • Ollama: ollama serve && ollama pull mistral");
        println!("\nThis test is optimized for:");
        println!("  📦 Product names and descriptions");
        println!("  🏷️  SKUs, UPCs, and product codes");
        println!("  💰 Prices and financial information");
        println!("  📊 Technical specifications");
        println!("  🏪 Inventory and availability data");
    }

    Ok(())
}
