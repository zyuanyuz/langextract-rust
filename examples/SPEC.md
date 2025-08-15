# LangExtract Core Capabilities Specification

This document outlines the core capabilities of the langextract-rust library, tracking both completed implementations and remaining work needed to achieve full feature parity with the Python version.

## Implementation Status Overview

### ✅ COMPLETED CORE INFRASTRUCTURE
- **Data Structures**: Complete type system with `ExampleData`, `Extraction`, `AnnotatedDocument`, `CharInterval`
- **Error Handling**: Comprehensive error types with `LangExtractError` and conversion traits
- **Provider System**: Universal provider architecture supporting OpenAI, Ollama, and Custom providers
- **Configuration**: Full `ExtractConfig` and `ProviderConfig` with auto-detection and validation
- **Basic I/O**: URL detection, text downloading, content type detection, HTML text extraction
- **Schema Foundation**: Base schema abstraction and format mode schema generation
- **Inference Trait**: Complete `BaseLanguageModel` trait with `ScoredOutput` types
- **Factory Pattern**: Provider creation and configuration from `ExtractConfig`
- **Advanced Prompt System**: Full template system with variable substitution, provider adaptation
- **Annotation Pipeline**: Complete text annotation through language models with character alignment
- **Character Alignment**: Full implementation mapping extractions to source text positions
- **Text Chunking**: Complete chunking system with parallel processing and result aggregation
- **Response Parsing**: Enhanced JSON/YAML parsing with wrapper format support
- **Advanced Validation**: Complete validation system with type coercion, raw data preservation, and corrected data generation
- **Test Coverage**: 60+ passing unit tests covering all core functionality including validation and type coercion

### 🚧 PARTIAL IMPLEMENTATIONS
- **Provider Implementations**: OpenAI and Ollama working, Azure/Custom providers need completion
- **Visualization**: Basic implementation exists, needs enhancement for rich output formats

## 1. ✅ COMPLETED: Full Prompt Template System

### ✅ Implementation Status: COMPLETE
**Location**: `src/prompting.rs`, `src/prompting/tests.rs`  
**Test Coverage**: 18 comprehensive tests covering all functionality

The prompt template system provides a flexible, structured way to build prompts for different extraction tasks while maintaining consistency and allowing customization.

### ✅ Completed Core Requirements

#### ✅ 1.1 Template Structure - COMPLETE
- ✅ **Base Template**: Core prompt structure with placeholders for task description, examples, context, and input text
- ✅ **Template Variables**: Support for dynamic variable substitution (e.g., `{task}`, `{examples}`, `{text}`)
- ✅ **Format Flexibility**: Templates work with both JSON and YAML output formats
- ✅ **Provider Adaptation**: Templates adapt to provider-specific requirements (OpenAI vs Ollama vs Custom)

#### ✅ 1.2 Example Management - COMPLETE
- ✅ **Example Serialization**: Convert `ExampleData` objects into properly formatted prompt examples
- ✅ **Example Filtering**: Template limits with `max_examples` parameter for token management
- ✅ **Example Limits**: Handle cases where examples exceed token limits via `max_examples`
- ✅ **Dynamic Examples**: Support for adding examples at runtime through `PromptContext`

#### ✅ 1.3 Context Integration - COMPLETE
- ✅ **Additional Context**: Seamlessly integrate user-provided context into prompts
- ✅ **System Messages**: Support for system-level instructions (OpenAI-style)
- ✅ **Few-shot Learning**: Optimize example presentation for few-shot learning
- ✅ **Chain-of-Thought**: Support for reasoning-based prompts with `include_reasoning`

#### ✅ 1.4 Format-Specific Templates - COMPLETE
- ✅ **JSON Templates**: Optimized prompts for JSON output with schema hints
- ✅ **YAML Templates**: Human-readable YAML format prompts
- ✅ **Structured Output**: Integration with provider-specific structured output features
- ✅ **Schema Enforcement**: Templates that encourage adherence to expected output schemas

### ✅ Completed Implementation Components

**Core structures and traits are fully implemented**:

```rust
// Implemented in src/prompting.rs
pub struct PromptTemplate {
    pub base_template: String,
    pub system_message: Option<String>,
    pub format_type: FormatType,
    pub provider_type: ProviderType,
    pub include_reasoning: bool,
    pub max_examples: Option<usize>,
}

pub struct PromptContext {
    pub task_description: String,
    pub examples: Vec<ExampleData>,
    pub input_text: String,
    pub additional_context: Option<String>,
    pub schema_hint: Option<String>,
    pub variables: HashMap<String, String>,
}

pub struct PromptTemplateStructured {
    pub examples: Vec<ExampleData>,
    pub task_description: Option<String>,
    template: PromptTemplate,
}
```

### ✅ Testing Status: COMPLETE (18 tests passing)

#### ✅ Completed Test Scenarios - ALL PASSING

1. ✅ **Basic Template Rendering**
   - ✅ Template rendering with task description and input text works correctly
   - ✅ Missing variable detection provides clear error messages

2. ✅ **Example Integration** 
   - ✅ Example limiting with `max_examples` properly selects subsets
   - ✅ Examples are formatted consistently with proper JSON/YAML syntax

3. ✅ **Provider Adaptation**
   - ✅ Provider-specific templates optimize for OpenAI vs Ollama strengths
   - ✅ System messages are handled appropriately per provider type

4. ✅ **Format Consistency**
   - ✅ JSON and YAML formatting maintains consistency and proper syntax
   - ✅ Format switching preserves semantic information while adapting syntax

## 2. ✅ COMPLETED: Response Parsing and Validation

### ✅ Implementation Status: CORE FUNCTIONALITY COMPLETE
**Location**: `src/annotation.rs`, `src/resolver.rs`  
**Current State**: JSON/YAML parsing with wrapper format support, integrated with alignment system

The response parsing system robustly extracts structured data from language model outputs, handles various response formats, and integrates with the character alignment system.

### ✅ Completed Core Features

#### ✅ 2.1 Multi-Format Parsing - COMPLETE
- ✅ **JSON Extraction**: Parse JSON from various response formats (fenced, embedded, raw)
- ✅ **Wrapper Format Support**: Handle `{"data": [...]}` and `{"results": [...]}` wrappers
- ✅ **Top-level Array Support**: Parse responses that are arrays vs objects
- ✅ **Nested Structure Support**: Handle complex nested JSON structures

#### ✅ 2.2 Response Processing - COMPLETE  
- ✅ **Individual Item Parsing**: Extract individual extractions from parsed JSON
- ✅ **Character Alignment Integration**: Automatically align parsed extractions to source text
- ✅ **Error Handling**: Graceful handling of parsing failures with fallback to raw text
- ✅ **Debug Support**: Comprehensive logging for troubleshooting parsing issues

#### ✅ 2.3 Advanced Validation - COMPLETE
- ✅ **Schema Validation**: Comprehensive validation with configurable validation rules
- ✅ **Type Coercion**: Automatic conversion of strings to appropriate types (integers, floats, booleans, currency, percentages, emails, phones, URLs, dates)
- ✅ **Required Fields**: Configurable validation ensuring all required extraction classes are present
- ✅ **Data Quality Checks**: Validation of extraction text length, empty content detection, and extraction count quality checks
- ✅ **Raw Data Preservation**: Automatic saving of raw LLM outputs before validation to prevent data loss
- ✅ **Corrected Data Generation**: Automatic creation of corrected JSON with properly typed values

### Implementation Components

```rust
pub struct ValidationConfig {
    pub enable_schema_validation: bool,
    pub enable_type_coercion: bool,
    pub require_all_fields: bool,
    pub save_raw_outputs: bool,
    pub raw_outputs_dir: String,
    pub quality_threshold: f32,
}

pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub corrected_data: Option<serde_json::Value>,
    pub raw_output_file: Option<String>,
    pub coercion_summary: Option<CoercionSummary>,
}

pub struct TypeCoercer {
    // Automatic type coercion with regex-based pattern matching
    // Supports: integers, floats, booleans, currency, percentages,
    // emails, phone numbers, dates, URLs
}
```

### Testing Requirements

#### Natural Language Test Scenarios

1. **Format Detection and Parsing**
   - "Given a response wrapped in ```json fences, when I parse it, then the JSON content should be extracted correctly and the fences should be removed"
   - "Given a response with both JSON and text content, when I parse it, then only the JSON portion should be extracted and parsed"
   - "Given a malformed JSON response with a missing closing brace, when I parse it, then the parser should attempt to fix the JSON and extract what it can"

2. **Data Validation**
   - "Given extracted data that matches the expected schema perfectly, when I validate it, then validation should pass with no errors or warnings"
   - "Given extracted data with a missing required field, when I validate it, then I should get a clear error indicating which field is missing and possibly suggest a fix"
   - "Given extracted data with the right structure but wrong data types (e.g., age as string instead of number), when I validate it, then the validator should either auto-correct or flag the type mismatch"

3. **Error Recovery**
   - "Given a completely malformed response, when parsing fails, then the system should return a helpful error message and possibly fall back to treating the entire response as raw text"
   - "Given a response that's 90% correct but has one syntax error, when I parse it, then the system should extract the valid parts and note what couldn't be parsed"

4. **Complex Structure Handling**
   - "Given a nested JSON response with multiple people and their attributes, when I parse it, then each person's data should be properly extracted into separate Extraction objects with appropriate naming"
   - "Given a response that uses different naming conventions than expected, when I parse it, then the system should attempt to map similar field names (e.g., 'name' vs 'person_name')"

## 3. ✅ COMPLETED: Text Chunking and Processing Pipelines

### ✅ Implementation Status: CORE FUNCTIONALITY COMPLETE
**Location**: `src/chunking.rs`, integrated with `src/annotation.rs`  
**Current State**: Full chunking system with multiple strategies, parallel processing, and result aggregation

The text processing system handles documents of any size by intelligently splitting them into manageable chunks while preserving context and ensuring comprehensive extraction coverage.

### ✅ Completed Core Features

#### ✅ 3.1 Intelligent Chunking - COMPLETE
- ✅ **Size-Based Chunking**: Split text based on character limits with configurable thresholds
- ✅ **Semantic Chunking**: Split at natural boundaries (sentences, paragraphs, sections)
- ✅ **Overlap Management**: Configurable overlap between chunks to prevent missed extractions
- ✅ **Adaptive Chunking**: Context-aware chunking that balances size and semantic boundaries

#### ✅ 3.2 Chunk Processing - COMPLETE
- ✅ **Parallel Processing**: Process multiple chunks concurrently using async/await
- ✅ **Character Alignment**: Each chunk maintains proper character offsets to original document
- ✅ **Progress Tracking**: Debug output showing chunk processing progress
- ✅ **Error Isolation**: Handle chunk processing failures without affecting other chunks

#### ✅ 3.3 Result Aggregation - COMPLETE
- ✅ **Deduplication**: Remove duplicate extractions across chunks using text similarity
- ✅ **Position Mapping**: Maintain character positions relative to original document
- ✅ **Overlap Merging**: Intelligent merging of extractions from overlapping chunks
- ✅ **Quality Preservation**: Preserve highest quality extractions during deduplication

#### 🚧 3.4 Multi-Pass Processing (FUTURE ENHANCEMENT)
- **Sequential Passes**: Support multiple extraction passes for improved recall
- **Targeted Re-processing**: Re-process chunks that had low extraction counts
- **Refinement Passes**: Use initial results to refine subsequent extractions

### Implementation Components

```rust
pub struct TextChunker {
    pub max_chunk_size: usize,
    pub overlap_size: usize,
    pub chunking_strategy: ChunkingStrategy,
    pub boundary_rules: Vec<BoundaryRule>,
}

pub enum ChunkingStrategy {
    FixedSize,
    Sentence,
    Paragraph,
    Semantic,
    Adaptive,
}

pub struct ProcessingPipeline {
    pub chunker: TextChunker,
    pub processor: ChunkProcessor,
    pub aggregator: ResultAggregator,
    pub max_workers: usize,
}

pub struct ChunkResult {
    pub chunk_id: usize,
    pub extractions: Vec<Extraction>,
    pub chunk_text: String,
    pub char_offset: usize,
    pub processing_time: Duration,
    pub success: bool,
    pub error: Option<ProcessingError>,
}
```

### Testing Requirements

#### Natural Language Test Scenarios

1. **Chunking Strategy Validation**
   - "Given a 10,000 character document with a 2,000 character limit, when I chunk it, then I should get approximately 5 chunks with proper overlap to ensure no information is lost at boundaries"
   - "Given a document with clear paragraph breaks, when I use semantic chunking, then chunks should break at paragraph boundaries rather than mid-sentence, even if it means slightly uneven chunk sizes"
   - "Given a document where an important entity spans across a natural chunk boundary, when I use overlapping chunks, then that entity should be captured in both chunks for conflict resolution"

2. **Parallel Processing**
   - "Given a document that produces 10 chunks and 4 worker threads, when I process it, then chunks should be distributed efficiently across workers and complete in roughly 1/4 the time of sequential processing"
   - "Given a scenario where one chunk fails to process due to an API error, when processing continues, then the other chunks should complete successfully and the final result should indicate which chunk failed"
   - "Given a very long document, when processing begins, then I should see regular progress updates showing how many chunks have been completed"

3. **Result Aggregation and Deduplication**
   - "Given overlapping chunks that both extract the same person's name, when results are aggregated, then the duplicate should be removed and the extraction should appear only once in the final results"
   - "Given two chunks that extract slightly different versions of the same information (e.g., 'John Smith' vs 'Dr. John Smith'), when aggregating results, then the system should intelligently choose the more complete version"
   - "Given extractions from multiple chunks, when aggregating results, then character positions should be correctly mapped back to the original document coordinates"

4. **Multi-Pass Processing**
   - "Given a document processed with 2 extraction passes, when the second pass finds additional entities that the first pass missed, then the final result should include extractions from both passes without duplicates"
   - "Given a chunk that initially produced zero extractions, when configured for targeted re-processing, then that chunk should be processed again with potentially different parameters or strategies"

5. **Memory and Performance**
   - "Given a 100MB document, when processing with reasonable chunk sizes, then the system should maintain stable memory usage and not load the entire document into memory at once"
   - "Given processing configuration with batch_length=10 and max_workers=5, when processing begins, then the system should never have more than 10 chunks in memory at once and should efficiently utilize all 5 workers"

## Integration Requirements

## 4. ✅ COMPLETED: Character Alignment System

### ✅ Implementation Status: COMPLETE
**Location**: `src/alignment.rs`, integrated with `src/annotation.rs`  
**Test Coverage**: 6 comprehensive tests covering all alignment scenarios
**Current State**: Full character alignment system with exact and fuzzy matching

The character alignment system maps extracted entities back to their precise positions in the source text, enabling highlighting, context visualization, and data provenance tracking.

### ✅ Completed Core Features

#### ✅ 4.1 Alignment Algorithms - COMPLETE
- ✅ **Exact Text Matching**: Find precise character positions for extracted text in source
- ✅ **Fuzzy Text Matching**: Intelligent similarity-based matching for partial matches
- ✅ **Case-Insensitive Matching**: Configurable case sensitivity for flexible alignment
- ✅ **Multi-word Alignment**: Handle extraction text that spans multiple words in source

#### ✅ 4.2 Position Mapping - COMPLETE
- ✅ **Character Intervals**: Precise start/end positions for each extraction
- ✅ **Chunk Offset Handling**: Proper position mapping for chunked document processing
- ✅ **Overlap Resolution**: Handle extractions that span multiple chunks
- ✅ **Alignment Status**: Track alignment quality (exact, fuzzy, lesser, greater, none)

#### ✅ 4.3 Configuration and Tuning - COMPLETE
- ✅ **Configurable Thresholds**: Adjustable fuzzy matching sensitivity (default 0.4)
- ✅ **Search Window Limits**: Prevent expensive searches in very large texts
- ✅ **Match Acceptance Policies**: Control whether to accept partial matches
- ✅ **Similarity Algorithms**: Coverage-based similarity for better fuzzy matching

#### ✅ 4.4 Integration and Statistics - COMPLETE
- ✅ **Annotation Pipeline Integration**: Automatic alignment during extraction
- ✅ **Chunking Integration**: Proper alignment for large document processing
- ✅ **Alignment Statistics**: Comprehensive metrics on alignment success rates
- ✅ **Debug Visualization**: Context display showing aligned text positions

### ✅ Implementation Components

```rust
pub struct TextAligner {
    config: AlignmentConfig,
}

pub struct AlignmentConfig {
    pub enable_fuzzy_alignment: bool,
    pub fuzzy_alignment_threshold: f32,
    pub accept_match_lesser: bool,
    pub case_sensitive: bool,
    pub max_search_window: usize,
}

pub struct AlignmentStats {
    pub total: usize,
    pub exact: usize,
    pub fuzzy: usize,
    pub lesser: usize,
    pub greater: usize,
    pub unaligned: usize,
}
```

### ✅ Testing Status: COMPLETE (6 tests passing)
- ✅ **Exact Alignment**: Perfect text matches with correct character positions
- ✅ **Case Insensitive**: Proper handling of case differences 
- ✅ **Fuzzy Alignment**: Similarity-based matching for separated words
- ✅ **No Alignment**: Correct handling of text not found in source
- ✅ **Chunk Offsets**: Proper position mapping with character offsets
- ✅ **Statistics**: Accurate calculation of alignment success metrics

## 5. ✅ COMPLETED: Provider System and Core Infrastructure

### ✅ Implementation Status: COMPLETE
**Location**: `src/providers/`, `src/factory.rs`, `src/inference.rs`  
**Test Coverage**: Comprehensive testing across all provider types

### ✅ Completed Features
- ✅ **Universal Provider Architecture**: Single provider implementation handling all types
- ✅ **Provider Auto-Detection**: Automatic provider type detection from model names
- ✅ **Configuration System**: Complete `ProviderConfig` and `ExtractConfig` system
- ✅ **OpenAI Integration**: Framework ready with async-openai support
- ✅ **Ollama Integration**: Basic HTTP client implementation for local models
- ✅ **Custom Provider Support**: Flexible configuration for any HTTP API
- ✅ **Error Handling**: Comprehensive error types and conversion

## NEXT DEVELOPMENT PRIORITIES

Based on the current implementation state, the following should be prioritized:

### Priority 1: Multi-Pass Extraction
1. **Sequential Processing Passes** (`src/annotation.rs`)
   - Multiple extraction rounds for better recall
   - Targeted re-processing of low-yield chunks
   - Refinement passes using initial results

2. **Extraction Quality Enhancement**
   - Confidence scoring for extractions
   - Quality-based filtering and ranking
   - Validation against full document context

### ✅ Priority 2: Advanced Validation System - COMPLETED (`src/resolver.rs`)
- ✅ **Schema Validation**: Comprehensive validation system with configurable rules
- ✅ **Type Coercion**: Advanced automatic type conversion system with regex pattern matching
- ✅ **Required Fields**: Configurable validation ensuring all required extraction classes are present
- ✅ **Data Quality Checks**: Complete validation including text length, empty content, and extraction quality
- ✅ **Raw Data Preservation**: Automatic saving of raw LLM outputs with timestamped filenames
- ✅ **Corrected Data Generation**: JSON output with properly coerced data types

### Priority 3: Rich Visualization System (`src/visualization.rs`)
- **HTML Export**: Rich HTML output with highlighting and interactive features
- **Markdown Export**: Structured markdown with extraction summaries
- **JSON/CSV Export**: Data export in various formats for analysis
- **Interactive Highlighting**: Visual representation of extractions in source text

### Priority 4: Enhanced Provider Support (`src/providers/universal.rs`)
- **Azure OpenAI**: Complete Azure-specific configuration and endpoints
- **Custom Providers**: Expand support for arbitrary HTTP APIs
- **Streaming Support**: Implement streaming for better UX with long requests
- **Rate Limiting**: Built-in rate limiting and retry logic

### Integration Requirements
- ✅ **Template-Provider Integration**: Complete - templates generate provider-optimized prompts
- ✅ **Template-Parser Integration**: Complete - parsing integrated with prompt responses
- ✅ **Chunking-Template Integration**: Complete - chunking works with full pipeline
- ✅ **Parser-Alignment Integration**: Complete - parsed extractions automatically aligned
- ✅ **Provider-Factory Integration**: Complete and tested
- ✅ **Alignment-Visualization Integration**: Complete - aligned positions displayed

### Quality Assurance Status
- ✅ **Unit Testing**: 53+ tests passing, excellent coverage of completed features
- ✅ **Integration Testing**: Core pipeline components fully integrated
- ✅ **End-to-End Testing**: Complete extraction pipeline working
- ✅ **Provider Compatibility**: OpenAI and Ollama providers working, Azure support ready
