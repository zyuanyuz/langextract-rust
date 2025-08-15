#!/bin/bash

# Academic Paper Extraction Test Script
# Tests LangExtract on academic/research paper content

echo "📚 LangExtract Academic Paper Extraction Test"
echo "============================================"

# Check if the academic paper file exists
if [ ! -f "agentic_design_review_system.txt" ]; then
    echo "❌ Error: agentic_design_review_system.txt not found!"
    echo "   Please ensure the academic paper file is in the current directory."
    exit 1
fi

# Show file info
echo "📄 Academic paper file found:"
echo "   Size: $(wc -c < agentic_design_review_system.txt) characters"
echo "   Lines: $(wc -l < agentic_design_review_system.txt) lines"
echo ""

# Check for available providers
echo "🔍 Checking for available LLM providers..."

if [ ! -z "$OPENAI_API_KEY" ]; then
    echo "✅ OpenAI API key found"
elif curl -s --connect-timeout 3 http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "✅ Ollama server detected"
else
    echo "❌ No providers available!"
    echo ""
    echo "To run this test, set up a provider:"
    echo "  • OpenAI: export OPENAI_API_KEY=your_key"
    echo "  • Ollama: ollama serve && ollama pull mistral"
    exit 1
fi

echo ""
echo "🚀 Starting academic paper extraction..."
echo "⏰ This may take 2-5 minutes depending on paper length and provider speed"
echo ""

# Run the test
if cargo run --example academic_paper_test; then
    echo ""
    echo "🎉 Academic extraction test completed successfully!"
    echo ""
    echo "📋 Generated files:"
    ls -la academic_paper_*.html academic_paper_*.json academic_paper_*.csv 2>/dev/null || echo "   No output files found"
    echo ""
    echo "💡 Next steps:"
    echo "   • Open the .html file to see highlighted academic content"
    echo "   • Use .csv for data analysis in spreadsheets"
    echo "   • Process .json programmatically for research insights"
else
    echo ""
    echo "❌ Academic extraction test failed!"
    echo "   Check the error messages above for troubleshooting"
    exit 1
fi
