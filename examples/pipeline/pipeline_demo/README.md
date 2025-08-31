# Pipeline Demo

This example demonstrates LangExtract's **multi-step pipeline processing** capabilities, showing how complex documents can be analyzed through dependent extraction workflows that build hierarchical structured data.

## What This Example Does

Processes a technical requirements document through a **3-step pipeline**:

1. **📋 Extract Requirements** - Find all "shall" statements and requirements
2. **📊 Extract Values** - Pull numeric values, units, and performance metrics (parallel)
3. **🔧 Extract Specifications** - Extract security specs and technical constraints (parallel)

**Steps 2 and 3 run in parallel** since they both depend only on Step 1, demonstrating pipeline optimization.

## Key Features Demonstrated

- 🔗 **Multi-step workflows** with step dependencies
- ⚡ **Parallel execution** of independent steps
- 📊 **Hierarchical data extraction** from complex documents
- 🎯 **Focused prompts** for each extraction stage
- 📈 **Performance optimization** through parallel processing
- 🔧 **Pipeline configuration** with YAML files

## Files

- **`config.yaml`** - Global configuration for pipeline processing
- **`requirements_pipeline.yaml`** - Complete pipeline definition with steps and dependencies
- **`input.txt`** - Technical requirements document with multiple categories
- **`run.sh`** - Script demonstrating the full pipeline workflow
- **`output/`** - Generated results from each pipeline step

## Quick Start

```bash
# Ensure you have a provider running
ollama serve
ollama pull mistral

# Run the pipeline demo
./run.sh
```

## Understanding Pipeline Processing

### Pipeline Concept
Instead of extracting everything at once, pipelines break complex extraction into focused steps:

```
Input Document
      ↓
Step 1: Extract Requirements
      ↓
   ┌─Step 2: Extract Values (parallel)
   └─Step 3: Extract Specifications (parallel)
      ↓
Combined Results
```

### Step Dependencies
- **Step 1** (Requirements): Independent - processes original document
- **Step 2** (Values): Depends on Step 1 - extracts from requirements found
- **Step 3** (Specifications): Depends on Step 1 - extracts from requirements found

Since Steps 2 and 3 only depend on Step 1, they can run **in parallel** for efficiency.

### Example Workflow

#### Step 1: Requirements Extraction
**Input**: Full requirements document
**Output**: 
```json
[
  {"extraction_class": "performance_req", "extraction_text": "system shall process 1,000 transactions per second"},
  {"extraction_class": "security_req", "extraction_text": "data transmissions shall use AES-256 encryption"}
]
```

#### Step 2: Values Extraction (Parallel)
**Input**: Requirements from Step 1
**Output**:
```json
[
  {"extraction_class": "numeric_value", "extraction_text": "1,000"},
  {"extraction_class": "unit", "extraction_text": "transactions per second"},
  {"extraction_class": "numeric_value", "extraction_text": "256"}
]
```

#### Step 3: Specifications Extraction (Parallel)
**Input**: Requirements from Step 1  
**Output**:
```json
[
  {"extraction_class": "encryption_spec", "extraction_text": "AES-256 encryption"},
  {"extraction_class": "auth_spec", "extraction_text": "multi-factor authentication"}
]
```

## Pipeline Configuration

### YAML Pipeline Definition

The `requirements_pipeline.yaml` file defines the complete pipeline:

```yaml
name: "Requirements Extraction Pipeline"
enable_parallel_execution: true  # Enable parallel steps

steps:
  - id: "extract_requirements"
    depends_on: []  # Independent step
    
  - id: "extract_values"
    depends_on: ["extract_requirements"]  # Depends on step 1
    
  - id: "extract_specifications"  
    depends_on: ["extract_requirements"]  # Depends on step 1
```

### Step Filtering

Steps can include filters to process only relevant data:

```yaml
filter:
  text_pattern: "shall|must|should|requirement|performance"
  max_items: 20
```

### Provider Configuration

Global configuration applies to all steps:

```yaml
global_config:
  model_id: "mistral"
  temperature: 0.3
  max_workers: 6
  parallel_execution: true
```

## Expected Results

### Processing Performance
- **Sequential processing**: ~90-120 seconds
- **Parallel processing**: ~60-75 seconds  
- **Efficiency gain**: ~30-40% time savings

### Extraction Results
```
Step 1 (Requirements): 15-20 requirement statements
Step 2 (Values): 10-15 numeric values with units
Step 3 (Specifications): 6-8 hierarchically organized categories
Total: 35-50 individual requirements organized into logical groups
```

### Result Quality
- **Focused extraction**: Each step optimized for specific data types
- **Reduced noise**: Steps filter out irrelevant information
- **Better accuracy**: Targeted prompts improve precision
- **Structured hierarchy**: Clear organization of extracted data

### 📋 **Important: Nested JSON Structure**
The pipeline produces **nested JSON structures** in the `extraction_text` field, representing **higher-quality, semantically-aware extraction**. Instead of flat, disconnected extractions, you get **organized categories** with related requirements grouped together.

**Example:**
```json
{
  "extraction_class": "security",
  "extraction_text": "{\"encryption\":\"AES-256\",\"authentication\":\"MFA\",\"logs\":\"2 years retention\"}"
}
```

This is **intentional and beneficial** - see `NESTED_JSON_GUIDE.md` for details on why this happens and how to work with it.

## Real-World Applications

### Technical Documentation
- **Requirements analysis**: Extract and categorize system requirements
- **Specification parsing**: Pull technical specs and constraints
- **Compliance checking**: Identify regulatory requirements

### Legal Document Processing
- **Contract analysis**: Extract clauses, terms, obligations
- **Regulatory compliance**: Identify compliance requirements
- **Risk assessment**: Extract risk factors and mitigation strategies

### Research Papers
- **Methodology extraction**: Extract research methods and procedures
- **Results parsing**: Pull quantitative results and metrics
- **Citation analysis**: Extract references and related work

### Business Intelligence
- **Financial analysis**: Extract financial metrics and KPIs
- **Competitive intelligence**: Analyze competitor capabilities
- **Market research**: Extract market data and trends

## Advanced Pipeline Features

### Dynamic Step Configuration
```yaml
steps:
  - id: "conditional_step"
    condition: "previous_step_count > 5"  # Only run if previous step found enough data
```

### Quality Thresholds
```yaml
quality_control:
  min_extractions_per_step: 3
  confidence_threshold: 0.8
  retry_failed_steps: true
```

### Error Handling
```yaml
error_handling:
  max_retries: 3
  fallback_strategy: "continue"  # or "fail_fast"
  timeout_seconds: 300
```

## Performance Optimization

### Parallel Execution Benefits
- **Independent steps**: Run simultaneously when dependencies allow
- **Resource utilization**: Better use of available workers
- **Scalability**: Handles larger documents more efficiently

### Configuration Tuning
```yaml
# High performance configuration
max_workers: 12
batch_length: 8
temperature: 0.2  # Lower for consistency
enable_multipass: false  # Disable for speed
```

### Memory Management
```yaml
# Large document configuration
max_char_buffer: 4000
chunking_strategy: "semantic"
overlap_handling: true
```

## Customizing Pipelines

### Create Custom Steps
1. **Define the step** in YAML with unique ID
2. **Set dependencies** on previous steps
3. **Configure prompts** for specific extraction goals
4. **Add examples** to train the model
5. **Set filters** to process relevant data only

### Example Custom Step
```yaml
- id: "extract_dates"
  name: "Extract Dates"
  prompt: "Extract all dates, deadlines, and time periods from the requirements"
  depends_on: ["extract_requirements"]
  filter:
    text_pattern: "date|deadline|timeline|schedule|period"
```

### Domain-Specific Pipelines
- **Medical**: Extract symptoms → diagnoses → treatments
- **Financial**: Extract transactions → amounts → accounts
- **Legal**: Extract parties → obligations → dates
- **Technical**: Extract components → specifications → requirements

## Troubleshooting

### Pipeline Failures
- **Check dependencies**: Ensure previous steps completed successfully
- **Validate YAML**: Syntax errors can break pipeline loading
- **Monitor resources**: Large pipelines may need more workers/memory

### Poor Step Results
- **Improve prompts**: Make step prompts more specific
- **Add examples**: Include more training examples for each step
- **Adjust filters**: Fine-tune filtering criteria

### Performance Issues
- **Enable parallelization**: Set `enable_parallel_execution: true`
- **Optimize workers**: Balance worker count with available resources
- **Reduce temperature**: Lower temperature for faster, more consistent results

## Integration with Other Features

### Pipeline + Multipass
```bash
# Run pipeline with multipass for each step
lx-rs pipeline --config pipeline.yaml --multipass --passes 3
```

### Pipeline + Validation
```bash
# Add validation to pipeline steps
lx-rs pipeline --config pipeline.yaml --validate --strict
```

### Pipeline + Visualization
```bash
# Generate rich visualizations of pipeline results
lx-rs pipeline --config pipeline.yaml --export html --show-steps

# Programmatic export in Rust (layered highlights across steps)
// Build your PipelineResult via executor, then:
use langextract_rust::visualization::{export_pipeline_html, ExportConfig, ExportFormat};
let cfg = ExportConfig {
    format: ExportFormat::Html,
    aggregate_pipeline_highlights: true,
    allow_overlapping_highlights: true,
    expand_nested_json: true,
    show_pipeline_legend: true,
    ..Default::default()
};
let html = export_pipeline_html(&pipeline_result, &original_text, &cfg).unwrap();
std::fs::write("pipeline_layered.html", html).unwrap();
```

## Next Steps

- Try modifying `requirements_pipeline.yaml` to add custom steps
- Test with your own technical documents and requirements
- Create domain-specific pipeline templates for your use case
- Experiment with different dependency structures and parallel execution
- Combine pipelines with multipass and validation for production workflows
