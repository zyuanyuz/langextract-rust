# LangExtract Examples - Token-Based Chunking Updates

## Overview

All examples in the `examples/` folder have been updated to use the new sophisticated token-based chunking strategy with intelligent sentence boundary detection.

## Key Changes Made

### 1. **Updated Configuration Parameters**

All examples now use the proper `ExtractConfig` parameters:

```rust
ExtractConfig {
    // Token-based chunking configuration
    max_char_buffer: 1000,      // Characters per chunk (respects sentence boundaries)
    batch_length: 3,            // Process chunks in parallel batches
    max_workers: 2,             // Concurrent workers
    extraction_passes: 1,       // Number of extraction passes
    enable_multipass: false,    // Enable multi-pass for complex documents
    
    // Provider configuration (required)
    // Set via language_model_params["provider_config"]
}
```

### 2. **Provider Configuration**

All examples now properly configure providers:

**For Ollama (Local):**
```rust
let provider_config = ProviderConfig::ollama("mistral", Some("http://localhost:11434".to_string()));
config.language_model_params.insert(
    "provider_config".to_string(),
    serde_json::to_value(&provider_config)?,
);
```

**For OpenAI:**
```rust
let provider_config = ProviderConfig::openai("gpt-3.5-turbo", None);
config.language_model_params.insert(
    "provider_config".to_string(),
    serde_json::to_value(&provider_config)?,
);
```

### 3. **Updated Examples**

| Example | Status | Description |
|---------|---------|-------------|
| `basic_usage.rs` | ✅ Updated | Shows basic token-based chunking configuration |
| `chunking_test.rs` | ✅ Updated | Demonstrates intelligent chunking with large documents |
| `openai_chunking_test.rs` | ✅ Updated | OpenAI-specific chunking optimization |
| `openai_test.rs` | ✅ Updated | Basic OpenAI test with chunking support |
| `ollama_test.rs` | ✅ Updated | Local Ollama test with token-based chunking |
| `multipass_demo.rs` | ✅ Updated | Shows single vs. multi-pass extraction comparison |
| `openai_providers_demo.rs` | ✅ Updated | Multi-provider configuration examples |
| `advanced_chunking_demo.rs` | 🆕 **NEW** | Comprehensive demo of advanced features |
| `real_provider_test.rs` | 🆕 **NEW** | Tests both OpenAI and Ollama with real setup |

## Testing Results

### ✅ **Working Examples (Tested)**

1. **Ollama Examples** - Working perfectly with local Mistral:
   - `ollama_test.rs` - ✅ 11 extractions found
   - `chunking_test.rs` - ✅ Token-based chunking working (8 chunks)
   - `multipass_demo.rs` - ✅ 23 extractions in single-pass
   - `real_provider_test.rs` - ✅ 13 extractions found

2. **OpenAI Examples** - Ready to work with API key:
   - `openai_test.rs` - ⚠️ Needs `OPENAI_API_KEY` in .env
   - `openai_chunking_test.rs` - ⚠️ Needs `OPENAI_API_KEY` in .env
   - `real_provider_test.rs` - ⚠️ Needs `OPENAI_API_KEY` in .env

### 🔧 **Key Features Demonstrated**

1. **Token-Based Chunking:**
   - Intelligent sentence boundary detection
   - Respects linguistic structure
   - No arbitrary text cuts

2. **Parallel Processing:**
   - Configurable batch sizes
   - Multiple workers for efficiency
   - Progress tracking and debugging

3. **Multi-Pass Extraction:**
   - Enhanced recall for complex documents
   - Quality thresholds and refinement
   - Comparison with single-pass baseline

4. **Provider Flexibility:**
   - OpenAI API integration
   - Local Ollama support
   - Consistent API across providers

## Running the Examples

### Prerequisites

1. **For Ollama examples:**
   ```bash
   # Install and start Ollama
   ollama pull mistral
   ollama serve  # If not running automatically
   ```

2. **For OpenAI examples:**
   ```bash
   # Create .env file with:
   echo "OPENAI_API_KEY=your_api_key_here" > .env
   ```

### Running Examples

```bash
# Test local Ollama with token-based chunking
cargo run --example ollama_test

# Test chunking with large documents
cargo run --example chunking_test

# Test multi-pass extraction
cargo run --example multipass_demo

# Test both providers (needs API key for OpenAI)
cargo run --example real_provider_test

# Test advanced chunking features
cargo run --example advanced_chunking_demo
```

## Expected Output

### Ollama Test Success:
```
✅ Extraction successful!
Found 11 extractions
   • person_name: 3 instance(s)
   • age: 2 instance(s)
   • job_title: 3 instance(s)
   • location: 3 instance(s)
```

### Token-Based Chunking Output:
```
📄 Processing document with 3 token-based chunks (2473 chars total)
   Token Chunk 0: 961 chars (offset: 0)
   Token Chunk 1: 989 chars (offset: 963)
   Token Chunk 2: 519 chars (offset: 1954)
```

## Benefits of Token-Based Chunking

1. **Preserves Context:** Chunks respect sentence and paragraph boundaries
2. **Better Accuracy:** No mid-sentence cuts that confuse the model
3. **Efficient Processing:** Parallel batch processing with configurable workers
4. **Scalable:** Handles documents of any size intelligently
5. **Debuggable:** Comprehensive logging and progress tracking

## Next Steps

- Set up your `.env` file with `OPENAI_API_KEY` to test OpenAI examples
- Experiment with different `max_char_buffer` sizes for your use case
- Try multi-pass extraction for complex documents with many entities
- Adjust worker counts based on your system capabilities and API limits