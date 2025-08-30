#!/bin/bash
# LangExtract CLI Demo Script
# This script demonstrates various CLI commands and features

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

CLI_BINARY="lx-rs"

echo -e "${BLUE}🚀 LangExtract CLI Demo${NC}"
echo "This script demonstrates the CLI capabilities"
echo

# Check if CLI is installed
if ! command -v $CLI_BINARY &> /dev/null; then
    echo -e "${YELLOW}⚠️  CLI not found. Please install it first:${NC}"
    echo "cargo install langextract-rust --features cli"
    echo "or run: ./install.sh"
    exit 1
fi

echo -e "${GREEN}✅ CLI found: $(which $CLI_BINARY)${NC}"
echo

# Demo 1: Basic text extraction
echo -e "${BLUE}📋 Demo 1: Basic Text Extraction${NC}"
echo "Extracting names and ages from sample text..."

SAMPLE_TEXT="Alice Johnson is 28 years old and works as a data scientist. Bob Smith, age 35, is a software engineer at Google."

echo "Sample text: $SAMPLE_TEXT"
echo

$CLI_BINARY extract "$SAMPLE_TEXT" \
    --prompt "Extract person names, ages, and job titles" \
    --format text \
    --show-intervals

echo
echo "---"
echo

# Demo 2: File processing with examples
echo -e "${BLUE}📋 Demo 2: File Processing with Custom Examples${NC}"
echo "Creating sample files and processing them..."

# Create sample input file
cat > /tmp/sample_document.txt << EOF
Dr. Sarah Wilson is a cardiologist at Mayo Clinic in Rochester, Minnesota. She has been practicing for 15 years.
Professor John Martinez teaches computer science at Stanford University. He received his PhD from MIT in 2005.
Maria Garcia, age 32, works as a software architect at Microsoft in Seattle, Washington.
EOF

# Create sample examples file
cat > /tmp/examples.json << EOF
[
  {
    "text": "Dr. Emily Brown is a neurologist at Johns Hopkins Hospital in Baltimore",
    "extractions": [
      {"extraction_class": "person", "extraction_text": "Dr. Emily Brown"},
      {"extraction_class": "profession", "extraction_text": "neurologist"},
      {"extraction_class": "organization", "extraction_text": "Johns Hopkins Hospital"},
      {"extraction_class": "location", "extraction_text": "Baltimore"}
    ]
  }
]
EOF

echo "Input file created: /tmp/sample_document.txt"
echo "Examples file created: /tmp/examples.json"
echo

$CLI_BINARY extract /tmp/sample_document.txt \
    --examples /tmp/examples.json \
    --prompt "Extract person names, professions, organizations, and locations" \
    --format json \
    --output /tmp/results.json

echo
echo "Results saved to: /tmp/results.json"
echo "Sample output:"
head -10 /tmp/results.json
echo

echo "---"
echo

# Demo 3: Provider information
echo -e "${BLUE}📋 Demo 3: Provider Information${NC}"
echo "Listing available providers and models..."

$CLI_BINARY providers

echo
echo "---"
echo

# Demo 4: Configuration initialization
echo -e "${BLUE}📋 Demo 4: Configuration Setup${NC}"
echo "Initializing configuration files..."

mkdir -p /tmp/langextract_demo
cd /tmp/langextract_demo

$CLI_BINARY init --force

echo
echo "Created configuration files:"
ls -la
echo

echo "Sample examples.json:"
cat examples.json
echo

echo "---"
echo

# Demo 5: Test connectivity (will likely fail without proper setup)
echo -e "${BLUE}📋 Demo 5: Testing Provider Connectivity${NC}"
echo "Testing Ollama connectivity (this may fail if Ollama is not running)..."

$CLI_BINARY test --provider ollama --model mistral || {
    echo -e "${YELLOW}⚠️  Ollama test failed (expected if not installed/running)${NC}"
    echo "To install Ollama:"
    echo "1. Visit: https://ollama.ai/"
    echo "2. Run: ollama serve"
    echo "3. Run: ollama pull mistral"
}

echo
echo "---"
echo

# Demo 6: Format conversion
echo -e "${BLUE}📋 Demo 6: Format Conversion${NC}"
echo "Converting results to different formats..."

if [ -f /tmp/results.json ]; then
    $CLI_BINARY convert /tmp/results.json \
        --output /tmp/results.html \
        --format html \
        --show-intervals
    
    echo "Converted to HTML: /tmp/results.html"
    echo "HTML file size: $(wc -c < /tmp/results.html) bytes"
fi

echo
echo "---"
echo

# Demo 7: Help and examples
echo -e "${BLUE}📋 Demo 7: Help and Examples${NC}"
echo "Showing CLI examples..."

$CLI_BINARY examples

echo
echo "---"
echo

# Cleanup
echo -e "${BLUE}🧹 Cleanup${NC}"
echo "Demo files created in /tmp/ (will be cleaned up on reboot)"
echo "Configuration demo created in: /tmp/langextract_demo"

echo
echo -e "${GREEN}🎉 CLI Demo Complete!${NC}"
echo
echo "Next steps:"
echo "1. Install a provider (e.g., Ollama: https://ollama.ai/)"
echo "2. Set up API keys in .env file"
echo "3. Run: $CLI_BINARY init"
echo "4. Test: $CLI_BINARY test"
echo "5. Extract: $CLI_BINARY extract 'your text here' --prompt 'what to extract'"
