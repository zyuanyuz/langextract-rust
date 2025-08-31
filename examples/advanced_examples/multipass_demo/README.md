# Multipass Demo

This example demonstrates LangExtract's **multipass extraction** capabilities, showing how multiple extraction rounds can significantly improve recall and entity coverage in complex documents.

## What This Example Does

Compares **single-pass** vs **multi-pass** extraction on a complex research text containing:

- 👥 **People**: Researchers, collaborators, authors
- 🏢 **Organizations**: Universities, companies, institutions  
- 📍 **Locations**: Cities, countries, addresses
- 📅 **Dates & Times**: Conference dates, deadlines, schedules
- 💰 **Financial Data**: Grant amounts, funding sources
- 📧 **Contact Info**: Emails, phone numbers, websites
- 🔬 **Technical Terms**: Architectures, metrics, publications

## Key Features Demonstrated

- 🔄 **Multiple extraction passes** for improved recall
- 📈 **Incremental entity discovery** across rounds
- 🎯 **Quality-based reprocessing** decisions
- 📊 **Statistical comparison** of single vs multi-pass
- 🧠 **Complex document handling** with many entity types

## Files

- **`examples.json`** - Diverse training examples for different entity types
- **`config.yaml`** - Configuration optimized for multipass extraction
- **`input.txt`** - Complex research text with 50+ potential entities
- **`run.sh`** - Script demonstrating multipass vs single-pass comparison
- **`output/`** - Generated results for both approaches

## Quick Start

```bash
# Ensure you have a provider running
ollama serve
ollama pull mistral

# Run the multipass comparison demo
./run.sh
```

## Understanding Multipass Extraction

### How It Works

1. **Pass 1**: Initial extraction with base prompt
2. **Pass 2**: Re-process chunks with low entity counts
3. **Pass 3**: Final pass with refined prompts for missed categories
4. **Aggregation**: Combine and deduplicate results

### Quality Thresholds

```yaml
multipass: true
passes: 3
multipass_min_extractions: 3       # Reprocess chunks with <3 entities
multipass_quality_threshold: 0.7   # Keep extractions above 70% quality
```

### When to Use Multipass

- **Complex documents** with many entity types
- **High recall requirements** (find all entities)
- **Academic/research text** with dense information
- **Comprehensive data extraction** projects

## Expected Results

The demo shows typical improvements:

### Single-Pass Results (~25-35 entities)
- Basic entities: obvious names, organizations
- Misses: secondary collaborators, funding details
- Coverage: ~60-70% of available entities

### Multi-Pass Results (~40-55 entities)  
- Enhanced recall: finds missed entities
- Better coverage: secondary collaborators, technical terms
- Improved precision: validates entity quality
- Coverage: ~85-95% of available entities

## What to Look For

### 1. Quantitative Improvements
```bash
Single-pass extractions: 28
Multi-pass extractions: 42
Improvement: +14 extractions (+50%)
```

### 2. Category Coverage
Multi-pass typically finds more:
- **Secondary people**: Collaborators mentioned once
- **Additional organizations**: Sponsors, partners
- **Technical details**: Model names, metrics
- **Contact information**: Complete email/phone sets
- **Location specifics**: Building numbers, room details

### 3. Quality Metrics
- **Precision**: How accurate are the extractions?
- **Recall**: What percentage of entities were found?
- **Coverage**: How many entity categories discovered?

## Configuration for Optimal Multipass

### Temperature Settings
```yaml
temperature: 0.4-0.6    # Higher for diverse passes
```

### Pass Configuration
```yaml
passes: 3-5             # More passes for complex documents
multipass: true         # Enable multipass mode
```

### Quality Thresholds
```yaml
multipass_min_extractions: 2-5     # Reprocess threshold
multipass_quality_threshold: 0.6-0.8   # Quality bar
```

### Worker Settings
```yaml
workers: 4-8            # Parallel processing
batch_size: 4-6         # Moderate batches for stability
```

## Performance Considerations

### Processing Time
- **Single-pass**: ~30-60 seconds
- **Multi-pass (3x)**: ~90-180 seconds  
- **Trade-off**: 3x time for 30-50% more entities

### Resource Usage
- **Memory**: Moderate increase for result tracking
- **API calls**: 2-3x more LLM requests
- **Quality**: Significantly better recall

### Cost-Benefit Analysis
- **Research/Academic**: High recall worth extra cost
- **Production pipelines**: Consider recall requirements
- **Batch processing**: Amortize setup costs

## Troubleshooting

### Low Improvement from Multipass
- **Check examples**: Ensure diverse entity types in training
- **Adjust temperature**: Try 0.4-0.6 for more variation
- **Lower quality threshold**: Allow more reprocessing

### High Processing Time
- **Reduce passes**: Try 2 instead of 3-5
- **Increase quality threshold**: Avoid unnecessary reprocessing
- **Use faster model**: Consider simpler models for initial passes

### Quality Issues
- **Lower temperature**: Use 0.2-0.3 for more consistent extractions
- **Improve examples**: Add more precise training examples
- **Validate manually**: Check a sample of results

## Advanced Usage

### Custom Pass Configuration
```bash
# Fine-tuned multipass
lx-rs extract input.txt \
    --examples examples.json \
    --passes 4 \
    --multipass \
    --temperature 0.5 \
    --workers 8
```

### Specialized Entity Types
Create examples focused on specific domains:
- **Medical**: Drugs, conditions, procedures
- **Legal**: Cases, statutes, parties
- **Financial**: Amounts, instruments, entities
- **Technical**: Models, metrics, specifications

### Quality Analysis
```bash
# Compare different pass counts
./run.sh                    # 3 passes
# Edit config.yaml, change passes: 2
./run.sh                    # 2 passes
# Compare results
```

## Real-World Applications

### Academic Research
- **Paper analysis**: Authors, citations, funding
- **Grant extraction**: Amounts, agencies, timelines
- **Collaboration mapping**: Co-authors, institutions

### Business Intelligence
- **Company research**: Competitors, partnerships, financials
- **Market analysis**: Players, trends, metrics
- **Due diligence**: Key facts, risks, opportunities

### Content Processing
- **News analysis**: People, events, locations
- **Social media**: Mentions, entities, sentiment
- **Document processing**: Contracts, reports, filings

## Next Steps

- Try **advanced_chunking_demo** for large document processing
- Explore **product_catalog** for e-commerce multipass scenarios
- Test with your own complex documents
- Experiment with different pass counts and thresholds
- Create domain-specific training examples for your use case
