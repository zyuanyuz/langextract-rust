//! Comprehensive demonstration of the advanced prompt template system
//!
//! This example shows all the capabilities of the new prompt template system:
//! - Provider-specific optimization
//! - Dynamic variable substitution  
//! - Format-specific templates (JSON vs YAML)
//! - Example management and formatting
//! - Template customization

use langextract::{
    data::{ExampleData, Extraction, FormatType},
    prompting::{PromptContext, PromptTemplate, TemplateRenderer},
    providers::ProviderType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Prompt Template System Demonstration\n");

    // Create some example data
    let examples = vec![
        ExampleData::new(
            "John Smith, 30, works as a software engineer at Google".to_string(),
            vec![
                Extraction::new("person_name".to_string(), "John Smith".to_string()),
                Extraction::new("age".to_string(), "30".to_string()),
                Extraction::new("job_title".to_string(), "software engineer".to_string()),
                Extraction::new("company".to_string(), "Google".to_string()),
            ],
        ),
        ExampleData::new(
            "Dr. Sarah Wilson is 45 and practices medicine at Stanford Hospital".to_string(),
            vec![
                Extraction::new("person_name".to_string(), "Dr. Sarah Wilson".to_string()),
                Extraction::new("age".to_string(), "45".to_string()),
                Extraction::new("job_title".to_string(), "doctor".to_string()),
                Extraction::new("company".to_string(), "Stanford Hospital".to_string()),
            ],
        ),
    ];

    let input_text = "Alice Johnson, 28, is a data scientist at Microsoft in Seattle";

    // Demo 1: Provider-Specific Templates
    println!("📋 1. Provider-Specific Template Optimization\n");

    for provider in [ProviderType::OpenAI, ProviderType::Ollama, ProviderType::Custom] {
        println!("🔧 {} Provider Template:", provider);
        
        let template = PromptTemplate::for_provider(provider, FormatType::Json);
        
        println!("   - System Message: {}", 
                template.system_message.as_ref().map_or("None", |s| s));
        println!("   - Include Reasoning: {}", template.include_reasoning);
        println!("   - Max Examples: {:?}", template.max_examples);
        
        let context = PromptContext::new(
            "Extract person information".to_string(),
            input_text.to_string(),
        ).with_examples(examples.clone());

        let rendered = template.render(&context)?;
        println!("   - Prompt Length: {} characters", rendered.len());
        
        if template.include_reasoning {
            println!("   - Includes reasoning steps: ✓");
        }
        
        println!();
    }

    // Demo 2: Format-Specific Templates
    println!("📋 2. Format-Specific Template Rendering\n");

    for format in [FormatType::Json, FormatType::Yaml] {
        println!("📄 {} Format:", format);
        
        let template = PromptTemplate::new(format, ProviderType::OpenAI);
        let context = PromptContext::new(
            "Extract information".to_string(),
            input_text.to_string(),
        ).with_examples(examples[..1].to_vec()); // Use just first example

        let rendered = template.render(&context)?;
        
        // Show how examples are formatted
        let example_section = rendered
            .lines()
            .skip_while(|line| !line.contains("Examples:"))
            .take(10)
            .collect::<Vec<_>>()
            .join("\n");
            
        println!("   Example formatting:");
        for line in example_section.lines().take(8) {
            println!("   {}", line);
        }
        println!("   ...\n");
    }

    // Demo 3: Dynamic Variable Substitution
    println!("📋 3. Dynamic Variable Substitution\n");

    let mut custom_template = PromptTemplate::new(FormatType::Json, ProviderType::Custom);
    custom_template.base_template = "Task: {task_description}\nDomain: {domain}\nDifficulty: {difficulty}\n\n{examples}\n\nInput: {input_text}\nOutput:".to_string();

    let context = PromptContext::new(
        "Extract professional information".to_string(),
        input_text.to_string(),
    )
    .with_examples(examples.clone())
    .with_variable("domain".to_string(), "Professional Data Extraction".to_string())
    .with_variable("difficulty".to_string(), "Intermediate".to_string());

    let rendered = custom_template.render(&context)?;
    
    println!("Custom template with variables:");
    for (i, line) in rendered.lines().take(10).enumerate() {
        println!("   {}: {}", i + 1, line);
    }
    println!("   ...\n");

    // Demo 4: Template Customization and Validation
    println!("📋 4. Template Customization and Validation\n");

    let template = PromptTemplate::new(FormatType::Json, ProviderType::Ollama)
        .with_max_examples(1)
        .with_reasoning(false)
        .with_system_message("You are a data extraction specialist.".to_string());

    println!("Customized template settings:");
    println!("   - Max Examples: {:?}", template.max_examples);
    println!("   - Reasoning Enabled: {}", template.include_reasoning);
    println!("   - System Message: {:?}", template.system_message);

    // Validate template
    match template.validate() {
        Ok(()) => println!("   - Template Validation: ✓ Passed"),
        Err(e) => println!("   - Template Validation: ✗ Failed - {}", e),
    }

    // Show required variables
    let required_vars = template.required_variables();
    println!("   - Required Variables: {:?}", required_vars);

    println!();

    // Demo 5: Context Features
    println!("📋 5. Advanced Context Features\n");

    let context = PromptContext::new(
        "Extract professional and personal information".to_string(),
        input_text.to_string(),
    )
    .with_examples(examples)
    .with_context("Focus on technology companies and roles".to_string())
    .with_schema_hint("Use consistent field names across all extractions".to_string())
    .with_variable("extraction_mode".to_string(), "comprehensive".to_string());

    let template = PromptTemplate::new(FormatType::Json, ProviderType::OpenAI);
    let rendered = template.render(&context)?;

    println!("Context features demonstrated:");
    println!("   - Task Description: ✓");
    println!("   - Examples: ✓ ({} examples)", context.examples.len());
    println!("   - Additional Context: ✓");
    println!("   - Schema Hint: ✓");
    println!("   - Custom Variables: ✓");
    
    println!("\nFinal rendered prompt preview:");
    for (i, line) in rendered.lines().take(15).enumerate() {
        println!("   {}: {}", i + 1, line);
    }
    println!("   ... (total {} lines)\n", rendered.lines().count());

    // Demo 6: Template Performance Comparison
    println!("📋 6. Template Performance Comparison\n");

    let providers = [
        ("OpenAI (optimized)", ProviderType::OpenAI),
        ("Ollama (local)", ProviderType::Ollama),
        ("Custom (generic)", ProviderType::Custom),
    ];

    for (name, provider) in providers {
        let template = PromptTemplate::for_provider(provider, FormatType::Json);
        let context = PromptContext::new(
            "Extract information".to_string(),
            "Test input with multiple people and details".to_string(),
        ).with_examples(context.examples.clone());

        let rendered = template.render(&context)?;
        
        println!("   {}: {} chars, {} lines", 
                name, 
                rendered.len(), 
                rendered.lines().count());
    }

    println!("\n🎉 Prompt Template System Demo Complete!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Provider-specific optimization");
    println!("   ✓ Format-specific example rendering");
    println!("   ✓ Dynamic variable substitution");
    println!("   ✓ Template validation and customization");
    println!("   ✓ Rich context support");
    println!("   ✓ Performance optimization");

    Ok(())
}
