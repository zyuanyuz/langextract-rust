//! Demo showing the retry logic in action
//!
//! This example demonstrates how the retry mechanism handles failures
//! by simulating network errors and showing the retry behavior.

use langextract_rust::providers::config::ProviderConfig;
use langextract_rust::providers::universal::UniversalProvider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn::std::error::Error>> {
    println!("🔄 LangExtract Retry Logic Demo");
    println!("================================");

    // Create a mock provider that will fail initially
    let config = ProviderConfig::ollama("test-model", None);
    let provider = UniversalProvider::new(config)?;

    println!("📡 Simulating API calls with retry logic...");
    println!("   - Max retries: 3");
    println!("   - Delay between retries: 30 seconds");
    println!("   - This demo will succeed after 2 failures");
    println!();

    let attempt_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let attempt_count_clone = attempt_count.clone();

    // Simulate an operation that fails twice then succeeds
    let result = provider.retry_with_backoff(
        move || {
            let attempt_count = attempt_count_clone.clone();
            async move {
                let current = attempt_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                match current {
                    0 => {
                        println!("❌ Attempt 1: Network timeout");
                        Err(langextract_rust::exceptions::LangExtractError::inference_simple(
                            "Connection timeout - simulated network error"
                        ))
                    }
                    1 => {
                        println!("❌ Attempt 2: Server error (HTTP 500)");
                        Err(langextract_rust::exceptions::LangExtractError::inference_simple(
                            "Server error: HTTP 500"
                        ))
                    }
                    _ => {
                        println!("✅ Attempt 3: Success!");
                        Ok("Extracted data successfully".to_string())
                    }
                }
            }
        },
        "Demo API call"
    ).await;

    match result {
        Ok(data) => {
            println!();
            println!("🎉 Final result: {}", data);
            println!("   Total attempts made: {}", attempt_count.load(std::sync::atomic::Ordering::SeqCst));
            println!("   Retry logic successfully handled the failures!");
        }
        Err(e) => {
            println!();
            println!("💥 All retry attempts exhausted: {}", e);
        }
    }

    Ok(())
}
