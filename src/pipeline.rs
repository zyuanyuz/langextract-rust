//! Pipeline processing for multi-step information extraction.
//!
//! This module provides a pipeline system for processing documents through
//! multiple extraction steps, creating nested hierarchical structures from text.

use crate::{
    data::{ExampleData, Extraction},
    exceptions::{LangExtractError, LangExtractResult},
    extract, ExtractConfig,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single step in a processing pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStep {
    /// Unique identifier for this step
    pub id: String,

    /// Human-readable name for this step
    pub name: String,

    /// Description of what this step extracts
    pub description: String,

    /// Examples for this extraction step
    pub examples: Vec<ExampleData>,

    /// Extraction prompt/description
    pub prompt: String,

    /// Output field name for the results of this step
    pub output_field: String,

    /// Optional filter to only process certain extractions from previous steps
    pub filter: Option<PipelineFilter>,

    /// Dependencies - this step depends on output from these step IDs
    pub depends_on: Vec<String>,
}

/// Filter configuration for processing specific extractions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineFilter {
    /// Filter by extraction class
    pub class_filter: Option<String>,

    /// Filter by regex pattern on extraction text
    pub text_pattern: Option<String>,

    /// Maximum number of items to process
    pub max_items: Option<usize>,
}

/// Configuration for the entire pipeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Pipeline name
    pub name: String,

    /// Pipeline description
    pub description: String,

    /// Pipeline version
    pub version: String,

    /// All processing steps
    pub steps: Vec<PipelineStep>,

    /// Global configuration that applies to all steps
    pub global_config: ExtractConfig,
}

/// Results from a single pipeline step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step ID
    pub step_id: String,

    /// Step name
    pub step_name: String,

    /// Extractions produced by this step
    pub extractions: Vec<Extraction>,

    /// Processing time in milliseconds
    pub processing_time_ms: u64,

    /// Number of input items processed
    pub input_count: usize,

    /// Success status
    pub success: bool,

    /// Error message if failed
    pub error_message: Option<String>,
}

/// Complete pipeline execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    /// Pipeline configuration used
    pub config: PipelineConfig,

    /// Results from each step
    pub step_results: Vec<StepResult>,

    /// Final nested output structure
    pub nested_output: serde_json::Value,

    /// Total processing time
    pub total_time_ms: u64,

    /// Overall success status
    pub success: bool,

    /// Error message if pipeline failed
    pub error_message: Option<String>,
}

/// Pipeline executor
pub struct PipelineExecutor {
    config: PipelineConfig,
}

impl PipelineExecutor {
    /// Create a new pipeline executor
    pub fn new(config: PipelineConfig) -> Self {
        Self { config }
    }

    /// Load pipeline configuration from YAML file
    pub fn from_yaml_file(path: &std::path::Path) -> LangExtractResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| LangExtractError::configuration(format!("Failed to read pipeline file: {}", e)))?;

        let config: PipelineConfig = serde_yaml::from_str(&content)
            .map_err(|e| LangExtractError::configuration(format!("Failed to parse pipeline YAML: {}", e)))?;

        Ok(Self::new(config))
    }

    /// Execute the entire pipeline
    pub async fn execute(&self, input_text: &str) -> LangExtractResult<PipelineResult> {
        let start_time = std::time::Instant::now();

        println!("🚀 Starting pipeline execution: {}", self.config.name);
        println!("📝 Description: {}", self.config.description);

        let mut step_results = Vec::new();
        let mut context_data = HashMap::new();

        // Execute steps in dependency order
        let execution_order = self.resolve_execution_order()?;

        for step_id in execution_order {
            let step_result = self.execute_step(&step_id, input_text, &context_data).await?;
            step_results.push(step_result.clone());

            // Store results for dependent steps
            if step_result.success {
                context_data.insert(step_id, step_result.extractions.clone());
            } else {
                return Err(LangExtractError::configuration(format!(
                    "Step '{}' failed: {}",
                    step_id,
                    step_result.error_message.unwrap_or("Unknown error".to_string())
                )));
            }
        }

        // Build nested output structure
        let nested_output = self.build_nested_output(&step_results)?;

        let total_time = start_time.elapsed().as_millis() as u64;

        println!("✅ Pipeline execution completed in {}ms", total_time);

        Ok(PipelineResult {
            config: self.config.clone(),
            step_results,
            nested_output,
            total_time_ms: total_time,
            success: true,
            error_message: None,
        })
    }

    /// Resolve the execution order based on dependencies
    fn resolve_execution_order(&self) -> LangExtractResult<Vec<String>> {
        let mut order = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut visiting = std::collections::HashSet::new();

        for step in &self.config.steps {
            self.resolve_step_dependencies(&step.id, &mut order, &mut visited, &mut visiting)?;
        }

        Ok(order)
    }

    /// Recursive function to resolve dependencies
    fn resolve_step_dependencies(
        &self,
        step_id: &str,
        order: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
        visiting: &mut std::collections::HashSet<String>,
    ) -> LangExtractResult<()> {
        if visited.contains(step_id) {
            return Ok(());
        }

        if visiting.contains(step_id) {
            return Err(LangExtractError::configuration(format!(
                "Circular dependency detected involving step: {}", step_id
            )));
        }

        visiting.insert(step_id.to_string());

        // Find the step and process its dependencies
        if let Some(step) = self.config.steps.iter().find(|s| s.id == step_id) {
            for dep in &step.depends_on {
                self.resolve_step_dependencies(dep, order, visited, visiting)?;
            }
        }

        visiting.remove(step_id);
        visited.insert(step_id.to_string());
        order.push(step_id.to_string());

        Ok(())
    }

    /// Execute a single pipeline step
    async fn execute_step(
        &self,
        step_id: &str,
        input_text: &str,
        context_data: &HashMap<String, Vec<Extraction>>,
    ) -> LangExtractResult<StepResult> {
        let step = self.config.steps.iter().find(|s| s.id == step_id)
            .ok_or_else(|| LangExtractError::configuration(format!("Step '{}' not found", step_id)))?;

        let step_start = std::time::Instant::now();

        println!("🔄 Executing step: {} ({})", step.name, step.id);

        // Determine input text for this step
        let step_input = self.prepare_step_input(step, input_text, context_data)?;
        let input_count = step_input.len();

        println!("📥 Processing {} input items", input_count);

        let mut all_extractions = Vec::new();

        // Process each input item
        for (i, input_item) in step_input.iter().enumerate() {
            println!("  📄 Processing item {}/{}", i + 1, input_count);

            // Create extraction config for this step
            let step_config = self.config.global_config.clone();
            // Use step-specific examples if provided, otherwise use global
            let examples = if step.examples.is_empty() {
                vec![] // Will need to be provided externally
            } else {
                step.examples.clone()
            };

            match extract(
                input_item,
                Some(&step.prompt),
                &examples,
                step_config,
            ).await {
                Ok(result) => {
                    if let Some(extractions) = result.extractions {
                        all_extractions.extend(extractions);
                    }
                }
                Err(e) => {
                    println!("  ❌ Step '{}' failed on item {}/{}: {}", step.id, i + 1, input_count, e);
                    return Ok(StepResult {
                        step_id: step.id.clone(),
                        step_name: step.name.clone(),
                        extractions: Vec::new(),
                        processing_time_ms: step_start.elapsed().as_millis() as u64,
                        input_count,
                        success: false,
                        error_message: Some(e.to_string()),
                    });
                }
            }
        }

        let processing_time = step_start.elapsed().as_millis() as u64;

        println!("  ✅ Step '{}' completed: {} extractions in {}ms",
                step.name, all_extractions.len(), processing_time);

        Ok(StepResult {
            step_id: step.id.clone(),
            step_name: step.name.clone(),
            extractions: all_extractions,
            processing_time_ms: processing_time,
            input_count,
            success: true,
            error_message: None,
        })
    }

    /// Prepare input text for a step based on its configuration
    fn prepare_step_input(
        &self,
        step: &PipelineStep,
        original_text: &str,
        context_data: &HashMap<String, Vec<Extraction>>,
    ) -> LangExtractResult<Vec<String>> {
        // If step has dependencies, use extractions from dependent steps
        if !step.depends_on.is_empty() {
            let mut inputs = Vec::new();

            for dep_id in &step.depends_on {
                if let Some(extractions) = context_data.get(dep_id) {
                    // Apply filter if specified
                    let filtered_extractions = self.apply_filter(extractions, &step.filter);

                    for extraction in filtered_extractions {
                        inputs.push(extraction.extraction_text.clone());
                    }
                }
            }

            Ok(inputs)
        } else {
            // First step - use original text
            Ok(vec![original_text.to_string()])
        }
    }

    /// Apply filter to extractions
    fn apply_filter<'a>(
        &self,
        extractions: &'a [Extraction],
        filter: &Option<PipelineFilter>,
    ) -> Vec<&'a Extraction> {
        if let Some(f) = filter {
            extractions.iter()
                .filter(|e| {
                    // Check class filter
                    if let Some(class) = &f.class_filter {
                        if e.extraction_class != *class {
                            return false;
                        }
                    }

                    // Check text pattern filter
                    if let Some(pattern) = &f.text_pattern {
                        if let Ok(regex) = regex::Regex::new(pattern) {
                            if !regex.is_match(&e.extraction_text) {
                                return false;
                            }
                        }
                    }

                    true
                })
                .take(f.max_items.unwrap_or(usize::MAX))
                .collect()
        } else {
            extractions.iter().collect()
        }
    }

    /// Build the final nested output structure
    fn build_nested_output(&self, step_results: &[StepResult]) -> LangExtractResult<serde_json::Value> {
        let mut output = serde_json::Map::new();

        // Group results by step
        for result in step_results {
            if result.success {
                let mut step_output = serde_json::Map::new();

                // Convert extractions to JSON
                let extractions_json: Vec<serde_json::Value> = result.extractions.iter()
                    .map(|e| {
                        let mut obj = serde_json::Map::new();
                        obj.insert("class".to_string(), serde_json::Value::String(e.extraction_class.clone()));
                        obj.insert("text".to_string(), serde_json::Value::String(e.extraction_text.clone()));
                        if let Some(interval) = &e.char_interval {
                            obj.insert("start".to_string(), serde_json::json!(interval.start_pos));
                            obj.insert("end".to_string(), serde_json::json!(interval.end_pos));
                        }
                        serde_json::Value::Object(obj)
                    })
                    .collect();

                step_output.insert("extractions".to_string(), serde_json::Value::Array(extractions_json));
                step_output.insert("count".to_string(), serde_json::json!(result.extractions.len()));
                step_output.insert("processing_time_ms".to_string(), serde_json::json!(result.processing_time_ms));

                output.insert(result.step_id.clone(), serde_json::Value::Object(step_output));
            }
        }

        Ok(serde_json::Value::Object(output))
    }
}

/// Utility functions for pipeline management
pub mod utils {
    use super::*;

    /// Create a sample pipeline configuration for requirements extraction
    pub fn create_requirements_pipeline() -> PipelineConfig {
        PipelineConfig {
            name: "Requirements Extraction Pipeline".to_string(),
            description: "Extract requirements and sub-divide into values, units, and specifications".to_string(),
            version: "1.0.0".to_string(),
            global_config: ExtractConfig {
                model_id: "gemini-2.5-flash".to_string(),
                api_key: None,
                format_type: crate::data::FormatType::Json,
                max_char_buffer: 8000,
                temperature: 0.3,
                fence_output: None,
                use_schema_constraints: true,
                batch_length: 4,
                max_workers: 6,
                additional_context: None,
                resolver_params: std::collections::HashMap::new(),
                language_model_params: std::collections::HashMap::new(),
                debug: false,
                model_url: None,
                extraction_passes: 1,
                enable_multipass: false,
                multipass_min_extractions: 1,
                multipass_quality_threshold: 0.3,
                progress_handler: None,
            },
            steps: vec![
                PipelineStep {
                    id: "extract_requirements".to_string(),
                    name: "Extract Requirements".to_string(),
                    description: "Extract all 'shall' statements and requirements from the document".to_string(),
                    examples: vec![
                        ExampleData::new(
                            "The system shall process 100 transactions per second and maintain 99.9% uptime.".to_string(),
                            vec![
                                Extraction::new("requirement".to_string(),
                                    "The system shall process 100 transactions per second and maintain 99.9% uptime.".to_string()),
                            ],
                        )
                    ],
                    prompt: "Extract all requirements, 'shall' statements, and specifications from the text. Include the complete statement.".to_string(),
                    output_field: "requirements".to_string(),
                    filter: None,
                    depends_on: vec![],
                },
                PipelineStep {
                    id: "extract_values".to_string(),
                    name: "Extract Values".to_string(),
                    description: "Extract numeric values, units, and specifications from requirements".to_string(),
                    examples: vec![
                        ExampleData::new(
                            "The system shall process 100 transactions per second and maintain 99.9% uptime.".to_string(),
                            vec![
                                Extraction::new("value".to_string(), "100".to_string()),
                                Extraction::new("unit".to_string(), "transactions per second".to_string()),
                                Extraction::new("value".to_string(), "99.9".to_string()),
                                Extraction::new("unit".to_string(), "%".to_string()),
                            ],
                        )
                    ],
                    prompt: "From this requirement, extract all numeric values and their associated units or specifications.".to_string(),
                    output_field: "values".to_string(),
                    filter: Some(PipelineFilter {
                        class_filter: Some("requirement".to_string()),
                        text_pattern: None,
                        max_items: None,
                    }),
                    depends_on: vec!["extract_requirements".to_string()],
                },
                PipelineStep {
                    id: "extract_specifications".to_string(),
                    name: "Extract Specifications".to_string(),
                    description: "Extract detailed specifications and constraints from requirements".to_string(),
                    examples: vec![
                        ExampleData::new(
                            "The system shall process 100 transactions per second and maintain 99.9% uptime.".to_string(),
                            vec![
                                Extraction::new("specification".to_string(), "process 100 transactions per second".to_string()),
                                Extraction::new("constraint".to_string(), "maintain 99.9% uptime".to_string()),
                            ],
                        )
                    ],
                    prompt: "Extract detailed specifications, constraints, and performance requirements from this text.".to_string(),
                    output_field: "specifications".to_string(),
                    filter: Some(PipelineFilter {
                        class_filter: Some("requirement".to_string()),
                        text_pattern: None,
                        max_items: None,
                    }),
                    depends_on: vec!["extract_requirements".to_string()],
                },
            ],
        }
    }

    /// Save pipeline configuration to YAML file
    pub fn save_pipeline_to_file(config: &PipelineConfig, path: &std::path::Path) -> LangExtractResult<()> {
        let yaml_content = serde_yaml::to_string(config)
            .map_err(|e| LangExtractError::configuration(format!("Failed to serialize pipeline: {}", e)))?;

        std::fs::write(path, yaml_content)
            .map_err(|e| LangExtractError::configuration(format!("Failed to write pipeline file: {}", e)))?;

        Ok(())
    }

    /// Load pipeline configuration from YAML file
    pub fn load_pipeline_from_file(path: &std::path::Path) -> LangExtractResult<PipelineConfig> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| LangExtractError::configuration(format!("Failed to read pipeline file: {}", e)))?;

        serde_yaml::from_str(&content)
            .map_err(|e| LangExtractError::configuration(format!("Failed to parse pipeline YAML: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_config_serialization() {
        let config = utils::create_requirements_pipeline();
        let yaml = serde_yaml::to_string(&config).unwrap();
        let deserialized: PipelineConfig = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(config.name, deserialized.name);
        assert_eq!(config.steps.len(), deserialized.steps.len());
    }

    #[test]
    fn test_dependency_resolution() {
        let config = utils::create_requirements_pipeline();
        let executor = PipelineExecutor::new(config);

        let order = executor.resolve_execution_order().unwrap();

        // Should start with step that has no dependencies
        assert_eq!(order[0], "extract_requirements");
        // Should include all steps
        assert_eq!(order.len(), 3);
    }

    #[test]
    fn test_filter_application() {
        let executor = PipelineExecutor::new(utils::create_requirements_pipeline());

        let extractions = vec![
            Extraction::new("requirement".to_string(), "Test requirement".to_string()),
            Extraction::new("other".to_string(), "Other text".to_string()),
        ];

        let filter = PipelineFilter {
            class_filter: Some("requirement".to_string()),
            text_pattern: None,
            max_items: None,
        };

        let filtered = executor.apply_filter(&extractions, &Some(filter));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].extraction_class, "requirement");
    }
}
