//! I/O utilities for loading text from various sources.

use crate::exceptions::{LangExtractError, LangExtractResult};
use regex::Regex;

/// Check if a string is a URL (starts with http:// or https://)
pub fn is_url(text: &str) -> bool {
    text.starts_with("http://") || text.starts_with("https://")
}

/// Download text content from a URL
pub async fn download_text_from_url(url: &str) -> LangExtractResult<String> {
    if !is_url(url) {
        return Err(LangExtractError::invalid_input(format!(
            "Invalid URL: {}",
            url
        )));
    }

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(LangExtractError::invalid_input(
            format!("HTTP error: {} for URL: {}", response.status(), url)
        ));
    }

    let content = response.text().await?;
    Ok(content)
}

/// Clean and normalize text content
pub fn normalize_text(text: &str) -> String {
    // Remove excessive whitespace and normalize line endings
    let whitespace_regex = Regex::new(r"\s+").unwrap();
    let normalized = whitespace_regex.replace_all(text.trim(), " ");
    normalized.to_string()
}

/// Extract plain text from HTML content (basic implementation)
pub fn extract_text_from_html(html: &str) -> String {
    // This is a very basic HTML tag removal
    // In a production system, you'd want to use a proper HTML parser
    let tag_regex = Regex::new(r"<[^>]*>").unwrap();
    let text = tag_regex.replace_all(html, " ");
    normalize_text(&text)
}

/// Load text from a file path
pub async fn load_text_from_file(file_path: &str) -> LangExtractResult<String> {
    let content = tokio::fs::read_to_string(file_path).await?;
    Ok(content)
}

/// Save text to a file path
pub async fn save_text_to_file(file_path: &str, content: &str) -> LangExtractResult<()> {
    tokio::fs::write(file_path, content).await?;
    Ok(())
}

/// Detect the content type of text (plain text, HTML, etc.)
#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    PlainText,
    Html,
    Json,
    Yaml,
    Unknown,
}

pub fn detect_content_type(content: &str) -> ContentType {
    let trimmed = content.trim();
    
    // Check for JSON
    if (trimmed.starts_with('{') && trimmed.ends_with('}'))
        || (trimmed.starts_with('[') && trimmed.ends_with(']'))
    {
        if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
            return ContentType::Json;
        }
    }
    
    // Check for YAML (very basic check)
    if trimmed.contains("---") || trimmed.contains(": ") {
        if serde_yaml::from_str::<serde_yaml::Value>(trimmed).is_ok() {
            return ContentType::Yaml;
        }
    }
    
    // Check for HTML
    let html_regex = Regex::new(r"<[^>]+>").unwrap();
    if html_regex.is_match(trimmed) {
        return ContentType::Html;
    }
    
    ContentType::PlainText
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_url() {
        assert!(is_url("https://example.com"));
        assert!(is_url("http://example.com"));
        assert!(!is_url("example.com"));
        assert!(!is_url("ftp://example.com"));
        assert!(!is_url("file:///path/to/file"));
    }

    #[test]
    fn test_normalize_text() {
        let input = "  Hello    world  \n\n  How are you?  ";
        let expected = "Hello world How are you?";
        assert_eq!(normalize_text(input), expected);
    }

    #[test]
    fn test_extract_text_from_html() {
        let html = "<html><body><h1>Hello</h1><p>World</p></body></html>";
        let text = extract_text_from_html(html);
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_detect_content_type() {
        assert_eq!(
            detect_content_type(r#"{"key": "value"}"#),
            ContentType::Json
        );
        
        assert_eq!(
            detect_content_type("key: value\nother: data"),
            ContentType::Yaml
        );
        
        assert_eq!(
            detect_content_type("<html><body>Hello</body></html>"),
            ContentType::Html
        );
        
        assert_eq!(
            detect_content_type("Just plain text"),
            ContentType::PlainText
        );
    }

    #[tokio::test]
    async fn test_download_invalid_url() {
        let result = download_text_from_url("not-a-url").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_content_type_edge_cases() {
        // Invalid JSON should not be detected as JSON
        assert_eq!(
            detect_content_type(r#"{"invalid": json"#),
            ContentType::PlainText
        );
        
        // Empty string
        assert_eq!(
            detect_content_type(""),
            ContentType::PlainText
        );
        
        // Whitespace only
        assert_eq!(
            detect_content_type("   \n\t  "),
            ContentType::PlainText
        );
    }
}
