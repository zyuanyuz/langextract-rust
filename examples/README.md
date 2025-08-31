# LangExtract Examples

This directory contains organized examples demonstrating the capabilities of LangExtract using the **CLI interface** (`lx-rs`). All examples are structured as self-contained demos with configuration files, training data, and executable scripts.

## 📁 Directory Structure

### 🟢 **basic_examples/**
Essential examples for getting started with LangExtract:

- **`basic_usage/`** - Simple introduction to person/profession extraction
- **`alignment_demo/`** - Character-level text alignment and positioning
- **`validation_demo/`** - Data validation, type coercion, and quality assurance

### 🔵 **advanced_examples/**
Advanced features and complex use cases:

- **`multipass_demo/`** - Multi-pass extraction for improved recall
- **`advanced_chunking_demo/`** - Intelligent document chunking strategies
- **`visualization_demo/`** - Rich export formats (HTML, CSV, Markdown, JSON)

### 🟡 **product_catalog/**
Real-world e-commerce and product catalog extraction:

- **`examples.json`** - Comprehensive product training examples
- **`sample_product_text.txt`** - Real product catalog data
- **`run.sh`** - CLI-based product extraction demo
- **`config.yaml`** - Optimized configuration for product data

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

All examples are now **CLI-based** and can be run directly with the `lx-rs` binary or through their individual `run.sh` scripts.

### Prerequisites
```bash
# Option 1: Install the CLI binary
cargo install langextract-rust --features cli

# Option 2: Build locally
cargo build --features cli --release

# Option 3: Use cargo run (slower)
# Examples will automatically fall back to this if binary not found
```

### Basic Examples
```bash
# Navigate to any example directory and run
cd basic_examples/basic_usage/
./run.sh

cd ../alignment_demo/
./run.sh

cd ../validation_demo/
./run.sh
```

### Advanced Examples
```bash
cd advanced_examples/multipass_demo/
./run.sh

cd ../advanced_chunking_demo/
./run.sh

cd ../visualization_demo/
./run.sh
```

### Specialized Examples
```bash
cd product_catalog/
./run.sh

# Or use the legacy script which now calls the CLI version
./test_product_extraction.sh
```

## 📂 Example Structure

Each CLI-based example follows a consistent structure:

```
example_name/
├── examples.json     # Training examples for the LLM
├── config.yaml       # Configuration parameters
├── input.txt         # Sample input data
├── run.sh           # Executable demo script
├── README.md        # Detailed documentation
└── output/          # Generated results (created on run)
```

## 📋 Prerequisites

### Provider Setup
All examples work with **Ollama** (local) by default:

```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh
ollama serve

# Pull a model (e.g., Mistral)
ollama pull mistral
```

### OpenAI Setup (Optional)
For OpenAI examples, set your API key:

```bash
export OPENAI_API_KEY="your-api-key-here"
# or create a .env file with OPENAI_API_KEY=your-api-key-here

# Then edit run.sh to use --provider openai --model gpt-4o-mini
```

### Custom Provider Setup
For custom providers:
```bash
# Edit run.sh to use:
# --provider custom --model-url http://your-api-endpoint
```

## 🎯 Example Categories

| Category | Purpose | Complexity | Features Demonstrated |
|----------|---------|------------|---------------------|
| **Basic Usage** | Learning fundamentals | ⭐ | Person/profession extraction, basic CLI usage |
| **Alignment Demo** | Character positioning | ⭐⭐ | Precise text alignment, character intervals |
| **Validation Demo** | Data quality | ⭐⭐ | Type coercion, validation rules, error handling |
| **Multipass Demo** | Enhanced recall | ⭐⭐⭐ | Multiple extraction rounds, quality comparison |
| **Chunking Demo** | Large documents | ⭐⭐⭐ | Intelligent text splitting, parallel processing |
| **Visualization** | Rich outputs | ⭐⭐ | HTML, CSV, Markdown, JSON exports |
| **Product Catalog** | Real-world use case | ⭐⭐⭐ | E-commerce data, product identifiers, pricing |

## 🔍 What Each Example Demonstrates

### Core Features
- **Text Extraction:** Converting unstructured text to structured data
- **Character Alignment:** Precise positioning of extracted entities in source text
- **CLI Interface:** Complete command-line workflow with `lx-rs`
- **Multiple Providers:** OpenAI, Ollama, and custom HTTP APIs
- **Configuration Management:** YAML config files and JSON examples

### Advanced Features
- **Intelligent Chunking:** Semantic text splitting for large documents
- **Multi-pass Processing:** Enhanced recall through multiple extraction rounds
- **Data Validation:** Type coercion, format validation, quality assurance
- **Rich Visualization:** Interactive HTML, CSV analysis, Markdown documentation
- **Performance Optimization:** Parallel processing, batch operations

### Real-World Applications
- **Product Catalogs:** E-commerce data extraction and analysis
- **Document Processing:** Academic papers, legal documents, reports
- **Contact Information:** Emails, phones, addresses, organizations
- **Financial Data:** Prices, budgets, funding amounts, transactions

## 📖 Getting Started Guide

### 1. Quick Start
```bash
# Install CLI
cargo install langextract-rust --features cli

# Set up Ollama
ollama serve
ollama pull mistral

# Run a basic example
cd examples/basic_examples/basic_usage/
./run.sh
```

### 2. Understanding the Workflow
Each example follows this pattern:
1. **Training Examples** (`examples.json`) teach the LLM what to extract
2. **Configuration** (`config.yaml`) sets processing parameters
3. **Input Data** (`input.txt`) contains the text to process
4. **Execution Script** (`run.sh`) runs the complete extraction pipeline
5. **Results** are generated in multiple formats for analysis

### 3. Customization
- **Modify `examples.json`** to change what gets extracted
- **Edit `config.yaml`** to adjust processing parameters
- **Replace `input.txt`** with your own data
- **Customize `run.sh`** for different providers or output formats

## 🛠️ CLI Commands Reference

Each example demonstrates different CLI features:

```bash
# Basic extraction
lx-rs extract input.txt --examples examples.json --provider ollama

# With specific output format
lx-rs extract input.txt --examples examples.json --format json --output results.json

# With visualization
lx-rs extract input.txt --examples examples.json --export html --show-intervals

# With multipass for better recall
lx-rs extract input.txt --examples examples.json --passes 3 --multipass

# With chunking for large documents
lx-rs extract input.txt --examples examples.json --max-chars 4000 --workers 8

# With specific provider
lx-rs extract input.txt --examples examples.json --provider openai --model gpt-4o-mini
```

## 🧪 Testing and Validation

### Test All Examples
```bash
# Run a comprehensive test of all examples
cd examples/scripts/
./test_all_examples.sh  # (if available)
```

### Individual Example Testing
```bash
# Each example can be tested independently
cd basic_examples/basic_usage/
./run.sh

# Check the generated output files
ls output/
cat output/results.json
```

### Provider Testing
```bash
# Test provider connectivity
lx-rs test --provider ollama --model mistral
lx-rs test --provider openai --model gpt-4o-mini
```

## 📈 Performance Guidelines

- **Small documents** (<5KB): Use basic examples, any provider
- **Medium documents** (5-50KB): Use chunking examples, optimize workers
- **Large documents** (50KB+): Use advanced chunking + multipass
- **Production use**: Combine validation + export for robust pipelines

## 🤝 Contributing

When adding new CLI-based examples:

1. **Follow the standard structure**: examples.json, config.yaml, input.txt, run.sh, README.md
2. **Include comprehensive documentation** in the README.md
3. **Provide realistic sample data** that demonstrates the use case
4. **Test with multiple providers** (at least Ollama and OpenAI)
5. **Add error handling** and fallback options in run.sh
6. **Update this main README** with the new example information

## 🔗 See Also

- **[Main README](../README.md)** - Library overview and installation
- **[CLI Documentation](../src/main.rs)** - Complete CLI command reference
- **[Provider Setup](../docs/PROVIDER_SYSTEM.md)** - Detailed provider configuration
- **[Performance Tuning](../docs/PERFORMANCE_TUNING.md)** - Optimization guidelines

Happy extracting with the LangExtract CLI! 🎉