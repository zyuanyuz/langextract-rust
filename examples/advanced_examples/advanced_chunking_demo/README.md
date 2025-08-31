# Advanced Chunking Demo

This example demonstrates LangExtract's **intelligent document chunking** capabilities, showing how large documents are efficiently processed through semantic text segmentation and parallel chunk processing.

## What This Example Does

Compares different **chunking strategies** on a large climate research document (6,000+ characters) containing:

- 👥 **People**: 20+ researchers, directors, coordinators
- 🏢 **Organizations**: 15+ universities, institutions, companies  
- 📍 **Locations**: Cities, countries, addresses across 6 continents
- 💰 **Funding**: Multi-currency amounts, grants, budgets
- 📧 **Contact Info**: Emails, phones, websites
- 🔬 **Technical Details**: Computing resources, timelines, objectives

## Key Features Demonstrated

- 🧩 **Intelligent chunking** with semantic boundary detection
- ⚡ **Parallel processing** of multiple chunks
- 📊 **Performance comparison** across chunk sizes  
- 🎯 **Consistent extraction** across document sections
- 📍 **Character alignment** preservation across chunks

## Files

- **`examples.json`** - Training examples for consistent chunk processing
- **`config.yaml`** - Configuration optimized for chunking performance
- **`input.txt`** - Large climate research document (6,000+ chars)
- **`run.sh`** - Script comparing different chunking strategies
- **`output/`** - Generated results for each chunk size

## Quick Start

```bash
# Ensure you have a provider running
ollama serve
ollama pull mistral

# Run the chunking comparison demo
./run.sh
```

## Understanding Document Chunking

### Why Chunking is Needed

LLMs have **context limits** (typically 4K-32K tokens). Large documents must be:
1. **Split** into processable chunks
2. **Processed** in parallel for speed
3. **Aggregated** with consistent results

### Chunking Strategies Compared

The demo tests three chunk sizes:

#### 1. Large Chunks (8KB)
- **Pros**: Fewer API calls, more context per chunk
- **Cons**: Slower processing, potential missed entities
- **Best for**: Simple documents, cost optimization

#### 2. Medium Chunks (4KB) ⭐ **Optimal**
- **Pros**: Balanced speed/accuracy, good parallelism
- **Cons**: Moderate complexity
- **Best for**: Most documents, recommended default

#### 3. Small Chunks (2KB)  
- **Pros**: High granularity, maximum parallelism
- **Cons**: More API calls, potential context loss
- **Best for**: Complex documents requiring detail

## Expected Results

### Performance Comparison
```
Large chunks (8KB):  ~45-60 seconds, ~35-45 extractions
Medium chunks (4KB): ~60-75 seconds, ~40-55 extractions  ⭐
Small chunks (2KB):  ~75-90 seconds, ~35-50 extractions
```

### Extraction Quality
- **Large chunks**: May miss entities at chunk boundaries
- **Medium chunks**: Optimal balance of speed and thoroughness
- **Small chunks**: High granularity but potential context fragmentation

## What to Look For

### 1. Processing Time vs. Chunk Size
- **Smaller chunks** = more parallel processing but more API calls
- **Larger chunks** = fewer calls but less parallelism
- **Sweet spot** usually around 4-6KB for most documents

### 2. Entity Coverage
- **Boundary effects**: Entities split across chunks
- **Context preservation**: Related entities in same chunk
- **Completeness**: Overall entity recall

### 3. Chunk Processing Details
With `--debug` enabled, see:
```
🧩 Chunk 1: 4000 chars (offset: 0)
🧩 Chunk 2: 3847 chars (offset: 3800)
🧩 Chunk 3: 2891 chars (offset: 7400)
```

## Optimization Guidelines

### Chunk Size Selection

**Document Type → Recommended Chunk Size**
- **Simple text** (news, blogs): 6-8KB
- **Technical docs** (research, reports): 4-6KB  
- **Complex structured** (legal, financial): 2-4KB
- **Dense entity text** (directories, catalogs): 2-3KB

### Worker Configuration
```yaml
workers: 4-12          # Based on available resources
batch_size: 4-8        # Chunks processed simultaneously  
max_chars: 2000-8000   # Chunk size in characters
```

### Performance Tuning
```bash
# High throughput (more workers, larger batches)
--workers 12 --batch-size 8 --max-chars 6000

# High accuracy (fewer workers, smaller chunks)  
--workers 4 --batch-size 4 --max-chars 3000

# Balanced (recommended starting point)
--workers 8 --batch-size 6 --max-chars 4000
```

## Advanced Chunking Features

### Semantic Boundary Detection
The system attempts to:
- **Preserve sentences** - avoid mid-sentence splits
- **Respect paragraphs** - maintain logical boundaries  
- **Handle overlap** - prevent entity fragmentation

### Chunk Processing Pipeline
1. **Document analysis** - size, structure, complexity
2. **Chunk generation** - semantic boundary detection
3. **Parallel processing** - distribute across workers
4. **Result aggregation** - combine and deduplicate
5. **Alignment restoration** - character positions in original text

### Character Position Preservation
Despite chunking, character positions reference the **original document**:
```json
{
  "extraction_text": "Dr. Elena Rodriguez",
  "char_interval": {
    "start_pos": 245,
    "end_pos": 262
  }
}
```

## Troubleshooting

### Poor Performance with Chunking
- **Reduce chunk size**: Try 2-3KB for complex documents
- **Increase workers**: More parallel processing
- **Check overlap**: Ensure important entities aren't split

### Missing Entities at Boundaries
- **Enable overlap**: Use semantic chunking with overlap
- **Adjust chunk size**: Larger chunks reduce boundary issues
- **Post-processing**: Manual review of chunk boundaries

### High Processing Time
- **Increase chunk size**: Fewer chunks = fewer API calls
- **Optimize workers**: Match worker count to available resources
- **Batch processing**: Process multiple documents together

## Real-World Applications

### Document Types Best Suited for Chunking

**Research Papers**
- **Size**: 10-50KB typically
- **Strategy**: 4-6KB chunks
- **Focus**: Authors, institutions, methodologies

**Legal Documents**  
- **Size**: 20-500KB typically
- **Strategy**: 3-5KB chunks with overlap
- **Focus**: Parties, dates, clauses, references

**Financial Reports**
- **Size**: 50-1000KB typically  
- **Strategy**: 4-8KB chunks
- **Focus**: Amounts, entities, dates, metrics

**Technical Specifications**
- **Size**: 5-100KB typically
- **Strategy**: 2-4KB chunks
- **Focus**: Components, specifications, measurements

## Combining with Other Features

### Chunking + Multipass
```bash
lx-rs extract large_doc.txt \
    --examples examples.json \
    --max-chars 4000 \
    --workers 8 \
    --passes 3 \
    --multipass
```

### Chunking + Validation
```bash
lx-rs extract document.txt \
    --examples examples.json \
    --max-chars 4000 \
    --show-intervals \
    --debug
```

### Chunking + Export
```bash
lx-rs extract large_doc.txt \
    --max-chars 4000 \
    --export html \
    --show-intervals
```

## Next Steps

- Try **multipass_demo** to combine chunking with multiple extraction passes
- Explore **product_catalog** for e-commerce document chunking
- Test with your own large documents  
- Experiment with different chunk sizes for your document types
- Create domain-specific training examples for consistent chunk processing
