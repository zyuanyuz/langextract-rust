#!/bin/bash
# Advanced Chunking Demo - Demonstrates intelligent text chunking for large documents

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

echo -e "${BOLD}${BLUE}🧩 LangExtract Advanced Chunking Demo${NC}"
echo "Demonstrating intelligent text chunking for large document processing"
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

# Analyze the input document
echo -e "${BLUE}📄 Document Analysis:${NC}"
input_text=$(cat "$SCRIPT_DIR/input.txt")
char_count=$(echo -n "$input_text" | wc -c)
word_count=$(echo "$input_text" | wc -w)
line_count=$(echo "$input_text" | wc -l)

echo "   📊 Size Statistics:"
echo "      • Characters: $char_count"
echo "      • Words: $word_count"  
echo "      • Lines: $line_count"
echo "      • Estimated chunks (4KB each): $((char_count / 4000 + 1))"
echo

# Calculate expected chunks
chunk_size=4000
expected_chunks=$((char_count / chunk_size + 1))

echo "   🧩 Chunking Parameters:"
echo "      • Chunk size: ${chunk_size} characters"
echo "      • Expected chunks: ${expected_chunks}"
echo "      • Overlap strategy: Semantic boundaries"
echo "      • Workers: 8 (parallel processing)"
echo

# Show document structure
echo -e "${BLUE}📖 Document Structure Preview:${NC}"
echo "┌─────────────────────────────────────────────────────────────────────────────────┐"
echo "$input_text" | head -10
echo "... (document continues with sections: LEADERSHIP, FUNDING, OBJECTIVES, etc.)"
echo "└─────────────────────────────────────────────────────────────────────────────────┘"
echo

# Show chunking examples
echo -e "${BLUE}📚 Chunking Training Examples:${NC}"
echo "These examples ensure consistent extraction across document chunks:"
echo

if command -v jq &> /dev/null; then
    jq -r '.[] | 
    "🎯 \"" + (.text | .[0:60]) + "...\"" +
    "\n   → " + (.extractions | map(.extraction_class) | join(", ")) + "\n"
    ' "$SCRIPT_DIR/examples.json" 2>/dev/null
else
    echo "View examples.json for training data format"
fi

echo

# Create output directory
mkdir -p "$SCRIPT_DIR/output"

# Run extraction with chunking - show different chunk sizes for comparison
echo -e "${CYAN}🔄 Step 1: Large Chunks (8KB) - Fewer, Larger Chunks${NC}"
echo "Processing with 8KB chunks to see chunking behavior..."

start_time=$(date +%s)

$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract people, organizations, locations, funding amounts, dates, contact information, and technical specifications from this climate research document" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/large_chunks_results.json" \
    --format text \
    --max-chars 8000 \
    --workers 8 \
    --batch-size 4 \
    --temperature 0.3 \
    --show-intervals \
    --debug

large_chunks_time=$(($(date +%s) - start_time))

echo
echo -e "${CYAN}🔄 Step 2: Medium Chunks (4KB) - Optimal Balance${NC}"
echo "Processing with 4KB chunks for optimal performance..."

start_time=$(date +%s)

$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract people, organizations, locations, funding amounts, dates, contact information, and technical specifications from this climate research document" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/medium_chunks_results.json" \
    --format text \
    --max-chars 4000 \
    --workers 8 \
    --batch-size 6 \
    --temperature 0.3 \
    --show-intervals \
    --debug

medium_chunks_time=$(($(date +%s) - start_time))

echo
echo -e "${CYAN}🔄 Step 3: Small Chunks (2KB) - High Granularity${NC}"
echo "Processing with 2KB chunks for fine-grained processing..."

start_time=$(date +%s)

$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract people, organizations, locations, funding amounts, dates, contact information, and technical specifications from this climate research document" \
    --provider ollama \
    --model mistral \
    --output "$SCRIPT_DIR/output/small_chunks_results.json" \
    --format text \
    --max-chars 2000 \
    --workers 8 \
    --batch-size 8 \
    --temperature 0.3 \
    --show-intervals \
    --debug

small_chunks_time=$(($(date +%s) - start_time))

# Save JSON versions for analysis
echo
echo "💾 Saving JSON results for all chunking strategies..."

# Save all JSON versions
for chunk_type in "large" "medium" "small"; do
    case $chunk_type in
        "large") chunk_size=8000 ;;
        "medium") chunk_size=4000 ;;
        "small") chunk_size=2000 ;;
    esac
    
    $CLI_CMD extract "$SCRIPT_DIR/input.txt" \
        --examples "$SCRIPT_DIR/examples.json" \
        --prompt "Extract people, organizations, locations, funding amounts, dates, contact information, and technical specifications from this climate research document" \
        --provider ollama \
        --model mistral \
        --output "$SCRIPT_DIR/output/${chunk_type}_chunks_results.json" \
        --format json \
        --max-chars $chunk_size \
        --workers 8 \
        --temperature 0.3 > /dev/null 2>&1
done

# Generate visualization for optimal chunks
echo
echo "🎨 Creating visualization with optimal chunking..."
$CLI_CMD extract "$SCRIPT_DIR/input.txt" \
    --examples "$SCRIPT_DIR/examples.json" \
    --prompt "Extract people, organizations, locations, funding amounts, dates, contact information, and technical specifications from this climate research document" \
    --provider ollama \
    --model mistral \
    --export html \
    --max-chars 4000 \
    --workers 8 \
    --temperature 0.3 \
    --show-intervals

echo
echo -e "${BLUE}📊 Chunking Performance Analysis:${NC}"

# Analyze the results
if command -v jq &> /dev/null; then
    echo
    echo -e "${CYAN}⏱️  Processing Time Comparison:${NC}"
    echo "   🔵 Large chunks (8KB): ${large_chunks_time}s"
    echo "   🟢 Medium chunks (4KB): ${medium_chunks_time}s" 
    echo "   🟡 Small chunks (2KB): ${small_chunks_time}s"
    echo
    
    echo -e "${CYAN}📈 Extraction Results Comparison:${NC}"
    
    for chunk_type in "large" "medium" "small"; do
        if [ -f "$SCRIPT_DIR/output/${chunk_type}_chunks_results.json" ]; then
            count=$(jq '.extractions | length' "$SCRIPT_DIR/output/${chunk_type}_chunks_results.json" 2>/dev/null || echo "0")
            case $chunk_type in
                "large") echo "   🔵 Large chunks (8KB): $count extractions" ;;
                "medium") echo "   🟢 Medium chunks (4KB): $count extractions" ;;
                "small") echo "   🟡 Small chunks (2KB): $count extractions" ;;
            esac
        fi
    done
    
    echo
    echo -e "${CYAN}📋 Detailed Analysis (Medium Chunks - Optimal):${NC}"
    
    if [ -f "$SCRIPT_DIR/output/medium_chunks_results.json" ]; then
        # Show category breakdown
        echo "   📊 Extraction Categories:"
        jq -r '
        if .extractions then
            (.extractions | group_by(.extraction_class) | 
            sort_by(-length) | 
            .[] | 
            "      • " + (.[0].extraction_class | ascii_upcase) + ": " + (length | tostring) + " items"
            )
        else
            "      No extractions found"
        end
        ' "$SCRIPT_DIR/output/medium_chunks_results.json" 2>/dev/null
        
        echo
        echo "   🔍 Sample Extractions by Category:"
        
        # Show samples from key categories
        jq -r '
        if .extractions then
            (.extractions | group_by(.extraction_class) | 
            sort_by(-length) | 
            .[0:5] |
            .[] | 
            "      🎯 " + (.[0].extraction_class | ascii_upcase) + ":" + 
            "\n" + 
            (.[0:2] | map("         • \"" + .extraction_text + "\"") | join("\n")) + 
            (if length > 2 then "\n         • ... and " + ((length - 2) | tostring) + " more" else "" end) +
            "\n"
            )
        else
            "      No extractions available"
        end
        ' "$SCRIPT_DIR/output/medium_chunks_results.json" 2>/dev/null
    fi
    
else
    echo "Install jq for detailed analysis, or check JSON files manually"
fi

echo
echo -e "${GREEN}✅ Advanced Chunking Demo Complete!${NC}"
echo
echo "Generated files:"
echo "  • $SCRIPT_DIR/output/large_chunks_results.json - 8KB chunk results"
echo "  • $SCRIPT_DIR/output/medium_chunks_results.json - 4KB chunk results"  
echo "  • $SCRIPT_DIR/output/small_chunks_results.json - 2KB chunk results"
echo "  • langextract_results.html - Interactive visualization"
echo
echo "🧩 Chunking Features Demonstrated:"
echo "  1. Intelligent document segmentation"
echo "  2. Parallel chunk processing"
echo "  3. Performance vs. granularity trade-offs"
echo "  4. Character-level alignment across chunks"
echo "  5. Consistent extraction across document sections"
echo
echo "📊 Key Insights:"
echo "  • Smaller chunks = more granular processing, longer time"
echo "  • Larger chunks = faster processing, potential missed entities"
echo "  • Optimal chunk size depends on document complexity"
echo "  • Parallel processing scales with chunk count"
echo
echo "🎯 Chunking Best Practices:"
echo "  • Use 4-8KB chunks for most documents"
echo "  • Increase workers for more chunks"
echo "  • Consider semantic boundaries (sentences, paragraphs)"
echo "  • Balance processing time vs. thoroughness"
echo
echo "🧪 Try This:"
echo "  • Compare extraction completeness across chunk sizes"
echo "  • Test with different document types and sizes"
echo "  • Experiment with worker/batch-size combinations"
echo "  • Combine chunking with multipass for maximum recall"
echo
echo "⚙️  Configuration Tips:"
echo "  • max-chars: 2000-8000 (document dependent)"
echo "  • workers: 4-12 (based on available resources)"
echo "  • batch-size: 4-8 (moderate for stability)"
echo "  • Use debug mode to see chunk boundaries and processing"
