use langextract::{
    data::{AnnotatedDocument, Extraction, CharInterval, AlignmentStatus},
    visualization::{export_document, ExportConfig, ExportFormat},
};
use std::collections::HashMap;

fn create_test_document() -> AnnotatedDocument {
    // Test text with known positions
    let text = "Apple MacBook Pro 16-inch M3 Max - Model SKU: MBP-M3-16-SLV-2TB\nProduct Code: APPLE-2024-001, UPC: 194253715726, GTIN: 00194253715726\nAdvanced M3 Max chip with 16-core CPU and 40-core GPU for professional workflows\nStarting at $3,999.00 (MSRP). Warranty Code: APL-WAR-24M-001".to_string();

    println!("Test text:\n{}\n", text);
    println!("Text length: {} characters\n", text.len());

    // Create extractions with the EXACT positions from our debug test
    let extractions = vec![
        Extraction {
            extraction_class: "product_name".to_string(),
            extraction_text: "Apple MacBook Pro 16-inch M3 Max".to_string(),
            char_interval: Some(CharInterval::new(Some(0), Some(33))), // Should be position 0-33
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(0),
            group_index: Some(0),
            description: Some("Product name".to_string()),
            attributes: Some(HashMap::new()),
            token_interval: None,
        },
        Extraction {
            extraction_class: "model_sku".to_string(),
            extraction_text: "MBP-M3-16-SLV-2TB".to_string(),
            char_interval: Some(CharInterval::new(Some(46), Some(63))), // Model SKU position
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(1),
            group_index: Some(0),
            description: Some("Model SKU".to_string()),
            attributes: Some(HashMap::new()),
            token_interval: None,
        },
        Extraction {
            extraction_class: "product_code".to_string(),
            extraction_text: "APPLE-2024-001".to_string(),
            char_interval: Some(CharInterval::new(Some(80), Some(94))), // Product code position
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(2),
            group_index: Some(0),
            description: Some("Product code".to_string()),
            attributes: Some(HashMap::new()),
            token_interval: None,
        },
        Extraction {
            extraction_class: "price".to_string(),
            extraction_text: "$3,999.00".to_string(),
            char_interval: Some(CharInterval::new(Some(227), Some(236))), // Price position  
            alignment_status: Some(AlignmentStatus::MatchExact),
            extraction_index: Some(3),
            group_index: Some(0),
            description: Some("Price".to_string()),
            attributes: Some(HashMap::new()),
            token_interval: None,
        },
    ];

    // Verify positions manually
    println!("Verifying positions:");
    for extraction in &extractions {
        if let Some(interval) = &extraction.char_interval {
            if let (Some(start), Some(end)) = (interval.start_pos, interval.end_pos) {
                if start < text.len() && end <= text.len() && start < end {
                    let actual_text = &text[start..end];
                    println!("  {} [{}..{}]: '{}' (expected: '{}')", 
                        extraction.extraction_class, start, end, actual_text, extraction.extraction_text);
                    
                    if actual_text != extraction.extraction_text {
                        println!("    ❌ MISMATCH!");
                    } else {
                        println!("    ✅ MATCH!");
                    }
                } else {
                    println!("  {} [{}..{}]: ❌ INVALID POSITIONS", 
                        extraction.extraction_class, start, end);
                }
            }
        }
    }

    AnnotatedDocument {
        document_id: Some("test_highlighting".to_string()),
        text: Some(text),
        extractions: Some(extractions),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 HTML Highlighting Debug Test");
    println!("================================\n");

    let document = create_test_document();

    // Generate HTML with highlighting
    let html_config = ExportConfig {
        format: ExportFormat::Html,
        title: Some("Highlighting Debug Test".to_string()),
        highlight_extractions: true,
        show_char_intervals: true,
        include_statistics: true,
        ..Default::default()
    };

    match export_document(&document, &html_config) {
        Ok(html_output) => {
            // Save the HTML file
            std::fs::write("highlighting_debug.html", &html_output)?;
            println!("✅ HTML file saved: highlighting_debug.html");
            
            // Extract just the highlighted text part for analysis
            if let Some(start) = html_output.find(r#"<div class="document-text">"#) {
                if let Some(end) = html_output[start..].find("</div>") {
                    let highlighted_section = &html_output[start..start + end];
                    println!("\n🔍 Highlighted section:");
                    println!("{}", highlighted_section);
                    
                    // Count the spans
                    let span_count = highlighted_section.matches(r#"<span class="extraction-highlight""#).count();
                    println!("\n📊 Found {} highlighted spans", span_count);
                    
                    // Check for any obvious issues
                    if highlighted_section.contains("&lt;span") {
                        println!("⚠️  Found double-escaped HTML tags!");
                    }
                    if highlighted_section.contains("span><span") {
                        println!("⚠️  Found adjacent spans (possible overlap issue)!");
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ HTML generation failed: {}", e);
        }
    }

    // Also test without highlighting for comparison
    let no_highlight_config = ExportConfig {
        format: ExportFormat::Html,
        title: Some("No Highlighting Test".to_string()),
        highlight_extractions: false,
        show_char_intervals: true,
        include_statistics: true,
        ..Default::default()
    };

    match export_document(&document, &no_highlight_config) {
        Ok(html_output) => {
            std::fs::write("no_highlighting_debug.html", &html_output)?;
            println!("✅ No-highlighting HTML file saved: no_highlighting_debug.html");
        }
        Err(e) => {
            println!("❌ No-highlighting HTML generation failed: {}", e);
        }
    }

    println!("\n💡 Open highlighting_debug.html in your browser to see the actual highlighting results!");
    println!("💡 Compare with no_highlighting_debug.html to see the difference.");

    Ok(())
}
