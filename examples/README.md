# LangExtract Examples

This directory contains organized examples demonstrating the capabilities of the LangExtract Rust library.

## 📁 Directory Structure

### 🟢 **basic_examples/**
Essential examples for getting started with LangExtract:

- **`basic_usage.rs`** - Simple introduction to the library
- **`alignment_demo.rs`** - Character-level text alignment features  
- **`validation_demo.rs`** - Type validation and error handling

### 🔵 **advanced_examples/**
Advanced features and complex use cases:

- **`multipass_demo.rs`** - Multi-pass extraction for improved recall
- **`advanced_chunking_demo.rs`** - Token-based and semantic chunking
- **`visualization_demo.rs`** - HTML, JSON, CSV export formats

### 🟡 **product_catalog/**
Real-world product catalog extraction example:

- **`product_catalog_test.rs`** - Comprehensive product data extraction
- **`sample_product_text.txt`** - Sample product catalog data
- **`test_product_extraction.sh`** - Shell script for testing

### 🟠 **pipeline/**
Multi-step pipeline processing examples:

- **`pipeline_demo.rs`** - Complex multi-step extraction workflows
- **`requirements_pipeline.yaml`** - YAML pipeline configuration
- **`sample_requirements.txt`** - Sample requirements document

### 📚 **docs/**
Documentation and guides:

- **`EXAMPLES_STATUS.md`** - Status and testing information for all examples
- **`PIPELINE_README.md`** - Detailed pipeline system documentation
- **`PERFORMANCE_TUNING.md`** - Performance optimization guide
- **`PROVIDER_SYSTEM.md`** - Provider configuration documentation
- **`E2E_TEST_README.md`** - End-to-end testing guide

### 🔧 **scripts/**
Utility scripts for testing and automation:

- **`cli_demo.sh`** - Command-line interface demonstration
- **`test_academic_extraction.sh`** - Academic paper extraction testing
- **`test_providers.sh`** - Provider connectivity testing

## 🚀 Running Examples

### Basic Examples
```bash
# Simple getting started example
cargo run --example basic_examples/basic_usage

# Character alignment demonstration
cargo run --example basic_examples/alignment_demo

# Validation system showcase
cargo run --example basic_examples/validation_demo
```

### Advanced Examples
```bash
# Multi-pass extraction
cargo run --example advanced_examples/multipass_demo

# Advanced chunking strategies
cargo run --example advanced_examples/advanced_chunking_demo

# Export and visualization
cargo run --example advanced_examples/visualization_demo
```

### Specialized Examples
```bash
# Product catalog extraction (requires data file)
cd examples/product_catalog && cargo run --example product_catalog_test

# Pipeline processing (requires YAML config)
cd examples/pipeline && cargo run --example pipeline_demo
```

## 📋 Prerequisites

### Provider Setup
Most examples work with **Ollama** (local) by default:

```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Pull a model (e.g., Mistral)
ollama pull mistral
```

### OpenAI Setup (Optional)
For OpenAI examples, set your API key:

```bash
export OPENAI_API_KEY="your-api-key-here"
# or create a .env file with OPENAI_API_KEY=your-api-key-here
```

## 🎯 Example Categories

| Category | Purpose | Complexity | Provider |
|----------|---------|------------|----------|
| **Basic** | Learning fundamentals | ⭐ | Any |
| **Advanced** | Complex features | ⭐⭐⭐ | Any |
| **Product Catalog** | Real-world use case | ⭐⭐ | Any |
| **Pipeline** | Multi-step workflows | ⭐⭐⭐⭐ | Any |

## 🔍 What Each Example Demonstrates

### Core Features
- **Text Extraction:** Converting unstructured text to structured data
- **Character Alignment:** Precise positioning of extracted entities
- **Chunking:** Intelligent text splitting for large documents
- **Multi-pass Processing:** Enhanced recall through multiple extraction passes
- **Validation:** Type coercion and error handling
- **Visualization:** Multiple export formats (HTML, JSON, CSV)

### Advanced Features
- **Pipeline Processing:** Multi-step dependent extraction workflows
- **Provider Flexibility:** Support for OpenAI, Ollama, and custom APIs
- **Batch Processing:** Efficient parallel processing of multiple chunks
- **Progress Tracking:** Real-time progress monitoring
- **Error Recovery:** Graceful handling of extraction failures

## 📖 Getting Help

1. **Start with `basic_examples/basic_usage.rs`** for a gentle introduction
2. **Check `docs/EXAMPLES_STATUS.md`** for current testing status
3. **Review `docs/PIPELINE_README.md`** for pipeline system details
4. **Run `scripts/test_providers.sh`** to verify your provider setup

## 🤝 Contributing

When adding new examples:

1. **Choose the appropriate subfolder** based on complexity and purpose
2. **Include sample data files** in the same subfolder if needed
3. **Add documentation** explaining the example's purpose
4. **Update this README** with the new example information
5. **Test with multiple providers** when possible

Happy extracting! 🎉
