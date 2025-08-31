#!/bin/bash
# Multipass Demo - Demonstrates improved recall through multiple extraction rounds

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${BOLD}${PURPLE}🔄 LangExtract Multipass Demo${NC}"
echo "Demonstrating improved recall through multiple extraction rounds"
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

# Show the input text statistics
echo -e "${BLUE}📄 Input Text Analysis:${NC}"
input_text=$(cat "$SCRIPT_DIR/input.txt")
char_count=$(echo -n "$input_text" | wc -c)
word_count=$(echo "$input_text" | wc -w)
line_count=$(echo "$input_text" | wc -l)

echo "   📊 Statistics:"
echo "      • Characters: $char_count"
echo "      • Words: $word_count"
echo "      • Lines: $line_count"
echo "      • Estimated entities: 50+ (people, organizations, locations, dates, etc.)"
echo

# Show a preview of the complex text
echo -e "${BLUE}📖 Text Preview:${NC}"
echo "┌─────────────────────────────────────────────────────────────────────────────────┐"
echo "$input_text" | head -6
echo "... (text continues with many more entities)"
echo "└─────────────────────────────────────────────────────────────────────────────────┘"
echo

# Show the multipass training examples
echo -e "${BLUE}📚 Multipass Training Examples:${NC}"
echo "These examples train the model to find diverse entity types:"
echo

if command -v jq &> /dev/null; then
    jq -r '.[] | 
    "🎯 Input: \(.text)\n" +
    "   Extractions: " + (.extractions | map(.extraction_class) | join(", ")) + "\n"
    ' "$SCRIPT_DIR/examples.json" 2>/dev/null
else
    echo "View examples.json for training data format"
fi

echo

# Create output directory
mkdir -p "$SCRIPT_DIR/output"

# First, run single-pass for comparison
echo -e "${CYAN}🔄 Step 1: Single-Pass Extraction (Baseline)${NC}"
echo "Running with 1 pass to establish baseline performance..."

$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract all people, organizations, locations, dates, funding amounts, events, contact information, and technical terms from this research text" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/single_pass_results.json" \
    --format text \
    --temperature 0.4 \
    --workers 6 \
    --passes 1 \
    --show-intervals

echo
echo "💾 Saving single-pass JSON results..."
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract all people, organizations, locations, dates, funding amounts, events, contact information, and technical terms from this research text" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/single_pass_results.json" \
    --format json \
    --temperature 0.4 \
    --workers 6 \
    --passes 1

# Now run multipass extraction
echo
echo -e "${CYAN}🔄 Step 2: Multi-Pass Extraction (Enhanced Recall)${NC}"
echo "Running with 3 passes to improve recall and find missed entities..."

$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract all people, organizations, locations, dates, funding amounts, events, contact information, and technical terms from this research text" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/multipass_results.json" \
    --format text \
    --temperature 0.4 \
    --workers 6 \
    --passes 3 \
    --multipass \
    --show-intervals

echo
echo "💾 Saving multi-pass JSON results..."
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract all people, organizations, locations, dates, funding amounts, events, contact information, and technical terms from this research text" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/multipass_results.json" \
    --format json \
    --temperature 0.4 \
    --workers 6 \
    --passes 3 \
    --multipass

# Generate comparison visualization
echo
echo "🎨 Creating comparison visualization..."
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract all people, organizations, locations, dates, funding amounts, events, contact information, and technical terms from this research text" \
    --provider ollama \
    --model mistral \
    --export html \
    --temperature 0.4 \
    --workers 6 \
    --passes 3 \
    --multipass \
    --show-intervals

echo
echo -e "${BLUE}📊 Comparing Results:${NC}"

# Analyze and compare the results
if [ -f "$SCRIPT_DIR/output/single_pass_results.json" ] && [ -f "$SCRIPT_DIR/output/multipass_results.json" ]; then
    
    if command -v jq &> /dev/null; then
        echo
        echo -e "${CYAN}📈 Extraction Statistics:${NC}"
        
        single_count=$(jq '.extractions | length' "$SCRIPT_DIR/output/single_pass_results.json" 2>/dev/null || echo "0")
        multi_count=$(jq '.extractions | length' "$SCRIPT_DIR/output/multipass_results.json" 2>/dev/null || echo "0")
        
        echo "   🔵 Single-pass extractions: $single_count"
        echo "   🟢 Multi-pass extractions: $multi_count"
        
        if [ "$multi_count" -gt "$single_count" ]; then
            improvement=$((multi_count - single_count))
            echo "   📈 Improvement: +$improvement extractions (+$(( (improvement * 100) / single_count ))%)"
        fi
        
        echo
        echo -e "${CYAN}📋 Extraction Categories (Multi-pass):${NC}"
        
        # Group by extraction class
        jq -r '
        if .extractions then
            (.extractions | group_by(.extraction_class) | 
            sort_by(-length) | 
            .[] | 
            "   🏷️  " + (.[0].extraction_class | ascii_upcase) + ": " + (length | tostring) + " items"
            )
        else
            "   No extractions found"
        end
        ' "$SCRIPT_DIR/output/multipass_results.json" 2>/dev/null
        
        echo
        echo -e "${CYAN}🔍 Sample Extractions by Category:${NC}"
        
        # Show sample extractions from each category
        jq -r '
        if .extractions then
            (.extractions | group_by(.extraction_class) | 
            sort_by(-length) | 
            .[] | 
            "🎯 " + (.[0].extraction_class | ascii_upcase) + ":" + 
            "\n" + 
            (.[0:3] | map("   • \"" + .extraction_text + "\"") | join("\n")) + 
            (if length > 3 then "\n   • ... and " + ((length - 3) | tostring) + " more" else "" end) +
            "\n"
            )
        else
            "No extractions available"
        end
        ' "$SCRIPT_DIR/output/multipass_results.json" 2>/dev/null
        
    else
        echo "Install jq for detailed analysis, or check JSON files manually"
    fi
else
    echo "❌ Could not find result files for comparison"
fi

echo
echo -e "${GREEN}✅ Multipass Demo Complete!${NC}"
echo
echo "Generated files:"
echo "  • $SCRIPT_DIR/output/single_pass_results.json - Baseline single-pass results"
echo "  • $SCRIPT_DIR/output/multipass_results.json - Enhanced multi-pass results"
echo "  • langextract_results.html - Interactive visualization"
echo
echo "🔬 Multipass Features Demonstrated:"
echo "  1. Improved recall through multiple extraction rounds"
echo "  2. Discovery of entities missed in single pass"
echo "  3. Quality-based reprocessing decisions"
echo "  4. Comprehensive entity coverage"
echo "  5. Statistical comparison of approaches"
echo
echo "🎯 Key Benefits of Multipass:"
echo "  • Higher entity recall (finds more entities)"
echo "  • Better coverage of diverse entity types"
echo "  • Reduced missed entities in complex text"
echo "  • Quality-aware processing (avoids unnecessary work)"
echo
echo "🧪 Try This:"
echo "  • Compare single_pass_results.json vs multipass_results.json"
echo "  • Modify the number of passes (--passes 2, 4, 5)"
echo "  • Adjust quality threshold for different recall/precision trade-offs"
echo "  • Test with your own complex documents"
echo
echo "⚙️  Configuration Tips:"
echo "  • Use higher temperature (0.4-0.6) for diverse passes"
echo "  • Set appropriate quality thresholds (0.6-0.8)"
echo "  • Increase passes (3-5) for very complex documents"
echo "  • Monitor processing time vs. quality improvement"
