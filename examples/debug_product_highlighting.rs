use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the actual product text
    let text = fs::read_to_string("sample_product_text.txt")?;
    
    // Read the JSON output to get the actual extraction positions
    let json_content = fs::read_to_string("product_catalog_ollama_mistral_20250815_062638.json")?;
    let json_data: serde_json::Value = serde_json::from_str(&json_content)?;
    
    println!("🔍 HIGHLIGHTING DEBUG ANALYSIS");
    println!("{}", "=".repeat(60));
    
    // Get the first few lines for analysis
    let first_lines: String = text.lines().take(5).collect::<Vec<_>>().join("\n");
    println!("📄 First few lines of text:");
    println!("{}", first_lines);
    println!();
    
    // Show character positions for the first line
    let first_line = text.lines().next().unwrap_or("");
    println!("📍 Character positions for first line:");
    println!("{}", first_line);
    for i in 0..first_line.len() {
        print!("{}", i % 10);
    }
    println!();
    for i in 0..first_line.len() {
        if i % 10 == 0 && i > 0 {
            print!("|");
        } else {
            print!(" ");
        }
    }
    println!();
    println!();
    
    // Analyze the problematic extractions you mentioned
    if let Some(extractions) = json_data["extractions"].as_array() {
        println!("🚨 PROBLEM ANALYSIS:");
        println!();
        
        // Find the specific extractions you mentioned
        let problem_cases = [
            ("department", "Electronics"),
            ("catalog_year", "2024"), 
            ("product_name", "Apple MacBook Pro 16-inch M3 Max"),
            ("model", "SKU: MBP-M3-16-SLV-2TB"),
            ("product_code", "APPLE-2024-001"),
        ];
        
        for (class, expected_text) in problem_cases {
            if let Some(extraction) = extractions.iter().find(|e| 
                e["extraction_class"].as_str() == Some(class) && 
                e["extraction_text"].as_str() == Some(expected_text)
            ) {
                if let (Some(start), Some(end)) = (
                    extraction["char_interval"]["start_char"].as_u64(),
                    extraction["char_interval"]["end_char"].as_u64()
                ) {
                    let start = start as usize;
                    let end = end as usize;
                    
                    println!("🎯 Extraction: {} = '{}'", class, expected_text);
                    println!("   Position: {}-{}", start, end);
                    
                    if end <= text.len() && start < end {
                        let actual_highlighted = &text[start..end];
                        println!("   Actually highlighting: '{}'", actual_highlighted);
                        
                        if actual_highlighted != expected_text {
                            println!("   ❌ MISMATCH!");
                            
                            // Try to find where the expected text actually appears
                            if let Some(correct_pos) = text.find(expected_text) {
                                println!("   ✅ '{}' actually found at position: {}-{}", 
                                    expected_text, correct_pos, correct_pos + expected_text.len());
                                let correct_text = &text[correct_pos..correct_pos + expected_text.len()];
                                println!("   ✅ Correct text would be: '{}'", correct_text);
                            } else {
                                // Try case-insensitive search
                                let lower_text = text.to_lowercase();
                                let lower_expected = expected_text.to_lowercase();
                                if let Some(correct_pos) = lower_text.find(&lower_expected) {
                                    println!("   ✅ '{}' found (case-insensitive) at position: {}-{}", 
                                        expected_text, correct_pos, correct_pos + expected_text.len());
                                    let correct_text = &text[correct_pos..correct_pos + expected_text.len()];
                                    println!("   ✅ Correct text would be: '{}'", correct_text);
                                }
                            }
                        } else {
                            println!("   ✅ CORRECT!");
                        }
                        
                        // Show context around the highlighted area
                        let context_start = start.saturating_sub(20);
                        let context_end = (end + 20).min(text.len());
                        let context = &text[context_start..context_end];
                        println!("   Context: '{}[{}]{}'", 
                            &context[..start-context_start],
                            &context[start-context_start..end-context_start], 
                            &context[end-context_start..]);
                    } else {
                        println!("   ❌ INVALID POSITION (start: {}, end: {}, text_len: {})", start, end, text.len());
                    }
                    println!();
                }
            } else {
                println!("🔍 Could not find extraction: {} = '{}'", class, expected_text);
            }
        }
        
        // Also show the first 10 extractions for general analysis
        println!("📊 FIRST 10 EXTRACTIONS ANALYSIS:");
        println!();
        
        for (i, extraction) in extractions.iter().take(10).enumerate() {
            if let (Some(class), Some(text), Some(start), Some(end)) = (
                extraction["extraction_class"].as_str(),
                extraction["extraction_text"].as_str(),
                extraction["char_interval"]["start_char"].as_u64(),
                extraction["char_interval"]["end_char"].as_u64()
            ) {
                let start = start as usize;
                let end = end as usize;
                
                println!("{}. {} = '{}'", i+1, class, text);
                println!("   Position: {}-{}", start, end);
                
                if end <= text.len() && start < end {
                    let actual = &text[start..end];
                    println!("   Highlighting: '{}'", actual);
                    if actual.to_lowercase() != text.to_lowercase() {
                        println!("   ❌ Mismatch detected");
                    } else {
                        println!("   ✅ Match");
                    }
                } else {
                    println!("   ❌ Invalid position");
                }
                println!();
            }
        }
    }
    
    Ok(())
}
