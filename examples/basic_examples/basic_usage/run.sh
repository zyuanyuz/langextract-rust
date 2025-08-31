#!/bin/bash
# Basic Usage Example - Extract person names, ages, and professions

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${BLUE}🚀 LangExtract Basic Usage Example${NC}"
echo "Extracting person names, ages, and professions from text"
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

echo "Command: $CLI_CMD"
echo

# Show the input text
echo -e "${BLUE}📄 Input Text:${NC}"
cat "$SCRIPT_DIR/input.txt"
echo
echo

# Show the examples being used
echo -e "${BLUE}📚 Training Examples:${NC}"
echo "Using examples from: examples.json"
jq '.[] | "Text: \(.text) → Extractions: \(.extractions | length) items"' "$SCRIPT_DIR/examples.json" 2>/dev/null || cat "$SCRIPT_DIR/examples.json"
echo

# Run the extraction
echo -e "${BLUE}🔄 Running Extraction...${NC}"
echo

# Create output directory
mkdir -p "$SCRIPT_DIR/output"

# Extract with different output formats
echo "📊 Extracting to JSON format:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract person names, ages, professions, and workplaces from the text" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/results.json" \
    --format json \
    --show-intervals \
    --debug

echo
echo "📝 Extracting to text format:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract person names, ages, professions, and workplaces from the text" \
    --provider ollama \
    --model mistral \
    --format text \
    --show-intervals

echo
echo "📊 Exporting to HTML visualization:"
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract person names, ages, professions, and workplaces from the text" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/results.json" \
    --export html \
    --show-intervals \
    --format json

echo
echo -e "${GREEN}✅ Basic Usage Example Complete!${NC}"
echo
echo "Generated files:"
echo "  • $SCRIPT_DIR/output/results.json - Structured extraction results"
echo "  • $SCRIPT_DIR/output/langextract_results.html - Interactive visualization"
echo
echo "💡 Tips:"
echo "  • Open the .html file in a browser to see highlighted extractions"
echo "  • Modify examples.json to change what gets extracted"
echo "  • Edit config.yaml to use different providers (OpenAI, etc.)"
echo "  • Try different input text in input.txt"
echo
echo "🔧 To use different providers:"
echo "  • OpenAI: export OPENAI_API_KEY=your_key && ./run.sh"
echo "  • Ollama: ollama serve && ollama pull mistral && ./run.sh"
