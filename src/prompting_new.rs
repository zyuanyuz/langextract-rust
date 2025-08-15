//! Advanced prompt template system with dynamic variables and provider adaptation.

use crate::{
    data::{ExampleData, FormatType},
    exceptions::{LangExtractError, LangExtractResult},
    providers::ProviderType,
};
use std::collections::HashMap;

/// Error types for template operations
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("Missing required variable: {variable}")]
    MissingVariable { variable: String },
    #[error("Invalid template syntax: {message}")]
    InvalidSyntax { message: String },
    #[error("Example formatting error: {message}")]
    ExampleError { message: String },
}

impl From<TemplateError> for LangExtractError {
    fn from(err: TemplateError) -> Self {
        LangExtractError::InvalidInput(err.to_string())
    }
}

/// Context for rendering prompts
#[derive(Debug, Clone)]
pub struct PromptContext {
    /// Task description for what to extract
    pub task_description: String,
    /// Example data to guide extraction
    pub examples: Vec<ExampleData>,
    /// Input text to process
    pub input_text: String,
    /// Additional context information
    pub additional_context: Option<String>,
    /// Schema hint for structured output
    pub schema_hint: Option<String>,
    /// Custom variables for template substitution
    pub variables: HashMap<String, String>,
}

impl PromptContext {
    /// Create a new prompt context
    pub fn new(task_description: String, input_text: String) -> Self {
        Self {
            task_description,
            input_text,
            examples: Vec::new(),
            additional_context: None,
            schema_hint: None,
            variables: HashMap::new(),
        }
    }

    /// Add examples to the context
    pub fn with_examples(mut self, examples: Vec<ExampleData>) -> Self {
        self.examples = examples;
        self
    }

    /// Add additional context
    pub fn with_context(mut self, context: String) -> Self {
        self.additional_context = Some(context);
        self
    }

    /// Add a custom variable
    pub fn with_variable(mut self, key: String, value: String) -> Self {
        self.variables.insert(key, value);
        self
    }

    /// Add schema hint
    pub fn with_schema_hint(mut self, hint: String) -> Self {
        self.schema_hint = Some(hint);
        self
    }
}

/// Trait for rendering prompt templates
pub trait TemplateRenderer {
    /// Render the template with the given context
    fn render(&self, context: &PromptContext) -> LangExtractResult<String>;
    
    /// Validate the template structure
    fn validate(&self) -> LangExtractResult<()>;
    
    /// Get required variables for this template
    fn required_variables(&self) -> Vec<String>;
}

/// Advanced prompt template with dynamic variables and provider adaptation
#[derive(Debug, Clone)]
pub struct PromptTemplate {
    /// Base template string with variable placeholders
    pub base_template: String,
    /// System message for providers that support it
    pub system_message: Option<String>,
    /// Template for formatting examples
    pub example_template: String,
    /// Output format type
    pub format_type: FormatType,
    /// Target provider type for optimization
    pub provider_type: ProviderType,
    /// Maximum number of examples to include
    pub max_examples: Option<usize>,
    /// Whether to include reasoning instructions
    pub include_reasoning: bool,
}

impl PromptTemplate {
    /// Create a new prompt template
    pub fn new(format_type: FormatType, provider_type: ProviderType) -> Self {
        let base_template = Self::default_base_template(format_type, provider_type);
        let example_template = Self::default_example_template(format_type);
        
        Self {
            base_template,
            system_message: None,
            example_template,
            format_type,
            provider_type,
            max_examples: Some(5),
            include_reasoning: false,
        }
    }

    /// Create template optimized for specific provider
    pub fn for_provider(provider_type: ProviderType, format_type: FormatType) -> Self {
        let mut template = Self::new(format_type, provider_type);
        
        match provider_type {
            ProviderType::OpenAI => {
                template.system_message = Some(
                    "You are an expert information extraction assistant. Extract structured information exactly as shown in the examples.".to_string()
                );
                template.include_reasoning = false; // OpenAI is good with direct instructions
            }
            ProviderType::Ollama => {
                template.include_reasoning = true; // Local models benefit from reasoning steps
                template.max_examples = Some(3); // Keep prompts shorter for local models
            }
            ProviderType::Custom => {
                // Conservative defaults for unknown providers
                template.max_examples = Some(3);
                template.include_reasoning = true;
            }
        }
        
        template
    }

    /// Set maximum number of examples
    pub fn with_max_examples(mut self, max: usize) -> Self {
        self.max_examples = Some(max);
        self
    }

    /// Set system message
    pub fn with_system_message(mut self, message: String) -> Self {
        self.system_message = Some(message);
        self
    }

    /// Enable or disable reasoning instructions
    pub fn with_reasoning(mut self, enable: bool) -> Self {
        self.include_reasoning = enable;
        self
    }

    /// Set custom base template
    pub fn with_base_template(mut self, template: String) -> Self {
        self.base_template = template;
        self
    }

    /// Default base template for different formats and providers
    fn default_base_template(format_type: FormatType, provider_type: ProviderType) -> String {
        let format_name = match format_type {
            FormatType::Json => "JSON",
            FormatType::Yaml => "YAML",
        };

        let reasoning_instruction = match provider_type {
            ProviderType::Ollama | ProviderType::Custom => 
                "\n\nThink step by step:\n1. Read the text carefully\n2. Identify the requested information\n3. Extract it in the exact format shown in examples\n",
            ProviderType::OpenAI => "",
        };

        format!(
            "{{task_description}}{{additional_context}}{{examples}}{{reasoning}}\nNow extract information from this text:\n\nInput: {{input_text}}\n\nOutput ({format_name} format):",
            reasoning = reasoning_instruction
        )
    }

    /// Default example template for different formats
    fn default_example_template(format_type: FormatType) -> String {
        match format_type {
            FormatType::Json => {
                "Input: {input}\nOutput: {output_json}\n".to_string()
            }
            FormatType::Yaml => {
                "Input: {input}\nOutput:\n{output_yaml}\n".to_string()
            }
        }
    }

    /// Format examples according to the template
    fn format_examples(&self, examples: &[ExampleData]) -> LangExtractResult<String> {
        if examples.is_empty() {
            return Ok(String::new());
        }

        let mut formatted = String::from("\n\nExamples:\n\n");
        
        // Limit examples if max_examples is set
        let examples_to_use = if let Some(max) = self.max_examples {
            &examples[..examples.len().min(max)]
        } else {
            examples
        };

        for (i, example) in examples_to_use.iter().enumerate() {
            formatted.push_str(&format!("Example {}:\n", i + 1));
            
            let output_formatted = match self.format_type {
                FormatType::Json => self.format_example_as_json(example)?,
                FormatType::Yaml => self.format_example_as_yaml(example)?,
            };

            let example_text = self.example_template
                .replace("{input}", &example.text)
                .replace("{output_json}", &output_formatted)
                .replace("{output_yaml}", &output_formatted);

            formatted.push_str(&example_text);
            formatted.push('\n');
        }

        Ok(formatted)
    }

    /// Format example as JSON
    fn format_example_as_json(&self, example: &ExampleData) -> LangExtractResult<String> {
        let mut json_obj = serde_json::Map::new();
        
        for extraction in &example.extractions {
            json_obj.insert(
                extraction.extraction_class.clone(),
                serde_json::Value::String(extraction.extraction_text.clone()),
            );
        }

        let json_value = serde_json::Value::Object(json_obj);
        serde_json::to_string_pretty(&json_value)
            .map_err(|e| TemplateError::ExampleError { 
                message: format!("Failed to format JSON: {}", e) 
            }.into())
    }

    /// Format example as YAML
    fn format_example_as_yaml(&self, example: &ExampleData) -> LangExtractResult<String> {
        let mut yaml_map = std::collections::BTreeMap::new();
        
        for extraction in &example.extractions {
            yaml_map.insert(
                extraction.extraction_class.clone(),
                extraction.extraction_text.clone(),
            );
        }

        serde_yaml::to_string(&yaml_map)
            .map_err(|e| TemplateError::ExampleError { 
                message: format!("Failed to format YAML: {}", e) 
            }.into())
    }

    /// Substitute variables in template
    fn substitute_variables(&self, template: &str, context: &PromptContext) -> LangExtractResult<String> {
        let mut result = template.to_string();
        
        // Built-in variables
        result = result.replace("{task_description}", &context.task_description);
        result = result.replace("{input_text}", &context.input_text);
        
        // Additional context
        if let Some(context_text) = &context.additional_context {
            result = result.replace("{additional_context}", &format!("\n\nAdditional Context: {}\n", context_text));
        } else {
            result = result.replace("{additional_context}", "");
        }

        // Examples
        let examples_text = self.format_examples(&context.examples)?;
        result = result.replace("{examples}", &examples_text);

        // Reasoning section
        if self.include_reasoning {
            result = result.replace("{reasoning}", "\n\nPlease think through this step by step before providing your answer.");
        } else {
            result = result.replace("{reasoning}", "");
        }

        // Schema hint
        if let Some(hint) = &context.schema_hint {
            result = result.replace("{schema_hint}", &format!("\n\nSchema guidance: {}\n", hint));
        } else {
            result = result.replace("{schema_hint}", "");
        }

        // Custom variables
        for (key, value) in &context.variables {
            result = result.replace(&format!("{{{}}}", key), value);
        }

        // Check for any remaining unsubstituted variables
        if result.contains('{') && result.contains('}') {
            let start = result.find('{').unwrap();
            let end = result[start..].find('}').unwrap() + start + 1;
            let var_name = &result[start+1..end-1];
            return Err(TemplateError::MissingVariable { 
                variable: var_name.to_string() 
            }.into());
        }

        Ok(result)
    }
}

impl TemplateRenderer for PromptTemplate {
    fn render(&self, context: &PromptContext) -> LangExtractResult<String> {
        self.substitute_variables(&self.base_template, context)
    }

    fn validate(&self) -> LangExtractResult<()> {
        // Check if base template has required placeholders
        if !self.base_template.contains("{task_description}") {
            return Err(TemplateError::InvalidSyntax { 
                message: "Base template must contain {task_description} placeholder".to_string() 
            }.into());
        }
        
        if !self.base_template.contains("{input_text}") {
            return Err(TemplateError::InvalidSyntax { 
                message: "Base template must contain {input_text} placeholder".to_string() 
            }.into());
        }

        Ok(())
    }

    fn required_variables(&self) -> Vec<String> {
        let mut vars = vec!["task_description".to_string(), "input_text".to_string()];
        
        // Extract custom variables from template
        let mut i = 0;
        while i < self.base_template.len() {
            if let Some(start) = self.base_template[i..].find('{') {
                let start = start + i;
                if let Some(end) = self.base_template[start..].find('}') {
                    let end = end + start;
                    let var_name = &self.base_template[start+1..end];
                    if !var_name.is_empty() && !vars.contains(&var_name.to_string()) {
                        vars.push(var_name.to_string());
                    }
                    i = end + 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        vars
    }
}

/// Backward compatibility - simplified prompt template
#[derive(Debug, Clone)]
pub struct PromptTemplateStructured {
    /// Description of what to extract
    pub description: Option<String>,
    /// Example data for guidance
    pub examples: Vec<ExampleData>,
    /// Advanced template for rendering
    template: PromptTemplate,
}

impl PromptTemplateStructured {
    /// Create a new structured prompt template
    pub fn new(description: Option<&str>) -> Self {
        Self {
            description: description.map(|s| s.to_string()),
            examples: Vec::new(),
            template: PromptTemplate::new(FormatType::Json, ProviderType::Ollama),
        }
    }

    /// Create with specific format and provider
    pub fn with_format_and_provider(
        description: Option<&str>,
        format_type: FormatType,
        provider_type: ProviderType,
    ) -> Self {
        Self {
            description: description.map(|s| s.to_string()),
            examples: Vec::new(),
            template: PromptTemplate::for_provider(provider_type, format_type),
        }
    }

    /// Render the prompt for given text
    pub fn render(&self, input_text: &str, additional_context: Option<&str>) -> LangExtractResult<String> {
        let mut context = PromptContext::new(
            self.description.clone().unwrap_or_default(),
            input_text.to_string(),
        );
        
        context.examples = self.examples.clone();
        
        if let Some(ctx) = additional_context {
            context.additional_context = Some(ctx.to_string());
        }

        self.template.render(&context)
    }

    /// Get the underlying template for advanced customization
    pub fn template(&self) -> &PromptTemplate {
        &self.template
    }

    /// Get mutable reference to the underlying template
    pub fn template_mut(&mut self) -> &mut PromptTemplate {
        &mut self.template
    }
}
