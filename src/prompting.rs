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
        use crate::templates::TemplateBuilder;
        
        let include_reasoning = matches!(provider_type, ProviderType::Ollama | ProviderType::Custom);
        
        TemplateBuilder::new(format_type)
            .with_reasoning(include_reasoning)
            .build()
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
        use crate::templates::ExampleFormatter;
        
        let formatter = if let Some(max) = self.max_examples {
            ExampleFormatter::new(self.format_type).with_max_examples(max)
        } else {
            ExampleFormatter::new(self.format_type)
        };
        
        formatter.format_examples(examples)
    }

    // Note: format_example_as_json and format_example_as_yaml methods have been moved
    // to the templates::ExampleFormatter to eliminate duplication

    /// Substitute variables in template
    fn substitute_variables(&self, template: &str, context: &PromptContext) -> LangExtractResult<String> {
        use crate::templates::TemplateEngine;
        use std::collections::HashMap;
        
        let mut variables = HashMap::new();
        
        // Built-in variables
        variables.insert("task_description".to_string(), context.task_description.clone());
        variables.insert("input_text".to_string(), context.input_text.clone());
        
        // Additional context
        if let Some(context_text) = &context.additional_context {
            variables.insert("additional_context".to_string(), 
                format!("\n\nAdditional Context: {}\n", context_text));
        } else {
            variables.insert("additional_context".to_string(), String::new());
        }

        // Examples
        let examples_text = self.format_examples(&context.examples)?;
        variables.insert("examples".to_string(), examples_text);

        // Reasoning section
        if self.include_reasoning {
            variables.insert("reasoning".to_string(), 
                "\n\nPlease think through this step by step before providing your answer.".to_string());
        } else {
            variables.insert("reasoning".to_string(), String::new());
        }

        // Schema hint
        if let Some(hint) = &context.schema_hint {
            variables.insert("schema_hint".to_string(), 
                format!("\n\nSchema guidance: {}\n", hint));
        } else {
            variables.insert("schema_hint".to_string(), String::new());
        }

        // Custom variables
        for (key, value) in &context.variables {
            variables.insert(key.clone(), value.clone());
        }

        // Use lenient template engine to avoid issues with JSON/YAML examples
        let engine = TemplateEngine::lenient();
        engine.render(template, &variables)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Extraction;

    #[test]
    fn test_prompt_context_creation() {
        let context = PromptContext::new(
            "Extract names".to_string(),
            "John is here".to_string(),
        )
        .with_context("Additional info".to_string())
        .with_variable("custom".to_string(), "value".to_string())
        .with_schema_hint("Use proper format".to_string());

        assert_eq!(context.task_description, "Extract names");
        assert_eq!(context.input_text, "John is here");
        assert_eq!(context.additional_context, Some("Additional info".to_string()));
        assert_eq!(context.variables.get("custom"), Some(&"value".to_string()));
        assert_eq!(context.schema_hint, Some("Use proper format".to_string()));
    }

    #[test]
    fn test_template_validation() {
        let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
        assert!(template.validate().is_ok());

        let mut invalid_template = template.clone();
        invalid_template.base_template = "No required placeholders".to_string();
        assert!(invalid_template.validate().is_err());
    }

    #[test]
    fn test_required_variables() {
        let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
        let vars = template.required_variables();
        
        assert!(vars.contains(&"task_description".to_string()));
        assert!(vars.contains(&"input_text".to_string()));
        assert!(vars.contains(&"examples".to_string()));
    }

    #[test]
    fn test_example_formatting_json() {
        let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
        let example = ExampleData::new(
            "John is 30".to_string(),
            vec![
                Extraction::new("name".to_string(), "John".to_string()),
                Extraction::new("age".to_string(), "30".to_string()),
            ],
        );

        // Test is now handled by the templates::ExampleFormatter tests
        // Let's test the template rendering instead
        let context = PromptContext::new("Extract information".to_string(), "Test input".to_string())
            .with_examples(vec![example]);
        let rendered = template.render(&context).unwrap();
        assert!(rendered.contains("Extract information"));
        assert!(rendered.contains("Test input"));
    }

    #[test]
    fn test_template_rendering() {
        let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
        let context = PromptContext::new(
            "Extract names and ages".to_string(),
            "Alice is 25 years old".to_string(),
        );

        let rendered = template.render(&context).unwrap();
        
        assert!(rendered.contains("Extract names and ages"));
        assert!(rendered.contains("Alice is 25 years old"));
        assert!(rendered.contains("JSON format"));
    }

    #[test]
    fn test_provider_specific_templates() {
        let openai_template = PromptTemplate::for_provider(ProviderType::OpenAI, FormatType::Json);
        let ollama_template = PromptTemplate::for_provider(ProviderType::Ollama, FormatType::Json);

        assert!(openai_template.system_message.is_some());
        assert!(!openai_template.include_reasoning);
        
        assert!(ollama_template.include_reasoning);
        assert_eq!(ollama_template.max_examples, Some(3));
    }

    #[test]
    fn test_backward_compatibility() {
        let mut template = PromptTemplateStructured::new(Some("Extract info"));
        template.examples.push(ExampleData::new(
            "Test".to_string(),
            vec![Extraction::new("test".to_string(), "value".to_string())],
        ));

        let rendered = template.render("Input text", None).unwrap();
        assert!(rendered.contains("Extract info"));
        assert!(rendered.contains("Input text"));
    }
}
