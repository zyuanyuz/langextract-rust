# Validation Demo

This example demonstrates LangExtract's **data validation and type coercion** capabilities, showing how extracted information can be validated, typed, and cleaned automatically.

## What This Example Does

Extracts various data types (**prices**, **percentages**, **weights**, **emails**, **phone numbers**, **product codes**) and demonstrates:

- 🔬 **Type validation** - Ensuring extracted data matches expected formats
- 💱 **Type coercion** - Converting strings to appropriate data types  
- ✅ **Data quality assurance** - Validation rules and error detection
- 📋 **Schema constraints** - Enforcing data structure requirements

## Key Features Demonstrated

- **Currency handling**: $149.99 → 149.99 (USD)
- **Percentage conversion**: 95% → 0.95 (decimal)
- **Email validation**: user@domain.com format checking
- **Phone formatting**: (555) 123-4567 standardization
- **Measurement parsing**: 2.5 kg → value + unit extraction
- **Product code validation**: ABC-123-X format verification

## Files

- **`examples.json`** - Training examples with validation attributes and type constraints
- **`config.yaml`** - Configuration with validation features enabled
- **`input.txt`** - Sample product/technical specification text
- **`run.sh`** - Script demonstrating validation features
- **`output/`** - Generated results with validation metadata

## Quick Start

```bash
# Ensure you have a provider running
ollama serve
ollama pull mistral

# Run the validation demo
./run.sh
```

## Understanding Validation Attributes

The examples include validation metadata:

```json
{
  "extraction_class": "price",
  "extraction_text": "$29.99",
  "attributes": {
    "value_type": ["currency", "dollars", "cents"],
    "format": "currency_with_symbol",
    "currency": "USD",
    "decimal_places": 2
  }
}
```

This tells the system:
- Expect currency format with dollar symbol
- Validate as USD currency
- Ensure exactly 2 decimal places
- Type-coerce to numeric value

## Validation Rules Examples

### Currency Validation
- **Input**: "$149.99", "$12.50", "199.99"
- **Validation**: Must start with $ symbol, have 2 decimal places
- **Coercion**: Convert to numeric values (149.99, 12.50, 199.99)

### Email Validation  
- **Input**: "support@techcorp.com", "help@company.com"
- **Validation**: Must contain @ symbol and valid domain
- **Coercion**: Extract domain part, validate format

### Percentage Validation
- **Input**: "95%", "25%", "8.5%"
- **Validation**: Must end with % symbol, be 0-100
- **Coercion**: Convert to decimal (0.95, 0.25, 0.085)

### Phone Number Validation
- **Input**: "(800) 555-0199", "(555) 123-4567"
- **Validation**: US phone format with parentheses
- **Coercion**: Extract digits only (8005550199)

## What to Look For

### 1. Type Coercion Results
The system automatically converts extracted text:
```bash
"$149.99" → 149.99 (number)
"95%" → 0.95 (decimal)  
"2.5 kg" → {value: 2.5, unit: "kg"}
```

### 2. Validation Errors
Look for validation warnings in the output:
- **Format mismatches**: Email without @ symbol
- **Range violations**: Percentage > 100%
- **Type inconsistencies**: Non-numeric weights

### 3. Data Quality Metrics
The HTML report shows:
- **Validation success rate**: % of extractions passing validation
- **Type distribution**: Breakdown by data type
- **Error summary**: Common validation failures

## Advanced Validation Features

### Custom Validation Rules
Add validation attributes to examples:

```json
{
  "extraction_class": "product_code",
  "extraction_text": "ABC-123-X",
  "attributes": {
    "format": "uppercase_with_hyphens",
    "max_length": 10,
    "validation": "alphanumeric_with_hyphens",
    "required": true
  }
}
```

### Numeric Range Validation
```json
{
  "extraction_class": "efficiency", 
  "extraction_text": "95%",
  "attributes": {
    "value_type": ["percentage"],
    "min_value": 0,
    "max_value": 100,
    "format": "percentage_with_symbol"
  }
}
```

### Enum Value Validation
```json
{
  "extraction_class": "status",
  "extraction_text": "in stock", 
  "attributes": {
    "enum_values": ["in stock", "out of stock", "backorder"],
    "format": "lowercase"
  }
}
```

## Configuration Options

### Enable Advanced Validation
```yaml
enable_validation: true           # Turn on validation system
enable_type_coercion: true       # Convert types automatically
save_raw_output: true            # Keep original LLM responses
validate_required_fields: true   # Check for missing required data
```

### Validation Strictness
```yaml
temperature: 0.2                 # Lower = more consistent formatting
validation_mode: "strict"        # Reject invalid extractions
coercion_warnings: true          # Show type conversion issues
```

## Troubleshooting

### High Validation Error Rate
- **Improve examples**: Add more validation training examples
- **Lower temperature**: Use 0.1-0.2 for more consistent output
- **Clearer prompts**: Be more specific about expected formats

### Type Coercion Failures
- **Check input format**: Ensure examples match actual data patterns
- **Add format variants**: Include multiple format examples (e.g., with/without currency symbols)
- **Validate training data**: Ensure examples have correct validation attributes

### Missing Validation Metadata
- **Add attributes**: Include validation rules in examples.json
- **Enable debug mode**: Use `--debug` to see validation process
- **Check raw outputs**: Examine raw_outputs/ directory for model responses

## Real-World Applications

### E-commerce Data
- **Product information**: SKUs, prices, descriptions
- **Customer data**: Emails, phones, addresses  
- **Inventory details**: Quantities, weights, dimensions

### Financial Documents
- **Transaction data**: Amounts, dates, account numbers
- **Currency conversion**: Multi-currency validation
- **Regulatory compliance**: Format standardization

### Technical Specifications
- **Measurements**: Weights, dimensions, capacities
- **Performance metrics**: Speeds, efficiencies, ratings
- **Contact information**: Support details, documentation

## Next Steps

- Try **advanced_examples** for validation with chunking and large documents
- Explore **product_catalog** example for e-commerce validation scenarios  
- Create custom validation rules for your specific data types
- Test with real-world data to tune validation accuracy
