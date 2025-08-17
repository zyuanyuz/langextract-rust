# Provider System Implementation

This document outlines the new agnostic provider system implemented for langextract-rust, which improves upon the Python version's auto-selection approach.

## Overview

The new provider system allows explicit specification of:
- **API Type**: OpenAI, Ollama, or Custom
- **Base URL**: For self-hosted or custom endpoints  
- **Model**: Any model supported by the provider
- **Headers**: Custom headers for authentication/routing
- **Parameters**: Provider-specific configuration

## Architecture

### Core Components

1. **`ProviderType`** - Enum defining supported provider types
2. **`ProviderConfig`** - Universal configuration structure
3. **`UniversalProvider`** - Single provider implementation that handles all types
4. **Factory Functions** - Convenience functions for creating providers

### Provider Types

```rust
pub enum ProviderType {
    OpenAI,   // OpenAI-compatible APIs (OpenAI, Azure OpenAI, etc.)
    Ollama,   // Local Ollama server
    Custom,   // Generic HTTP API
}
```

## Configuration Examples

### 1. Ollama (Local Models)

```rust
use langextract::{ProviderConfig, ExtractConfig};

// Via ProviderConfig (explicit)
let provider_config = ProviderConfig::ollama("mistral", None)
    .with_base_url("http://localhost:11434".to_string());

// Via ExtractConfig (explicit provider config required)
let provider_config = ProviderConfig::ollama("mistral", Some("http://localhost:11434".to_string()));
let mut extract_config = ExtractConfig {
    model_id: "mistral".to_string(),
    model_url: Some("http://localhost:11434".to_string()),
    ..Default::default()
};
extract_config.language_model_params.insert(
    "provider_config".to_string(),
    serde_json::to_value(&provider_config).unwrap()
);
```

### 2. OpenAI-Compatible APIs

```rust
// OpenAI
let openai_config = ProviderConfig::openai("gpt-4", Some(api_key));

// Azure OpenAI
let azure_config = ProviderConfig::openai("gpt-4", Some(api_key))
    .with_base_url("https://my-resource.openai.azure.com/".to_string())
    .with_header("api-version".to_string(), "2024-02-01".to_string());
```

### 3. Custom APIs

```rust
let custom_config = ProviderConfig::custom("https://my-api.com/v1", "my-model")
    .with_api_key("custom-key".to_string())
    .with_header("X-Custom-Auth".to_string(), "bearer-token".to_string())
    .with_extra_param("custom_param".to_string(), serde_json::json!("value"));
```

## Explicit Configuration Required

**Auto-detection has been removed!** Provider configuration is now required to be explicit. You must specify the provider type either:

1. Via CLI: `--provider <openai|ollama|custom>`
2. Via ProviderConfig in code
3. Via language_model_params in ExtractConfig

This ensures predictable behavior and eliminates hidden "magic" based on model names.

## Implementation Status

### ✅ Completed
- Provider type definitions and configuration
- Universal provider structure
- Ollama HTTP API integration
- OpenAI provider framework (with async-openai)
- Explicit provider configuration requirement (auto-detection removed)
- Factory functions for easy creation
- Comprehensive test coverage

### 🚧 In Progress
- Complete OpenAI API integration (async-openai compatibility)
- Custom provider HTTP handling
- Schema constraints integration
- Response parsing and validation

## Advantages Over Python Version

1. **No Magic**: Explicit provider configuration required, no hidden model-name-based auto-detection
2. **Predictability**: Always know exactly which provider will be used
3. **Flexibility**: Support for any base URL, headers, and custom parameters
4. **Type Safety**: Compile-time guarantees for provider configurations
5. **Extensibility**: Easy to add new provider types without changing core logic
6. **Testing**: Providers can be easily mocked and tested
7. **Performance**: Single provider implementation reduces overhead

## Usage Patterns

### Simple Usage (Explicit Configuration)
```rust
let provider_config = ProviderConfig::ollama("mistral", None);
let mut config = ExtractConfig {
    model_id: "mistral".to_string(),
    ..Default::default()
};
config.language_model_params.insert(
    "provider_config".to_string(),
    serde_json::to_value(&provider_config).unwrap()
);
```

### Explicit Configuration
```rust
let provider_config = ProviderConfig::ollama("mistral", None);
let provider = create_provider(provider_config)?;
```

### Custom Endpoint
```rust
let provider_config = ProviderConfig::custom("https://my-custom-api.com", "my-model")
    .with_api_key("my-key".to_string());
let mut config = ExtractConfig {
    model_id: "my-model".to_string(),
    model_url: Some("https://my-custom-api.com".to_string()),
    api_key: Some("my-key".to_string()),
    ..Default::default()
};
config.language_model_params.insert(
    "provider_config".to_string(),
    serde_json::to_value(&provider_config).unwrap()
);
```

## Testing

Run the provider system tests:

```bash
# Basic compilation and unit tests
cargo test

# Provider configuration demo
cargo run --example provider_demo

# Ollama integration test (requires Ollama running)
cargo run --example ollama_test
```

## Future Enhancements

1. **Provider Plugins**: Dynamic loading of provider implementations
2. **Circuit Breakers**: Automatic failover between providers
3. **Rate Limiting**: Built-in rate limiting per provider
4. **Metrics**: Provider performance monitoring
5. **Caching**: Response caching for expensive operations

## Migration from Python

The Rust provider system requires explicit configuration instead of auto-detection:

| Python (Auto-Selection) | Rust (Explicit Required) |
|-------------------------|---------------------------|
| `model_id="gpt-4"` → Auto-detected OpenAI | `--provider openai --model gpt-4` (CLI) or `ProviderConfig::openai("gpt-4", api_key)` (code) |
| Model name pattern detection | Explicit provider type specification |
| Hidden provider instantiation | Transparent provider creation with configuration |

This design eliminates magic behavior and makes the system more predictable, testable, and maintainable.
