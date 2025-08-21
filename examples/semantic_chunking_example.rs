//! Example demonstrating semantic chunking with semchunk-rs

use langextract_rust::chunking::{TextChunker, ChunkingConfig, ChunkingStrategy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 LangExtract Semantic Chunking Example");
    println!("=====================================\n");

    // Configure semantic chunking
    let config = ChunkingConfig {
        strategy: ChunkingStrategy::Semantic,
        max_chunk_size: 20, // Very small chunks to force multiple chunks
        semantic_similarity_threshold: 0.8,
        ..Default::default()
    };

    let chunker = TextChunker::with_config(config);

    // Sample text with different semantic topics that should be chunked semantically
    let text = "Machine learning is a subset of artificial intelligence that involves training algorithms. Deep learning uses neural networks with multiple layers for complex pattern recognition. Natural language processing enables computers to understand and generate human language through sophisticated algorithms. Data science combines statistics and programming to extract insights from large datasets. Computer vision allows machines to interpret and understand visual information from the world around them.";

    println!("📄 Original Text:");
    println!("{}\n", text);
    println!("📊 Text Length: {} characters\n", text.len());

    // Perform semantic chunking
    println!("🤖 Performing Semantic Chunking...");
    let chunks = chunker.chunk_text(text, Some("example_doc".to_string()))?;

    println!("✅ Created {} chunks:\n", chunks.len());

    for (i, chunk) in chunks.iter().enumerate() {
        println!("📦 Chunk {}: ({} chars, offset: {})", i + 1, chunk.char_length, chunk.char_offset);
        println!("   \"{}\"", chunk.text);
        println!();
    }

    println!("🎯 Semantic chunking completed successfully!");
    println!("   - Total chunks: {}", chunks.len());
    println!("   - Average chunk size: {:.1} characters", text.len() as f64 / chunks.len() as f64);

    Ok(())
}
