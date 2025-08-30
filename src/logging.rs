//! Logging and progress reporting system for LangExtract.
//!
//! This module provides a unified system for logging and progress reporting
//! that can be controlled by library users and CLI applications.

use std::sync::Arc;

/// Progress event types for different stages of processing
#[derive(Debug, Clone)]
pub enum ProgressEvent {
    /// Text processing started
    ProcessingStarted {
        text_length: usize,
        model: String,
        provider: String,
    },
    /// Text is being chunked
    ChunkingStarted {
        total_chars: usize,
        chunk_count: usize,
        strategy: String,
    },
    /// Batch processing progress
    BatchProgress {
        batch_number: usize,
        total_batches: usize,
        chunks_processed: usize,
        total_chunks: usize,
    },
    /// Language model call in progress
    ModelCall {
        provider: String,
        model: String,
        input_length: usize,
    },
    /// Model response received
    ModelResponse {
        success: bool,
        output_length: Option<usize>,
    },
    /// Extraction validation and parsing
    ValidationStarted {
        raw_output_length: usize,
    },
    /// Validation completed
    ValidationCompleted {
        extractions_found: usize,
        aligned_count: usize,
        errors: usize,
        warnings: usize,
    },
    /// Results aggregation
    AggregationStarted {
        chunk_count: usize,
    },
    /// Processing completed
    ProcessingCompleted {
        total_extractions: usize,
        processing_time_ms: u64,
    },
    /// Retry attempt
    RetryAttempt {
        operation: String,
        attempt: usize,
        max_attempts: usize,
        delay_seconds: u64,
    },
    /// Error occurred
    Error {
        operation: String,
        error: String,
    },
    /// Debug information
    Debug {
        operation: String,
        details: String,
    },
}

/// Trait for handling progress events
pub trait ProgressHandler: Send + Sync {
    /// Handle a progress event
    fn handle_progress(&self, event: ProgressEvent);
}

/// Console progress handler that outputs to stdout
pub struct ConsoleProgressHandler {
    /// Whether to show progress messages
    pub show_progress: bool,
    /// Whether to show debug information
    pub show_debug: bool,
    /// Whether to use emoji and colors
    pub use_styling: bool,
}

impl ConsoleProgressHandler {
    /// Create a new console handler with default settings
    pub fn new() -> Self {
        Self {
            show_progress: true,
            show_debug: false,
            use_styling: true,
        }
    }

    /// Create a quiet console handler (only errors)
    pub fn quiet() -> Self {
        Self {
            show_progress: false,
            show_debug: false,
            use_styling: true,
        }
    }

    /// Create a verbose console handler (everything)
    pub fn verbose() -> Self {
        Self {
            show_progress: true,
            show_debug: true,
            use_styling: true,
        }
    }

    /// Create a machine-readable handler (no styling)
    pub fn machine_readable() -> Self {
        Self {
            show_progress: true,
            show_debug: false,
            use_styling: false,
        }
    }

    fn format_message(&self, emoji: &str, message: &str) -> String {
        if self.use_styling {
            format!("{} {}", emoji, message)
        } else {
            format!("[{}] {}", 
                match emoji {
                    "🤖" => "MODEL",
                    "📄" => "CHUNK",
                    "🔄" => "PROGRESS",
                    "✅" => "SUCCESS",
                    "❌" => "ERROR",
                    "⏳" => "WAIT",
                    "🎯" => "COMPLETE",
                    "🔧" => "DEBUG",
                    "📥" => "RESPONSE",
                    "📡" => "REQUEST",
                    "🎉" => "SUCCESS",
                    _ => "INFO",
                },
                message
            )
        }
    }
}

impl Default for ConsoleProgressHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressHandler for ConsoleProgressHandler {
    fn handle_progress(&self, event: ProgressEvent) {
        match event {
            ProgressEvent::ProcessingStarted { text_length, model, provider } => {
                if self.show_progress {
                    let msg = format!("Calling {} model: {} ({} chars input)", provider, model, text_length);
                    println!("{}", self.format_message("🤖", &msg));
                }
            }
            ProgressEvent::ChunkingStarted { total_chars, chunk_count, strategy } => {
                if self.show_progress {
                    let msg = format!("Processing document with {} {} chunks ({} chars total)", 
                        chunk_count, strategy, total_chars);
                    println!("{}", self.format_message("📄", &msg));
                }
            }
            ProgressEvent::BatchProgress { batch_number, total_batches: _, chunks_processed, total_chunks } => {
                if self.show_progress {
                    let msg = format!("Processing batch {} ({}/{} chunks processed)", 
                        batch_number, chunks_processed, total_chunks);
                    println!("{}", self.format_message("🔄", &msg));
                }
            }
            ProgressEvent::ModelCall { provider, model: _, input_length } => {
                if self.show_progress {
                    let msg = format!("Starting {} API call with retry logic for input size {}", provider, input_length);
                    println!("{}", self.format_message("🔄", &msg));
                }
            }
            ProgressEvent::ModelResponse { success, output_length: _ } => {
                if self.show_progress {
                    let msg = if success {
                        "Received response from language model".to_string()
                    } else {
                        "Failed to receive response from language model".to_string()
                    };
                    println!("{}", self.format_message("📥", &msg));
                }
            }
            ProgressEvent::AggregationStarted { chunk_count } => {
                if self.show_progress {
                    let msg = format!("Aggregating results from {} chunks...", chunk_count);
                    println!("{}", self.format_message("🔄", &msg));
                }
            }
            ProgressEvent::ProcessingCompleted { total_extractions, processing_time_ms: _ } => {
                if self.show_progress {
                    let msg = format!("Extraction complete! Found {} total extractions", total_extractions);
                    println!("{}", self.format_message("🎯", &msg));
                }
            }
            ProgressEvent::RetryAttempt { operation, attempt, max_attempts, delay_seconds } => {
                if self.show_progress {
                    println!("{}", self.format_message("❌", 
                        &format!("{} failed (attempt {}/{})", operation, attempt, max_attempts)));
                    println!("{}", self.format_message("⏳", 
                        &format!("Retrying in {} seconds...", delay_seconds)));
                }
            }
            ProgressEvent::Error { operation, error } => {
                // Always show errors
                println!("{}", self.format_message("❌", &format!("{}: {}", operation, error)));
            }
            ProgressEvent::Debug { operation, details } => {
                if self.show_debug {
                    println!("{}", self.format_message("🔧", &format!("DEBUG: {}: {}", operation, details)));
                }
            }
            ProgressEvent::ValidationStarted { raw_output_length: _ } => {
                // Internal event, no output needed
            }
            ProgressEvent::ValidationCompleted { extractions_found, aligned_count, errors, warnings } => {
                if self.show_debug {
                    let msg = format!("Validation result: {} extractions ({} aligned), {} errors, {} warnings", 
                        extractions_found, aligned_count, errors, warnings);
                    println!("{}", self.format_message("🔧", &format!("DEBUG: {}", msg)));
                }
            }
        }
    }
}

/// Silent progress handler that does nothing
pub struct SilentProgressHandler;

impl ProgressHandler for SilentProgressHandler {
    fn handle_progress(&self, _event: ProgressEvent) {
        // Do nothing
    }
}

/// Logger that integrates with the standard log crate
pub struct LogProgressHandler;

impl ProgressHandler for LogProgressHandler {
    fn handle_progress(&self, event: ProgressEvent) {
        match event {
            ProgressEvent::ProcessingStarted { text_length, model, provider } => {
                log::info!("Starting extraction with {} model {} ({} chars)", provider, model, text_length);
            }
            ProgressEvent::ChunkingStarted { total_chars, chunk_count, strategy } => {
                log::info!("Chunking document: {} {} chunks ({} chars)", chunk_count, strategy, total_chars);
            }
            ProgressEvent::BatchProgress { batch_number, total_batches: _, chunks_processed, total_chunks } => {
                log::debug!("Processing batch {}: {}/{} chunks", batch_number, chunks_processed, total_chunks);
            }
            ProgressEvent::ModelCall { provider, model, input_length } => {
                log::debug!("Calling {} model {} with {} chars input", provider, model, input_length);
            }
            ProgressEvent::ModelResponse { success, output_length } => {
                if success {
                    log::debug!("Received response: {} chars", output_length.unwrap_or(0));
                } else {
                    log::warn!("Failed to receive model response");
                }
            }
            ProgressEvent::ValidationCompleted { extractions_found, aligned_count, errors, warnings } => {
                log::debug!("Validation: {} extractions ({} aligned), {} errors, {} warnings", 
                    extractions_found, aligned_count, errors, warnings);
            }
            ProgressEvent::AggregationStarted { chunk_count } => {
                log::debug!("Aggregating {} chunks", chunk_count);
            }
            ProgressEvent::ProcessingCompleted { total_extractions, processing_time_ms } => {
                log::info!("Extraction completed: {} extractions in {}ms", total_extractions, processing_time_ms);
            }
            ProgressEvent::RetryAttempt { operation, attempt, max_attempts, delay_seconds } => {
                log::warn!("Retry {}/{} for {}, waiting {}s", attempt, max_attempts, operation, delay_seconds);
            }
            ProgressEvent::Error { operation, error } => {
                log::error!("{}: {}", operation, error);
            }
            ProgressEvent::Debug { operation, details } => {
                log::debug!("{}: {}", operation, details);
            }
            ProgressEvent::ValidationStarted { .. } => {
                log::trace!("Starting validation");
            }
        }
    }
}

/// Global progress handler
static mut PROGRESS_HANDLER: Option<Arc<dyn ProgressHandler>> = None;
static INIT: std::sync::Once = std::sync::Once::new();

/// Initialize the global progress handler
pub fn init_progress_handler(handler: Arc<dyn ProgressHandler>) {
    unsafe {
        PROGRESS_HANDLER = Some(handler);
    }
}

/// Get the current progress handler, or create a default one
fn get_progress_handler() -> Arc<dyn ProgressHandler> {
    unsafe {
        PROGRESS_HANDLER.clone().unwrap_or_else(|| {
            INIT.call_once(|| {
                PROGRESS_HANDLER = Some(Arc::new(ConsoleProgressHandler::new()));
            });
            PROGRESS_HANDLER.clone().unwrap()
        })
    }
}

/// Report a progress event
pub fn report_progress(event: ProgressEvent) {
    let handler = get_progress_handler();
    handler.handle_progress(event);
}

/// Convenience macros for common progress events
#[macro_export]
macro_rules! progress_info {
    ($($arg:tt)*) => {
        $crate::logging::report_progress($crate::logging::ProgressEvent::Debug {
            operation: "info".to_string(),
            details: format!($($arg)*),
        });
    };
}

#[macro_export]
macro_rules! progress_debug {
    ($operation:expr, $($arg:tt)*) => {
        $crate::logging::report_progress($crate::logging::ProgressEvent::Debug {
            operation: $operation.to_string(),
            details: format!($($arg)*),
        });
    };
}

#[macro_export]
macro_rules! progress_error {
    ($operation:expr, $($arg:tt)*) => {
        $crate::logging::report_progress($crate::logging::ProgressEvent::Error {
            operation: $operation.to_string(),
            error: format!($($arg)*),
        });
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_handler_formatting() {
        let handler = ConsoleProgressHandler::new();
        let message = handler.format_message("🤖", "Test message");
        assert!(message.contains("🤖"));
        assert!(message.contains("Test message"));

        let machine_handler = ConsoleProgressHandler::machine_readable();
        let machine_message = machine_handler.format_message("🤖", "Test message");
        assert!(machine_message.contains("[MODEL]"));
        assert!(machine_message.contains("Test message"));
    }

    #[test]
    fn test_progress_events() {
        let handler = ConsoleProgressHandler::quiet();
        
        // Should not panic
        handler.handle_progress(ProgressEvent::ProcessingStarted {
            text_length: 1000,
            model: "test-model".to_string(),
            provider: "test-provider".to_string(),
        });
    }
}
