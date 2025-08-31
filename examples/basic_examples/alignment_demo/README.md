# Alignment Demo

This example demonstrates LangExtract's **character-level text alignment** capabilities, showing how extracted information is precisely positioned within the original text.

## What This Example Does

Extracts various entities (**dates**, **times**, **locations**, **emails**, **phone numbers**, **prices**, **URLs**) and shows their exact character positions in the source text.

## Key Features Demonstrated

- 🎯 **Character-level positioning** - Exact start/end positions for each extraction
- 📍 **Alignment algorithms** - How LangExtract maps extracted text back to source
- 🔍 **Position accuracy** - Verification that extracted text matches source positions
- 🎨 **Visual highlighting** - HTML output showing extractions highlighted in context

## Files

- **`examples.json`** - Training examples with contact/event information
- **`config.yaml`** - Configuration optimized for alignment accuracy (low temperature)
- **`input.txt`** - Sample conference announcement with rich entity types
- **`run.sh`** - Script demonstrating alignment features
- **`output/`** - Generated results with character positions

## Quick Start

```bash
# Ensure you have a provider running
ollama serve
ollama pull mistral

# Run the alignment demo
./run.sh
```

## Understanding Character Intervals

The output shows character positions for each extraction:

```json
{
  "extraction_class": "date",
  "extraction_text": "April 22, 2024",
  "char_interval": {
    "start_pos": 45,
    "end_pos": 59
  }
}
```

This means:
- The text "April 22, 2024" was found at characters 45-58 (end_pos is exclusive)
- You can verify: `input_text[45:59]` equals "April 22, 2024"

## What to Look For

### 1. Position Accuracy
Check that extracted text exactly matches the source at the given positions:
- **dates**: "April 22, 2024" should align perfectly
- **emails**: "m.chen@mit.edu" should have correct boundaries  
- **phones**: "(617) 555-0123" should include parentheses and formatting
- **prices**: "$250.00" should include the dollar sign

### 2. Alignment Status
The system reports alignment quality:
- **MatchExact**: Perfect character-level match
- **MatchFuzzy**: Close match with minor differences
- **MatchGreater/Lesser**: Length differences between extracted and source text

### 3. Complex Entity Handling
See how the system handles:
- **Multi-word entities**: "Convention Center in Austin, Texas"
- **Formatted data**: Phone numbers with parentheses and dashes
- **URLs**: Complete web addresses with protocols
- **Monetary values**: Currency symbols and decimal places

## Configuration for Alignment

The example uses settings optimized for alignment accuracy:

```yaml
temperature: 0.2      # Lower = more consistent positioning
max_workers: 2        # Fewer workers = more consistent results
show_intervals: true  # Always display character positions
```

## Troubleshooting Alignment Issues

### Poor Alignment Accuracy
- **Lower temperature**: Try 0.1 for more deterministic results
- **Simpler examples**: Use more precise training examples
- **Shorter text**: Long documents can have cumulative alignment errors

### Missing Character Intervals
- **Check model output**: Some models may not preserve exact text
- **Verify examples**: Ensure training examples show desired precision
- **Enable debug**: Use `--debug` to see alignment process details

### Misaligned Positions
- **Unicode handling**: Special characters can affect positioning
- **Whitespace**: Extra spaces in model output vs. source text
- **Fuzzy matching**: System falls back to approximate matching

## Advanced Usage

### Custom Alignment Examples
Edit `examples.json` to train for specific entity types:

```json
{
  "text": "The meeting is scheduled for 2:30 PM on January 15th",
  "extractions": [
    {
      "extraction_class": "time", 
      "extraction_text": "2:30 PM"
    },
    {
      "extraction_class": "date",
      "extraction_text": "January 15th" 
    }
  ]
}
```

### Different Input Types
Test alignment with various text types:
- **Structured documents**: Forms, tables, lists
- **Natural language**: Narratives, descriptions
- **Mixed content**: Code snippets, formatted text
- **International text**: Unicode characters, different languages

### Visualization Analysis
The HTML output provides:
- **Color-coded highlighting** for different entity types
- **Hover tooltips** showing exact character positions
- **Statistics panel** with alignment accuracy metrics
- **Side-by-side comparison** of extracted vs. source text

## Next Steps

- Try **validation_demo** to see how alignment works with type validation
- Explore **advanced_examples** for alignment with large documents and chunking
- Test with your own text to see alignment accuracy for your use case
- Experiment with different models to compare alignment quality
