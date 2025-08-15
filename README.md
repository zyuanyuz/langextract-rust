# LangExtract (Rust Implementation)

A powerful Rust library for extracting structured and grounded information from text using Large Language Models (LLMs).

LangExtract processes unstructured text and extracts specific information with precise character-level alignment, making it perfect for document analysis, research paper processing, product catalogs, and more.

## ✨ Key Features

- 🚀 **High-Performance Async Processing** - Concurrent chunk processing with configurable parallelism
- 🎯 **Universal Provider Support** - OpenAI, Ollama, and custom HTTP APIs
- 📍 **Character-Level Alignment** - Precise text positioning with fuzzy matching fallback  
- 🔧 **Advanced Validation System** - Schema validation, type coercion, and raw data preservation
- 🎨 **Rich Visualization** - Export to HTML, Markdown, JSON, and CSV formats
- 📊 **Multi-Pass Extraction** - Improved recall through multiple extraction rounds
- 🧩 **Intelligent Chunking** - Automatic text splitting with overlap handling
- 🔒 **Memory-safe** and **thread-safe** by design

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
langextract = "0.1.0"
```

### Basic Usage Example

```rust
use langextract::{
    extract, ExtractConfig, FormatType,
    data::{ExampleData, Extraction},
    providers::ProviderConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up examples to guide extraction
    let examples = vec![
        ExampleData::new(
            "John Doe is 30 years old and works as a doctor".to_string(),
            vec![
                Extraction::new("person".to_string(), "John Doe".to_string()),
                Extraction::new("age".to_string(), "30".to_string()),
                Extraction::new("profession".to_string(), "doctor".to_string()),
            ],
        )
    ];

    // Configure for Ollama
    let provider_config = ProviderConfig::ollama("mistral", None);
    
    let config = ExtractConfig {
        model_id: "mistral".to_string(),
        format_type: FormatType::Json,
        max_char_buffer: 8000,
        max_workers: 6,
        batch_length: 4,
        temperature: 0.3,
        model_url: Some("http://localhost:11434".to_string()),
        language_model_params: {
            let mut params = std::collections::HashMap::new();
            params.insert("provider_config".to_string(), serde_json::to_value(&provider_config)?);
            params
        },
        debug: true,
        ..Default::default()
    };

    // Extract information
    let result = extract(
        "Alice Smith is 25 years old and works as a doctor. Bob Johnson is 35 and is an engineer.",
        Some("Extract person names, ages, and professions from the text"),
        &examples,
        config,
    ).await?;

    println!("✅ Extracted {} items", result.extraction_count());
    
    // Show extractions with character positions
    if let Some(extractions) = &result.extractions {
        for extraction in extractions {
            println!("• [{}] '{}' at {:?}", 
                extraction.extraction_class, 
                extraction.extraction_text,
                extraction.char_interval
            );
        }
    }
    
    Ok(())
}
```

### Advanced Features

#### Validation and Type Coercion

```rust
use langextract::{ValidationConfig, ValidationResult};

// Enable advanced validation
let validation_config = ValidationConfig {
    enable_schema_validation: true,
    enable_type_coercion: true,
    save_raw_output: true,
    validate_required_fields: true,
    raw_output_dir: Some("./raw_outputs".to_string()),
    ..Default::default()
};

// Automatic type coercion handles:
// - Currencies: "$1,234.56" → 1234.56
// - Percentages: "95.5%" → 0.955  
// - Booleans: "true", "yes", "1" → true
// - Numbers: "42" → 42, "3.14" → 3.14
// - Emails, phones, URLs, dates
```

#### Rich Visualization

```rust
use langextract::visualization::{export_document, ExportConfig, ExportFormat};

// Export to interactive HTML
let html_config = ExportConfig {
    format: ExportFormat::Html,
    title: Some("Document Analysis".to_string()),
    highlight_extractions: true,
    show_char_intervals: true,
    include_statistics: true,
    ..Default::default()
};

let html_output = export_document(&annotated_doc, &html_config)?;
std::fs::write("analysis.html", html_output)?;

// Also supports Markdown, JSON, and CSV exports
```

#### Provider Configuration

```rust
use langextract::providers::ProviderConfig;

// OpenAI configuration  
let openai_config = ProviderConfig::openai("gpt-4o-mini", Some(api_key));

// Ollama configuration
let ollama_config = ProviderConfig::ollama("mistral", Some("http://localhost:11434".to_string()));

// Custom HTTP API
let custom_config = ProviderConfig::custom("https://my-api.com/v1", "my-model");
```

## 🚀 Example Applications

### Product Catalog Processing
```bash
# Extract product information from catalogs
./test_product_extraction.sh
```

### Academic Paper Analysis  
```bash
# Extract research information from papers
./test_academic_extraction.sh
```

### End-to-End Provider Testing
```bash
# Test with multiple LLM providers
./test_providers.sh
```

## 📋 Supported Providers

| Provider | Models | Features | Use Case |
|----------|--------|----------|----------|
| **OpenAI** | gpt-4o, gpt-4o-mini, gpt-3.5-turbo | High accuracy, JSON mode | Production applications |
| **Ollama** | mistral, llama2, codellama, qwen | Local, privacy-first | Development, sensitive data |
| **Custom** | Any OpenAI-compatible API | Flexible integration | Custom deployments |

### Environment Setup

```bash
# For OpenAI
export OPENAI_API_KEY="your-openai-key"

# For Ollama (local)
ollama serve
ollama pull mistral

# For custom providers
export CUSTOM_API_KEY="your-key"
```

## ⚙️ Performance Configuration

The `ExtractConfig` struct provides fine-grained control over extraction performance:

```rust
let config = ExtractConfig {
    model_id: "mistral".to_string(),
    temperature: 0.3,                    // Lower = more consistent
    max_char_buffer: 8000,               // Chunk size for large documents
    batch_length: 6,                     // Chunks per batch  
    max_workers: 8,                      // Parallel workers (key for speed!)
    extraction_passes: 1,                // Multiple passes for better recall
    enable_multipass: false,             // Advanced multi-pass extraction
    multipass_min_extractions: 5,        // Minimum extractions to avoid re-processing
    multipass_quality_threshold: 0.8,    // Quality threshold for keeping extractions
    debug: true,                         // Enable debug information
    ..Default::default()
};
```

### Performance Tuning Tips

- **max_workers**: Increase for faster processing (6-12 recommended)
- **batch_length**: Larger batches = better throughput (4-8 optimal)  
- **max_char_buffer**: Balance speed vs accuracy (6000-12000 characters)
- **temperature**: Lower values (0.1-0.3) for consistent extraction

See [PERFORMANCE_TUNING.md](PERFORMANCE_TUNING.md) for detailed optimization guide.

## 📚 Real-World Examples

### Document Analysis
Perfect for processing contracts, research papers, or reports:

```rust
let examples = vec![
    ExampleData::new(
        "Dr. Sarah Johnson (contact: s.johnson@mayo.edu) works at Mayo Clinic in Rochester, MN since 2019".to_string(),
        vec![
            Extraction::new("person".to_string(), "Dr. Sarah Johnson".to_string()),
            Extraction::new("email".to_string(), "s.johnson@mayo.edu".to_string()),
            Extraction::new("institution".to_string(), "Mayo Clinic".to_string()),
            Extraction::new("location".to_string(), "Rochester, MN".to_string()),
            Extraction::new("year".to_string(), "2019".to_string()),
        ],
    )
];
```

### Large Document Processing

The library handles large documents automatically with intelligent chunking:

```rust
// Configure for academic papers or catalogs
let config = ExtractConfig {
    max_char_buffer: 8000,     // Optimal chunk size
    max_workers: 8,            // High parallelism  
    batch_length: 6,           // Process multiple chunks per batch
    enable_multipass: true,    // Multiple extraction rounds
    multipass_min_extractions: 3,
    multipass_quality_threshold: 0.8,
    debug: true,               // See processing details
    ..Default::default()
};
```

## Error Handling

The library provides comprehensive error types:

```rust
use langextract::LangExtractError;

match extract(/* ... */).await {
    Ok(result) => println!("Success: {} extractions", result.extraction_count()),
    Err(LangExtractError::ConfigurationError(msg)) => {
        eprintln!("Configuration issue: {}", msg);
    }
    Err(LangExtractError::InferenceError { message, provider, .. }) => {
        eprintln!("Inference failed ({}): {}", provider.unwrap_or("unknown"), message);
    }
    Err(LangExtractError::NetworkError(e)) => {
        eprintln!("Network error: {}", e);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## 🏗️ Architecture & Performance

### High-Performance Features
- **Concurrent processing**: Multiple workers process chunks in parallel
- **UTF-8 safe**: Handles Unicode text with proper character boundary detection
- **Memory efficient**: Streaming processing for large documents  
- **Async I/O**: Non-blocking network operations
- **Smart chunking**: Intelligent text splitting with overlap handling

### Development Status

This Rust implementation provides a complete, production-ready text extraction system:

#### ✅ Core Infrastructure (COMPLETE)
- **Data structures and type system** - Robust extraction and document models
- **Error handling and results** - Comprehensive error types with context
- **Universal provider system** - OpenAI, Ollama, and custom HTTP APIs
- **Async processing pipeline** - High-performance concurrent chunk processing

#### ✅ Text Processing (COMPLETE) 
- **Intelligent chunking** - Automatic document splitting with overlap management
- **Character alignment** - Precise text positioning with fuzzy matching fallback
- **Multi-pass extraction** - Improved recall through multiple extraction rounds
- **Prompt template system** - Flexible LLM prompt generation

#### ✅ Validation & Quality (COMPLETE)
- **Advanced validation system** - Schema validation with type coercion
- **Raw data preservation** - Save original LLM outputs before processing
- **Type coercion** - Automatic conversion of strings to appropriate types
- **Quality assurance** - Validation reporting and data correction

#### ✅ Visualization & Export (COMPLETE)
- **Rich HTML export** - Interactive highlighting with modern styling
- **Multiple formats** - HTML, Markdown, JSON, and CSV export options
- **Character-level highlighting** - Precise extraction positioning in source text
- **Statistical reporting** - Comprehensive extraction analytics

### Architecture Advantages

- **Type Safety**: Compile-time guarantees for configurations and data structures
- **Memory Safety**: Rust's ownership system prevents common memory errors
- **Performance**: Zero-cost abstractions and efficient async processing
- **Explicit Configuration**: Clear, predictable provider and processing setup
- **Unicode Support**: Proper handling of international text and mathematical symbols

## 🧪 Testing & Examples

Run the included test scripts to explore LangExtract capabilities:

```bash
# Test with product catalogs
./test_product_extraction.sh

# Test with academic papers  
./test_academic_extraction.sh

# Test multiple LLM providers
./test_providers.sh
```

Each test generates interactive HTML reports, structured JSON data, and CSV exports for analysis.

## 📄 Documentation

- **[SPEC.md](SPEC.md)** - Complete technical specification and implementation status
- **[PERFORMANCE_TUNING.md](PERFORMANCE_TUNING.md)** - Detailed performance optimization guide
- **[E2E_TEST_README.md](E2E_TEST_README.md)** - End-to-end testing instructions

## 🤝 Contributing

We welcome contributions! Key areas for enhancement:

- Additional LLM provider implementations
- New export formats and visualization options  
- Performance optimizations for specific document types
- Enhanced validation and quality assurance features

## 📜 License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details. For health-related applications, use of LangExtract is also subject to the [Health AI Developer Foundations Terms of Use](https://developers.google.com/health-ai-developer-foundations/terms).


## 📖 Citations & Acknowledgments

This work builds upon research and implementations from the broader NLP and information extraction community:

```bibtex
@misc{langextract,
  title={langextract},
  author={Google Research Team},
  year={2024},
  publisher={GitHub},
  url={https://github.com/google/langextract}
}
```

**Acknowledgments:**
- Inspired by the folks at Google that open-sourced [langextract](https://github.com/google/langextract)
- Thank you so much for providing such a complicated tool to the AI-Engineers of the world trying for more deterministic outcomes.