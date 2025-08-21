//! Advanced chunking demo with multi-pass extraction
//!
//! This example demonstrates the advanced features of the semantic chunking system:
//! - AI-powered content understanding for intelligent boundaries
//! - Multi-pass extraction for improved recall
//! - Parallel chunk processing
//! - Result aggregation and deduplication

use langextract_rust::{extract, ExampleData, Extraction, ExtractConfig, FormatType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("🚀 Advanced Semantic Chunking Demo\n");

    // Create example data to guide the extraction
    let examples = vec![
        ExampleData::new(
            "John Doe is 30 years old and works as a software engineer at Google".to_string(),
            vec![
                Extraction::new("person_name".to_string(), "John Doe".to_string()),
                Extraction::new("age".to_string(), "30".to_string()),
                Extraction::new("job_title".to_string(), "software engineer".to_string()),
                Extraction::new("company".to_string(), "Google".to_string()),
            ],
        ),
        ExampleData::new(
            "Dr. Sarah Wilson, 45, is a cardiologist at Stanford Hospital".to_string(),
            vec![
                Extraction::new("person_name".to_string(), "Dr. Sarah Wilson".to_string()),
                Extraction::new("age".to_string(), "45".to_string()),
                Extraction::new("job_title".to_string(), "cardiologist".to_string()),
                Extraction::new("company".to_string(), "Stanford Hospital".to_string()),
            ],
        ),
    ];

    // Create a complex document that benefits from chunking
    let large_text = create_complex_test_document();
    
    println!("📄 Document Statistics:");
    println!("   Total Length: {} characters", large_text.len());
    println!("   Word Count: ~{} words", large_text.split_whitespace().count());
    println!("   Estimated sentences: {}", large_text.matches('.').count());
    println!();

    // Configuration for advanced semantic chunking with multi-pass
    let config = ExtractConfig {
        model_id: "gpt-3.5-turbo".to_string(), // Replace with your preferred model
        api_key: None,  // Load from environment
        format_type: FormatType::Json,
        temperature: 0.1,  // Low temperature for consistency across passes
        use_schema_constraints: false,
        debug: true,
        
        // Advanced semantic chunking configuration
        max_char_buffer: 1200,     // Larger chunks for better context
        batch_length: 4,           // Process 4 chunks per batch
        max_workers: 6,            // Use 6 concurrent workers
        
        // Multi-pass extraction settings
        enable_multipass: true,    // Enable multi-pass for better recall
        extraction_passes: 2,      // Run 2 extraction passes
        multipass_min_extractions: 2,   // Minimum extractions to trigger second pass
        multipass_quality_threshold: 0.4, // Quality threshold for extractions
        
        ..Default::default()
    };

    println!("⚙️  Advanced Configuration:");
    println!("   Model: {}", config.model_id);
    println!("   Semantic chunking: Enabled");
    println!("   Max chars per buffer: {} (respects semantic boundaries)", config.max_char_buffer);
    println!("   Batch processing: {} chunks per batch", config.batch_length);
    println!("   Concurrent workers: {}", config.max_workers);
    println!("   Multi-pass extraction: {}", config.enable_multipass);
    println!("   Extraction passes: {}", config.extraction_passes);
    println!("   Quality threshold: {}", config.multipass_quality_threshold);
    println!();

    println!("🔄 Starting advanced extraction pipeline...");
    println!("   Phase 1: Semantic analysis and content understanding");
    println!("   Phase 2: AI-powered chunk creation with semantic boundaries");
    println!("   Phase 3: Parallel chunk processing");
    if config.enable_multipass {
        println!("   Phase 4: Multi-pass refinement");
    }
    println!("   Phase 5: Result aggregation and deduplication");
    println!();

    // Perform extraction with advanced chunking
    match extract(
        &large_text,
        Some("Extract person names, ages, job titles, companies, and locations from the text. Focus on finding all mentions, including partial names and indirect references."),
        &examples,
        config,
    ).await {
        Ok(result) => {
            println!("✅ Advanced extraction completed successfully!");
            println!("📊 Results Summary:");
            println!("   Total extractions found: {}", result.extraction_count());
            
            if let Some(extractions) = &result.extractions {
                // Group extractions by type for analysis
                let mut by_type = std::collections::HashMap::new();
                for extraction in extractions {
                    by_type.entry(extraction.extraction_class.clone())
                        .or_insert_with(Vec::new)
                        .push(extraction);
                }

                println!("\n📈 Extraction Breakdown by Type:");
                for (extraction_type, items) in &by_type {
                    println!("   {}: {} instances", extraction_type, items.len());
                }

                println!("\n🎯 Sample Extractions (first 15):");
                for (i, extraction) in extractions.iter().take(15).enumerate() {
                    println!("{}. [{}] {} (pos: {:?})", 
                        i + 1, 
                        extraction.extraction_class, 
                        extraction.extraction_text,
                        extraction.char_interval.as_ref().map(|ci| format!("{}-{}", 
                            ci.start_pos.unwrap_or(0), 
                            ci.end_pos.unwrap_or(0)))
                    );
                }
                
                if extractions.len() > 15 {
                    println!("   ... and {} more", extractions.len() - 15);
                }
            }

            // Demonstrate visualization of chunked results
            match langextract_rust::visualize(&result, false) {
                Ok(viz) => {
                    println!("\n📄 Document Visualization (first 25 lines):");
                    let lines: Vec<&str> = viz.lines().take(25).collect();
                    println!("{}", lines.join("\n"));
                    if viz.lines().count() > 25 {
                        println!("   ... (truncated for display, {} total lines)", viz.lines().count());
                    }
                }
                Err(e) => {
                    println!("\n⚠️  Visualization generation failed: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Advanced extraction failed: {}", e);
            
            // Provide debugging guidance
            println!("\n🔍 Troubleshooting Guide:");
            println!("1. Verify API credentials are correctly set");
            println!("2. Check model availability and access permissions");
            println!("3. Ensure sufficient API rate limits for parallel processing");
            println!("4. Try reducing batch_length or max_workers if rate limited");
            println!("5. Disable multi-pass (enable_multipass: false) for simpler debugging");
            
            return Err(e.into());
        }
    }

    Ok(())
}

/// Create a complex test document with various entities and challenging structures
fn create_complex_test_document() -> String {
    let mut document = String::new();
    
    document.push_str("# Technology Industry Leadership Analysis Report\n\n");
    
    document.push_str("## Executive Summary\n\n");
    document.push_str("This comprehensive analysis examines key leadership personnel across major technology companies and research institutions. ");
    document.push_str("The study includes professionals from diverse backgrounds, ranging from startup founders to established corporate executives, ");
    document.push_str("as well as academic researchers driving innovation in artificial intelligence, biotechnology, and sustainable computing.\n\n");

    // Add complex sentences with multiple entities
    let complex_entries = vec![
        "Dr. Alexandra Chen, age 42, serves as Chief Technology Officer at Meta Platforms, where she previously worked as a senior software architect before her promotion in 2021. Prior to Meta, Chen was a research scientist at Google DeepMind for five years.",
        "The startup ecosystem has been significantly influenced by entrepreneurs like Marcus Rodriguez, 35, who founded three successful companies including DataFlow Inc. (acquired by Amazon for $2.1B) and currently serves as CEO of Neural Networks Corp in Palo Alto.",
        "In academic circles, Professor Emily Watson (Stanford University, Department of Computer Science) has made groundbreaking contributions to machine learning. Watson, who is 51 years old, collaborates frequently with Dr. James Kim from MIT's AI Lab.",
        "The biotechnology sector has seen remarkable growth under leaders such as Dr. Sarah Thompson, 38, who transitioned from being a research director at Pfizer to founding BioInnovate Labs. Thompson's work focuses on AI-driven drug discovery.",
        "Notable venture capitalists include David Park, 45, managing partner at Silicon Valley Ventures, and Lisa Chang, 39, who leads the AI investment team at Andreessen Horowitz. Both have been instrumental in funding breakthrough technologies.",
        "International collaborations have flourished with experts like Dr. Hiroshi Tanaka from Tokyo Institute of Technology (age 48) working alongside Professor Maria Santos, 44, from Barcelona's Institute for Advanced Studies.",
        "The clean energy transition has been accelerated by innovators including CEO Jennifer Wilson, 36, of SolarTech Dynamics, and CTO Michael Brown, 41, from Wind Power Solutions. Wilson previously held engineering positions at Tesla and SpaceX.",
        "Emerging leaders in quantum computing include Dr. Robert Johnson, 29, a prodigy who completed his PhD at Oxford at age 24 and now leads the quantum research division at IBM. Johnson works closely with senior researcher Dr. Anna Kowalski, 52.",
        "The cybersecurity landscape is shaped by professionals like former NSA analyst turned entrepreneur Kevin Lee, 37, who founded SecureNet Technologies, and Chief Security Officer Rachel Green, 43, at CloudGuard Systems.",
        "Cross-industry innovation has been driven by polymaths like Dr. Thomas Anderson, 55, who holds positions as professor emeritus at Caltech, advisor to NASA's JPL, and board member of three aerospace startups including Stellar Dynamics Corp.",
    ];

    document.push_str("## Detailed Personnel Analysis\n\n");
    
    for (i, entry) in complex_entries.iter().enumerate() {
        if i % 3 == 0 {
            document.push_str(&format!("### Section {}: Industry Focus Area\n\n", (i / 3) + 1));
        }
        
        document.push_str(entry);
        document.push_str(" ");
        
        // Add contextual information to make extraction more challenging
        match i % 4 {
            0 => document.push_str("Their leadership style emphasizes collaborative innovation and cross-functional team development. "),
            1 => document.push_str("The company has seen 300% revenue growth under their guidance over the past two years. "),
            2 => document.push_str("Recent publications include groundbreaking research in Nature and Science journals. "),
            _ => document.push_str("Industry recognition includes multiple awards for technical excellence and business innovation. "),
        }
        
        document.push_str("\n\n");
    }

    // Add conclusion with additional entities
    document.push_str("## Future Outlook\n\n");
    document.push_str("The technology industry continues to evolve rapidly, with emerging leaders like ");
    document.push_str("startup founder Alex Rivera, 28, of QuantumLeap AI, and veteran executive Susan Davis, 59, ");
    document.push_str("recently appointed as CEO of TechGlobal Corporation. These leaders represent both the ");
    document.push_str("entrepreneurial spirit of Silicon Valley and the institutional knowledge of established firms.\n\n");
    
    document.push_str("Collaboration between industry and academia remains crucial, exemplified by partnerships ");
    document.push_str("between companies like those led by CTO Mark Thompson, 44 (DataCorp), and research ");
    document.push_str("institutions headed by academics such as Dr. Linda Garcia, 50 (Harvard Medical School), ");
    document.push_str("who specializes in computational biology and serves on multiple corporate advisory boards.");

    document
}