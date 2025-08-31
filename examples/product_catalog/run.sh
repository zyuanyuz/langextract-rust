#!/bin/bash
# Product Catalog Demo - Extract product information from catalogs

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

echo -e "${BOLD}${PURPLE}🛍️  LangExtract Product Catalog Demo${NC}"
echo "Specialized extraction for product catalogs and e-commerce data"
echo

# Check if lx-rs is available
if command -v lx-rs &> /dev/null; then
    CLI_CMD="lx-rs"
    echo -e "${GREEN}✅ Using installed lx-rs binary${NC}"
elif [ -f "../target/release/lx-rs" ]; then
    CLI_CMD="../target/release/lx-rs"
    echo -e "${GREEN}✅ Using local release binary${NC}"
elif [ -f "../../target/release/lx-rs" ]; then
    CLI_CMD="../../target/release/lx-rs"
    echo -e "${GREEN}✅ Using local release binary${NC}"
else
    CLI_CMD="cargo run --features=cli --bin lx-rs --"
    echo -e "${YELLOW}⚠️  Using cargo run (slower, but works without install)${NC}"
fi

echo

# Check for sample data file
if [ ! -f "$SCRIPT_DIR/sample_product_text.txt" ]; then
    echo -e "${RED}❌ Error: sample_product_text.txt not found!${NC}"
    echo "   Please ensure the product catalog file is in the examples/product_catalog/ directory."
    exit 1
fi

# Show catalog info
echo -e "${BLUE}📄 Product Catalog Overview:${NC}"
file_size=$(wc -c < "$SCRIPT_DIR/sample_product_text.txt")
line_count=$(wc -l < "$SCRIPT_DIR/sample_product_text.txt")
word_count=$(wc -w < "$SCRIPT_DIR/sample_product_text.txt")

echo "   📊 Catalog Statistics:"
echo "      • File size: $file_size characters"
echo "      • Lines: $line_count"
echo "      • Words: $word_count"
echo "      • Estimated products: 15-20 items"
echo

# Show preview of catalog content
echo -e "${BLUE}📖 Catalog Preview:${NC}"
echo "┌─────────────────────────────────────────────────────────────────────────────────┐"
head -8 "$SCRIPT_DIR/sample_product_text.txt"
echo "... (catalog continues with more products)"
echo "└─────────────────────────────────────────────────────────────────────────────────┘"
echo

# Show product extraction examples
echo -e "${BLUE}📚 Product Extraction Examples:${NC}"
echo "Training the model to extract product catalog entities:"
echo

if command -v jq &> /dev/null; then
    jq -r '.[] | 
    "🛍️  \"" + (.text | .[0:80]) + "...\"" +
    "\n   Categories: " + (.extractions | map(.extraction_class) | unique | join(", ")) + "\n"
    ' "$SCRIPT_DIR/examples.json" 2>/dev/null
else
    echo "View examples.json for product training data"
fi

echo

# Create output directory
mkdir -p "$SCRIPT_DIR/output"

echo -e "${CYAN}🎯 Extraction Target Categories:${NC}"
echo "   📦 Product names and descriptions"
echo "   🏷️  SKUs, UPCs, model numbers, product codes"
echo "   💰 Prices, sale prices, MSRP, discounts"
echo "   📊 Technical specifications and features"
echo "   🏪 Inventory codes, availability, stock status"
echo "   💊 Medical/pharmaceutical data (NDC, lot numbers, expiration)"
echo "   🔧 Tool specifications, warranties, contractor pricing"
echo "   📱 Electronics specs (chips, memory, displays)"
echo

# Run the product catalog extraction
echo -e "${CYAN}🔄 Extracting Product Information...${NC}"

start_time=$(date +%s)

$CLI_CMD extract "$SCRIPT_DIR/sample_product_text.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract comprehensive product information from this electronics and retail catalog including product names, models, SKUs, UPCs, prices, specifications, availability, and all product identifiers" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/product_catalog_results.json" \
    --format text \
    --temperature 0.3 \
    --workers 8 \
    --batch-size 6 \
    --max-chars 8000 \
    --show-intervals \
    --debug

extraction_time=$(($(date +%s) - start_time))

echo
echo "💾 Saving structured JSON data..."
$CLI_CMD extract "$SCRIPT_DIR/sample_product_text.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract comprehensive product information from this electronics and retail catalog including product names, models, SKUs, UPCs, prices, specifications, availability, and all product identifiers" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/product_catalog_results.json" \
    --format json \
    --temperature 0.3 \
    --workers 8 \
    --batch-size 6 \
    --max-chars 8000

echo
echo "🎨 Creating interactive product visualization..."
$CLI_CMD extract "$SCRIPT_DIR/sample_product_text.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract comprehensive product information from this electronics and retail catalog including product names, models, SKUs, UPCs, prices, specifications, availability, and all product identifiers" \
    --provider ollama \
    --model mistral \
    --export html \
    --temperature 0.3 \
    --workers 8 \
    --max-chars 8000 \
    --show-intervals

# Move generated HTML to output directory with timestamp
timestamp=$(date +%Y%m%d_%H%M%S)
if [ -f "langextract_results.html" ]; then
    mv "langextract_results.html" "$SCRIPT_DIR/output/product_catalog_${timestamp}.html"
fi

echo
echo "📊 Generating CSV for product analysis..."
$CLI_CMD convert "$SCRIPT_DIR/output/product_catalog_results.json" \
    --output "$SCRIPT_DIR/output/product_catalog_${timestamp}.csv" \
    --format csv \
    --show-intervals

echo
echo -e "${BLUE}📊 Product Extraction Analysis:${NC}"

# Analyze the results
if [ -f "$SCRIPT_DIR/output/product_catalog_results.json" ]; then
    
    if command -v jq &> /dev/null; then
        echo
        echo -e "${CYAN}📈 Extraction Performance:${NC}"
        
        total_count=$(jq '.extractions | length' "$SCRIPT_DIR/output/product_catalog_results.json" 2>/dev/null || echo "0")
        echo "   📊 Total extractions: $total_count"
        echo "   ⏱️  Processing time: ${extraction_time}s"
        echo "   🔄 Extractions per second: $((total_count / (extraction_time + 1)))"
        
        echo
        echo -e "${CYAN}🛍️  Product Category Breakdown:${NC}"
        
        # Show category analysis optimized for products
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
        ' "$SCRIPT_DIR/output/product_catalog_results.json" 2>/dev/null
        
        echo
        echo -e "${CYAN}💰 Pricing Analysis:${NC}"
        
        # Analyze pricing information
        price_count=$(jq '[.extractions[] | select(.extraction_class | test("price|cost|msrp|sale"))] | length' "$SCRIPT_DIR/output/product_catalog_results.json" 2>/dev/null || echo "0")
        echo "   💵 Price entries found: $price_count"
        
        if [ "$price_count" -gt "0" ]; then
            echo "   💸 Sample prices:"
            jq -r '
            [.extractions[] | select(.extraction_class | test("price|cost|msrp|sale"))] | 
            .[0:5] | 
            .[] | 
            "      • " + .extraction_class + ": " + .extraction_text
            ' "$SCRIPT_DIR/output/product_catalog_results.json" 2>/dev/null
        fi
        
        echo
        echo -e "${CYAN}🔢 Product Identifiers Analysis:${NC}"
        
        # Analyze product codes and identifiers
        id_count=$(jq '[.extractions[] | select(.extraction_class | test("sku|upc|model|code|id"))] | length' "$SCRIPT_DIR/output/product_catalog_results.json" 2>/dev/null || echo "0")
        echo "   🏷️  Product identifiers found: $id_count"
        
        if [ "$id_count" -gt "0" ]; then
            echo "   📋 Sample identifiers:"
            jq -r '
            [.extractions[] | select(.extraction_class | test("sku|upc|model|code|id"))] | 
            .[0:5] | 
            .[] | 
            "      • " + .extraction_class + ": " + .extraction_text
            ' "$SCRIPT_DIR/output/product_catalog_results.json" 2>/dev/null
        fi
        
        echo
        echo -e "${CYAN}📦 Sample Product Extractions:${NC}"
        
        # Show sample products
        product_count=$(jq '[.extractions[] | select(.extraction_class | test("product"))] | length' "$SCRIPT_DIR/output/product_catalog_results.json" 2>/dev/null || echo "0")
        echo "   🛍️  Products identified: $product_count"
        
        if [ "$product_count" -gt "0" ]; then
            echo "   📱 Sample products:"
            jq -r '
            [.extractions[] | select(.extraction_class | test("product"))] | 
            .[0:3] | 
            .[] | 
            "      • \"" + .extraction_text + "\""
            ' "$SCRIPT_DIR/output/product_catalog_results.json" 2>/dev/null
        fi
        
    else
        echo "Install jq for detailed analysis, or check JSON files manually"
    fi
    
else
    echo "❌ Could not find extraction results for analysis"
fi

echo
echo -e "${GREEN}✅ Product Catalog Demo Complete!${NC}"
echo
echo "Generated files in $SCRIPT_DIR/output/:"
echo "  • product_catalog_results.json - 📋 Structured product data"
echo "  • product_catalog_${timestamp}.html - 🌐 Interactive product visualization"
echo "  • product_catalog_${timestamp}.csv - 📊 Spreadsheet-ready product data"
echo
echo "🛍️  Product Catalog Features Demonstrated:"
echo "  1. E-commerce product data extraction"
echo "  2. Multi-category product identification"  
echo "  3. Price and financial data parsing"
echo "  4. Product code and identifier extraction"
echo "  5. Technical specification recognition"
echo "  6. Inventory and availability tracking"
echo
echo "📊 Analysis Insights:"
echo "  • Total products identified: $product_count"
echo "  • Price entries extracted: $price_count"
echo "  • Product identifiers found: $id_count"
echo "  • Processing efficiency: $((total_count / (extraction_time + 1))) extractions/second"
echo
echo "🎨 Visualization Features:"
echo "  • Color-coded product categories"
echo "  • Highlighted prices and discounts"
echo "  • Interactive product code tooltips"
echo "  • Statistical dashboard with category breakdowns"
echo
echo "🧪 Try This:"
echo "  • Open the HTML file to see highlighted products in context"
echo "  • Import the CSV file into Excel for pricing analysis"
echo "  • Filter products by category or price range"
echo "  • Add your own product catalog data to sample_product_text.txt"
echo
echo "⚙️  Optimization Tips:"
echo "  • Use lower temperature (0.2) for more consistent product data"
echo "  • Enable multipass for comprehensive product coverage"
echo "  • Adjust examples.json for domain-specific products"
echo "  • Combine with validation for data quality assurance"
echo
echo "🔧 For Production Use:"
echo "  • Scale up workers (--workers 12) for large catalogs"
echo "  • Use batch processing for multiple catalog files"
echo "  • Implement data validation rules for product formats"
echo "  • Set up automated catalog processing pipelines"
