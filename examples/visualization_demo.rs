use langextract_rust::{
    data::{AnnotatedDocument, Extraction, CharInterval, AlignmentStatus},
    visualization::{export_document, ExportConfig, ExportFormat},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create sample annotated document
    let sample_text = "John Smith, age 25, works at TechCorp and earns $75,000 annually. His email is john.smith@techcorp.com and he lives in San Francisco, CA.";
    
    let extractions = vec![
        Extraction {
            extraction_class: "person".to_string(),
            extraction_text: "John Smith".to_string(),
            char_interval: Some(CharInterval::new(Some(0), Some(10))),
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(0),
            group_index: Some(0),
            description: Some("Full name of the person".to_string()),
            attributes: Some(std::collections::HashMap::new()),
            token_interval: None,
        },
        Extraction {
            extraction_class: "age".to_string(),
            extraction_text: "25".to_string(),
            char_interval: Some(CharInterval::new(Some(16), Some(18))),
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(1),
            group_index: Some(0),
            description: Some("Age in years".to_string()),
            attributes: Some(std::collections::HashMap::new()),
            token_interval: None,
        },
        Extraction {
            extraction_class: "company".to_string(),
            extraction_text: "TechCorp".to_string(),
            char_interval: Some(CharInterval::new(Some(29), Some(37))),
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(2),
            group_index: Some(0),
            description: Some("Employer company".to_string()),
            attributes: Some(std::collections::HashMap::new()),
            token_interval: None,
        },
        Extraction {
            extraction_class: "salary".to_string(),
            extraction_text: "$75,000".to_string(),
            char_interval: Some(CharInterval::new(Some(48), Some(55))),
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(3),
            group_index: Some(0),
            description: Some("Annual salary".to_string()),
            attributes: Some(std::collections::HashMap::new()),
            token_interval: None,
        },
        Extraction {
            extraction_class: "email".to_string(),
            extraction_text: "john.smith@techcorp.com".to_string(),
            char_interval: Some(CharInterval::new(Some(79), Some(102))),
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(4),
            group_index: Some(0),
            description: Some("Email address".to_string()),
            attributes: Some(std::collections::HashMap::new()),
            token_interval: None,
        },
        Extraction {
            extraction_class: "location".to_string(),
            extraction_text: "San Francisco, CA".to_string(),
            char_interval: Some(CharInterval::new(Some(119), Some(136))),
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(5),
            group_index: Some(0),
            description: Some("City and state".to_string()),
            attributes: Some(std::collections::HashMap::new()),
            token_interval: None,
        },
    ];

    let annotated_document = AnnotatedDocument {
        document_id: Some("demo_doc_001".to_string()),
        text: Some(sample_text.to_string()),
        extractions: Some(extractions),

    };

    println!("🎨 LangExtract Rich Visualization System Demo");
    println!("{}", "=".repeat(50));
    println!();

    // 1. Text Export (Original format)
    println!("1️⃣  TEXT FORMAT");
    println!("{}", "-".repeat(20));
    let text_config = ExportConfig {
        format: ExportFormat::Text,
        show_char_intervals: true,
        title: Some("LangExtract Demo - Text Format".to_string()),
        ..Default::default()
    };
    let text_output = export_document(&annotated_document, &text_config)?;
    println!("{}", text_output);

    // 2. HTML Export
    println!("2️⃣  HTML FORMAT");
    println!("{}", "-".repeat(20));
    let html_config = ExportConfig {
        format: ExportFormat::Html,
        show_char_intervals: true,
        highlight_extractions: true,
        title: Some("LangExtract Demo - Rich HTML Export".to_string()),
        custom_css: Some("
            body { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); }
            .container { margin-top: 20px; }
        ".to_string()),
        ..Default::default()
    };
    let html_output = export_document(&annotated_document, &html_config)?;
    
    // Save HTML to file
    std::fs::write("langextract_demo.html", &html_output)?;
    println!("✅ HTML export saved to 'langextract_demo.html'");
    println!("   Preview: {} characters of rich HTML with interactive highlighting", html_output.len());

    // 3. Markdown Export
    println!();
    println!("3️⃣  MARKDOWN FORMAT");
    println!("{}", "-".repeat(20));
    let markdown_config = ExportConfig {
        format: ExportFormat::Markdown,
        show_char_intervals: true,
        highlight_extractions: true,
        title: Some("LangExtract Demo - Structured Markdown".to_string()),
        ..Default::default()
    };
    let markdown_output = export_document(&annotated_document, &markdown_config)?;
    
    // Save Markdown to file
    std::fs::write("langextract_demo.md", &markdown_output)?;
    println!("✅ Markdown export saved to 'langextract_demo.md'");
    println!("   Preview:");
    let preview = &markdown_output[..markdown_output.len().min(500)];
    println!("{}{}", preview, if markdown_output.len() > 500 { "..." } else { "" });

    // 4. JSON Export
    println!();
    println!("4️⃣  JSON FORMAT");
    println!("{}", "-".repeat(20));
    let json_config = ExportConfig {
        format: ExportFormat::Json,
        show_char_intervals: true,
        include_text: true,
        include_statistics: true,
        title: Some("LangExtract Demo - Structured Data".to_string()),
        ..Default::default()
    };
    let json_output = export_document(&annotated_document, &json_config)?;
    
    // Save JSON to file
    std::fs::write("langextract_demo.json", &json_output)?;
    println!("✅ JSON export saved to 'langextract_demo.json'");
    
    // Parse and show formatted statistics
    let parsed: serde_json::Value = serde_json::from_str(&json_output)?;
    if let Some(stats) = parsed.get("statistics") {
        println!("   📊 Statistics: {} extractions, {} characters, {} unique classes",
            stats.get("total_extractions").unwrap_or(&serde_json::Value::Null),
            stats.get("text_length").unwrap_or(&serde_json::Value::Null),
            stats.get("unique_classes").unwrap_or(&serde_json::Value::Null)
        );
    }

    // 5. CSV Export
    println!();
    println!("5️⃣  CSV FORMAT");
    println!("{}", "-".repeat(20));
    let csv_config = ExportConfig {
        format: ExportFormat::Csv,
        show_char_intervals: true,
        ..Default::default()
    };
    let csv_output = export_document(&annotated_document, &csv_config)?;
    
    // Save CSV to file
    std::fs::write("langextract_demo.csv", &csv_output)?;
    println!("✅ CSV export saved to 'langextract_demo.csv'");
    println!("   Preview:");
    let lines: Vec<&str> = csv_output.lines().take(4).collect();
    for line in lines {
        println!("   {}", line);
    }
    if csv_output.lines().count() > 4 {
        println!("   ... ({} more rows)", csv_output.lines().count() - 4);
    }

    println!();
    println!("🎉 Rich Visualization Demo Complete!");
    println!("Generated files:");
    println!("  📄 langextract_demo.html  - Interactive HTML with highlighting");
    println!("  📝 langextract_demo.md    - Structured markdown report");
    println!("  🔧 langextract_demo.json  - Machine-readable JSON data");
    println!("  📊 langextract_demo.csv   - Spreadsheet-compatible CSV");

    Ok(())
}
