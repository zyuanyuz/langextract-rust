# LangExtract Performance Tuning Guide

## Key Performance Parameters

### 1. `max_workers` - Parallel Processing
- **What it does**: Controls how many text chunks are processed simultaneously
- **Default**: Usually 2-3
- **Recommended**: 6-12 for most systems
- **Considerations**:
  - **Higher = Faster**: More parallel requests to the LLM
  - **But**: Limited by API rate limits and system resources
  - **OpenAI**: Can handle 10+ parallel requests
  - **Ollama**: Limited by local GPU/CPU resources (typically 4-8)

### 2. `batch_length` - Chunk Batch Size
- **What it does**: How many chunks to process in each batch
- **Default**: Usually 3-5
- **Recommended**: 6-10 for better throughput
- **Considerations**:
  - **Higher = More efficient**: Reduces overhead between batches
  - **But**: Longer wait times for first results

### 3. `max_char_buffer` - Chunk Size
- **What it does**: Maximum characters per chunk sent to LLM
- **Considerations**:
  - **Larger chunks = Fewer API calls**: More efficient, faster overall
  - **Smaller chunks = Better accuracy**: More focused extraction per chunk
  - **Balance**: 6000-12000 characters usually optimal

## Performance Optimization Strategies

### For Speed (Fast Processing)
```rust
ExtractConfig {
    max_workers: 10,        // High parallelism
    batch_length: 8,        // Large batches
    max_char_buffer: 12000, // Large chunks (fewer API calls)
    temperature: 0.1,       // Low temperature for consistency
    // ...
}
```

### For Accuracy (Detailed Extraction)
```rust
ExtractConfig {
    max_workers: 4,         // Moderate parallelism
    batch_length: 3,        // Smaller batches
    max_char_buffer: 4000,  // Smaller chunks (more focused)
    temperature: 0.3,       // Slightly higher for creativity
    // ...
}
```

### For Balanced Performance
```rust
ExtractConfig {
    max_workers: 6,         // Good parallelism
    batch_length: 6,        // Moderate batches
    max_char_buffer: 8000,  // Balanced chunk size
    temperature: 0.2,       // Balanced temperature
    // ...
}
```

## Provider-Specific Recommendations

### OpenAI (GPT-4o, GPT-4o-mini)
- **max_workers**: 8-12 (high rate limits)
- **max_char_buffer**: 10000-15000 (large context windows)
- **batch_length**: 8-10

### Ollama (Local Models)
- **max_workers**: 4-8 (limited by local resources)
- **max_char_buffer**: 6000-10000 (depends on model size)
- **batch_length**: 4-6

### Claude/Anthropic
- **max_workers**: 6-10 (good rate limits)
- **max_char_buffer**: 8000-12000
- **batch_length**: 6-8

## System Resource Considerations

### CPU/Memory
- Each worker uses additional CPU and memory
- Monitor system resources when increasing `max_workers`

### Network/API Limits
- **Rate limits**: Don't exceed provider rate limits
- **Token limits**: Larger chunks use more tokens per request
- **Cost**: More parallel workers = higher concurrent token usage

## Testing Performance

### Quick Test
```bash
time ./test_academic_extraction.sh
```

### Benchmark Different Settings
1. Run with `max_workers: 2, batch_length: 3`
2. Run with `max_workers: 8, batch_length: 6`
3. Compare total processing time

### Monitor Debug Output
Enable `debug: true` to see:
- Chunk processing times
- Number of chunks created
- Parallel processing efficiency

## Example Configurations

### High-Speed Configuration (Trading accuracy for speed)
```rust
max_workers: 12,
batch_length: 10,
max_char_buffer: 15000,
temperature: 0.1,
```

### High-Accuracy Configuration (Trading speed for precision)
```rust
max_workers: 3,
batch_length: 2,
max_char_buffer: 4000,
temperature: 0.3,
```

### Recommended Starting Point
```rust
max_workers: 6,
batch_length: 6,
max_char_buffer: 8000,
temperature: 0.2,
```

Then adjust based on your specific needs and system capabilities!
