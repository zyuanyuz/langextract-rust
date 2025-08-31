# Basic Usage Example

This example demonstrates the fundamental capabilities of LangExtract for extracting structured information from text using the CLI interface.

## What This Example Does

Extracts **person names**, **ages**, **professions**, and **workplaces** from biographical text using example-guided extraction.

## Files

- **`examples.json`** - Training examples that teach the LLM what to extract
- **`config.yaml`** - Configuration parameters (model, provider, processing options)
- **`input.txt`** - Sample text to process
- **`run.sh`** - Executable script that runs the extraction
- **`output/`** - Generated results (created when you run the example)

## Quick Start

```bash
# Ensure you have a provider running (e.g., Ollama)
ollama serve
ollama pull mistral

# Run the example
./run.sh
```

## Alternative Providers

### OpenAI
```bash
export OPENAI_API_KEY="your-api-key"
# Edit run.sh to use --provider openai --model gpt-4o-mini
```

### Custom Provider
```bash
# Edit run.sh to use --provider custom --model-url http://your-api-endpoint
```

## Understanding the Output

The script generates multiple output formats:

1. **`results.json`** - Structured data with character positions
2. **`langextract_results.html`** - Interactive visualization showing extracted text highlighted in context
3. **Console output** - Human-readable format

## Customization

### Modify What Gets Extracted

Edit `examples.json` to change the extraction targets:

```json
{
  "text": "Example sentence with the data you want to extract",
  "extractions": [
    {
      "extraction_class": "your_category",
      "extraction_text": "the specific text to extract"
    }
  ]
}
```

### Change Processing Options

Edit `config.yaml` or modify the `run.sh` script parameters:

- `--temperature 0.1` - More deterministic (0.0-1.0)
- `--workers 8` - More parallel processing
- `--multipass` - Enable multiple extraction rounds
- `--max-chars 4000` - Adjust chunk size for large documents

### Different Input

Replace the content in `input.txt` with your own text, or modify `run.sh` to process files/URLs:

```bash
# Process a file
$CLI_CMD extract /path/to/your/file.txt --examples examples.json ...

# Process a URL
$CLI_CMD extract "https://example.com/article" --examples examples.json ...

# Process direct text
$CLI_CMD extract "Your text here" --examples examples.json ...
```

## Expected Results

From the sample input text, you should see extractions like:

- **person**: "Alice Smith", "Bob Wilson", "Professor Maria Garcia"
- **age**: "28", "42", "55"  
- **profession**: "data scientist", "mechanical engineer", "computer science"
- **workplace**: "Google", "Tesla", "Stanford University"

## Troubleshooting

### "Command not found: lx-rs"
Install the CLI:
```bash
cargo install langextract-rust --features cli
```

Or the script will fall back to `cargo run --features=cli` (slower).

### "Provider connection failed"
- **Ollama**: Ensure `ollama serve` is running and model is pulled
- **OpenAI**: Check your API key is set correctly
- **Custom**: Verify the endpoint URL and API compatibility

### "No extractions found"
- Check that your examples.json matches the type of data in your input
- Try lowering the temperature for more consistent results
- Ensure the prompt clearly describes what to extract

## Next Steps

- Try the **alignment_demo** example to see character-level positioning
- Check out **validation_demo** for type coercion and data validation
- Explore **advanced_examples** for chunking and multipass extraction
