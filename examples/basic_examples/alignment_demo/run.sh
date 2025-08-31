#!/bin/bash
# Alignment Demo - Demonstrates character-level text positioning

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${PURPLE}🎯 LangExtract Alignment Demo${NC}"
echo "Demonstrating character-level text positioning and intervals"
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

# Show the input text with character positions
echo -e "${BLUE}📄 Input Text with Character Positions:${NC}"
echo "┌─────────────────────────────────────────────────────────────────────────────────┐"

# Read the input file and show it with position markers
input_text=$(cat "$SCRIPT_DIR/input.txt")
echo "$input_text" | cat -n
echo "└─────────────────────────────────────────────────────────────────────────────────┘"
echo

# Show character count
char_count=$(echo -n "$input_text" | wc -c)
echo "Total characters: $char_count"
echo

# Show the examples being used for alignment training
echo -e "${BLUE}📚 Alignment Training Examples:${NC}"
echo "These examples teach the model to identify and position text accurately:"
jq -r '.[] | "Input: \(.text)\nExtractions: \(.extractions | map("  - \(.extraction_class): \"\(.extraction_text)\"") | join("\n"))\n"' "$SCRIPT_DIR/examples.json" 2>/dev/null || cat "$SCRIPT_DIR/examples.json"
echo

# Create output directory
mkdir -p "$SCRIPT_DIR/output"

# Run the extraction with detailed character positioning
echo -e "${BLUE}🔄 Running Extraction with Character Alignment...${NC}"
echo

echo "📊 Extracting with character intervals enabled:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract dates, times, locations, person names, email addresses, phone numbers, prices, and URLs with precise character positioning" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/alignment_results.json" \
    --format text \
    --show-intervals \
    --temperature 0.2 \
    --debug

echo
echo "💾 Saving detailed JSON with character intervals:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract dates, times, locations, person names, email addresses, phone numbers, prices, and URLs with precise character positioning" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/alignment_results.json" \
    --format json \
    --show-intervals \
    --temperature 0.2

echo
echo "🎨 Creating HTML visualization with highlighted positions:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract dates, times, locations, person names, email addresses, phone numbers, prices, and URLs with precise character positioning" \
    --provider ollama \
    --model mistral \
    --export html \
    --show-intervals \
    --temperature 0.2

echo
echo -e "${BLUE}📊 Analyzing Character Alignment Results:${NC}"

if [ -f "$SCRIPT_DIR/output/alignment_results.json" ]; then
    echo "Extraction results with character positions:"
    echo
    
    # Use jq to format the alignment information nicely
    if command -v jq &> /dev/null; then
        jq -r '
        if .extractions then
            .extractions[] | 
            "🎯 \(.extraction_class | ascii_upcase): \"\(.extraction_text)\"" +
            (if .char_interval then
                " → Position: \(.char_interval.start_pos // "?")-\(.char_interval.end_pos // "?")"
            else
                " → Position: Not aligned"
            end)
        else
            "No extractions found"
        end
        ' "$SCRIPT_DIR/output/alignment_results.json"
    else
        cat "$SCRIPT_DIR/output/alignment_results.json"
    fi
    
    echo
    echo "💡 Character Position Guide:"
    echo "   • start_pos: Character index where extraction begins (0-based)"
    echo "   • end_pos: Character index where extraction ends (exclusive)"
    echo "   • Position format: start-end (e.g., 45-58 means chars 45 through 57)"
    echo
fi

echo -e "${GREEN}✅ Alignment Demo Complete!${NC}"
echo
echo "Generated files:"
echo "  • $SCRIPT_DIR/output/alignment_results.json - Results with character intervals"
echo "  • langextract_results.html - Interactive visualization with highlighting"
echo
echo "🔍 What to Examine:"
echo "  1. Character positions in the JSON output"
echo "  2. How extracted text aligns with the original input"
echo "  3. Alignment accuracy for different types of entities"
echo "  4. HTML visualization showing highlighted text in context"
echo
echo "🧪 Try This:"
echo "  • Change the input text and see how positions update"
echo "  • Compare alignment accuracy with different temperature values"
echo "  • Check alignment for complex entities (emails, URLs, dates)"
echo "  • Use lower temperature (0.1) for more consistent positioning"
