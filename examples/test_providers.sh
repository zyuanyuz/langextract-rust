#!/bin/bash

# 🚀 LangExtract Provider Testing Script
# This script helps you easily test different language model providers

set -e

echo "🚀 LangExtract End-to-End Provider Testing"
echo "=========================================="

# Check if any providers are configured
PROVIDERS_FOUND=false

# Check OpenAI
if [ ! -z "$OPENAI_API_KEY" ]; then
    echo "✅ OpenAI API key found"
    PROVIDERS_FOUND=true
else
    echo "⚠️  OpenAI API key not found (set OPENAI_API_KEY)"
fi

# Check Ollama
if command -v ollama &> /dev/null; then
    if curl -s http://localhost:11434/api/tags &> /dev/null; then
        echo "✅ Ollama server is running"
        PROVIDERS_FOUND=true
    else
        echo "⚠️  Ollama server not running (run 'ollama serve')"
    fi
else
    echo "⚠️  Ollama not installed"
fi

# Check for custom provider
if [ ! -z "$CUSTOM_LLM_URL" ] && [ ! -z "$CUSTOM_LLM_KEY" ]; then
    echo "✅ Custom provider configured"
    PROVIDERS_FOUND=true
fi

if [ "$PROVIDERS_FOUND" = false ]; then
    echo ""
    echo "❌ No providers found! Please configure at least one:"
    echo "   • OpenAI: export OPENAI_API_KEY=your_key"
    echo "   • Ollama: ollama serve && ollama pull mistral"
    echo "   • Custom: export CUSTOM_LLM_URL=... && export CUSTOM_LLM_KEY=..."
    echo ""
    echo "📖 See E2E_TEST_README.md for detailed setup instructions"
    exit 1
fi

echo ""
echo "🔧 Building and running end-to-end test..."
echo ""

# Run the test
cargo run --example e2e_provider_test

echo ""
echo "🎉 Test completed!"
echo ""
echo "📁 Check the current directory for generated files:"
echo "   • *.html - Interactive visualization"
echo "   • *.json - Structured data"
echo "   • *.csv  - Spreadsheet format"
echo ""
echo "📖 Open the .html files in your browser to see rich interactive results!"
