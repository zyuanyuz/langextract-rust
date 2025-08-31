#!/bin/bash
# Validation Demo - Demonstrates data validation and type coercion

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${CYAN}🔬 LangExtract Validation Demo${NC}"
echo "Demonstrating data validation, type coercion, and quality assurance"
echo

# Check if lx-rs is available
if command -v lx-rs &> /dev/null; then
    CLI_CMD="lx-rs"
    echo -e "${GREEN}✅ Using installed lx-rs binary${NC}"
elif [ -f "../../target/release/lx-rs" ]; then
    CLI_CMD="../../target/release/lx-rs"
    echo -e "${GREEN}✅ Using local release binary${NC}"
elif [ -f "../../../target/release/lx-rs" ]; then
    CLI_CMD="../../../target/release/lx-rs"
    echo -e "${GREEN}✅ Using local release binary${NC}"
else
    CLI_CMD="cargo run --features=cli --bin lx-rs --"
    echo -e "${YELLOW}⚠️  Using cargo run (slower, but works without install)${NC}"
fi

echo

# Show the input text
echo -e "${BLUE}📄 Input Text with Various Data Types:${NC}"
echo "┌─────────────────────────────────────────────────────────────────────────────────┐"
cat "$SCRIPT_DIR/input.txt"
echo
echo "└─────────────────────────────────────────────────────────────────────────────────┘"
echo

# Show the validation examples
echo -e "${BLUE}📚 Validation Training Examples:${NC}"
echo "These examples include type constraints and validation rules:"
echo

if command -v jq &> /dev/null; then
    jq -r '.[] | 
    "Input: \(.text)\n" +
    "Extractions with validation:\n" +
    (.extractions | map(
        "  • \(.extraction_class): \"\(.extraction_text)\"" +
        (if .attributes then
            "\n    Type: \(.attributes.value_type // []) " +
            "Format: \(.attributes.format // "any")" +
            (if .attributes.validation then " Validation: \(.attributes.validation)" else "" end)
        else "" end)
    ) | join("\n")) + "\n"
    ' "$SCRIPT_DIR/examples.json" 2>/dev/null
else
    cat "$SCRIPT_DIR/examples.json"
fi

echo

# Create output directory
mkdir -p "$SCRIPT_DIR/output"

# Run the extraction with validation features
echo -e "${BLUE}🔄 Running Extraction with Validation...${NC}"
echo

echo "📊 Extracting with validation and type coercion enabled:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract product codes, prices, percentages, weights, contact information (emails, phones), times, URLs, dimensions, and technical specifications with proper data types and validation" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/validation_results.json" \
    --format text \
    --show-intervals \
    --temperature 0.3 \
    --debug

echo
echo "💾 Saving detailed JSON with validation information:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract product codes, prices, percentages, weights, contact information (emails, phones), times, URLs, dimensions, and technical specifications with proper data types and validation" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/validation_results.json" \
    --format json \
    --show-intervals \
    --temperature 0.3

echo
echo "🎨 Creating validation report with HTML visualization:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract product codes, prices, percentages, weights, contact information (emails, phones), times, URLs, dimensions, and technical specifications with proper data types and validation" \
    --provider ollama \
    --model mistral \
    --export html \
    --show-intervals \
    --temperature 0.3

echo
echo -e "${BLUE}🔍 Analyzing Validation Results:${NC}"

if [ -f "$SCRIPT_DIR/output/validation_results.json" ]; then
    echo "Extraction results with validation analysis:"
    echo
    
    # Analyze the validation results
    if command -v jq &> /dev/null; then
        echo -e "${CYAN}📋 Extracted Data by Type:${NC}"
        echo
        
        # Group extractions by type and analyze patterns
        jq -r '
        if .extractions then
            (.extractions | group_by(.extraction_class) | .[] | 
            "🔹 " + (.[0].extraction_class | ascii_upcase) + " (" + (length | tostring) + " items):" +
            "\n" +
            (map("   • \"" + .extraction_text + "\"" + 
                (if .char_interval then " [pos: " + (.char_interval.start_pos // "?" | tostring) + "-" + (.char_interval.end_pos // "?" | tostring) + "]" else "" end)
            ) | join("\n")) + "\n")
        else
            "No extractions found"
        end
        ' "$SCRIPT_DIR/output/validation_results.json"
        
        echo
        echo -e "${CYAN}💡 Type Coercion Examples:${NC}"
        echo "The following shows how raw extracted text can be converted to proper data types:"
        echo
        
        # Show examples of type coercion
        jq -r '
        if .extractions then
            .extractions[] | 
            select(.extraction_class | test("price|percentage|weight|phone|email")) |
            "• " + .extraction_class + ": \"" + .extraction_text + "\"" +
            (if .extraction_class == "price" then " → Numeric: " + (.extraction_text | gsub("[$,]"; "") | tonumber | tostring) + " USD"
            elif .extraction_class == "percentage" then " → Decimal: " + (.extraction_text | gsub("%"; "") | tonumber / 100 | tostring)
            elif .extraction_class == "weight" then " → Measurement: " + (.extraction_text | split(" ")[0]) + " in " + (.extraction_text | split(" ")[1])
            elif .extraction_class == "phone" then " → Format: " + (.extraction_text | gsub("[^0-9]"; ""))
            elif .extraction_class == "email" then " → Domain: " + (.extraction_text | split("@")[1])
            else ""
            end)
        else
            "No type coercion examples available"
        end
        ' "$SCRIPT_DIR/output/validation_results.json" 2>/dev/null || echo "Error analyzing type coercion"
        
    else
        echo "Raw JSON results:"
        cat "$SCRIPT_DIR/output/validation_results.json"
    fi
    
    echo
    echo -e "${CYAN}✅ Validation Quality Indicators:${NC}"
    echo "Look for these validation features in the results:"
    echo "   📍 Character alignment accuracy"
    echo "   💱 Currency format consistency ($XX.XX)"
    echo "   📧 Email format validation (user@domain.com)"
    echo "   📞 Phone number format consistency"
    echo "   📊 Percentage format (XX%)"
    echo "   ⚖️  Weight/measurement units (XX kg, XX GB, etc.)"
    echo "   🆔 Product code format consistency"
    echo
fi

# Check for raw output files (if validation system saves them)
if [ -d "raw_outputs" ]; then
    echo -e "${BLUE}🔍 Raw Output Analysis:${NC}"
    echo "Raw LLM outputs saved to raw_outputs/ directory for debugging"
    echo "Latest raw output files:"
    ls -la raw_outputs/ | tail -3
    echo
fi

echo -e "${GREEN}✅ Validation Demo Complete!${NC}"
echo
echo "Generated files:"
echo "  • $SCRIPT_DIR/output/validation_results.json - Results with validation metadata"
echo "  • langextract_results.html - Interactive visualization"
echo "  • raw_outputs/ - Raw LLM responses (if saved)"
echo
echo "🔬 Validation Features Demonstrated:"
echo "  1. Type-aware extraction (currency, percentage, weight, etc.)"
echo "  2. Format validation (emails, phones, URLs)"
echo "  3. Data consistency checking"
echo "  4. Type coercion capabilities"
echo "  5. Raw output preservation for debugging"
echo
echo "🧪 Try This:"
echo "  • Modify input.txt with invalid data (bad email, malformed phone)"
echo "  • Add new validation rules to examples.json"
echo "  • Compare extraction accuracy with/without validation examples"
echo "  • Check raw_outputs/ to see original LLM responses"
echo
echo "🔧 Advanced Validation:"
echo "  • Use attributes in examples.json to define validation rules"
echo "  • Set min/max values for numeric data"
echo "  • Define format patterns for structured data"
echo "  • Enable strict validation mode for production use"
