//! Pipeline Processing Demo
//!
//! This example demonstrates the multi-step pipeline processing system in LangExtract.
//! It shows how to create complex extraction workflows with dependent steps that build
//! upon each other to create hierarchical structured data from unstructured text.

use langextract_rust::{
    data::{ExampleData, Extraction, FormatType},
    pipeline::{PipelineConfig, PipelineStep, PipelineFilter, PipelineExecutor},
    providers::ProviderConfig,
    ExtractConfig,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenvy::dotenv().ok();

    println!("🔬 LangExtract Pipeline Processing Demo");
    println!("======================================\n");

    // Sample text for processing - a complex document with multiple types of information
    let sample_text = r#"
TECHNICAL REQUIREMENTS DOCUMENT - PROJECT APOLLO

System Performance Requirements:
- The system shall process at least 1,000 transactions per second with 99.9% uptime
- Response time shall not exceed 200 milliseconds for 95% of requests
- The database shall support concurrent access by up to 500 users
- Memory usage shall not exceed 8GB during peak operations

Security Requirements:
- All data transmissions shall use AES-256 encryption
- User authentication shall require multi-factor authentication (MFA)
- Access logs shall be retained for a minimum of 2 years
- Password complexity shall require at least 12 characters with mixed case

Team Information:
- Project Manager: Sarah Chen (s.chen@company.com, ext. 1234)
- Lead Developer: Michael Rodriguez (m.rodriguez@company.com, ext. 5678)
- Security Analyst: Dr. Lisa Zhang (l.zhang@company.com, ext. 9012)
- QA Engineer: David Kim (d.kim@company.com, ext. 3456)

Project Timeline:
- Phase 1 (Requirements): January 1 - February 15, 2024
- Phase 2 (Development): February 16 - August 31, 2024
- Phase 3 (Testing): September 1 - October 31, 2024
- Phase 4 (Deployment): November 1 - December 15, 2024

Budget Allocation:
- Development: $2.5 million (60% of total budget)
- Testing & QA: $800,000 (20% of total budget)
- Infrastructure: $500,000 (12% of total budget)
- Contingency: $333,000 (8% of total budget)
Total Project Budget: $4.133 million
"#.trim();

    println!("📄 Sample Document:");
    println!("{}", &sample_text[..400]);
    println!("... (document continues for {} total characters)\n", sample_text.len());

    // Demo 1: Create and execute a pipeline programmatically
    println!("🔧 Demo 1: Programmatic Pipeline Creation");
    println!("==========================================");
    
    let pipeline_config = create_sample_pipeline().await?;
    println!("✅ Created pipeline: {}", pipeline_config.name);
    println!("   Description: {}", pipeline_config.description);
    println!("   Steps: {}", pipeline_config.steps.len());
    
    for (i, step) in pipeline_config.steps.iter().enumerate() {
        println!("   {}. {} (depends on: {:?})", i + 1, step.name, step.depends_on);
    }
    println!();

    // Execute the pipeline
    println!("🚀 Executing pipeline...");
    let executor = PipelineExecutor::new(pipeline_config);
    
    match executor.execute(sample_text).await {
        Ok(result) => {
            println!("✅ Pipeline execution completed successfully!");
            println!("   Total processing time: {}ms", result.total_time_ms);
            println!("   Steps executed: {}", result.step_results.len());
            println!();

            // Display results for each step
            for step_result in &result.step_results {
                let status = if step_result.success { "✅" } else { "❌" };
                println!("   {} Step: {}", status, step_result.step_name);
                println!("      Extractions: {}", step_result.extractions.len());
                println!("      Processing time: {}ms", step_result.processing_time_ms);
                
                // Show sample extractions
                for extraction in step_result.extractions.iter().take(3) {
                    println!("        - [{}] {}", 
                        extraction.extraction_class, 
                        &extraction.extraction_text[..std::cmp::min(50, extraction.extraction_text.len())]
                    );
                }
                if step_result.extractions.len() > 3 {
                    println!("        ... and {} more", step_result.extractions.len() - 3);
                }
                println!();
            }

            // Display nested output structure
            println!("📊 Nested Output Structure:");
            println!("{}", serde_json::to_string_pretty(&result.nested_output)?);
        }
        Err(e) => {
            println!("❌ Pipeline execution failed: {}", e);
            println!("   This is expected if no LLM provider is configured");
        }
    }

    println!("\n{}", "=".repeat(60));

    // Demo 2: Load pipeline from YAML file
    println!("\n🔧 Demo 2: YAML Pipeline Configuration");
    println!("======================================");
    
    // Create a sample YAML pipeline file
    create_sample_yaml_pipeline().await?;
    println!("✅ Created sample pipeline YAML file: sample_pipeline.yaml");
    
    // Try to load and execute it
    match PipelineExecutor::from_yaml_file(std::path::Path::new("sample_pipeline.yaml")) {
        Ok(yaml_executor) => {
            println!("✅ Successfully loaded pipeline from YAML");
            println!("   Pipeline loaded successfully from YAML file");
            
            // Execute with a simpler text for demo
            let simple_text = "The system shall process 100 requests per second. Contact John Doe at john@example.com for questions.";
            println!("\n🚀 Executing YAML pipeline with simple text...");
            
            match yaml_executor.execute(simple_text).await {
                Ok(yaml_result) => {
                    println!("✅ YAML pipeline completed in {}ms", yaml_result.total_time_ms);
                    
                    for step_result in &yaml_result.step_results {
                        println!("   Step '{}': {} extractions", 
                            step_result.step_name, 
                            step_result.extractions.len()
                        );
                    }
                }
                Err(e) => {
                    println!("⚠️  YAML pipeline execution failed: {}", e);
                    println!("   This is expected without a configured LLM provider");
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to load YAML pipeline: {}", e);
        }
    }

    println!("\n{}", "=".repeat(60));

    // Demo 3: Advanced pipeline features
    println!("\n🔧 Demo 3: Advanced Pipeline Features");
    println!("=====================================");
    
    demonstrate_pipeline_features().await?;

    println!("\n🎯 Pipeline Demo Summary:");
    println!("=========================");
    println!("✅ Multi-step extraction with dependencies");
    println!("✅ Hierarchical data structure creation");
    println!("✅ YAML configuration support");
    println!("✅ Filtering and conditional processing");
    println!("✅ Performance tracking and error handling");
    println!("✅ Nested output generation");
    println!();
    println!("💡 Use Cases:");
    println!("   • Requirements analysis and decomposition");
    println!("   • Document structure extraction");
    println!("   • Multi-level data classification");
    println!("   • Complex information hierarchies");
    println!("   • Dependent extraction workflows");

    Ok(())
}

/// Create a sample pipeline configuration programmatically
async fn create_sample_pipeline() -> Result<PipelineConfig, Box<dyn std::error::Error>> {
    // Create global configuration
    let global_config = ExtractConfig {
        model_id: "mistral".to_string(),
        api_key: None,
        format_type: FormatType::Json,
        max_char_buffer: 4000,
        temperature: 0.2,
        fence_output: Some(true),
        use_schema_constraints: false,
        batch_length: 2,
        max_workers: 2,
        additional_context: None,
        resolver_params: HashMap::new(),
        language_model_params: {
            let mut params = HashMap::new();
            let provider_config = ProviderConfig::ollama("mistral", Some("http://localhost:11434".to_string()));
            params.insert("provider_config".to_string(), serde_json::to_value(&provider_config)?);
            params
        },
        debug: true,
        model_url: Some("http://localhost:11434".to_string()),
        extraction_passes: 1,
        enable_multipass: false,
        multipass_min_extractions: 1,
        multipass_quality_threshold: 0.3,
        progress_handler: None,
    };

    // Step 1: Extract requirements
    let step1 = PipelineStep {
        id: "extract_requirements".to_string(),
        name: "Extract Requirements".to_string(),
        description: "Extract all technical requirements and specifications".to_string(),
        prompt: "Extract all technical requirements, 'shall' statements, and specifications from the document.".to_string(),
        output_field: "requirements".to_string(),
        depends_on: vec![],
        filter: None,
        examples: vec![
            ExampleData::new(
                "The system shall process 100 requests per second with 99% uptime.".to_string(),
                vec![
                    Extraction::new("requirement".to_string(), "The system shall process 100 requests per second with 99% uptime.".to_string()),
                ]
            )
        ],
    };

    // Step 2: Extract team information
    let step2 = PipelineStep {
        id: "extract_team".to_string(),
        name: "Extract Team Information".to_string(),
        description: "Extract team member names, roles, and contact information".to_string(),
        prompt: "Extract team member names, their roles, email addresses, and contact information.".to_string(),
        output_field: "team_members".to_string(),
        depends_on: vec![],
        filter: None,
        examples: vec![
            ExampleData::new(
                "Project Manager: John Smith (j.smith@company.com, ext. 1234)".to_string(),
                vec![
                    Extraction::new("person".to_string(), "John Smith".to_string()),
                    Extraction::new("role".to_string(), "Project Manager".to_string()),
                    Extraction::new("email".to_string(), "j.smith@company.com".to_string()),
                    Extraction::new("phone".to_string(), "ext. 1234".to_string()),
                ]
            )
        ],
    };

    // Step 3: Extract numeric values from requirements
    let step3 = PipelineStep {
        id: "extract_values".to_string(),
        name: "Extract Numeric Values".to_string(),
        description: "Extract numeric values and units from requirements".to_string(),
        prompt: "From these requirements, extract all numeric values, measurements, and their units.".to_string(),
        output_field: "values".to_string(),
        depends_on: vec!["extract_requirements".to_string()],
        filter: Some(PipelineFilter {
            class_filter: Some("requirement".to_string()),
            text_pattern: None,
            max_items: None,
        }),
        examples: vec![
            ExampleData::new(
                "The system shall process 100 requests per second with 99% uptime.".to_string(),
                vec![
                    Extraction::new("value".to_string(), "100".to_string()),
                    Extraction::new("unit".to_string(), "requests per second".to_string()),
                    Extraction::new("value".to_string(), "99".to_string()),
                    Extraction::new("unit".to_string(), "%".to_string()),
                ]
            )
        ],
    };

    // Step 4: Extract budget information
    let step4 = PipelineStep {
        id: "extract_budget".to_string(),
        name: "Extract Budget Information".to_string(),
        description: "Extract budget amounts and allocations".to_string(),
        prompt: "Extract budget amounts, cost allocations, and financial information.".to_string(),
        output_field: "budget".to_string(),
        depends_on: vec![],
        filter: None,
        examples: vec![
            ExampleData::new(
                "Development: $2.5 million (60% of total budget)".to_string(),
                vec![
                    Extraction::new("category".to_string(), "Development".to_string()),
                    Extraction::new("amount".to_string(), "$2.5 million".to_string()),
                    Extraction::new("percentage".to_string(), "60%".to_string()),
                ]
            )
        ],
    };

    Ok(PipelineConfig {
        name: "Technical Document Analysis Pipeline".to_string(),
        description: "Multi-step extraction of requirements, team info, values, and budget from technical documents".to_string(),
        version: "1.0.0".to_string(),
        steps: vec![step1, step2, step3, step4],
        global_config,
    })
}

/// Create a sample YAML pipeline file
async fn create_sample_yaml_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    let yaml_content = r#"name: "Simple Requirements Pipeline"
description: "Extract requirements and contact information"
version: "1.0.0"

global_config:
  model_id: "mistral"
  api_key: null
  format_type: "json"
  max_char_buffer: 2000
  temperature: 0.2
  fence_output: true
  use_schema_constraints: false
  batch_length: 1
  max_workers: 1
  additional_context: null
  resolver_params: {}
  language_model_params:
    provider_config:
      provider_type: "ollama"
      base_url: "http://localhost:11434"
      model: "mistral"
      api_key: null
      headers: {}
      extra_params: {}
  debug: true
  model_url: "http://localhost:11434"
  extraction_passes: 1
  enable_multipass: false
  multipass_min_extractions: 1
  multipass_quality_threshold: 0.3

steps:
  - id: "extract_requirements"
    name: "Extract Requirements"
    description: "Extract all requirements and specifications"
    prompt: "Extract all requirements, 'shall' statements, and specifications from the text."
    output_field: "requirements"
    depends_on: []
    examples:
      - text: "The system shall process 100 requests per second."
        extractions:
          - extraction_class: "requirement"
            extraction_text: "The system shall process 100 requests per second."

  - id: "extract_contacts"
    name: "Extract Contact Information"
    description: "Extract names and contact details"
    prompt: "Extract person names, email addresses, and contact information."
    output_field: "contacts"
    depends_on: []
    examples:
      - text: "Contact John Doe at john@example.com for questions."
        extractions:
          - extraction_class: "person"
            extraction_text: "John Doe"
          - extraction_class: "email"
            extraction_text: "john@example.com"
"#;

    std::fs::write("sample_pipeline.yaml", yaml_content)?;
    Ok(())
}

/// Demonstrate advanced pipeline features
async fn demonstrate_pipeline_features() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Pipeline Features Overview:");
    println!();
    
    println!("1. 📋 Multi-Step Processing:");
    println!("   • Steps can depend on outputs from previous steps");
    println!("   • Dependency resolution ensures correct execution order");
    println!("   • Each step can have its own examples and prompts");
    println!();
    
    println!("2. 🎯 Filtering and Conditional Processing:");
    println!("   • Filter by extraction class (e.g., only process 'requirement' extractions)");
    println!("   • Filter by regex patterns on extraction text");
    println!("   • Limit number of items processed per step");
    println!();
    
    println!("3. 📊 Hierarchical Output Structure:");
    println!("   • Each step contributes to a nested JSON structure");
    println!("   • Results organized by step output fields");
    println!("   • Maintains relationships between dependent extractions");
    println!();
    
    println!("4. ⚙️ Configuration Flexibility:");
    println!("   • Global configuration applies to all steps");
    println!("   • YAML-based configuration for easy editing");
    println!("   • Support for different providers and models");
    println!();
    
    println!("5. 📈 Performance Tracking:");
    println!("   • Individual step timing");
    println!("   • Success/failure status per step");
    println!("   • Total pipeline execution time");
    println!("   • Input/output counts for each step");
    println!();
    
    println!("6. 🛡️ Error Handling:");
    println!("   • Graceful handling of step failures");
    println!("   • Detailed error messages and context");
    println!("   • Pipeline continues with successful steps");
    
    Ok(())
}
