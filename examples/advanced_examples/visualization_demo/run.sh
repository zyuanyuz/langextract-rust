#!/bin/bash
# Visualization Demo - Demonstrates rich export and visualization features

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

echo -e "${BOLD}${CYAN}🎨 LangExtract Visualization Demo${NC}"
echo "Demonstrating rich export formats and interactive visualizations"
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

# Show the input document info
echo -e "${BLUE}📄 Input Document Overview:${NC}"
input_text=$(cat "$SCRIPT_DIR/input.txt")
char_count=$(echo -n "$input_text" | wc -c)
word_count=$(echo "$input_text" | wc -w)

echo "   📊 Document Statistics:"
echo "      • Characters: $char_count"
echo "      • Words: $word_count"
echo "      • Content: Company directory with rich entity types"
echo "      • Entity types: People, contact info, financials, URLs, addresses"
echo

# Show preview
echo -e "${BLUE}📖 Document Preview:${NC}"
echo "┌─────────────────────────────────────────────────────────────────────────────────┐"
echo "$input_text" | head -6
echo "... (document continues with company information, products, contacts)"
echo "└─────────────────────────────────────────────────────────────────────────────────┘"
echo

# Show visualization examples
echo -e "${BLUE}📚 Visualization Training Examples:${NC}"
echo "These examples ensure rich entity extraction for visualization:"
echo

if command -v jq &> /dev/null; then
    jq -r '.[] | 
    "🎯 \"" + (.text | .[0:60]) + "...\"" +
    "\n   → " + (.extractions | map(.extraction_class) | join(", ")) + "\n"
    ' "$SCRIPT_DIR/examples.json" 2>/dev/null
else
    echo "View examples.json for training data format"
fi

# Create output directory
mkdir -p "$SCRIPT_DIR/output"

# Step 1: Extract the data
echo -e "${CYAN}🔄 Step 1: Data Extraction${NC}"
echo "Extracting entities optimized for visualization..."

$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract people, contact information (emails, phones), companies, locations, URLs, prices, financial amounts, dates, and job titles from this company directory" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/extraction_data.json" \
    --format json \
    --temperature 0.3 \
    --workers 6 \
    --show-intervals \
    --debug

echo
echo -e "${CYAN}🎨 Step 2: HTML Visualization${NC}"
echo "Generating interactive HTML visualization with highlighting..."

$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract people, contact information (emails, phones), companies, locations, URLs, prices, financial amounts, dates, and job titles from this company directory" \
    --provider ollama \
    --model mistral \
    --export html \
    --temperature 0.3 \
    --workers 6 \
    --show-intervals

# Move the generated HTML file to our output directory
if [ -f "langextract_results.html" ]; then
    mv "langextract_results.html" "$SCRIPT_DIR/output/interactive_visualization.html"
fi

echo
echo -e "${CYAN}📊 Step 3: CSV Export${NC}"
echo "Generating structured CSV for data analysis..."

$CLI_CMD convert "$SCRIPT_DIR/output/extraction_data.json" \
    --output "$SCRIPT_DIR/output/structured_data.csv" \
    --format csv \
    --show-intervals

echo
echo -e "${CYAN}📝 Step 4: Markdown Export${NC}"
echo "Generating Markdown documentation with highlighted entities..."

$CLI_CMD convert "$SCRIPT_DIR/output/extraction_data.json" \
    --output "$SCRIPT_DIR/output/highlighted_document.md" \
    --format markdown \
    --show-intervals

echo
echo -e "${CYAN}📋 Step 5: Text Format${NC}"
echo "Displaying human-readable extraction summary..."

$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract people, contact information (emails, phones), companies, locations, URLs, prices, financial amounts, dates, and job titles from this company directory" \
    --provider ollama \
    --model mistral \
    --format text \
    --temperature 0.3 \
    --show-intervals

echo
echo -e "${BLUE}📊 Visualization Analysis:${NC}"

# Analyze the extraction results
if [ -f "$SCRIPT_DIR/output/extraction_data.json" ]; then
    
    if command -v jq &> /dev/null; then
        echo
        echo -e "${CYAN}📈 Extraction Statistics:${NC}"
        
        total_count=$(jq '.extractions | length' "$SCRIPT_DIR/output/extraction_data.json" 2>/dev/null || echo "0")
        echo "   📊 Total extractions: $total_count"
        
        echo
        echo -e "${CYAN}📋 Entity Type Distribution:${NC}"
        
        # Show category breakdown with counts
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
        ' "$SCRIPT_DIR/output/extraction_data.json" 2>/dev/null
        
        echo
        echo -e "${CYAN}🎯 Sample Extractions for Visualization:${NC}"
        
        # Show specific examples that look good in visualization
        jq -r '
        if .extractions then
            (.extractions | 
            map(select(.extraction_class == "person" or .extraction_class == "email" or .extraction_class == "price" or .extraction_class == "url")) |
            .[0:6] |
            .[] |
            "   🔹 " + .extraction_class + ": \"" + .extraction_text + "\"" +
            (if .char_interval then " [" + (.char_interval.start_pos // 0 | tostring) + "-" + (.char_interval.end_pos // 0 | tostring) + "]" else "" end)
            )
        else
            "   No sample extractions available"
        end
        ' "$SCRIPT_DIR/output/extraction_data.json" 2>/dev/null
        
    else
        echo "Install jq for detailed analysis, or check JSON files manually"
    fi
    
    echo
    echo -e "${CYAN}📁 Generated Visualization Files:${NC}"
    echo "   🌐 interactive_visualization.html - Interactive web visualization"
    echo "   📊 structured_data.csv - Spreadsheet-ready data"
    echo "   📝 highlighted_document.md - Markdown with entity highlighting"
    echo "   📋 extraction_data.json - Raw structured data"
    
else
    echo "❌ Could not find extraction results for analysis"
fi

echo
echo -e "${GREEN}✅ Visualization Demo Complete!${NC}"
echo
echo "Generated visualization files in $SCRIPT_DIR/output/:"
echo "  • interactive_visualization.html - 🌐 Open in browser for interactive experience"
echo "  • structured_data.csv - 📊 Import into Excel/Google Sheets"
echo "  • highlighted_document.md - 📝 View in Markdown editor"
echo "  • extraction_data.json - 📋 Use for programmatic access"
echo
echo "🎨 Visualization Features Demonstrated:"
echo "  1. Interactive HTML with entity highlighting"
echo "  2. Color-coded entity types"
echo "  3. Hover tooltips with character positions"
echo "  4. Statistical dashboards"
echo "  5. Multiple export formats for different use cases"
echo
echo "🌐 HTML Visualization Features:"
echo "  • Click entities to see details"
echo "  • Color-coded categories (blue=person, green=contact, gold=financial)"
echo "  • Character position tooltips"
echo "  • Statistics panel with extraction counts"
echo "  • Responsive design for mobile/desktop"
echo
echo "📊 CSV Export Uses:"
echo "  • Data analysis in spreadsheets"
echo "  • Integration with BI tools"
echo "  • Database imports"
echo "  • Statistical analysis"
echo
echo "📝 Markdown Export Uses:"
echo "  • Documentation generation"
echo"  • GitHub/GitLab README files"
echo "  • Wiki pages"
echo "  • Technical documentation"
echo
echo "🧪 Try This:"
echo "  • Open interactive_visualization.html in your browser"
echo "  • Import structured_data.csv into Excel for pivot tables"
echo "  • View highlighted_document.md in VS Code or GitHub"
echo "  • Modify the input text and regenerate visualizations"
echo
echo "⚙️  Customization Options:"
echo "  • Add custom CSS for different color schemes"
echo "  • Modify export templates for branded outputs"
echo "  • Create domain-specific visualizations"
echo "  • Integrate with web applications or dashboards"
