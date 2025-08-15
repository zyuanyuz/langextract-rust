//! Language model provider implementations.

pub mod config;
pub mod universal;

pub use config::{ProviderConfig, ProviderType};
pub use universal::UniversalProvider;

use crate::exceptions::LangExtractResult;

/// Create a provider from configuration
pub fn create_provider(config: ProviderConfig) -> LangExtractResult<UniversalProvider> {
    UniversalProvider::new(config)
}