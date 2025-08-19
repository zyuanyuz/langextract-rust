//! Real provider test with OpenAI and Ollama
//!
//! This example tests the token-based chunking system with your actual
//! configured providers: OpenAI (from .env) and local Ollama with Mistral.

use langextract_rust::{
    extract, ExampleData, Extraction, ExtractConfig, FormatType, ProviderConfig
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging and environment
    env_logger::init();
    dotenvy::dotenv().ok();

    println!("🚀 Real Provider Testing with Token-Based Chunking");
    println!("====================================================\n");

    // Create comprehensive example data
    let examples = vec![
        ExampleData::new(
            "Dr. John Smith, age 45, is a cardiologist at Johns Hopkins Hospital in Baltimore".to_string(),
            vec![
                Extraction::new("person_name".to_string(), "Dr. John Smith".to_string()),
                Extraction::new("age".to_string(), "45".to_string()),
                Extraction::new("profession".to_string(), "cardiologist".to_string()),
                Extraction::new("organization".to_string(), "Johns Hopkins Hospital".to_string()),
                Extraction::new("location".to_string(), "Baltimore".to_string()),
            ],
        ),
        ExampleData::new(
            "Contact Sarah Wilson at s.wilson@stanford.edu or call (650) 555-0123".to_string(),
            vec![
                Extraction::new("person_name".to_string(), "Sarah Wilson".to_string()),
                Extraction::new("email".to_string(), "s.wilson@stanford.edu".to_string()),
                Extraction::new("phone".to_string(), "(650) 555-0123".to_string()),
            ],
        ),
        ExampleData::new(
            "The project received $2.5 million in funding from Google and Microsoft".to_string(),
            vec![
                Extraction::new("funding_amount".to_string(), "$2.5 million".to_string()),
                Extraction::new("organization".to_string(), "Google".to_string()),
                Extraction::new("organization".to_string(), "Microsoft".to_string()),
            ],
        ),
    ];

    // Create test document with multiple entities
    let test_document = create_test_document();
    
    println!("📄 Test Document Statistics:");
    println!("   Length: {} characters", test_document.len());
    println!("   Words: ~{} words", test_document.split_whitespace().count());
    println!("   Preview: {}...\n", &test_document[..200]);

    // Test 1: OpenAI with token-based chunking
    println!("🔸 Test 1: OpenAI (gpt-3.5-turbo) with Token-Based Chunking");
    println!("{}", "─".repeat(60));
    
    let openai_result = test_openai_extraction(&test_document, &examples).await;
    
    println!("\n🔸 Test 2: Local Ollama (mistral) with Token-Based Chunking");
    println!("{}", "─".repeat(60));
    
    let ollama_result = test_ollama_extraction(&test_document, &examples).await;
    
    // Compare results
    println!("\n📊 Results Comparison");
    println!("{}", "═".repeat(60));
    
    match (&openai_result, &ollama_result) {
        (Ok(openai), Ok(ollama)) => {
            println!("✅ Both providers completed successfully!");
            println!("   OpenAI extractions: {}", openai.extraction_count());
            println!("   Ollama extractions: {}", ollama.extraction_count());
            
            // Show sample extractions from each
            if let Some(openai_extractions) = &openai.extractions {
                println!("\n📋 OpenAI Sample Results:");
                for (i, extraction) in openai_extractions.iter().take(5).enumerate() {
                    println!("   {}. [{}] {}", i + 1, extraction.extraction_class, extraction.extraction_text);
                }
            }
            
            if let Some(ollama_extractions) = &ollama.extractions {
                println!("\n📋 Ollama Sample Results:");
                for (i, extraction) in ollama_extractions.iter().take(5).enumerate() {
                    println!("   {}. [{}] {}", i + 1, extraction.extraction_class, extraction.extraction_text);
                }
            }
        }
        (Ok(openai), Err(ollama_err)) => {
            println!("✅ OpenAI succeeded: {} extractions", openai.extraction_count());
            println!("❌ Ollama failed: {}", ollama_err);
        }
        (Err(openai_err), Ok(ollama)) => {
            println!("❌ OpenAI failed: {}", openai_err);
            println!("✅ Ollama succeeded: {} extractions", ollama.extraction_count());
        }
        (Err(openai_err), Err(ollama_err)) => {
            println!("❌ Both providers failed:");
            println!("   OpenAI: {}", openai_err);
            println!("   Ollama: {}", ollama_err);
        }
    }

    Ok(())
}

async fn test_openai_extraction(
    text: &str, 
    examples: &[ExampleData]
) -> Result<langextract_rust::AnnotatedDocument, Box<dyn std::error::Error>> {
    
    let provider_config = ProviderConfig::openai("gpt-3.5-turbo", None);
    
    let mut config = ExtractConfig {
        model_id: "gpt-3.5-turbo".to_string(),
        api_key: None, // Will load from .env OPENAI_API_KEY
        format_type: FormatType::Json,
        temperature: 0.2,
        debug: true,
        
        // Token-based chunking optimized for OpenAI API
        max_char_buffer: 1200,  // Larger chunks for API efficiency
        batch_length: 3,        // Moderate batch size to respect rate limits
        max_workers: 4,         // Concurrent requests
        extraction_passes: 1,   // Single pass for cost efficiency
        enable_multipass: false,
        
        ..Default::default()
    };
    
    // Set provider configuration
    config.language_model_params.insert(
        "provider_config".to_string(),
        serde_json::to_value(&provider_config)?,
    );
    
    println!("   Configuration:");
    println!("   - Max chars per buffer: {} (respects sentence boundaries)", config.max_char_buffer);
    println!("   - Batch size: {}", config.batch_length);
    println!("   - Workers: {}", config.max_workers);
    println!("   - Multi-pass: {}", config.enable_multipass);
    
    match extract(
        text,
        Some("Extract person names, ages, professions, organizations, locations, contact information, and funding amounts from the text"),
        examples,
        config,
    ).await {
        Ok(result) => {
            println!("   ✅ OpenAI extraction successful: {} extractions", result.extraction_count());
            Ok(result)
        }
        Err(e) => {
            println!("   ❌ OpenAI extraction failed: {}", e);
            if e.to_string().contains("API key") {
                println!("   💡 Tip: Make sure OPENAI_API_KEY is set in your .env file");
            }
            Err(e.into())
        }
    }
}

async fn test_ollama_extraction(
    text: &str, 
    examples: &[ExampleData]
) -> Result<langextract_rust::AnnotatedDocument, Box<dyn std::error::Error>> {
    
    let provider_config = ProviderConfig::ollama("mistral", Some("http://localhost:11434".to_string()));
    
    let mut config = ExtractConfig {
        model_id: "mistral".to_string(),
        api_key: None, // Not needed for Ollama
        model_url: Some("http://localhost:11434".to_string()),
        format_type: FormatType::Json,
        temperature: 0.2,
        debug: true,
        
        // Token-based chunking optimized for local Ollama
        max_char_buffer: 1000,  // Moderate chunks for local processing
        batch_length: 2,        // Smaller batches for local compute
        max_workers: 2,         // Conservative worker count
        extraction_passes: 1,   // Single pass for speed
        enable_multipass: false,
        
        ..Default::default()
    };
    
    // Set provider configuration
    config.language_model_params.insert(
        "provider_config".to_string(),
        serde_json::to_value(&provider_config)?,
    );
    
    println!("   Configuration:");
    println!("   - Max chars per buffer: {} (respects sentence boundaries)", config.max_char_buffer);
    println!("   - Batch size: {}", config.batch_length);
    println!("   - Workers: {}", config.max_workers);
    println!("   - Multi-pass: {}", config.enable_multipass);
    
    match extract(
        text,
        Some("Extract person names, ages, professions, organizations, locations, contact information, and funding amounts from the text"),
        examples,
        config,
    ).await {
        Ok(result) => {
            println!("   ✅ Ollama extraction successful: {} extractions", result.extraction_count());
            Ok(result)
        }
        Err(e) => {
            println!("   ❌ Ollama extraction failed: {}", e);
            if e.to_string().contains("Network error") || e.to_string().contains("Connection") {
                println!("   💡 Tip: Make sure Ollama is running with 'ollama serve' and mistral model is pulled");
            }
            Err(e.into())
        }
    }
}

fn create_test_document() -> String {
    r#"
# Silicon Valley Tech Report 2024

## Executive Leadership

Dr. Sarah Chen, 42, recently joined Meta as Chief AI Officer after spending five years at Google DeepMind. 
Chen's groundbreaking work on neural architecture search has earned her recognition as one of the top 10 
AI researchers under 50. She can be reached at s.chen@meta.com or (650) 555-0198.

The startup ecosystem continues to thrive with young entrepreneurs like Marcus Rodriguez, 28, who founded 
DataFlow Systems and secured $15 million in Series A funding from Sequoia Capital and Andreessen Horowitz. 
Rodriguez previously worked as a senior engineer at Stripe before launching his company.

## Academic Partnerships

Stanford University's AI Lab, led by Professor Emily Watson (age 55), has established partnerships with 
major tech companies including Apple, Microsoft, and NVIDIA. Watson's research team of 25 graduate students 
focuses on computer vision and natural language processing.

The lab's recent breakthrough in multimodal learning, published in Nature, was funded by a $3.2 million 
grant from the National Science Foundation. Contact Prof. Watson at e.watson@stanford.edu or visit 
her office at 353 Jane Stanford Way, Building 160, Room 382.

## Industry Investments

Venture capital activity remains strong, with firms like Kleiner Perkins and Greylock Partners investing 
heavily in AI startups. Notable recent investments include:

- $50 million Series B for QuantumAI Corp (CEO: Dr. Michael Chang, 39)
- $25 million Series A for BioML Technologies (Founder: Lisa Park, 31)  
- $75 million Series C for RoboVision Inc (CTO: David Kim, 45)

## International Collaboration

Cross-border partnerships are expanding, with European companies like DeepMind (London) and Canadian 
firms such as Element AI (Montreal) establishing offices in Silicon Valley. Japanese tech giant SoftBank 
continues its aggressive investment strategy through its $100 billion Vision Fund.

Contact information for key international partners:
- Dr. James Wilson (DeepMind London): j.wilson@deepmind.com, +44 20 7946 0958
- Prof. Marie Dubois (Université de Montréal): m.dubois@umontreal.ca, +1 514 555 0234
- Hiroshi Tanaka (SoftBank Vision Fund): h.tanaka@softbank.com, +81 3 6889 2000

The research lab addresses are:
- DeepMind: 5 New Street Square, London EC4A 3TW, UK
- Element AI: 2020 Rue University, Montreal QC H3A 2A5, Canada
- Vision Fund: 1-9-1 Higashi-Shimbashi, Minato-ku, Tokyo 105-7303, Japan
"#.trim().to_string()
}