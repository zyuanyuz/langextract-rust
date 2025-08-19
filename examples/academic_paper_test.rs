use langextract_rust::{
    data::{ExampleData, FormatType, Extraction},
    providers::ProviderConfig,
    visualization::{export_document, ExportConfig, ExportFormat},
    extract, ExtractConfig,
};
use std::env;
use tokio;

// Academic paper examples to guide the model in extracting research-related information
fn create_academic_examples() -> Vec<ExampleData> {
    vec![
        ExampleData::new(
            "The paper 'Attention Is All You Need' by Vaswani et al. (2017) introduced the Transformer architecture. arXiv:1706.03762 [cs.CL]. The model achieved a BLEU score of 28.4 on WMT 2014 English-to-German translation.".to_string(),
            vec![
                Extraction::new("title".to_string(), "Attention Is All You Need".to_string()),
                Extraction::new("author".to_string(), "Vaswani et al.".to_string()),
                Extraction::new("year".to_string(), "2017".to_string()),
                Extraction::new("arxiv_id".to_string(), "arXiv:1706.03762".to_string()),
                Extraction::new("category".to_string(), "cs.CL".to_string()),
                Extraction::new("metric".to_string(), "BLEU score".to_string()),
                Extraction::new("score".to_string(), "28.4".to_string()),
                Extraction::new("dataset".to_string(), "WMT 2014 English-to-German".to_string()),
                Extraction::new("contribution".to_string(), "Transformer architecture".to_string()),
            ],
        ),
        ExampleData::new(
            "Abstract: Recent advances in deep learning have shown remarkable success in computer vision tasks. This work proposes a novel convolutional neural network (CNN) architecture that achieves 95.2% accuracy on ImageNet classification. The method is evaluated on CIFAR-10 and CIFAR-100 datasets.".to_string(),
            vec![
                Extraction::new("section".to_string(), "Abstract".to_string()),
                Extraction::new("domain".to_string(), "computer vision".to_string()),
                Extraction::new("method".to_string(), "convolutional neural network".to_string()),
                Extraction::new("architecture".to_string(), "CNN".to_string()),
                Extraction::new("accuracy".to_string(), "95.2%".to_string()),
                Extraction::new("dataset".to_string(), "ImageNet".to_string()),
                Extraction::new("task".to_string(), "classification".to_string()),
                Extraction::new("evaluation_dataset".to_string(), "CIFAR-10".to_string()),
                Extraction::new("evaluation_dataset".to_string(), "CIFAR-100".to_string()),
            ],
        ),
        ExampleData::new(
            "License: CC BY-NC-SA 4.0. The research was conducted at MIT Computer Science and Artificial Intelligence Laboratory (CSAIL). Contact: john.doe@mit.edu. Funding provided by NSF Grant #1234567.".to_string(),
            vec![
                Extraction::new("license".to_string(), "CC BY-NC-SA 4.0".to_string()),
                Extraction::new("institution".to_string(), "MIT Computer Science and Artificial Intelligence Laboratory".to_string()),
                Extraction::new("institution_abbreviation".to_string(), "CSAIL".to_string()),
                Extraction::new("email".to_string(), "john.doe@mit.edu".to_string()),
                Extraction::new("funding_source".to_string(), "NSF".to_string()),
                Extraction::new("grant_number".to_string(), "#1234567".to_string()),
            ],
        ),
        ExampleData::new(
            "Related Work: Previous approaches [12, 25, 34] have addressed this problem using traditional machine learning methods. However, recent work by Zhang et al. [45] demonstrated significant improvements using transformer-based models. The baseline method achieved F1-score of 0.82 while our approach reaches 0.91.".to_string(),
            vec![
                Extraction::new("section".to_string(), "Related Work".to_string()),
                Extraction::new("citation".to_string(), "[12, 25, 34]".to_string()),
                Extraction::new("method_type".to_string(), "traditional machine learning".to_string()),
                Extraction::new("recent_author".to_string(), "Zhang et al.".to_string()),
                Extraction::new("recent_citation".to_string(), "[45]".to_string()),
                Extraction::new("recent_method".to_string(), "transformer-based models".to_string()),
                Extraction::new("baseline_metric".to_string(), "F1-score".to_string()),
                Extraction::new("baseline_score".to_string(), "0.82".to_string()),
                Extraction::new("our_score".to_string(), "0.91".to_string()),
            ],
        ),
        ExampleData::new(
            "Table 1: Performance comparison on benchmark datasets. Our method (Ours) vs Baseline shows improvements of 12.3% on COCO, 8.7% on Pascal VOC, and 15.1% on Open Images. Statistical significance tested with p < 0.05.".to_string(),
            vec![
                Extraction::new("table_reference".to_string(), "Table 1".to_string()),
                Extraction::new("comparison_type".to_string(), "Performance comparison".to_string()),
                Extraction::new("method_name".to_string(), "Ours".to_string()),
                Extraction::new("baseline".to_string(), "Baseline".to_string()),
                Extraction::new("improvement".to_string(), "12.3%".to_string()),
                Extraction::new("dataset".to_string(), "COCO".to_string()),
                Extraction::new("improvement".to_string(), "8.7%".to_string()),
                Extraction::new("dataset".to_string(), "Pascal VOC".to_string()),
                Extraction::new("improvement".to_string(), "15.1%".to_string()),
                Extraction::new("dataset".to_string(), "Open Images".to_string()),
                Extraction::new("statistical_test".to_string(), "p < 0.05".to_string()),
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
        
        additional_context: Some("Extract detailed academic and research information including authors, institutions, citations, methodologies, datasets, metrics, scores, technical terms, and research contributions from this academic paper. Focus on identifying specific names, numbers, references, and technical concepts.".to_string()),
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
        Some("Extract comprehensive academic information from this research paper"),
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

                // Show sample extractions by category
                println!("\n🔍 Sample Extractions by Category:");
                let mut shown_categories = std::collections::HashSet::new();
                
                for extraction in extractions.iter() {
                    if !shown_categories.contains(&extraction.extraction_class) && shown_categories.len() < 12 {
                        println!("   [{}] {}", extraction.extraction_class, extraction.extraction_text);
                        shown_categories.insert(extraction.extraction_class.clone());
                    }
                }
                
                if shown_categories.len() < category_counts.len() {
                    println!("   ... and {} more categories", category_counts.len() - shown_categories.len());
                }

                // Academic-specific analysis
                println!("\n👥 Author & Citation Analysis:");
                let authors: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class.contains("author") || e.extraction_class.contains("researcher"))
                    .collect();
                println!("   Found {} author/researcher mentions", authors.len());

                let citations: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class.contains("citation") || e.extraction_class.contains("reference") || e.extraction_class.contains("arxiv"))
                    .collect();
                println!("   Found {} citations/references", citations.len());

                println!("\n🔬 Research Content Analysis:");
                let methods: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class.contains("method") || e.extraction_class.contains("algorithm") || e.extraction_class.contains("approach"))
                    .collect();
                println!("   Found {} methodology references", methods.len());

                let datasets: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class.contains("dataset") || e.extraction_class.contains("benchmark"))
                    .collect();
                println!("   Found {} dataset/benchmark mentions", datasets.len());

                let metrics: Vec<_> = extractions.iter()
                    .filter(|e| e.extraction_class.contains("metric") || e.extraction_class.contains("score") || e.extraction_class.contains("accuracy") || e.extraction_class.contains("performance"))
                    .collect();
                println!("   Found {} performance metrics", metrics.len());
            }

            // Generate academic-focused reports
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let base_filename = format!("academic_paper_{}_{}", provider_name.to_lowercase().replace(" ", "_"), timestamp);

            // Generate HTML report with academic focus
            let html_config = ExportConfig {
                format: ExportFormat::Html,
                title: Some(format!("Academic Paper Analysis - {} ({} extractions)", provider_name, extraction_count)),
                highlight_extractions: true,
                show_char_intervals: false, // Skip intervals for cleaner academic view
                include_statistics: true,
                custom_css: Some(r#"
                    .author-highlight { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
                    .citation-highlight { background: #f59e0b; color: white; }
                    .method-highlight { background: #10b981; color: white; }
                    .dataset-highlight { background: #3b82f6; color: white; }
                    .metric-highlight { background: #8b5cf6; color: white; }
                    .institution-highlight { background: #ef4444; color: white; }
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

            println!("\n🎉 Academic paper analysis complete!");
            println!("💡 Tips:");
            println!("   • Open the .html file to see highlighted academic content in context");
            println!("   • Use the .csv file for academic data analysis and categorization");
            println!("   • Check the .json file for programmatic research data processing");
            println!("   • Look for author networks, citation patterns, and methodology trends");

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
    println!("📚 LangExtract Academic Paper Extraction Test");
    println!("🎯 Specialized test for extracting research information from academic papers");
    println!("{}", "=".repeat(80));

    // Check if the academic paper file exists
    if !std::path::Path::new("examples/system_design.txt").exists() {
        println!("❌ Error: examples/system_design.txt not found!");
        println!("   Please ensure the academic paper file is in the examples directory.");
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
        println!("\nThis test is optimized for:");
        println!("  📝 Academic paper titles and abstracts");
        println!("  👥 Author names and institutional affiliations");
        println!("  📚 Citations and reference information");
        println!("  🔬 Research methodologies and approaches");
        println!("  📊 Performance metrics and experimental results");
        println!("  🗃️ Dataset and benchmark mentions");
        println!("  🏷️ Technical terms and domain-specific vocabulary");
    }

    Ok(())
}
