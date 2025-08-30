# LangExtract Pipeline Feature

The Pipeline feature enables multi-step information extraction with nested hierarchical processing. This allows you to break down complex extraction tasks into sequential steps, creating structured outputs from unstructured text.

## 🚀 Quick Start

### 1. Create a Sample Pipeline

```bash
# Create a sample requirements extraction pipeline
lx-rs pipeline --create-sample --config requirements_pipeline.yaml

# Or specify a different sample type
lx-rs pipeline --create-sample --config medical_pipeline.yaml --sample-type medical
```

### 2. Execute the Pipeline

```bash
# Process text directly
lx-rs pipeline --config requirements_pipeline.yaml "The system shall process 100 transactions per second."

# Process a file
lx-rs pipeline --config requirements_pipeline.yaml requirements.txt --output results.json

# Process a URL
lx-rs pipeline --config requirements_pipeline.yaml "https://example.com/requirements" --output results.json
```

## 📋 Pipeline Configuration

Pipelines are defined in YAML format with the following structure:

```yaml
name: "Requirements Extraction Pipeline"
description: "Extract requirements and sub-divide into values, units, and specifications"
version: "1.0.0"

global_config:
  model_id: "gpt-4o-mini"
  format_type: "json"
  temperature: 0.3
  max_char_buffer: 8000
  max_workers: 6
  language_model_params:
    provider_config:
      provider_type: "openai"
      base_url: "https://api.openai.com/v1"
      model: "gpt-4o-mini"
  # ... other global settings

steps:
  - id: "extract_requirements"
    name: "Extract Requirements"
    description: "Extract all 'shall' statements and requirements"
    prompt: "Extract all requirements, 'shall' statements, and specifications from the text."
    output_field: "requirements"
    depends_on: []  # No dependencies - first step
    examples:
      - text: "The system shall process 100 transactions per second."
        extractions:
          - extraction_class: "requirement"
            extraction_text: "The system shall process 100 transactions per second."

  - id: "extract_values"
    name: "Extract Values"
    description: "Extract numeric values and units from requirements"
    prompt: "Extract all numeric values and their units from this requirement."
    output_field: "values"
    depends_on: ["extract_requirements"]  # Depends on previous step
    filter:
      class_filter: "requirement"  # Only process requirement extractions
    examples:
      - text: "The system shall process 100 transactions per second."
        extractions:
          - extraction_class: "value"
            extraction_text: "100"
          - extraction_class: "unit"
            extraction_text: "transactions per second"
```

## 🔧 Configuration Options

### Global Configuration

| Field | Description | Example |
|-------|-------------|---------|
| `model_id` | LLM model to use | `"gpt-4o-mini"`, `"mistral"` |
| `format_type` | Output format | `"json"`, `"yaml"` |
| `temperature` | Sampling temperature (0.0-1.0) | `0.3` |
| `max_char_buffer` | Maximum characters per chunk | `8000` |
| `max_workers` | Concurrent processing workers | `6` |
| `provider_config` | LLM provider configuration | See provider docs |

### Step Configuration

| Field | Description | Required |
|-------|-------------|----------|
| `id` | Unique step identifier | ✅ |
| `name` | Human-readable step name | ✅ |
| `description` | Step description | ✅ |
| `prompt` | Extraction prompt | ✅ |
| `output_field` | Output field name | ✅ |
| `depends_on` | Step dependencies | ✅ |
| `examples` | Training examples | ✅ |
| `filter` | Input filtering | ❌ |

### Filter Configuration

```yaml
filter:
  class_filter: "requirement"    # Only process specific extraction classes
  text_pattern: "shall.*"        # Regex pattern for text filtering
  max_items: 10                  # Maximum items to process
```

## 📊 Pipeline Execution

### Execution Flow

1. **Dependency Resolution**: Steps are executed in dependency order
2. **Input Processing**: Each step processes outputs from dependent steps
3. **Filtering**: Optional filtering of input data
4. **Extraction**: LLM processing with step-specific prompts
5. **Aggregation**: Results are collected and structured

### Output Structure

Pipeline results are nested JSON structures:

```json
{
  "extract_requirements": {
    "extractions": [
      {
        "class": "requirement",
        "text": "The system shall process 100 transactions per second",
        "start": 0,
        "end": 55
      }
    ],
    "count": 1,
    "processing_time_ms": 1250
  },
  "extract_values": {
    "extractions": [
      {
        "class": "value",
        "text": "100"
      },
      {
        "class": "unit",
        "text": "transactions per second"
      }
    ],
    "count": 2,
    "processing_time_ms": 980
  }
}
```

## 🛠️ Use Cases

### Requirements Engineering

Extract and categorize requirements from specifications:

```yaml
steps:
  - id: "extract_functional_reqs"
    name: "Functional Requirements"
    prompt: "Extract all functional requirements (what the system shall do)"
    filter:
      class_filter: "requirement"

  - id: "extract_performance_reqs"
    name: "Performance Requirements"
    prompt: "Extract performance metrics, timing, and capacity requirements"
    filter:
      class_filter: "requirement"
```

### Medical Record Processing

Process medical documents hierarchically:

```yaml
steps:
  - id: "extract_symptoms"
    name: "Extract Symptoms"
    prompt: "Extract all mentioned symptoms and conditions"

  - id: "extract_medications"
    name: "Extract Medications"
    prompt: "Extract medication names, dosages, and frequencies"
    depends_on: ["extract_symptoms"]

  - id: "extract_treatments"
    name: "Extract Treatments"
    prompt: "Extract treatment procedures and interventions"
    depends_on: ["extract_symptoms", "extract_medications"]
```

### Financial Document Analysis

Process financial statements and reports:

```yaml
steps:
  - id: "extract_financial_statements"
    name: "Financial Statements"
    prompt: "Extract balance sheet, income statement, and cash flow items"

  - id: "extract_values"
    name: "Extract Values"
    prompt: "Extract monetary values, percentages, and ratios"
    depends_on: ["extract_financial_statements"]

  - id: "categorize_accounts"
    name: "Categorize Accounts"
    prompt: "Categorize extracted items by account type and classification"
    depends_on: ["extract_values"]
```

## 🔍 Advanced Features

### Conditional Processing

Use filters to create conditional processing paths:

```yaml
steps:
  - id: "check_document_type"
    name: "Document Classification"
    prompt: "Classify the document type and main topic"

  - id: "process_contract"
    name: "Process Contract"
    prompt: "Extract contract terms, parties, and obligations"
    filter:
      text_pattern: "contract|agreement"

  - id: "process_technical_spec"
    name: "Process Technical Spec"
    prompt: "Extract technical specifications and requirements"
    filter:
      text_pattern: "specification|requirement"
```

### Parallel Processing

Independent steps can be processed in parallel:

```yaml
steps:
  - id: "extract_entities"
    name: "Entity Extraction"
    depends_on: []
    # Runs in parallel with other root steps

  - id: "extract_relationships"
    name: "Relationship Extraction"
    depends_on: []
    # Also runs in parallel

  - id: "combine_results"
    name: "Combine Results"
    depends_on: ["extract_entities", "extract_relationships"]
    # Waits for both previous steps to complete
```

## 📈 Performance Optimization

### Chunking Strategy

Configure text chunking for large documents:

```yaml
global_config:
  max_char_buffer: 8000  # Optimal chunk size
  max_workers: 8         # Parallel processing
  batch_length: 6        # Batches per worker
```

### Caching and Reuse

Reuse pipeline configurations across multiple documents:

```bash
# Process multiple files with same pipeline
for file in documents/*.txt; do
  lx-rs pipeline --config analysis_pipeline.yaml "$file" \
    --output "results/$(basename "$file" .txt).json"
done
```

## 🐛 Troubleshooting

### Common Issues

**Dependency Cycle Error**
```
Error: Circular dependency detected
```
- Check that `depends_on` doesn't create loops
- Ensure dependency graph is a DAG (Directed Acyclic Graph)

**Provider Configuration Missing**
```
Error: Provider configuration is required
```
- Add provider configuration to `language_model_params.provider_config`
- Or set environment variables for your provider

**Step Execution Failed**
```
Step 'step_name' failed: Parse error
```
- Check step examples and prompts
- Verify input data format
- Enable debug mode: add `debug: true` to global_config

### Debug Mode

Enable detailed logging:

```yaml
global_config:
  debug: true
```

## 📚 Examples

### Complete Requirements Pipeline

See `requirements_pipeline.yaml` for a complete working example that:
- Extracts "shall" statements from requirements documents
- Sub-divides them into numeric values and units
- Categorizes specifications and constraints

### Custom Pipeline Creation

Create custom pipelines for your domain:

```bash
# Start with a sample
lx-rs pipeline --create-sample --config my_pipeline.yaml

# Edit the configuration
nano my_pipeline.yaml

# Test with sample data
lx-rs pipeline --config my_pipeline.yaml "Your test text here"
```

## 🔗 Integration

### Programmatic Usage

```rust
use langextract_rust::pipeline::{PipelineExecutor, utils};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load pipeline from YAML
    let executor = PipelineExecutor::from_yaml_file("pipeline.yaml")?;

    // Execute pipeline
    let result = executor.execute("Your text here").await?;

    // Process results
    println!("Pipeline completed in {}ms", result.total_time_ms);
    println!("Results: {}", serde_json::to_string_pretty(&result.nested_output)?);

    Ok(())
}
```

### CI/CD Integration

```yaml
# .github/workflows/extract.yml
name: Document Extraction
on: [push]

jobs:
  extract:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Extract Requirements
        run: |
          lx-rs pipeline --config requirements_pipeline.yaml \
            --input docs/requirements.md \
            --output results/extracted.json
      - name: Upload Results
        uses: actions/upload-artifact@v3
        with:
          name: extraction-results
          path: results/
```

## 🎯 Best Practices

### Pipeline Design

1. **Start Simple**: Begin with 2-3 steps maximum
2. **Clear Dependencies**: Keep dependency chains short
3. **Meaningful Names**: Use descriptive step IDs and names
4. **Comprehensive Examples**: Provide diverse training examples
5. **Incremental Testing**: Test each step individually

### Performance

1. **Chunk Wisely**: Balance chunk size with processing speed
2. **Parallel Processing**: Maximize worker utilization
3. **Filter Early**: Use filters to reduce processing load
4. **Cache Results**: Reuse pipelines across similar documents

### Maintenance

1. **Version Control**: Track pipeline configuration changes
2. **Documentation**: Document pipeline purpose and usage
3. **Monitoring**: Track performance and accuracy metrics
4. **Updates**: Regularly update examples and prompts

This pipeline system transforms LangExtract from a single-step extraction tool into a powerful multi-step processing framework for complex document analysis workflows.
