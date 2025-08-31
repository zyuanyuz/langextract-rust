# Visualization Demo

This example demonstrates LangExtract's **rich visualization and export** capabilities, showing how extraction results can be presented in multiple interactive and shareable formats.

## What This Example Does

Extracts entities from a company directory and demonstrates **4 different export formats**:

- 🌐 **Interactive HTML** - Color-coded highlighting with hover tooltips
- 📊 **CSV Export** - Structured data for spreadsheet analysis  
- 📝 **Markdown** - Documentation-ready format with entity highlighting
- 📋 **JSON** - Programmatic access to structured data

**Entity Types Extracted:**
- 👥 **People**: Names, titles, roles
- 📧 **Contact Info**: Emails, phones, addresses
- 🏢 **Companies**: Organizations, divisions
- 💰 **Financial Data**: Salaries, budgets, valuations
- 🌐 **URLs**: Websites, social media, documentation
- 📍 **Locations**: Cities, addresses, offices

## Key Features Demonstrated

- 🎨 **Interactive HTML visualization** with color-coded entities
- 📊 **Statistical dashboards** showing extraction metrics
- 🔍 **Character-level highlighting** with precise positioning
- 📱 **Responsive design** for mobile and desktop viewing
- 📈 **Multiple export formats** for different use cases

## Files

- **`examples.json`** - Training examples for comprehensive entity extraction
- **`config.yaml`** - Configuration optimized for visualization
- **`input.txt`** - Company directory with diverse entity types
- **`run.sh`** - Script demonstrating all visualization formats
- **`output/`** - Generated visualization files

## Quick Start

```bash
# Ensure you have a provider running
ollama serve
ollama pull mistral

# Run the visualization demo
./run.sh
```

## Generated Outputs

The demo creates 4 different visualization formats:

### 1. Interactive HTML (`interactive_visualization.html`)
**Features:**
- **Color-coded entities**: Different colors for each entity type
- **Hover tooltips**: Show character positions and entity details
- **Statistics panel**: Extraction counts and performance metrics
- **Responsive layout**: Works on desktop and mobile
- **Interactive elements**: Click entities for detailed information

**Best for:** Presentations, reports, web integration

### 2. CSV Export (`structured_data.csv`)
**Columns:** entity_type, entity_text, start_position, end_position, document_section

**Features:**
- **Spreadsheet-ready**: Direct import to Excel/Google Sheets
- **Sortable data**: Filter and sort by any column
- **Pivot table support**: Aggregate by entity type
- **Database import**: Ready for SQL databases

**Best for:** Data analysis, BI tools, database integration

### 3. Markdown Export (`highlighted_document.md`)
**Features:**
- **GitHub-compatible**: Renders in GitHub, GitLab, and markdown editors
- **Entity highlighting**: Entities marked with special formatting
- **Table of contents**: Automatic navigation
- **Documentation-ready**: Perfect for wikis and documentation

**Best for:** Documentation, GitHub repos, technical writing

### 4. JSON Export (`extraction_data.json`)
**Features:**
- **Programmatic access**: Easy integration with applications
- **Character positions**: Precise start/end positions
- **Metadata included**: Extraction confidence, alignment status
- **API-ready**: Direct consumption by web services

**Best for:** Application integration, APIs, data processing

## Understanding the HTML Visualization

### Color Coding System
```
🔵 Blue     - People (names, titles, roles)
🟢 Green    - Contact info (emails, phones, addresses)  
🟡 Gold     - Financial data (prices, salaries, budgets)
🟣 Purple   - URLs and web resources
🔴 Red      - Locations and addresses
🟠 Orange   - Companies and organizations
```

### Interactive Features
- **Hover effects**: See character positions and entity metadata
- **Click details**: Expanded information for complex entities
- **Statistics panel**: Real-time metrics and extraction quality
- **Search functionality**: Find specific entities quickly
- **Export buttons**: Download data in different formats

### Customization Options
The HTML template supports:
- **Custom CSS**: Modify colors, fonts, and styling
- **Branding**: Add company logos and themes  
- **Additional metadata**: Include custom entity attributes
- **JavaScript hooks**: Add interactive functionality

## Real-World Use Cases

### Business Intelligence
```bash
# Extract company data for competitive analysis
lx-rs extract competitor_report.txt \
    --examples business_examples.json \
    --export html \
    --export csv
```

### Document Processing
```bash
# Process legal documents with highlighting
lx-rs extract contract.txt \
    --examples legal_examples.json \
    --export html \
    --export markdown
```

### Research Analysis
```bash
# Extract research data with visualizations
lx-rs extract research_paper.pdf \
    --examples academic_examples.json \
    --export html \
    --show-intervals
```

### Content Management
```bash
# Process content for CMS integration
lx-rs extract articles/ \
    --examples content_examples.json \
    --export json \
    --export csv
```

## Integration Examples

### Web Application Integration
```javascript
// Load JSON data into web application
fetch('extraction_data.json')
  .then(response => response.json())
  .then(data => {
    data.extractions.forEach(entity => {
      console.log(`${entity.extraction_class}: ${entity.extraction_text}`);
    });
  });
```

### Spreadsheet Analysis
```excel
# Excel pivot table from CSV
1. Import structured_data.csv
2. Create pivot table: entity_type (rows) vs count (values)
3. Filter by character position ranges
4. Generate charts and graphs
```

### Documentation Generation
```markdown
# Include highlighted markdown in documentation
1. Copy highlighted_document.md content
2. Add to documentation system
3. Entities automatically highlighted
4. Table of contents generated
```

### Database Integration
```sql
-- Import CSV data to database
CREATE TABLE extractions (
  entity_type VARCHAR(50),
  entity_text VARCHAR(200),
  start_pos INT,
  end_pos INT,
  document_id VARCHAR(50)
);

LOAD DATA FROM 'structured_data.csv';
```

## Customizing Visualizations

### Custom HTML Styling
```css
/* Add to HTML template */
.entity-person { 
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
.entity-financial { 
  background: #f6ad55; 
  border: 2px solid #ed8936;
}
```

### Markdown Templates
```markdown
# Custom markdown formatting
**Person**: {{entity_text}} ({{start_pos}}-{{end_pos}})
*Contact*: {{entity_text}}
```

### CSV Column Customization
```yaml
# Add custom columns to CSV export
csv_columns:
  - entity_type
  - entity_text  
  - confidence_score
  - document_section
  - extraction_method
```

## Performance Considerations

### File Size Management
- **Large documents**: HTML files can be large with many entities
- **Optimization**: Use pagination or filtering for 1000+ entities
- **Mobile**: Consider simplified mobile versions

### Browser Compatibility
- **Modern browsers**: Full feature support in Chrome, Firefox, Safari
- **IE compatibility**: Basic functionality only
- **Mobile browsers**: Responsive design works on all devices

### Export Timing
```
JSON export:     ~1-2 seconds
CSV export:      ~2-3 seconds  
Markdown export: ~3-5 seconds
HTML export:     ~5-10 seconds (includes styling and JavaScript)
```

## Troubleshooting

### HTML Not Displaying Properly
- **Check file size**: Large files may load slowly
- **Browser cache**: Clear cache and reload
- **JavaScript errors**: Check browser console for errors
- **Character encoding**: Ensure UTF-8 encoding

### CSV Import Issues
- **Delimiter problems**: CSV uses commas, check for embedded commas in text
- **Character encoding**: Use UTF-8 when importing
- **Large files**: Split large CSV files for Excel compatibility

### Markdown Rendering Issues
- **Special characters**: Some markdown parsers handle entities differently
- **Table formatting**: Complex tables may need manual adjustment
- **Link formatting**: URLs may need escaping

## Advanced Features

### Batch Visualization
```bash
# Process multiple documents
for file in documents/*.txt; do
  lx-rs extract "$file" \
    --examples examples.json \
    --export html \
    --output "visualizations/$(basename "$file" .txt).html"
done
```

### Custom Templates
```bash
# Use custom HTML template
lx-rs extract document.txt \
    --export html \
    --template custom_template.html \
    --css custom_styles.css
```

### API Integration
```bash
# Generate JSON for API consumption
lx-rs extract document.txt \
    --export json \
    --api-format \
    --include-metadata
```

## Next Steps

- Try combining visualization with **multipass_demo** for comprehensive entity coverage
- Explore **product_catalog** for e-commerce visualization scenarios
- Create custom templates for your specific domain
- Integrate visualizations into your web applications or documentation systems
- Experiment with different styling and branding options
