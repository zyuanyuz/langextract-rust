# Pipeline Examples

This directory demonstrates LangExtract's **multi-step pipeline processing** capabilities using the CLI interface.

## 📁 Directory Structure

### 🎯 **Complete Pipeline Demo**

#### **`pipeline_demo/`** - Multi-Step Extraction Workflow
```bash
cd pipeline_demo/
./run.sh
```

**What it demonstrates:**
- **3-step pipeline** with parallel execution
- **Requirements extraction** from technical documents
- **Dependency management** between pipeline steps
- **YAML configuration** for reproducible workflows
- **Rich visualizations** with HTML output
- **Performance optimization** through parallel processing

**Pipeline workflow:**
```
Input Document (Technical Requirements)
           ↓
Step 1: Extract Requirements (find "shall" statements)
           ↓
    ┌─Step 2: Extract Values (parallel)
    └─Step 3: Extract Specifications (parallel)
           ↓
Combined Results + HTML Visualization
```

## 🚀 Quick Start

```bash
# Navigate to pipeline demo
cd pipeline_demo/

# Run the complete pipeline demonstration
./run.sh

# View results
ls output/
```

## 🔬 Pipeline Features

### **Multi-Step Processing**
- **Sequential steps**: Later steps build on earlier results
- **Parallel execution**: Independent steps run simultaneously  
- **Dependency resolution**: Automatic step ordering
- **Result aggregation**: Combine outputs into hierarchical structure

### **Advanced Capabilities**
- **YAML configuration**: Define pipelines declaratively
- **Step filtering**: Process only relevant data in each step
- **Quality control**: Validation and error handling
- **Multiple outputs**: JSON, HTML, CSV formats
- **Performance metrics**: Processing time and efficiency analysis

### **Real-World Applications**
- **Technical requirements analysis** - Extract and categorize system requirements
- **Legal document processing** - Multi-step contract and compliance analysis  
- **Research paper analysis** - Methodology, results, and citation extraction
- **Financial document parsing** - Entity, amount, and transaction extraction

## 📊 Expected Results

### **Processing Performance**
- **Document size**: ~2KB technical requirements
- **Sequential time**: ~90-120 seconds
- **Parallel time**: ~60-75 seconds  
- **Efficiency gain**: 30-40% improvement

### **Extraction Quality**
- **Step 1 (Requirements)**: 15-20 requirement statements
- **Step 2 (Values)**: 10-15 numeric values with units
- **Step 3 (Specifications)**: 8-12 technical specifications
- **Total extractions**: 35-50 structured entities

### **Output Files**
```
pipeline_demo/output/
├── step1_requirements.json      # Requirements and "shall" statements
├── step2_values.json           # Numeric values and units
├── step3_specifications.json   # Technical specs and security requirements
├── pipeline_results.json       # Combined hierarchical results
└── pipeline_results_*.html     # Interactive visualization
```

## ⚙️ Configuration

### **Pipeline Definition** (`requirements_pipeline.yaml`)
```yaml
name: "Requirements Extraction Pipeline"
enable_parallel_execution: true

steps:
  - id: "extract_requirements"
    depends_on: []  # Independent step
    
  - id: "extract_values"
    depends_on: ["extract_requirements"]  # Parallel with step 3
    
  - id: "extract_specifications"  
    depends_on: ["extract_requirements"]  # Parallel with step 2
```

### **Global Configuration** (`config.yaml`)
```yaml
model: "mistral"
provider: "ollama"
temperature: 0.3
max_workers: 6
parallel_execution: true
```

## 🎯 Customization

### **Create Custom Pipelines**
1. **Define extraction goals**: What information do you need?
2. **Design step dependencies**: Which steps build on others?
3. **Write focused prompts**: Optimize each step for specific data
4. **Configure parallelization**: Enable parallel execution where beneficial
5. **Test and iterate**: Refine prompts and dependencies

### **Domain-Specific Examples**

#### **Medical Documents**
```yaml
steps:
  - extract_conditions    # Medical conditions and diagnoses
  - extract_medications   # Medications and dosages (parallel)
  - extract_procedures    # Medical procedures (parallel)
```

#### **Legal Contracts**
```yaml
steps:
  - extract_clauses      # Contract clauses
  - extract_parties      # Parties involved (parallel)
  - extract_obligations  # Legal obligations (parallel)
  - extract_dates        # Important dates (parallel)
```

#### **Financial Reports**
```yaml
steps:
  - extract_entities     # Companies and people
  - extract_amounts      # Financial figures (parallel)
  - extract_transactions # Transaction details (parallel)
```

## 🧪 Testing and Development

### **Run the Demo**
```bash
cd pipeline_demo/
./run.sh
```

### **Modify Configuration**
```bash
# Edit pipeline steps
nano pipeline_demo/requirements_pipeline.yaml

# Adjust global settings
nano pipeline_demo/config.yaml

# Test with custom input
echo "Your requirements document..." > pipeline_demo/input.txt
```

### **Debug Pipeline Issues**
```bash
# Enable debug mode in config.yaml
debug: true

# Check individual step results
cat pipeline_demo/output/step1_requirements.json | jq .
```

## 📈 Performance Guidelines

### **Small Documents** (<5KB)
- **Steps**: 2-3 steps maximum
- **Workers**: 2-4 sufficient
- **Parallel benefit**: Limited

### **Medium Documents** (5-50KB)  
- **Steps**: 3-5 steps optimal
- **Workers**: 6-8 recommended
- **Parallel benefit**: Significant

### **Large Documents** (50KB+)
- **Steps**: 4-6 steps + chunking
- **Workers**: 8-12 + batching
- **Parallel benefit**: Essential

## 🔗 Integration

### **CLI Commands**
```bash
# Run pipeline directly (if CLI supports it)
lx-rs pipeline --config requirements_pipeline.yaml input.txt

# Extract with pipeline-like workflow
lx-rs extract input.txt --prompt "step1..." --output step1.json
lx-rs extract input.txt --prompt "step2..." --output step2.json
```

### **Combine with Other Features**
- **Multipass extraction**: `--multipass --passes 3`
- **Validation**: `--validate --strict`  
- **Visualization**: `--export html --show-intervals`
- **Chunking**: `--max-char-buffer 2000`

## 🎨 Visualization Features

The pipeline demo generates rich visualizations:

- **📊 Step-by-step results** - Individual JSON files for each pipeline step
- **🔗 Hierarchical output** - Combined results showing step relationships
- **🌐 Interactive HTML** - Visual representation with highlighted text
- **📈 Performance metrics** - Processing time and efficiency analysis
- **🎯 Quality indicators** - Extraction counts and confidence scores

## 🤝 Best Practices

### **Pipeline Design**
- **Start simple**: Begin with 2-3 steps, add complexity gradually
- **Focus prompts**: Each step should have a clear, specific purpose
- **Minimize dependencies**: Enable parallel execution where possible
- **Test iteratively**: Validate each step before adding the next

### **Configuration Management**
- **Use version control**: Track pipeline configurations
- **Document changes**: Comment YAML files thoroughly
- **Test with real data**: Use representative documents for validation
- **Monitor performance**: Track processing time and accuracy

### **Production Deployment**
- **Error handling**: Implement robust failure recovery
- **Resource management**: Configure appropriate worker counts
- **Quality control**: Set validation thresholds
- **Monitoring**: Track pipeline performance and results

## 📚 Learn More

- **[Pipeline Demo README](pipeline_demo/README.md)** - Detailed implementation guide
- **[Main Examples README](../README.md)** - Overview of all examples
- **[CLI Documentation](../../README.md)** - Complete CLI reference

Transform your complex document analysis with LangExtract pipelines! 🚀