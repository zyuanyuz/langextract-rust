//! Visualization utilities for annotated documents.

use crate::{data::AnnotatedDocument, exceptions::LangExtractResult};
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::Extraction;
/// Export format options for visualization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    /// Simple text format (existing functionality)
    Text,
    /// Rich HTML with highlighting and interactivity
    Html,
    /// Structured markdown with summaries
    Markdown,
    /// Raw JSON export for analysis
    Json,
    /// CSV export for spreadsheet analysis
    Csv,
}

/// Configuration for visualization exports
#[derive(Debug, Clone)]
pub struct ExportConfig {
    /// Export format to use
    pub format: ExportFormat,
    /// Show character intervals in output
    pub show_char_intervals: bool,
    /// Include original text in export
    pub include_text: bool,
    /// Highlight extractions in text (for HTML/Markdown)
    pub highlight_extractions: bool,
    /// Include extraction statistics
    pub include_statistics: bool,
    /// Custom CSS for HTML export
    pub custom_css: Option<String>,
    /// Title for the export
    pub title: Option<String>,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::Text,
            show_char_intervals: false,
            include_text: true,
            highlight_extractions: true,
            include_statistics: true,
            custom_css: None,
            title: None,
        }
    }
}

/// Export an annotated document in the specified format
pub fn export_document(
    annotated_document: &AnnotatedDocument,
    config: &ExportConfig,
) -> LangExtractResult<String> {
    match config.format {
        ExportFormat::Text => visualize_text(annotated_document, config.show_char_intervals),
        ExportFormat::Html => export_html(annotated_document, config),
        ExportFormat::Markdown => export_markdown(annotated_document, config),
        ExportFormat::Json => export_json(annotated_document, config),
        ExportFormat::Csv => export_csv(annotated_document, config),
    }
}

/// Visualize an annotated document (legacy function for backward compatibility)
pub fn visualize(
    annotated_document: &AnnotatedDocument,
    show_char_intervals: bool,
) -> LangExtractResult<String> {
    visualize_text(annotated_document, show_char_intervals)
}

/// Export as simple text format (original implementation)
fn visualize_text(
    annotated_document: &AnnotatedDocument,
    show_char_intervals: bool,
) -> LangExtractResult<String> {
    let mut result = String::new();
    
    result.push_str("📄 EXTRACTION VISUALIZATION\n");
    result.push_str("=" .repeat(50).as_str());
    result.push('\n');
    
    // Show document text
    let text = annotated_document.text.as_deref().unwrap_or("No text");
    result.push_str(&format!("📝 Document Text ({} chars):\n", text.len()));
    result.push_str(&format!("   {}\n\n", text));
    
    // Show extractions
    if let Some(extractions) = &annotated_document.extractions {
        result.push_str(&format!("🎯 Found {} Extractions:\n", extractions.len()));
        result.push_str("-".repeat(30).as_str());
        result.push('\n');
        
        for (i, extraction) in extractions.iter().enumerate() {
            result.push_str(&format!("{}. [{}] {}\n", 
                i + 1, 
                extraction.extraction_class, 
                extraction.extraction_text
            ));
            
            if show_char_intervals {
                if let Some(interval) = &extraction.char_interval {
                    result.push_str(&format!("   Position: {:?}\n", interval));
                }
            }
            
            if let Some(description) = &extraction.description {
                result.push_str(&format!("   Description: {}\n", description));
            }
            
            result.push('\n');
        }
    } else {
        result.push_str("ℹ️  No extractions found\n");
    }
    
    // Show statistics
    result.push_str("📊 Statistics:\n");
    result.push_str("-".repeat(15).as_str());
    result.push('\n');
    result.push_str(&format!("   Document ID: {}\n", 
        annotated_document.document_id.as_deref().unwrap_or("None")));
    result.push_str(&format!("   Text Length: {} characters\n", text.len()));
    result.push_str(&format!("   Total Extractions: {}\n", annotated_document.extraction_count()));
    
    if let Some(extractions) = &annotated_document.extractions {
        // Count unique extraction classes
        let mut class_counts = std::collections::HashMap::new();
        for extraction in extractions {
            *class_counts.entry(&extraction.extraction_class).or_insert(0) += 1;
        }
        
        result.push_str("   Extraction Classes:\n");
        for (class, count) in class_counts {
            result.push_str(&format!("     • {}: {} instance(s)\n", class, count));
        }
    }
    
    Ok(result)
}

/// Export as rich HTML with highlighting and interactivity
fn export_html(
    annotated_document: &AnnotatedDocument,
    config: &ExportConfig,
) -> LangExtractResult<String> {
    let title = config.title.as_deref().unwrap_or("LangExtract Results");
    let text = annotated_document.text.as_deref().unwrap_or("No text");
    
    let mut html = String::new();
    
    // HTML Header
    html.push_str(&format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background: #f8fafc;
            color: #334155;
        }}
        .container {{
            background: white;
            border-radius: 12px;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
            overflow: hidden;
        }}
        .header {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 30px;
            text-align: center;
        }}
        .header h1 {{
            margin: 0;
            font-size: 2.5em;
            font-weight: 300;
        }}
        .content {{
            padding: 30px;
        }}
        .section {{
            margin-bottom: 40px;
        }}
        .section h2 {{
            color: #1e293b;
            border-bottom: 2px solid #e2e8f0;
            padding-bottom: 10px;
            margin-bottom: 20px;
        }}
        .document-text {{
            background: #f1f5f9;
            border-radius: 8px;
            padding: 20px;
            font-family: 'Monaco', 'Menlo', monospace;
            line-height: 1.6;
            white-space: pre-wrap;
            position: relative;
            margin-bottom: 20px;
        }}
        .extraction-highlight {{
            background: rgba(59, 130, 246, 0.2);
            border: 1px solid rgba(59, 130, 246, 0.4);
            border-radius: 3px;
            padding: 1px 2px;
            cursor: pointer;
            transition: all 0.2s ease;
        }}
        .extraction-highlight:hover {{
            background: rgba(59, 130, 246, 0.3);
            transform: translateY(-1px);
        }}
        .extractions-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }}
        .extraction-card {{
            background: #f8fafc;
            border: 1px solid #e2e8f0;
            border-radius: 8px;
            padding: 15px;
            transition: all 0.2s ease;
        }}
        .extraction-card:hover {{
            border-color: #3b82f6;
            box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
        }}
        .extraction-class {{
            background: #3b82f6;
            color: white;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 0.8em;
            font-weight: 600;
            display: inline-block;
            margin-bottom: 8px;
        }}
        .extraction-text {{
            font-weight: 600;
            color: #1e293b;
            margin-bottom: 8px;
        }}
        .extraction-meta {{
            font-size: 0.9em;
            color: #64748b;
        }}
        .stats-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
        }}
        .stat-card {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 8px;
            text-align: center;
        }}
        .stat-number {{
            font-size: 2em;
            font-weight: bold;
            margin-bottom: 5px;
        }}
        .stat-label {{
            opacity: 0.9;
            font-size: 0.9em;
        }}
        .class-counts {{
            background: #f1f5f9;
            border-radius: 8px;
            padding: 20px;
        }}
        .class-count-item {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 8px 0;
            border-bottom: 1px solid #e2e8f0;
        }}
        .class-count-item:last-child {{
            border-bottom: none;
        }}
        .class-badge {{
            background: #10b981;
            color: white;
            padding: 2px 6px;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: 600;
        }}
        {}
    </style>
</head>
<body>
"#, title, config.custom_css.as_deref().unwrap_or("")));

    // Header
    html.push_str(&format!(r#"    <div class="container">
        <div class="header">
            <h1>{}</h1>
        </div>
        <div class="content">
"#, title));

    // Document text section (with highlighting if enabled)
    if config.include_text {
        html.push_str(r#"            <div class="section">
                <h2>📄 Document Text</h2>
                <div class="document-text">"#);
        
        if config.highlight_extractions {
            html.push_str(&highlight_text_html(text, annotated_document)?);
        } else {
            html.push_str(&html_escape(text));
        }
        
        html.push_str("</div>\n            </div>\n");
    }

    // Extractions section
    if let Some(extractions) = &annotated_document.extractions {
        html.push_str(&format!(r#"            <div class="section">
                <h2>🎯 Extractions ({} found)</h2>
                <div class="extractions-grid">
"#, extractions.len()));

        for extraction in extractions {
            html.push_str(&format!(r#"                    <div class="extraction-card">
                        <div class="extraction-class">{}</div>
                        <div class="extraction-text">{}</div>
"#, html_escape(&extraction.extraction_class), html_escape(&extraction.extraction_text)));

            if config.show_char_intervals {
                if let Some(interval) = &extraction.char_interval {
                    html.push_str(&format!(r#"                        <div class="extraction-meta">Position: {}-{}</div>
"#, interval.start_pos.unwrap_or(0), interval.end_pos.unwrap_or(0)));
                }
            }

            if let Some(description) = &extraction.description {
                html.push_str(&format!(r#"                        <div class="extraction-meta">Description: {}</div>
"#, html_escape(description)));
            }

            html.push_str("                    </div>\n");
        }
        
        html.push_str("                </div>\n            </div>\n");
    }

    // Statistics section
    if config.include_statistics {
        html.push_str(r#"            <div class="section">
                <h2>📊 Statistics</h2>
                <div class="stats-grid">
"#);

        let extraction_count = annotated_document.extraction_count();
        html.push_str(&format!(r#"                    <div class="stat-card">
                        <div class="stat-number">{}</div>
                        <div class="stat-label">Total Extractions</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-number">{}</div>
                        <div class="stat-label">Characters</div>
                    </div>
"#, extraction_count, text.len()));

        if let Some(extractions) = &annotated_document.extractions {
            let class_counts = count_extraction_classes(extractions);
            html.push_str(&format!(r#"                    <div class="stat-card">
                        <div class="stat-number">{}</div>
                        <div class="stat-label">Unique Classes</div>
                    </div>
"#, class_counts.len()));

            html.push_str("                </div>\n");
            
            // Class breakdown
            html.push_str(r#"                <h3>Extraction Classes</h3>
                <div class="class-counts">
"#);
            
            for (class, count) in class_counts {
                html.push_str(&format!(r#"                    <div class="class-count-item">
                        <span>{}</span>
                        <span class="class-badge">{}</span>
                    </div>
"#, html_escape(class), count));
            }
            
            html.push_str("                </div>\n");
        } else {
            html.push_str("                </div>\n");
        }
        
        html.push_str("            </div>\n");
    }

    // Footer
    html.push_str(r#"        </div>
    </div>
    
    <script>
        // Add interactivity for extraction highlights
        document.querySelectorAll('.extraction-highlight').forEach(element => {
            element.addEventListener('click', function() {
                const className = this.getAttribute('data-class');
                const text = this.getAttribute('data-text');
                alert(`Extraction: ${className}\nText: ${text}`);
            });
        });
    </script>
</body>
</html>"#);

    Ok(html)
}

/// Helper function to escape HTML characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Helper function to find the nearest valid UTF-8 character boundary
fn find_char_boundary(text: &str, mut index: usize) -> usize {
    // Clamp to text length first
    if index >= text.len() {
        return text.len();
    }
    
    // If we're already at a character boundary, return as-is
    if text.is_char_boundary(index) {
        return index;
    }
    
    // Search backwards for the nearest character boundary
    while index > 0 && !text.is_char_boundary(index) {
        index -= 1;
    }
    
    index
}

/// Helper function to highlight extractions in text
fn highlight_text_html(text: &str, annotated_document: &AnnotatedDocument) -> LangExtractResult<String> {
    if let Some(extractions) = &annotated_document.extractions {
        // Collect all valid intervals with their extraction info
        let mut intervals: Vec<(usize, usize, &Extraction)> = Vec::new();
        
        for extraction in extractions {
            if let Some(interval) = &extraction.char_interval {
                if let (Some(start), Some(end)) = (interval.start_pos, interval.end_pos) {
                    if start < end && end <= text.len() {
                        intervals.push((start, end, extraction));
                    }
                }
            }
        }
        
        // Sort by start position
        intervals.sort_by_key(|(start, _, _)| *start);
        
        // Remove overlapping intervals - keep the first one when intervals overlap
        let mut filtered_intervals = Vec::new();
        let mut last_end = 0;
        
        for (start, end, extraction) in intervals {
            if start >= last_end {
                filtered_intervals.push((start, end, extraction));
                last_end = end;
            } else {
                // Skip overlapping interval, but log it for debugging
                log::debug!("Skipping overlapping extraction: '{}' at {}-{} (overlaps with previous ending at {})", 
                    extraction.extraction_text, start, end, last_end);
            }
        }
        
        // Now build the HTML with non-overlapping intervals
        let mut result = String::new();
        let mut last_pos = 0;
        
        for (start, end, extraction) in filtered_intervals {
            // Ensure we're at valid UTF-8 boundaries
            let safe_start = find_char_boundary(text, start);
            let safe_end = find_char_boundary(text, end);
            
            // Add text before this extraction
            if safe_start > last_pos {
                let safe_last_pos = find_char_boundary(text, last_pos);
                if safe_last_pos < safe_start {
                    result.push_str(&html_escape(&text[safe_last_pos..safe_start]));
                }
            }
            
            // Add the highlighted extraction (only if we have valid boundaries)
            if safe_start < safe_end && safe_end <= text.len() {
                let actual_text = &text[safe_start..safe_end];
                result.push_str(&format!(
                    r#"<span class="extraction-highlight" data-class="{}" data-text="{}">{}</span>"#,
                    html_escape(&extraction.extraction_class),
                    html_escape(&extraction.extraction_text),
                    html_escape(actual_text)
                ));
                last_pos = safe_end;
            } else {
                // Skip invalid boundaries but log for debugging
                log::debug!("Skipping extraction with invalid UTF-8 boundaries: '{}' at {}-{}", 
                    extraction.extraction_text, start, end);
            }
        }
        
        // Add remaining text
        if last_pos < text.len() {
            let safe_last_pos = find_char_boundary(text, last_pos);
            if safe_last_pos < text.len() {
                result.push_str(&html_escape(&text[safe_last_pos..]));
            }
        }
        
        Ok(result)
    } else {
        Ok(html_escape(text))
    }
}

/// Helper function to count extraction classes
fn count_extraction_classes(extractions: &[crate::data::Extraction]) -> HashMap<&str, usize> {
    let mut class_counts = HashMap::new();
    for extraction in extractions {
        *class_counts.entry(extraction.extraction_class.as_str()).or_insert(0) += 1;
    }
    class_counts
}

/// Export as structured markdown with extraction summaries
fn export_markdown(
    annotated_document: &AnnotatedDocument,
    config: &ExportConfig,
) -> LangExtractResult<String> {
    let title = config.title.as_deref().unwrap_or("LangExtract Results");
    let text = annotated_document.text.as_deref().unwrap_or("No text");
    
    let mut md = String::new();
    
    // Title
    md.push_str(&format!("# {}\n\n", title));
    
    // Document text section
    if config.include_text {
        md.push_str("## 📄 Document Text\n\n");
        
        if config.highlight_extractions {
            md.push_str(&highlight_text_markdown(text, annotated_document)?);
        } else {
            md.push_str(&format!("```\n{}\n```\n", text));
        }
        
        md.push_str("\n");
    }
    
    // Extractions section
    if let Some(extractions) = &annotated_document.extractions {
        md.push_str(&format!("## 🎯 Extractions ({} found)\n\n", extractions.len()));
        
        for (i, extraction) in extractions.iter().enumerate() {
            md.push_str(&format!("### {}. {}\n\n", i + 1, extraction.extraction_class));
            md.push_str(&format!("**Text:** {}\n\n", extraction.extraction_text));
            
            if config.show_char_intervals {
                if let Some(interval) = &extraction.char_interval {
                    md.push_str(&format!("**Position:** {}-{}\n\n", interval.start_pos.unwrap_or(0), interval.end_pos.unwrap_or(0)));
                }
            }
            
            if let Some(description) = &extraction.description {
                md.push_str(&format!("**Description:** {}\n\n", description));
            }
        }
    }
    
    // Statistics section
    if config.include_statistics {
        md.push_str("## 📊 Statistics\n\n");
        
        let extraction_count = annotated_document.extraction_count();
        md.push_str(&format!("- **Total Extractions:** {}\n", extraction_count));
        md.push_str(&format!("- **Text Length:** {} characters\n", text.len()));
        
        if let Some(extractions) = &annotated_document.extractions {
            let class_counts = count_extraction_classes(extractions);
            md.push_str(&format!("- **Unique Classes:** {}\n\n", class_counts.len()));
            
            md.push_str("### Extraction Classes\n\n");
            md.push_str("| Class | Count |\n");
            md.push_str("|-------|-------|\n");
            
            for (class, count) in class_counts {
                md.push_str(&format!("| {} | {} |\n", class, count));
            }
        }
        
        md.push_str("\n");
    }
    
    Ok(md)
}

/// Helper function to highlight extractions in markdown
fn highlight_text_markdown(text: &str, annotated_document: &AnnotatedDocument) -> LangExtractResult<String> {
    if let Some(extractions) = &annotated_document.extractions {
        let mut result = String::new();
        let mut last_pos = 0;
        
        // Sort extractions by start position
        let mut sorted_extractions: Vec<_> = extractions.iter().collect();
        sorted_extractions.sort_by_key(|e| {
            e.char_interval.as_ref().and_then(|i| i.start_pos).unwrap_or(usize::MAX)
        });
        
        result.push_str("```\n");
        
        for extraction in sorted_extractions {
            if let Some(interval) = &extraction.char_interval {
                // Add text before the extraction
                if interval.start_pos.unwrap_or(0) > last_pos && interval.start_pos.unwrap_or(0) <= text.len() {
                    result.push_str(&text[last_pos..interval.start_pos.unwrap_or(0)]);
                }
                
                // Add highlighted extraction with markdown bold
                if interval.end_pos.unwrap_or(0) <= text.len() && interval.start_pos.unwrap_or(0) < interval.end_pos.unwrap_or(0) {
                    let extraction_text = &text[interval.start_pos.unwrap_or(0)..interval.end_pos.unwrap_or(0)];
                    result.push_str(&format!("**{}**", extraction_text));
                    last_pos = interval.end_pos.unwrap_or(0);
                }
            }
        }
        
        // Add remaining text
        if last_pos < text.len() {
            result.push_str(&text[last_pos..]);
        }
        
        result.push_str("\n```\n");
        Ok(result)
    } else {
        Ok(format!("```\n{}\n```\n", text))
    }
}

/// Export as JSON for analysis
fn export_json(
    annotated_document: &AnnotatedDocument,
    config: &ExportConfig,
) -> LangExtractResult<String> {
    let mut json_data = json!({
        "document_id": annotated_document.document_id,
        "export_config": {
            "format": "json",
            "show_char_intervals": config.show_char_intervals,
            "include_text": config.include_text,
            "include_statistics": config.include_statistics,
            "title": config.title
        }
    });
    
    // Add text if requested
    if config.include_text {
        json_data["text"] = json!(annotated_document.text);
    }
    
    // Add extractions
    if let Some(extractions) = &annotated_document.extractions {
        let extractions_json: Vec<Value> = extractions.iter().map(|extraction| {
            let mut ext_json = json!({
                "extraction_class": extraction.extraction_class,
                "extraction_text": extraction.extraction_text,
                "description": extraction.description
            });
            
            if config.show_char_intervals {
                if let Some(interval) = &extraction.char_interval {
                    ext_json["char_interval"] = json!({
                        "start_char": interval.start_pos.unwrap_or(0),
                        "end_char": interval.end_pos.unwrap_or(0),
                        "alignment_status": extraction.alignment_status.as_ref().map(|s| format!("{:?}", s)).unwrap_or_else(|| "None".to_string())
                    });
                }
            }
            
            if let Some(group_index) = extraction.group_index {
                ext_json["group_index"] = json!(group_index);
            }
            
            ext_json
        }).collect();
        
        json_data["extractions"] = json!(extractions_json);
    }
    
    // Add statistics if requested
    if config.include_statistics {
        let text = annotated_document.text.as_deref().unwrap_or("");
        let mut stats = json!({
            "total_extractions": annotated_document.extraction_count(),
            "text_length": text.len()
        });
        
        if let Some(extractions) = &annotated_document.extractions {
            let class_counts = count_extraction_classes(extractions);
            stats["unique_classes"] = json!(class_counts.len());
            stats["extraction_classes"] = json!(class_counts);
        }
        
        json_data["statistics"] = stats;
    }
    
    Ok(serde_json::to_string_pretty(&json_data)?)
}

/// Export as CSV for spreadsheet analysis
fn export_csv(
    annotated_document: &AnnotatedDocument,
    config: &ExportConfig,
) -> LangExtractResult<String> {
    let mut csv = String::new();
    
    // CSV Header
    if config.show_char_intervals {
        csv.push_str("extraction_class,extraction_text,description,start_char,end_char,alignment_status,group_index\n");
    } else {
        csv.push_str("extraction_class,extraction_text,description,group_index\n");
    }
    
    // CSV Rows
    if let Some(extractions) = &annotated_document.extractions {
        for extraction in extractions {
            let class = csv_escape(&extraction.extraction_class);
            let text = csv_escape(&extraction.extraction_text);
            let description = extraction.description.as_ref().map(|d| csv_escape(d)).unwrap_or_else(|| "".to_string());
            let group_index = extraction.group_index.map(|i| i.to_string()).unwrap_or_else(|| "".to_string());
            
            if config.show_char_intervals {
                if let Some(interval) = &extraction.char_interval {
                    csv.push_str(&format!("{},{},{},{},{},{:?},{}\n",
                        class, text, description,
                        interval.start_pos.unwrap_or(0), interval.end_pos.unwrap_or(0),
                        extraction.alignment_status.as_ref().map(|s| format!("{:?}", s)).unwrap_or_else(|| "None".to_string()), group_index));
                } else {
                    csv.push_str(&format!("{},{},{},,,None,{}\n",
                        class, text, description, group_index));
                }
            } else {
                csv.push_str(&format!("{},{},{},{}\n",
                    class, text, description, group_index));
            }
        }
    }
    
    Ok(csv)
}

/// Helper function to escape CSV values
fn csv_escape(text: &str) -> String {
    if text.contains(',') || text.contains('"') || text.contains('\n') {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{AlignmentStatus, CharInterval, Extraction};
    use std::collections::HashMap;

    fn create_sample_document() -> AnnotatedDocument {
        let text = "John Smith works at TechCorp and earns $50,000.";
        let extractions = vec![
            Extraction {
                extraction_class: "person".to_string(),
                extraction_text: "John Smith".to_string(),
                char_interval: Some(CharInterval::new(Some(0), Some(10))),
                alignment_status: Some(AlignmentStatus::MatchExact),
                extraction_index: Some(0),
                group_index: Some(0),
                description: Some("Person name".to_string()),
                attributes: Some(HashMap::new()),
                token_interval: None,
            },
            Extraction {
                extraction_class: "company".to_string(),
                extraction_text: "TechCorp".to_string(),
                char_interval: Some(CharInterval::new(Some(20), Some(28))),
                alignment_status: Some(AlignmentStatus::MatchExact),
                extraction_index: Some(1),
                group_index: Some(0),
                description: None,
                attributes: Some(HashMap::new()),
                token_interval: None,
            },
            Extraction {
                extraction_class: "salary".to_string(),
                extraction_text: "$50,000".to_string(),
                char_interval: Some(CharInterval::new(Some(39), Some(46))),
                alignment_status: Some(AlignmentStatus::MatchFuzzy),
                extraction_index: Some(2),
                group_index: Some(0),
                description: Some("Annual salary".to_string()),
                attributes: Some(HashMap::new()),
                token_interval: None,
            },
        ];

        AnnotatedDocument {
            document_id: Some("test_doc".to_string()),
            text: Some(text.to_string()),
            extractions: Some(extractions),
        }
    }

    #[test]
    fn test_text_export() {
        let document = create_sample_document();
        let config = ExportConfig {
            format: ExportFormat::Text,
            show_char_intervals: true,
            ..Default::default()
        };

        let result = export_document(&document, &config).unwrap();
        
        assert!(result.contains("EXTRACTION VISUALIZATION"));
        assert!(result.contains("John Smith"));
        assert!(result.contains("TechCorp"));
        assert!(result.contains("$50,000"));
        assert!(result.contains("Position:"));
        assert!(result.contains("Statistics:"));
    }

    #[test]
    fn test_html_export() {
        let document = create_sample_document();
        let config = ExportConfig {
            format: ExportFormat::Html,
            title: Some("Test HTML Export".to_string()),
            highlight_extractions: true,
            show_char_intervals: true,
            ..Default::default()
        };

        let result = export_document(&document, &config).unwrap();
        
        assert!(result.contains("<!DOCTYPE html>"));
        assert!(result.contains("<title>Test HTML Export</title>"));
        assert!(result.contains("extraction-highlight"));
        assert!(result.contains("John Smith"));
        assert!(result.contains("TechCorp"));
        assert!(result.contains("extraction-card"));
        assert!(result.contains("stats-grid"));
        assert!(result.contains("</html>"));
    }

    #[test]
    fn test_html_export_with_custom_css() {
        let document = create_sample_document();
        let custom_css = "body { background: red; }";
        let config = ExportConfig {
            format: ExportFormat::Html,
            custom_css: Some(custom_css.to_string()),
            ..Default::default()
        };

        let result = export_document(&document, &config).unwrap();
        
        assert!(result.contains(custom_css));
    }

    #[test]
    fn test_markdown_export() {
        let document = create_sample_document();
        let config = ExportConfig {
            format: ExportFormat::Markdown,
            title: Some("Test Markdown".to_string()),
            show_char_intervals: true,
            highlight_extractions: true,
            ..Default::default()
        };

        let result = export_document(&document, &config).unwrap();
        
        assert!(result.starts_with("# Test Markdown"));
        assert!(result.contains("## 📄 Document Text"));
        assert!(result.contains("## 🎯 Extractions"));
        assert!(result.contains("### 1. person"));
        assert!(result.contains("**Text:** John Smith"));
        assert!(result.contains("**Position:** 0-10"));
        assert!(result.contains("| Class | Count |"));
        assert!(result.contains("| person | 1 |"));
    }

    #[test]
    fn test_json_export() {
        let document = create_sample_document();
        let config = ExportConfig {
            format: ExportFormat::Json,
            show_char_intervals: true,
            include_text: true,
            include_statistics: true,
            ..Default::default()
        };

        let result = export_document(&document, &config).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(parsed["document_id"], "test_doc");
        assert!(parsed["text"].is_string());
        assert!(parsed["extractions"].is_array());
        assert!(parsed["statistics"].is_object());
        
        let extractions = parsed["extractions"].as_array().unwrap();
        assert_eq!(extractions.len(), 3);
        
        let first_extraction = &extractions[0];
        assert_eq!(first_extraction["extraction_class"], "person");
        assert_eq!(first_extraction["extraction_text"], "John Smith");
        assert!(first_extraction["char_interval"].is_object());
        
        let stats = &parsed["statistics"];
        assert_eq!(stats["total_extractions"], 3);
        assert_eq!(stats["unique_classes"], 3);
    }

    #[test]
    fn test_csv_export() {
        let document = create_sample_document();
        let config = ExportConfig {
            format: ExportFormat::Csv,
            show_char_intervals: true,
            ..Default::default()
        };

        let result = export_document(&document, &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        
        // Check header
        assert_eq!(lines[0], "extraction_class,extraction_text,description,start_char,end_char,alignment_status,group_index");
        
        // Check data rows
        assert_eq!(lines.len(), 4); // Header + 3 data rows
        assert!(lines[1].contains("person,John Smith"));
        assert!(lines[2].contains("company,TechCorp"));
        assert!(lines[3].contains("salary,\"$50,000\""));
        assert!(lines[1].contains("MatchExact"));
        assert!(lines[3].contains("MatchFuzzy"));
    }

    #[test]
    fn test_csv_export_without_intervals() {
        let document = create_sample_document();
        let config = ExportConfig {
            format: ExportFormat::Csv,
            show_char_intervals: false,
            ..Default::default()
        };

        let result = export_document(&document, &config).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        
        // Check header
        assert_eq!(lines[0], "extraction_class,extraction_text,description,group_index");
        
        // Should not contain position columns
        assert!(!result.contains("start_char"));
        assert!(!result.contains("end_char"));
    }

    #[test]
    fn test_csv_escape() {
        assert_eq!(csv_escape("simple"), "simple");
        assert_eq!(csv_escape("has,comma"), "\"has,comma\"");
        assert_eq!(csv_escape("has\"quote"), "\"has\"\"quote\"");
        assert_eq!(csv_escape("has\nnewline"), "\"has\nnewline\"");
        assert_eq!(csv_escape("has,comma\"and quote"), "\"has,comma\"\"and quote\"");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("simple"), "simple");
        assert_eq!(html_escape("has<tag>"), "has&lt;tag&gt;");
        assert_eq!(html_escape("has\"quote"), "has&quot;quote");
        assert_eq!(html_escape("has'apostrophe"), "has&#x27;apostrophe");
        assert_eq!(html_escape("has&ampersand"), "has&amp;ampersand");
    }

    #[test]
    fn test_export_config_defaults() {
        let config = ExportConfig::default();
        assert_eq!(config.format, ExportFormat::Text);
        assert!(!config.show_char_intervals);
        assert!(config.include_text);
        assert!(config.highlight_extractions);
        assert!(config.include_statistics);
        assert!(config.custom_css.is_none());
        assert!(config.title.is_none());
    }

    #[test]
    fn test_empty_document() {
        let document = AnnotatedDocument {
            document_id: Some("empty".to_string()),
            text: Some("".to_string()),
            extractions: None,
        };

        let config = ExportConfig::default();
        let result = export_document(&document, &config).unwrap();
        
        assert!(result.contains("No extractions found"));
    }

    #[test]
    fn test_document_without_text() {
        let document = AnnotatedDocument {
            document_id: Some("no_text".to_string()),
            text: None,
            extractions: None,
        };

        let config = ExportConfig::default();
        let result = export_document(&document, &config).unwrap();
        
        assert!(result.contains("No text"));
    }

    #[test]
    fn test_export_format_variants() {
        let document = create_sample_document();
        
        // Test all export formats don't panic
        for format in [ExportFormat::Text, ExportFormat::Html, ExportFormat::Markdown, ExportFormat::Json, ExportFormat::Csv] {
            let config = ExportConfig {
                format,
                ..Default::default()
            };
            let result = export_document(&document, &config);
            assert!(result.is_ok(), "Format {:?} failed", format);
        }
    }

    #[test]
    fn test_highlight_text_html() {
        let document = create_sample_document();
        let text = document.text.as_ref().unwrap();
        
        let result = highlight_text_html(text, &document).unwrap();
        
        assert!(result.contains("extraction-highlight"));
        assert!(result.contains("data-class=\"person\""));
        assert!(result.contains("data-text=\"John Smith\""));
        assert!(result.contains("John Smith"));
    }

    #[test]
    fn test_count_extraction_classes() {
        let extractions = vec![
            Extraction {
                extraction_class: "person".to_string(),
                extraction_text: "John".to_string(),
                char_interval: None,
                alignment_status: None,
                extraction_index: None,
                group_index: None,
                description: None,
                attributes: Some(HashMap::new()),
                token_interval: None,
            },
            Extraction {
                extraction_class: "person".to_string(),
                extraction_text: "Jane".to_string(),
                char_interval: None,
                alignment_status: None,
                extraction_index: None,
                group_index: None,
                description: None,
                attributes: Some(HashMap::new()),
                token_interval: None,
            },
            Extraction {
                extraction_class: "company".to_string(),
                extraction_text: "TechCorp".to_string(),
                char_interval: None,
                alignment_status: None,
                extraction_index: None,
                group_index: None,
                description: None,
                attributes: Some(HashMap::new()),
                token_interval: None,
            },
        ];

        let counts = count_extraction_classes(&extractions);
        
        assert_eq!(counts.get("person"), Some(&2));
        assert_eq!(counts.get("company"), Some(&1));
        assert_eq!(counts.len(), 2);
    }
}
