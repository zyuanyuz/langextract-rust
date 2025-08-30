#!/bin/bash

# 🛍️ Product Catalog Extraction Test Script
# Specialized test for extracting structured product data

set -e

echo "🛍️  LangExtract Product Catalog Extraction Test"
echo "=============================================="

# Check if the product file exists
if [ ! -f "sample_product_text.txt" ]; then
    echo "❌ Error: sample_product_text.txt not found!"
    echo "   Please ensure the product catalog file is in the current directory."
    exit 1
fi

# Show file info
FILE_SIZE=$(wc -c < sample_product_text.txt)
LINE_COUNT=$(wc -l < sample_product_text.txt)
echo "📄 Product catalog: $LINE_COUNT lines, $FILE_SIZE characters"

# Check providers
PROVIDERS_FOUND=false

if [ ! -z "$OPENAI_API_KEY" ]; then
    echo "✅ OpenAI API key found - will use GPT-4o-mini"
    PROVIDERS_FOUND=true
elif command -v ollama &> /dev/null && curl -s http://localhost:11434/api/tags &> /dev/null; then
    echo "✅ Ollama server found - will use Mistral"
    PROVIDERS_FOUND=true
else
    echo "❌ No providers available!"
    echo "   • For OpenAI: export OPENAI_API_KEY=your_key"
    echo "   • For Ollama: ollama serve && ollama pull mistral"
    exit 1
fi

echo ""
echo "🎯 This test will extract:"
echo "   📦 Product names and descriptions"
echo "   🏷️  SKUs, UPCs, model numbers, and product codes"
echo "   💰 Prices, sale prices, and financial data"
echo "   📊 Technical specifications and features"
echo "   🏪 Inventory codes and availability"
echo "   💊 Medical/pharmaceutical data (NDC, lot numbers)"
echo "   🔧 Tool specifications and warranties"
echo ""

echo "🔄 Starting product extraction test..."
cargo run --example product_catalog_test

echo ""
echo "🎉 Product extraction test completed!"
echo ""
echo "📁 Generated files:"
echo "   • product_catalog_*.html - Interactive product visualization"
echo "   • product_catalog_*.json - Structured product data"  
echo "   • product_catalog_*.csv  - Product data for spreadsheet analysis"
echo ""
echo "💡 The HTML file will show highlighted products in the original catalog text!"
echo "📊 Use the CSV file to analyze pricing, categories, and product codes."
