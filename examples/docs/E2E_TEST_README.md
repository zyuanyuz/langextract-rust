# 🚀 LangExtract End-to-End Provider Testing

This comprehensive test demonstrates the complete LangExtract pipeline with real language model providers, showcasing extraction, validation, type coercion, and rich visualization.

## 🎯 What It Tests

- **Multiple Provider Support**: OpenAI, Ollama (Mistral, Llama3.1), and custom providers
- **Complete Extraction Pipeline**: From raw text to structured, typed data
- **Advanced Validation System**: With type coercion (strings → numbers, percentages, emails, etc.)
- **Rich Visualization**: HTML, JSON, CSV, and Markdown exports
- **Real-World Data**: Business press release with entities, financials, and contact info

## 🛠️ Setup Instructions

### OpenAI Testing
```bash
export OPENAI_API_KEY=your_openai_api_key_here
```

### Ollama Testing (Local)
```bash
# Start Ollama server
ollama serve

# Pull required models
ollama pull mistral
ollama pull llama3.1

# Optional: Set custom Ollama URL
export OLLAMA_BASE_URL=http://localhost:11434
```

### Custom Provider Testing
```bash
export CUSTOM_LLM_URL=https://your-custom-api.com
export CUSTOM_LLM_KEY=your_custom_api_key
```

## 🏃‍♂️ Running the Test

```bash
# Run the comprehensive end-to-end test
cargo run --example e2e_provider_test

# Or run with additional logging
RUST_LOG=debug cargo run --example e2e_provider_test
```

## 📊 Generated Outputs

The test generates timestamped files for each provider:

- **`e2e_test_openai_gpt4omini_YYYYMMDD_HHMMSS.html`** - Interactive HTML with highlighting
- **`e2e_test_ollama_mistral_YYYYMMDD_HHMMSS.json`** - Structured JSON data
- **`e2e_test_openai_gpt35turbo_YYYYMMDD_HHMMSS.csv`** - CSV for spreadsheet analysis

### HTML Features
- 🎨 **Interactive highlighting** of extracted entities in source text
- 📊 **Statistics dashboard** with extraction counts and class breakdowns
- 🎯 **Extraction cards** with detailed information and character positions
- 💅 **Modern UI** with provider-specific color coding

### JSON Features
- 🔧 **Machine-readable** structured data
- 📍 **Character positions** for precise alignment
- 📈 **Validation metadata** and quality metrics
- 🔄 **Type coercion results** with conversion details

### CSV Features
- 📋 **Spreadsheet-compatible** format
- 🔍 **Filterable columns** for analysis
- 📊 **Easy data manipulation** and reporting

## 🧪 Test Data

The test uses a realistic business press release containing:

- **👥 People**: Dr. Sarah Johnson, Ms. Johnson
- **🏢 Companies**: TechCorp Inc., TechCorp
- **💰 Financial Data**: $2.3 million revenue, $87.50 stock price, 15% growth
- **📞 Contact Info**: email addresses, phone numbers
- **📍 Locations & Dates**: San Francisco CA, October 15 2024, Q3 2024

## 🎯 Expected Results

A successful test should extract **15-25 entities** including:
- Person names and titles
- Company names
- Financial figures (revenue, stock prices, percentages)
- Contact information (emails, phone numbers)
- Locations and dates

## 🔧 Troubleshooting

### No Providers Available
```
⚠️  No provider configurations available!
```
**Solution**: Set up at least one provider using the environment variables above.

### Ollama Connection Failed
```
⚠️  Ollama server not available at http://localhost:11434
```
**Solution**: 
1. Start Ollama: `ollama serve`
2. Pull models: `ollama pull mistral`
3. Check server status: `curl http://localhost:11434/api/tags`

### OpenAI API Errors
```
❌ OpenAI extraction failed: API key invalid
```
**Solution**: 
1. Verify your API key: `echo $OPENAI_API_KEY`
2. Check your OpenAI account has sufficient credits
3. Ensure the API key has correct permissions

## 📈 Performance Benchmarks

Typical performance on modern hardware:

- **OpenAI GPT-4o-mini**: ~3-5 seconds, 20-25 extractions
- **OpenAI GPT-3.5-turbo**: ~2-4 seconds, 15-20 extractions  
- **Ollama Mistral**: ~10-15 seconds, 18-22 extractions
- **Ollama Llama3.1**: ~8-12 seconds, 20-25 extractions

## 🎉 Success Indicators

✅ **Complete Success**: All providers tested, rich visualizations generated, type coercion working

✅ **Partial Success**: Some providers working, demonstrating system flexibility

✅ **Feature Showcase**: Even single provider demonstrates full pipeline capabilities

---

This test showcases LangExtract's production-ready capabilities for enterprise text extraction workflows! 🚀
