use langextract_rust::{
    data::{ExampleData, FormatType, Extraction},
    providers::ProviderConfig,
    visualization::{export_document, ExportConfig, ExportFormat},
    extract, ExtractConfig,
};
use std::env;
use tokio;

// Academic paper examples specifically tailored for the LLM-assisted planning paper
fn create_academic_examples() -> Vec<ExampleData> {
    vec![
        ExampleData::new(
            "Inspire or Predict? Exploring New Paradigms in Assisting Classical Planners with Large Language Models by Wenkai Yu, Jianhang Tang, Yang Zhang, Shanjiang Tang, Kebing Jin, Hankz Hankui Zhuo. arXiv:2508.11524v1 [cs.AI] 15 Aug 2025.".to_string(),
            vec![
                Extraction::new("paper_title".to_string(), "Inspire or Predict? Exploring New Paradigms in Assisting Classical Planners with Large Language Models".to_string()),
                Extraction::new("author".to_string(), "Wenkai Yu".to_string()),
                Extraction::new("author".to_string(), "Jianhang Tang".to_string()),
                Extraction::new("author".to_string(), "Yang Zhang".to_string()),
                Extraction::new("author".to_string(), "Shanjiang Tang".to_string()),
                Extraction::new("author".to_string(), "Kebing Jin".to_string()),
                Extraction::new("author".to_string(), "Hankz Hankui Zhuo".to_string()),
                Extraction::new("arxiv_id".to_string(), "arXiv:2508.11524v1".to_string()),
                Extraction::new("category".to_string(), "cs.AI".to_string()),
                Extraction::new("publication_date".to_string(), "15 Aug 2025".to_string()),
                Extraction::new("research_domain".to_string(), "Classical Planning".to_string()),
                Extraction::new("research_domain".to_string(), "Large Language Models".to_string()),
            ],
        ),
        ExampleData::new(
            "This paper proposes two novel paradigms: LLM4Inspire and LLM4Predict, for integrating LLMs into classical planning frameworks. LLM4Inspire uses LLMs to provide heuristic guidance, while LLM4Predict employs domain-specific knowledge to predict intermediate states.".to_string(),
            vec![
                Extraction::new("proposed_method".to_string(), "LLM4Inspire".to_string()),
                Extraction::new("proposed_method".to_string(), "LLM4Predict".to_string()),
                Extraction::new("method_description".to_string(), "LLM4Inspire uses LLMs to provide heuristic guidance".to_string()),
                Extraction::new("method_description".to_string(), "LLM4Predict employs domain-specific knowledge to predict intermediate states".to_string()),
                Extraction::new("integration_type".to_string(), "heuristic guidance".to_string()),
                Extraction::new("integration_type".to_string(), "domain-specific knowledge".to_string()),
                Extraction::new("integration_type".to_string(), "intermediate state prediction".to_string()),
            ],
        ),
        ExampleData::new(
            "The experiments evaluate performance across four domains: Blocks (50 instances), Logistics (42 instances), Depot (22 instances), and Mystery Round 1 (30 instances). LLM4Predict shows superior performance with 95%+ success rates on Blocks, Logistics, and Depot domains.".to_string(),
            vec![
                Extraction::new("domain".to_string(), "Blocks".to_string()),
                Extraction::new("domain".to_string(), "Logistics".to_string()),
                Extraction::new("domain".to_string(), "Depot".to_string()),
                Extraction::new("domain".to_string(), "Mystery Round 1".to_string()),
                Extraction::new("domain_instances".to_string(), "Blocks: 50 instances".to_string()),
                Extraction::new("domain_instances".to_string(), "Logistics: 42 instances".to_string()),
                Extraction::new("domain_instances".to_string(), "Depot: 22 instances".to_string()),
                Extraction::new("domain_instances".to_string(), "Mystery Round 1: 30 instances".to_string()),
                Extraction::new("success_rate".to_string(), "95%+".to_string()),
                Extraction::new("best_method".to_string(), "LLM4Predict".to_string()),
            ],
        ),
        ExampleData::new(
            "The paper compares LLM4Predict against Fast Downward, DeepSeek-R1, and LLM4Inspire baselines. Results show LLM4Predict achieves 49/50 success rate on Blocks domain, 42/42 on Logistics, and 19/22 on Depot, significantly outperforming other methods.".to_string(),
            vec![
                Extraction::new("baseline_method".to_string(), "Fast Downward".to_string()),
                Extraction::new("baseline_method".to_string(), "DeepSeek-R1".to_string()),
                Extraction::new("baseline_method".to_string(), "LLM4Inspire".to_string()),
                Extraction::new("target_method".to_string(), "LLM4Predict".to_string()),
                Extraction::new("blocks_success".to_string(), "49/50".to_string()),
                Extraction::new("logistics_success".to_string(), "42/42".to_string()),
                Extraction::new("depot_success".to_string(), "19/22".to_string()),
                Extraction::new("performance_comparison".to_string(), "significantly outperforms other methods".to_string()),
            ],
        ),
        ExampleData::new(
            "Key findings: LLM4Predict requires fewer LLM calls and less solver time compared to LLM4Inspire. The approach demonstrates that domain-specific constraints are crucial for effective LLM integration in planning tasks.".to_string(),
            vec![
                Extraction::new("key_finding".to_string(), "LLM4Predict requires fewer LLM calls than LLM4Inspire".to_string()),
                Extraction::new("key_finding".to_string(), "LLM4Predict requires less solver time than LLM4Inspire".to_string()),
                Extraction::new("key_finding".to_string(), "Domain-specific constraints are crucial for effective LLM integration".to_string()),
                Extraction::new("efficiency_metric".to_string(), "fewer LLM calls".to_string()),
                Extraction::new("efficiency_metric".to_string(), "less solver time".to_string()),
                Extraction::new("conclusion".to_string(), "Domain-specific constraints are crucial for effective LLM integration in planning tasks".to_string()),
            ],
        ),
        ExampleData::new(
            "Technical details: The system uses problem decomposition with directed acyclic dependency graphs (DADGs) to divide large planning problems into subproblems. Each subproblem is solved using existing planners like Fast Downward.".to_string(),
            vec![
                Extraction::new("technical_approach".to_string(), "problem decomposition".to_string()),
                Extraction::new("technical_approach".to_string(), "directed acyclic dependency graphs".to_string()),
                Extraction::new("technical_component".to_string(), "DADGs".to_string()),
                Extraction::new("solver_used".to_string(), "Fast Downward".to_string()),
                Extraction::new("decomposition_method".to_string(), "divide large planning problems into subproblems".to_string()),
            ],
        ),
    ]
}

async fn test_academic_extraction(provider_config: ProviderConfig, provider_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📚 Testing Academic Paper Extraction with {}", provider_name);
    println!("{}", "=".repeat(60));

    // Read the academic paper file
    let paper_text = std::fs::read_to_string("examples/system_design.txt")?;
    println!("📄 Loaded academic paper: {} characters", paper_text.len());
    
    // Show first few lines for context
    let preview_lines: Vec<&str> = paper_text.lines().take(10).collect();
    println!("📖 Paper preview:");
    for (i, line) in preview_lines.iter().enumerate() {
        if !line.trim().is_empty() {
            println!("   {}: {}", i+1, line.trim());
            if i >= 5 { break; } // Show max 6 non-empty lines
        }
    }
    println!();

    let mut extract_config = ExtractConfig {
        model_id: provider_config.model.clone(),
        api_key: provider_config.api_key.clone(),
        format_type: FormatType::Json,
        temperature: 0.2, // Low temperature for consistent academic extraction
        fence_output: None,
        use_schema_constraints: false,
        debug: true,
        model_url: Some(provider_config.base_url.clone()),
        
        // Token-based chunking configuration optimized for academic papers
        max_char_buffer: 2000, // Larger chunks for academic context
        batch_length: 4,       // Process 4 chunks in parallel
        max_workers: 6,        // Concurrent workers for faster processing
        
        // Single-pass extraction for academic content
        extraction_passes: 1,
        enable_multipass: false, // Can be enabled for very complex papers
        multipass_min_extractions: 5,
        multipass_quality_threshold: 0.8,
        
        additional_context: Some("Extract specific information from this LLM-assisted planning research paper. Focus on: paper title, all authors, arXiv ID, publication date, research domains, proposed methods (LLM4Inspire, LLM4Predict), method descriptions, evaluation domains (Blocks, Logistics, Depot, Mystery), instance counts, success rates, baseline methods (Fast Downward, DeepSeek-R1), performance comparisons, key findings, efficiency metrics, technical approaches, and conclusions.".to_string()),
        resolver_params: std::collections::HashMap::new(),
        language_model_params: std::collections::HashMap::new(),
    };
    
    // Set provider configuration
    extract_config.language_model_params.insert(
        "provider_config".to_string(),
        serde_json::to_value(&provider_config)?,
    );

    let examples = create_academic_examples();
    let start_time = std::time::Instant::now();

    println!("🔄 Starting academic extraction with token-based chunking...");
    println!("   Configuration:");
    println!("   - Max chars per buffer: {} (respects sentence boundaries)", extract_config.max_char_buffer);
    println!("   - Batch size: {}", extract_config.batch_length);
    println!("   - Workers: {}", extract_config.max_workers);
    println!("   - Multi-pass: {}", extract_config.enable_multipass);
    println!();
    
    match extract(
        &paper_text,
        Some("Extract specific details from this LLM-assisted planning paper: title, authors, arXiv info, methods (LLM4Inspire/LLM4Predict), experimental results, performance metrics, domains tested (Blocks/Logistics/Depot/Mystery), success rates, and key findings"),
        &examples,
        extract_config,
    ).await {
        Ok(annotated_document) => {
            let processing_time = start_time.elapsed();
            let extraction_count = annotated_document.extraction_count();

            println!("✅ Extraction completed in {:?}", processing_time);
            println!("📊 Found {} total extractions", extraction_count);

            // Analyze extraction categories
            if let Some(extractions) = &annotated_document.extractions {
                let mut category_counts = std::collections::HashMap::new();
                for extraction in extractions {
                    *category_counts.entry(&extraction.extraction_class).or_insert(0) += 1;
                }

                println!("\n📈 Academic Information Breakdown:");
                let mut sorted_categories: Vec<_> = category_counts.iter().collect();
                sorted_categories.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
                
                for (category, count) in &sorted_categories {
                    println!("   • {}: {} items", category, count);
                }

                // Show sample extractions by category (LLM planning specific)
                println!("\n🔍 Sample Extractions by Category:");
                let mut shown_categories = std::collections::HashSet::new();

                for extraction in extractions.iter() {
                    if !shown_categories.contains(&extraction.extraction_class) && shown_categories.len() < 15 {
                        println!("   [{}] {}", extraction.extraction_class, extraction.extraction_text);
                        shown_categories.insert(extraction.extraction_class.clone());
                    }
                }

                if shown_categories.len() < category_counts.len() {
                    println!("   ... and {} more categories", category_counts.len() - shown_categories.len());
                }

                // LLM Planning paper-specific analysis
                println!("\n👥 Author Analysis:");
                let authors: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "author")
                    .collect();
                println!("   Found {} authors: {:?}", authors.len(), authors.iter().map(|e| &e.extraction_text).collect::<Vec<_>>());

                println!("\n📋 Method Analysis:");
                let methods: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "proposed_method")
                    .collect();
                println!("   Found {} proposed methods: {:?}", methods.len(), methods.iter().map(|e| &e.extraction_text).collect::<Vec<_>>());

                let descriptions: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "method_description")
                    .collect();
                if !descriptions.is_empty() {
                    println!("   Method descriptions:");
                    for desc in descriptions.iter().take(3) {
                        println!("     • {}", desc.extraction_text);
                    }
                }

                println!("\n🏆 Performance Analysis:");
                let success_rates: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "success_rate")
                    .collect();
                if !success_rates.is_empty() {
                    println!("   Success rates: {:?}", success_rates.iter().map(|e| &e.extraction_text).collect::<Vec<_>>());
                }

                let domains: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "domain")
                    .collect();
                if !domains.is_empty() {
                    println!("   Test domains: {:?}", domains.iter().map(|e| &e.extraction_text).collect::<Vec<_>>());
                }

                println!("\n📊 Comparison Analysis:");
                let baselines: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "baseline_method")
                    .collect();
                if !baselines.is_empty() {
                    println!("   Baseline methods: {:?}", baselines.iter().map(|e| &e.extraction_text).collect::<Vec<_>>());
                }

                let findings: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class == "key_finding")
                    .collect();
                if !findings.is_empty() {
                    println!("   Key findings:");
                    for finding in findings.iter().take(5) {
                        println!("     • {}", finding.extraction_text);
                    }
                }
            }

            // Generate LLM planning-focused reports
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let base_filename = format!("llm_planning_paper_{}_{}", provider_name.to_lowercase().replace(" ", "_"), timestamp);

            // Generate HTML report with LLM planning focus
            let html_config = ExportConfig {
                format: ExportFormat::Html,
                title: Some(format!("LLM Planning Paper Analysis - {} ({} extractions)", provider_name, extraction_count)),
                highlight_extractions: true,
                show_char_intervals: false, // Skip intervals for cleaner academic view
                include_statistics: true,
                custom_css: Some(r#"
                    .author-highlight { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
                    .paper_title-highlight { background: #f59e0b; color: white; font-weight: bold; }
                    .proposed_method-highlight { background: #10b981; color: white; }
                    .domain-highlight { background: #3b82f6; color: white; }
                    .success_rate-highlight { background: #8b5cf6; color: white; }
                    .key_finding-highlight { background: #ef4444; color: white; }
                    .baseline_method-highlight { background: #f97316; color: white; }
                    .extraction-class {
                        font-size: 0.85em;
                        text-transform: capitalize;
                        letter-spacing: 0.3px;
                    }
                "#.to_string()),
                ..Default::default()
            };

            if let Ok(html_output) = export_document(&annotated_document, &html_config) {
                let html_file = format!("{}.html", base_filename);
                std::fs::write(&html_file, html_output)?;
                println!("\n📄 Interactive HTML report: {}", html_file);
            }

            // Generate structured JSON for research analysis
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

            // Generate CSV for research data analysis
            let csv_config = ExportConfig {
                format: ExportFormat::Csv,
                show_char_intervals: false,
                ..Default::default()
            };

            if let Ok(csv_output) = export_document(&annotated_document, &csv_config) {
                let csv_file = format!("{}.csv", base_filename);
                std::fs::write(&csv_file, csv_output)?;
                println!("📊 Research data CSV: {}", csv_file);
            }

            println!("\n🎉 LLM Planning paper analysis complete!");
            println!("💡 Analysis Focus:");
            println!("   • Open the .html file to see highlighted paper content with LLM planning focus");
            println!("   • Use the .csv file for structured analysis of methods, domains, and performance");
            println!("   • Check the .json file for programmatic processing of research findings");
            println!("   • Key insights: LLM4Predict vs LLM4Inspire comparison, domain-specific constraints");
            println!("   • Performance metrics: Success rates across Blocks, Logistics, Depot, Mystery domains");

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
    println!("🤖 LangExtract LLM Planning Paper Extraction Test");
    println!("🎯 Specialized test for extracting research information from the LLM-assisted planning paper");
    println!("{}", "=".repeat(90));

    // Check if the academic paper file exists
    if !std::path::Path::new("examples/system_design.txt").exists() {
        println!("❌ Error: examples/system_design.txt not found!");
        println!("   Please ensure the LLM planning paper file is in the examples directory.");
        return Ok(());
    }

    // Try to find an available provider
    if let Some((config, provider_name)) = create_provider_config().await {
        println!("✅ Using provider: {}", provider_name);
        println!("🔧 Model: {}", config.model);
        
        test_academic_extraction(config, &provider_name).await?;
    } else {
        println!("❌ No language model providers available!");
        println!("\nTo run this test, please configure a provider:");
        println!("  • OpenAI: export OPENAI_API_KEY=your_openai_key");
        println!("  • Ollama: ollama serve && ollama pull mistral");
        println!("\nThis test is optimized for the LLM-assisted planning paper and extracts:");
        println!("  📝 Paper title, authors, arXiv ID, publication date");
        println!("  🤖 Proposed methods (LLM4Inspire, LLM4Predict)");
        println!("  🧪 Experimental domains (Blocks, Logistics, Depot, Mystery)");
        println!("  📊 Success rates, performance comparisons, baseline methods");
        println!("  🔬 Key findings, technical approaches, efficiency metrics");
        println!("  📋 Method descriptions and research conclusions");
    }

    Ok(())
}
