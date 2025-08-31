# Product Catalog Demo

This example demonstrates LangExtract's capabilities for **e-commerce and product catalog** processing, showing how to extract comprehensive product information from retail catalogs and product listings.

## What This Example Does

Extracts detailed product information from an electronics catalog containing:

- 📦 **Product Names**: Apple MacBook Pro, Samsung QLED TV, iPhone, etc.
- 🏷️  **Product Identifiers**: SKUs, UPCs, model numbers, product codes
- 💰 **Pricing Data**: MSRP, sale prices, discounts, contractor pricing
- 📊 **Technical Specs**: Processors, memory, display specifications
- 💊 **Medical/Pharma**: NDC codes, lot numbers, expiration dates
- 🔧 **Tool Specifications**: Warranties, contractor prices, features
- 🏪 **Inventory Data**: Stock status, availability, shipping codes

## Key Features Demonstrated

- 🛍️  **E-commerce data extraction** optimized for product catalogs
- 🏷️  **Multi-format identifier recognition** (SKU, UPC, GTIN, NDC, etc.)
- 💱 **Financial data parsing** with currency and pricing recognition
- 📊 **Technical specification extraction** for electronics and tools
- 🎯 **Category-specific optimization** for different product types
- 📈 **Performance analytics** for catalog processing efficiency

## Files

- **`examples.json`** - Comprehensive product training examples across categories
- **`config.yaml`** - Configuration optimized for product catalog processing
- **`sample_product_text.txt`** - Real-world product catalog data
- **`run.sh`** - Script demonstrating product extraction pipeline
- **`output/`** - Generated product data and visualizations

## Quick Start

```bash
# Ensure you have a provider running
ollama serve
ollama pull mistral

# Run the product catalog demo
./run.sh
```

## Sample Product Categories

The demo processes various product types:

### Electronics
- **Apple Products**: MacBook Pro, iPhone, accessories
- **Samsung TVs**: QLED, Neo QLED, with specifications
- **Specifications**: Processors, memory, display tech

### Pharmaceuticals  
- **Medications**: Lipitor, with NDC codes
- **Medical Info**: Generic names, strengths, lot numbers
- **Compliance**: Expiration dates, package sizes

### Tools & Hardware
- **Power Tools**: DeWalt drills, specifications
- **Pricing**: Regular and contractor pricing
- **Warranties**: Coverage periods and terms

### Consumer Goods
- **Footwear**: Nike shoes with style codes
- **Apparel**: Size ranges, color options
- **Release Data**: Launch dates and availability

## Expected Results

### Extraction Categories
```
PRODUCT_NAME: 15-20 items (Apple MacBook Pro, Samsung TV, etc.)
SKU: 12-18 items (MBP-M3-16-SLV-2TB, SAM-TV-85-8K-001, etc.)
PRICE: 10-15 items ($3,999.00, $4,299.99, etc.)
MODEL: 8-12 items (A3101, QN85QN900C, etc.)
UPC: 6-10 items (194253715726, 887276661234, etc.)
MATERIAL: 4-8 items (titanium, aluminum, etc.)
```

### Performance Metrics
- **Processing time**: ~45-75 seconds for full catalog
- **Extraction rate**: ~15-25 extractions per second
- **Product coverage**: 95%+ of identifiable products
- **Accuracy**: 90%+ for structured identifiers (SKUs, prices)

## Understanding Product Extraction

### Product Identifier Hierarchy
```
Product Name → Model → SKU → UPC/GTIN
"Apple MacBook Pro" → "M3 Max" → "MBP-M3-16-SLV-2TB" → "194253715726"
```

### Pricing Structure Recognition
```
MSRP: $3,999.00
Sale Price: $4,299.99  
Was: $5,499.99
Contractor Price: $129.00
```

### Technical Specification Parsing
```
Processor: M3 Max chip
Memory: 16-core CPU, 40-core GPU
Display: Liquid Retina XDR
Storage: 2TB SSD
```

## Configuration Optimization

### For Large Catalogs (1000+ products)
```yaml
max_char_buffer: 8000     # Larger chunks for product descriptions
max_workers: 12           # High parallelism  
batch_size: 8             # Increased throughput
temperature: 0.2          # Consistent product data
```

### For Pricing Accuracy
```yaml
temperature: 0.1          # Very consistent for financial data
enable_validation: true   # Validate price formats
type_coercion: true       # Convert currency strings to numbers
```

### For Technical Products
```yaml
additional_context: "Focus on technical specifications, model numbers, and detailed product features"
examples: technical_products.json  # Specialized examples
```

## Advanced Product Processing

### Multi-Category Extraction
The system handles diverse product categories in a single pass:

**Electronics**: Chips, displays, memory specs
**Pharmaceuticals**: NDC codes, dosages, expiration dates  
**Tools**: Power ratings, warranties, contractor pricing
**Fashion**: Sizes, colors, style codes, release dates

### Price and Financial Analysis
```bash
# Extract pricing data for analysis
jq '.extractions[] | select(.extraction_class | test("price|cost|msrp"))' results.json
```

### Product Code Validation
```bash
# Analyze product identifier patterns
jq '.extractions[] | select(.extraction_class | test("sku|upc|model"))' results.json
```

### Inventory and Availability
```bash
# Track stock status and availability
jq '.extractions[] | select(.extraction_class | test("stock|availability"))' results.json
```

## Real-World Applications

### E-commerce Platform Integration
```bash
# Process product feeds for online stores
lx-rs extract supplier_catalog.txt \
    --examples ecommerce_examples.json \
    --export csv \
    --export json
```

### Competitive Analysis
```bash
# Extract competitor product information  
lx-rs extract competitor_website.html \
    --examples product_examples.json \
    --export html \
    --show-intervals
```

### Inventory Management
```bash
# Process inventory reports
lx-rs extract inventory_report.pdf \
    --examples inventory_examples.json \
    --export csv
```

### Compliance and Regulation
```bash
# Extract pharmaceutical compliance data
lx-rs extract pharma_catalog.txt \
    --examples medical_examples.json \
    --validation \
    --export json
```

## Data Quality and Validation

### Product Code Validation
The system validates common product identifier formats:
- **UPC**: 12-digit numeric codes
- **SKU**: Alphanumeric with hyphens
- **Model**: Manufacturer-specific patterns
- **NDC**: Medical product codes (11 digits)

### Price Format Consistency
```
✅ Valid: $1,299.99, $149.00, €198.50
❌ Invalid: 149$, $1299, 1,299.99$
```

### Technical Specification Standards
```
✅ Valid: 16GB RAM, 2.4GHz, 1TB SSD
❌ Invalid: 16 gigs, 2.4 hertz, 1000 GB
```

## Visualization Features

### Interactive HTML Report
- **Color-coded products**: Different colors for each product category
- **Price highlighting**: Financial data in distinctive styling
- **Product grouping**: Related products visually connected
- **Technical specs**: Expandable specification details

### CSV Analysis Features
- **Pricing analysis**: Sort and filter by price ranges
- **Category breakdowns**: Pivot tables by product type
- **Identifier tracking**: Deduplicate by SKU/UPC
- **Inventory status**: Filter by availability

## Performance Optimization

### Large Catalog Processing
```bash
# Optimize for throughput
lx-rs extract large_catalog.txt \
    --workers 16 \
    --batch-size 8 \
    --max-chars 10000 \
    --temperature 0.2
```

### Memory-Efficient Processing
```bash
# Process large files with limited memory
lx-rs extract huge_catalog.txt \
    --workers 4 \
    --batch-size 2 \
    --max-chars 4000
```

### Parallel Catalog Processing
```bash
# Process multiple catalogs simultaneously
for catalog in catalogs/*.txt; do
  lx-rs extract "$catalog" \
    --examples product_examples.json \
    --export csv &
done
wait
```

## Troubleshooting

### Low Product Recognition Rate
- **Improve examples**: Add more product categories to examples.json
- **Adjust prompt**: Be more specific about product types
- **Lower temperature**: Use 0.1-0.2 for consistent extraction

### Missing Price Information
- **Check format examples**: Ensure price examples match catalog format
- **Add currency variants**: Include different currency symbols
- **Validate pricing patterns**: Ensure regex patterns match your data

### Inconsistent Product Codes
- **Standardize examples**: Use consistent SKU/UPC formats in training
- **Add validation rules**: Implement format checking
- **Manual verification**: Spot-check critical product identifiers

## Next Steps

- Try combining with **multipass_demo** for comprehensive product coverage
- Explore **validation_demo** for product data quality assurance
- Test with your own product catalogs and supplier feeds
- Create domain-specific examples for your product categories
- Implement automated catalog processing pipelines for production use
