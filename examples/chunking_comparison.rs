//! Comparison between different chunking strategies

use langextract_rust::chunking::{TextChunker, ChunkingConfig, ChunkingStrategy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 LangExtract Chunking Strategy Comparison");
    println!("=========================================\n");

    // Sample text with clear semantic boundaries
    let text = "The history of artificial intelligence began in ancient times. Philosophers pondered the nature of thought and reasoning. Modern AI research started in the 1950s with computer scientists developing programs that could simulate human intelligence. Machine learning emerged as a key approach in the 1980s, focusing on algorithms that improve through experience. Today, deep learning dominates AI research, using neural networks with many layers to solve complex problems.";

    println!("📄 Original Text ({} characters):", text.len());
    println!("{}\n", text);

    // Compare different chunking strategies
    let strategies = vec![
        (ChunkingStrategy::Semantic, "Semantic Chunking (RECOMMENDED)"),
        #[allow(deprecated)]
        (ChunkingStrategy::Sentence, "Sentence-Based Chunking (DEPRECATED)"),
    ];

    for (strategy, name) in strategies {
        println!("🔍 {}:", name);
        println!("{}", "=".repeat(name.len() + 5));

        let config = ChunkingConfig {
            strategy,
            max_chunk_size: 120, // Reasonable size for comparison
            ..Default::default()
        };

        let chunker = TextChunker::with_config(config);
        let chunks = chunker.chunk_text(text, None)?;

        println!("📊 Created {} chunks:", chunks.len());

        for (i, chunk) in chunks.iter().enumerate() {
            println!("\n📦 Chunk {} ({} chars, offset: {})", i + 1, chunk.char_length, chunk.char_offset);
            println!("   \"{}\"", chunk.text);

            // Show semantic coherence
            let words: Vec<&str> = chunk.text.split_whitespace().collect();
            if words.len() > 3 {
                println!("   📝 Key terms: {}... ({} words)", words[..3].join(", "), words.len());
            }
        }

        println!("\n");
    }

    println!("🎯 Analysis:");
    println!("===========");
    println!("• ✅ Semantic: Uses AI-powered content understanding to create coherent, meaningful chunks");
    println!("• ⚠️  Sentence: Respects sentence boundaries but lacks semantic understanding");
    println!("• 📝 Recommendation: Use Semantic chunking for best results");
    println!("\nSemantic chunking provides superior coherence and maintains topic relationships within chunks.");

    Ok(())
}
