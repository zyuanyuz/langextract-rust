//! Tests for the prompt template system

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

    let formatted = template.format_example_as_json(&example).unwrap();
    assert!(formatted.contains("\"name\": \"John\""));
    assert!(formatted.contains("\"age\": \"30\""));
}

#[test]
fn test_example_formatting_yaml() {
    let template = PromptTemplate::new(FormatType::Yaml, ProviderType::OpenAI);
    let example = ExampleData::new(
        "John is 30".to_string(),
        vec![
            Extraction::new("name".to_string(), "John".to_string()),
            Extraction::new("age".to_string(), "30".to_string()),
        ],
    );

    let formatted = template.format_example_as_yaml(&example).unwrap();
    assert!(formatted.contains("name: John"));
    assert!(formatted.contains("age: '30'"));
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
fn test_template_rendering_with_examples() {
    let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
    let mut context = PromptContext::new(
        "Extract names and ages".to_string(),
        "Alice is 25 years old".to_string(),
    );
    
    context.examples = vec![
        ExampleData::new(
            "Bob is 30".to_string(),
            vec![
                Extraction::new("name".to_string(), "Bob".to_string()),
                Extraction::new("age".to_string(), "30".to_string()),
            ],
        )
    ];

    let rendered = template.render(&context).unwrap();
    
    assert!(rendered.contains("Extract names and ages"));
    assert!(rendered.contains("Alice is 25 years old"));
    assert!(rendered.contains("Examples:"));
    assert!(rendered.contains("Bob is 30"));
    assert!(rendered.contains("\"name\": \"Bob\""));
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
fn test_template_customization() {
    let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI)
        .with_max_examples(2)
        .with_reasoning(true)
        .with_system_message("Custom system message".to_string());

    assert_eq!(template.max_examples, Some(2));
    assert!(template.include_reasoning);
    assert_eq!(template.system_message, Some("Custom system message".to_string()));
}

#[test]
fn test_example_limiting() {
    let mut template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
    template.max_examples = Some(2);
    
    let examples = vec![
        ExampleData::new("Test 1".to_string(), vec![]),
        ExampleData::new("Test 2".to_string(), vec![]),
        ExampleData::new("Test 3".to_string(), vec![]),
        ExampleData::new("Test 4".to_string(), vec![]),
    ];

    let formatted = template.format_examples(&examples).unwrap();
    
    // Should only include first 2 examples
    assert!(formatted.contains("Test 1"));
    assert!(formatted.contains("Test 2"));
    assert!(!formatted.contains("Test 3"));
    assert!(!formatted.contains("Test 4"));
}

#[test]
fn test_variable_substitution() {
    let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
    let context = PromptContext::new(
        "Extract info".to_string(),
        "Input text".to_string(),
    )
    .with_context("Additional context".to_string())
    .with_variable("custom_var".to_string(), "custom_value".to_string());

    // Create a template with custom variable
    let mut custom_template = template.clone();
    custom_template.base_template = "{task_description} {custom_var} {input_text}".to_string();

    let rendered = custom_template.render(&context).unwrap();
    assert!(rendered.contains("Extract info"));
    assert!(rendered.contains("custom_value"));
    assert!(rendered.contains("Input text"));
}

#[test]
fn test_missing_variable_error() {
    let mut template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
    template.base_template = "{task_description} {missing_var} {input_text}".to_string();
    
    let context = PromptContext::new(
        "Extract info".to_string(),
        "Input text".to_string(),
    );

    let result = template.render(&context);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("missing_var"));
}

#[test]
fn test_backward_compatibility() {
    let mut template = PromptTemplateStructured::new(Some("Extract info"));
    template.examples.push(ExampleData::new(
        "Test".to_string(),
        vec![Extraction::new("test".to_string(), "value".to_string())],
    ));

    let rendered = template.render("Input text", Some("Additional context")).unwrap();
    assert!(rendered.contains("Extract info"));
    assert!(rendered.contains("Input text"));
    assert!(rendered.contains("Additional Context: Additional context"));
}

#[test]
fn test_format_specific_templates() {
    let json_template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
    let yaml_template = PromptTemplate::new(FormatType::Yaml, ProviderType::OpenAI);

    let context = PromptContext::new("Extract".to_string(), "Test".to_string());

    let json_rendered = json_template.render(&context).unwrap();
    let yaml_rendered = yaml_template.render(&context).unwrap();

    assert!(json_rendered.contains("JSON format"));
    assert!(yaml_rendered.contains("YAML format"));
}

#[test]
fn test_reasoning_instructions() {
    let mut template = PromptTemplate::new(FormatType::Json, ProviderType::Ollama);
    template.include_reasoning = true;

    let context = PromptContext::new("Extract".to_string(), "Test".to_string());
    let rendered = template.render(&context).unwrap();

    assert!(rendered.contains("Think step by step"));
    assert!(rendered.contains("think through this step by step"));
}

#[test]
fn test_schema_hint_integration() {
    let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
    let context = PromptContext::new("Extract".to_string(), "Test".to_string())
        .with_schema_hint("Follow this schema format".to_string());

    let rendered = template.render(&context).unwrap();
    assert!(rendered.contains("Schema guidance: Follow this schema format"));
}

#[test]
fn test_template_with_format_and_provider() {
    let template = PromptTemplateStructured::with_format_and_provider(
        Some("Extract names"),
        FormatType::Yaml,
        ProviderType::Ollama,
    );

    assert_eq!(template.template().format_type, FormatType::Yaml);
    assert_eq!(template.template().provider_type, ProviderType::Ollama);
    assert!(template.template().include_reasoning);
}
