use langextract::{
    data::{ExampleData, FormatType, Extraction},
    providers::ProviderConfig,
    visualization::{export_document, ExportConfig, ExportFormat},
    extract, ExtractConfig,
};
use std::env;
use tokio;

// Test document containing various entities to extract
const TEST_DOCUMENT: &str = r#"
Dr. Sarah Johnson, Chief Technology Officer at TechCorp Inc., announced the company's quarterly results today.
The company's revenue reached $2.3 million in Q3 2024, exceeding expectations by 15%. 

Ms. Johnson can be reached at sarah.johnson@techcorp.com or by phone at (555) 123-4567.
The company is headquartered in San Francisco, CA, and employs over 250 people.

The announcement was made during a press conference at the TechCorp headquarters on October 15, 2024.
Stock prices increased by 12% following the announcement, closing at $87.50 per share.
"#;

// Example extraction data to guide the model
fn create_examples() -> Vec<ExampleData> {
    vec![
        ExampleData::new(
            "Dr. Sarah Johnson works at TechCorp. Ms. Johnson can be reached at her office.".to_string(),
            vec![
                Extraction::new("person".to_string(), "Dr. Sarah Johnson".to_string()),
                Extraction::new("person".to_string(), "Ms. Johnson".to_string()),
            ],
        ),
        ExampleData::new(
            "TechCorp Inc. and Google are major technology companies.".to_string(),
            vec![
                Extraction::new("company".to_string(), "TechCorp Inc.".to_string()),
                Extraction::new("company".to_string(), "Google".to_string()),
            ],
        ),
        ExampleData::new(
            "The company earned $2.3 million in revenue, exceeding expectations by 15%. The stock price closed at $87.50, up 12%.".to_string(),
            vec![
                Extraction::new("revenue".to_string(), "$2.3 million".to_string()),
                Extraction::new("percentage".to_string(), "15%".to_string()),
                Extraction::new("stock_price".to_string(), "$87.50".to_string()),
                Extraction::new("percentage".to_string(), "12%".to_string()),
            ],
        ),
        ExampleData::new(
            "Contact Sarah at sarah.johnson@techcorp.com or call (555) 123-4567.".to_string(),
            vec![
                Extraction::new("email".to_string(), "sarah.johnson@techcorp.com".to_string()),
                Extraction::new("phone".to_string(), "(555) 123-4567".to_string()),
            ],
        ),
        ExampleData::new(
            "The meeting is in San Francisco, CA on October 15, 2024 during Q3 2024.".to_string(),
            vec![
                Extraction::new("location".to_string(), "San Francisco, CA".to_string()),
                Extraction::new("date".to_string(), "October 15, 2024".to_string()),
                Extraction::new("period".to_string(), "Q3 2024".to_string()),
            ],
        ),
    ]
}

#[derive(Debug)]
struct TestResult {
    provider_name: String,
    model_name: String,
    success: bool,
    extraction_count: usize,
    processing_time: std::time::Duration,
    error: Option<String>,
}

async fn test_provider(config: ProviderConfig, provider_name: &str) -> TestResult {
    let start_time = std::time::Instant::now();
    let model_name = config.model.clone();

    println!("\n🔄 Testing {} with model '{}'...", provider_name, model_name);

    let extract_config = ExtractConfig {
        model_id: config.model.clone(),
        api_key: config.api_key.clone(),
        format_type: FormatType::Json,
        max_char_buffer: 4000,
        temperature: 0.7,
        fence_output: None,
        use_schema_constraints: false,
        batch_length: 10,
        max_workers: 4,
        additional_context: Some("Extract business entities, financial data, and contact information from this press release.".to_string()),
        resolver_params: std::collections::HashMap::new(),
        language_model_params: {
            let mut params = std::collections::HashMap::new();
            params.insert("provider_config".to_string(), serde_json::to_value(&config).unwrap());
            params
        },
        debug: true,
        model_url: Some(config.base_url.clone()),
        extraction_passes: 1,
        enable_multipass: false,
        multipass_min_extractions: 2,
        multipass_quality_threshold: 0.7,
    };

    let examples = create_examples();

    match extract(
        TEST_DOCUMENT,
        Some("Extract key business information from this press release"),
        &examples,
        extract_config,
    ).await {
        Ok(annotated_document) => {
            let processing_time = start_time.elapsed();
            let extraction_count = annotated_document.extraction_count();

            println!("✅ {} extraction completed!", provider_name);
            println!("   📊 Found {} extractions in {:?}", extraction_count, processing_time);

            // Generate all visualization formats
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let base_filename = format!("e2e_test_{}_{}", provider_name.to_lowercase(), timestamp);

            // HTML Export
            let html_config = ExportConfig {
                format: ExportFormat::Html,
                title: Some(format!("{} Extraction Results - {}", provider_name, model_name)),
                highlight_extractions: true,
                show_char_intervals: true,
                include_statistics: true,
                custom_css: Some(format!(
                    ".provider-badge {{ background: {}; color: white; padding: 4px 8px; border-radius: 4px; font-size: 0.8em; }}",
                    if provider_name.contains("OpenAI") { "#10a37f" } else { "#ff6b35" }
                )),
                ..Default::default()
            };

            if let Ok(html_output) = export_document(&annotated_document, &html_config) {
                let html_file = format!("{}.html", base_filename);
                if std::fs::write(&html_file, html_output).is_ok() {
                    println!("   📄 HTML report saved: {}", html_file);
                }
            }

            // JSON Export
            let json_config = ExportConfig {
                format: ExportFormat::Json,
                show_char_intervals: true,
                include_text: true,
                include_statistics: true,
                ..Default::default()
            };

            if let Ok(json_output) = export_document(&annotated_document, &json_config) {
                let json_file = format!("{}.json", base_filename);
                if std::fs::write(&json_file, json_output).is_ok() {
                    println!("   🔧 JSON data saved: {}", json_file);
                }
            }

            // CSV Export
            let csv_config = ExportConfig {
                format: ExportFormat::Csv,
                show_char_intervals: true,
                ..Default::default()
            };

            if let Ok(csv_output) = export_document(&annotated_document, &csv_config) {
                let csv_file = format!("{}.csv", base_filename);
                if std::fs::write(&csv_file, csv_output).is_ok() {
                    println!("   📊 CSV data saved: {}", csv_file);
                }
            }

            TestResult {
                provider_name: provider_name.to_string(),
                model_name,
                success: true,
                extraction_count,
                processing_time,
                error: None,
            }
        }
        Err(e) => {
            let processing_time = start_time.elapsed();
            println!("❌ {} extraction failed: {}", provider_name, e);

            TestResult {
                provider_name: provider_name.to_string(),
                model_name,
                success: false,
                extraction_count: 0,
                processing_time,
                error: Some(e.to_string()),
            }
        }
    }
}

async fn create_provider_configs() -> Vec<(ProviderConfig, String)> {
    let mut configs = Vec::new();

    // OpenAI Configuration
    if let Ok(openai_key) = env::var("OPENAI_API_KEY") {
        configs.push((
            ProviderConfig::openai("gpt-4o-mini", Some(openai_key.clone())),
            "OpenAI GPT-4o-mini".to_string(),
        ));
        configs.push((
            ProviderConfig::openai("gpt-3.5-turbo", Some(openai_key)),
            "OpenAI GPT-3.5-turbo".to_string(),
        ));
    } else {
        println!("⚠️  OPENAI_API_KEY not found in environment - skipping OpenAI tests");
        println!("   Set OPENAI_API_KEY=your_key to test OpenAI providers");
    }

    // Ollama Configuration - Test if server is available
    let ollama_base_url = env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://localhost:11434".to_string());
    
    // Check if Ollama is available by testing the base URL
    if is_ollama_available(&ollama_base_url).await {
        configs.push((
            ProviderConfig::ollama("mistral", Some(ollama_base_url.clone())),
            "Ollama Mistral".to_string(),
        ));
        
        // Add other Ollama models if needed
        configs.push((
            ProviderConfig::ollama("llama3.1", Some(ollama_base_url.clone())),
            "Ollama Llama3.1".to_string(),
        ));
    } else {
        println!("⚠️  Ollama server not available at {} - skipping Ollama tests", ollama_base_url);
        println!("   Start Ollama server and pull models: `ollama pull mistral && ollama pull llama3.1`");
    }

    // Custom provider example (if configured)
    if let Ok(custom_url) = env::var("CUSTOM_LLM_URL") {
        if let Ok(custom_key) = env::var("CUSTOM_LLM_KEY") {
            configs.push((
                ProviderConfig::openai_compatible(&custom_url, "custom-model", Some(custom_key)),
                "Custom Provider".to_string(),
            ));
        }
    }

    configs
}

async fn is_ollama_available(base_url: &str) -> bool {
    use std::time::Duration;
    
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
    
    client.get(&format!("{}/api/tags", base_url))
        .send()
        .await
        .is_ok()
}

fn print_summary(results: &[TestResult]) {
    println!("\n{}", "=".repeat(60));
    println!("🎯 END-TO-END TEST SUMMARY");
    println!("{}", "=".repeat(60));

    let successful = results.iter().filter(|r| r.success).count();
    let total = results.len();

    println!("📊 Overall Results: {}/{} providers successful", successful, total);
    println!();

    // Results table
    println!("{:<25} {:<15} {:<8} {:<12} {:<15}", "Provider", "Model", "Status", "Extractions", "Time");
    println!("{}", "-".repeat(80));

    for result in results {
        let status = if result.success { "✅ Success" } else { "❌ Failed" };
        let time_str = format!("{:.1}s", result.processing_time.as_secs_f64());
        
        println!("{:<25} {:<15} {:<8} {:<12} {:<15}", 
            result.provider_name, 
            result.model_name,
            status,
            result.extraction_count,
            time_str
        );

        if let Some(ref error) = result.error {
            println!("    Error: {}", error);
        }
    }

    if successful > 0 {
        println!("\n📁 Generated Files:");
        println!("   🔍 Check for *_e2e_test_*.html files for interactive results");
        println!("   📋 Check for *_e2e_test_*.json files for structured data");
        println!("   📊 Check for *_e2e_test_*.csv files for spreadsheet analysis");
    }

    println!("\n🎉 End-to-end testing complete!");
    
    if successful == 0 {
        println!("\n⚠️  No providers were available for testing. Please:");
        println!("   • Set OPENAI_API_KEY for OpenAI testing");
        println!("   • Start Ollama server for local testing: `ollama serve`");
        println!("   • Pull models: `ollama pull mistral && ollama pull llama3.1`");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 LangExtract End-to-End Provider Testing");
    println!("Testing with real language models and comprehensive output generation");
    println!("{}", "=".repeat(60));

    let configs = create_provider_configs().await;
    
    if configs.is_empty() {
        println!("❌ No provider configurations available!");
        println!("\nTo run this test, please configure at least one provider:");
        println!("  • OpenAI: export OPENAI_API_KEY=your_openai_key");
        println!("  • Ollama: ollama serve (then ollama pull mistral)");
        return Ok(());
    }

    println!("🔧 Found {} provider configuration(s)", configs.len());
    
    let mut results = Vec::new();

    for (config, provider_name) in configs {
        let result = test_provider(config, &provider_name).await;
        results.push(result);
        
        // Add a small delay between tests to be respectful to APIs
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    print_summary(&results);

    Ok(())
}
