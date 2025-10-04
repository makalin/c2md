# c2md Advanced Features

This document outlines the advanced features and capabilities of the c2md Universal Markdown Converter.

## 🚀 Enhanced Conversion Capabilities

### PDF Processing
- **Text Extraction**: Advanced PDF text extraction using pdf-extract
- **OCR Integration**: Tesseract OCR for scanned PDFs
- **Image Extraction**: Extract images from PDFs using pdfimages
- **Metadata Extraction**: Document properties, creation date, author
- **Smart Layout Detection**: Preserves document structure

### Office Documents
- **LibreOffice Integration**: Full LibreOffice headless conversion
- **Pandoc Fallback**: Universal document conversion
- **Excel Support**: Spreadsheet to markdown tables
- **PowerPoint Support**: Presentation to structured markdown
- **Metadata Preservation**: Author, title, creation date

### Image Processing
- **OCR Text Extraction**: Extract text from images using Tesseract
- **Multiple Formats**: JPG, PNG, GIF, BMP, TIFF, WebP
- **Image Metadata**: Dimensions, file size, format detection
- **Asset Management**: Download, inline, or keep references
- **Image Resizing**: Automatic resizing with max-width

### EPUB Support
- **Chapter Extraction**: Process all chapters in order
- **Metadata Extraction**: Title, author, publisher, language
- **Table of Contents**: Generate TOC from EPUB structure
- **HTML to Markdown**: Convert EPUB HTML content

### RTF Processing
- **Pandoc Integration**: High-quality RTF conversion
- **Format Preservation**: Maintains document structure
- **Metadata Support**: Author, title, date extraction

## 🔧 Advanced Features

### File Validation
```bash
# Validate files before conversion
c2md document.pdf --validate

# Check file integrity and format
c2md --validate *.pdf *.docx
```

### Metadata Extraction
```bash
# Extract metadata only
c2md document.pdf --metadata-only

# Include metadata in output
c2md document.pdf --include-metadata
```

### Template System
```bash
# Use different templates
c2md document.pdf --template academic
c2md document.pdf --template report
c2md document.pdf --template minimal
```

### Batch Processing
```bash
# Process entire directories
c2md documents/ --out-dir converted/ --preserve-structure

# Watch mode for automatic conversion
c2md documents/ --watch --out-dir converted/

# Parallel processing
c2md documents/ --jobs 8
```

## 📊 Output Formats

### Front Matter Options
- **YAML**: Standard YAML front matter
- **JSON**: JSON metadata format
- **None**: Plain markdown without metadata

### Table Styles
- **Grid**: GitHub-style tables with borders
- **Pipe**: Standard markdown pipe tables
- **Simple**: Minimal table format
- **Auto**: Automatic best-fit selection

### Heading Styles
- **ATX**: `# Heading` style (default)
- **Setext**: Underlined headings

### Text Wrapping
- **Soft**: Preserve original line breaks
- **Hard**: Wrap to specified width
- **None**: No wrapping

## 🎨 Template System

### Built-in Templates

#### Default Template
```yaml
---
title: Document Title
author: Author Name
date: 2024-01-01
---

# Content here

## Metadata
- **Format**: PDF
- **Size**: 1.2 MB
```

#### Academic Template
```yaml
---
title: Research Paper
author: Dr. Jane Smith
date: 2024-01-01
abstract: |
  This document was converted from PDF.
---

# Research Paper

**Author**: Dr. Jane Smith
**Date**: 2024-01-01

---

# Content here

---

## Document Metadata
- **Pages**: 25
- **Words**: 5,000
```

#### Report Template
```yaml
# Quarterly Report

**Prepared by**: John Doe
**Date**: 2024-01-01
**Source**: report.pdf (PDF, 1.2 MB)

---

## Executive Summary

# Content here

---

## Document Details
- **Format**: PDF
- **Pages**: 25
```

### Custom Templates
Create custom templates using Handlebars syntax:

```handlebars
---
title: {{#if title}}{{title}}{{else}}Untitled{{/if}}
author: {{#if author}}{{author}}{{else}}Unknown{{/if}}
date: {{#if date}}{{date}}{{else}}{{now}}{{/if}}
---

# {{#if title}}{{title}}{{else}}Untitled Document{{/if}}

{{#if author}}
**Author**: {{author}}
{{/if}}

{{content}}

{{#if metadata}}
## Document Information
{{#each metadata}}
- **{{@key}}**: {{this}}
{{/each}}
{{/if}}
```

## 🔍 File Validation

### Supported Validations
- **PDF**: Magic number and structure validation
- **Office**: ZIP structure and XML validation
- **RTF**: RTF format validation
- **HTML**: Tag structure validation
- **EPUB**: Container and manifest validation
- **CSV**: Comma-separated structure validation
- **Images**: Format and integrity validation

### Validation Output
```bash
File validation: FileInfo {
    path: "document.pdf",
    size: 1234567,
    format: "pdf",
    mime_type: Some("application/pdf"),
    is_valid: true,
    error: None
}
```

## 📈 Metadata Extraction

### Document Metadata
- **Title**: Document title
- **Author**: Document author/creator
- **Subject**: Document subject
- **Keywords**: Document keywords
- **Creator**: Application that created the document
- **Producer**: Application that produced the document
- **Creation Date**: When the document was created
- **Modification Date**: When the document was last modified
- **Language**: Document language
- **Page Count**: Number of pages
- **Word Count**: Number of words
- **Character Count**: Number of characters
- **File Size**: File size in bytes
- **Format**: Document format

### Example Metadata Output
```yaml
title: "Research Paper"
author: "Dr. Jane Smith"
subject: "Machine Learning"
keywords: "AI, ML, research"
creator: "Microsoft Word"
producer: "Microsoft Word"
creation_date: "2024-01-01T10:00:00Z"
modification_date: "2024-01-02T15:30:00Z"
language: "en"
page_count: 25
word_count: 5000
character_count: 30000
file_size: 1234567
format: "pdf"
```

## 🛠️ Advanced Configuration

### Configuration File (c2md.yaml)
```yaml
# Output format
to: gfm                    # md | gfm | commonmark

# Text wrapping
wrap: soft                 # none | soft | hard
width: 100                 # Line width for hard wrapping

# Front matter
frontmatter: yaml          # yaml | json | none

# Heading formatting
slug: github               # github | kebab | none

# Table formatting
tables: grid               # simple | grid | pipe | auto

# Image handling
images:
  mode: download           # keep | download | inline
  assets_dir: assets       # Directory for downloaded images

# PDF processing
pdf:
  layout: smart            # auto | raw | smart

# OCR settings
ocr:
  enabled: false           # Enable OCR for scanned documents
  lang: eng                # Language codes (e.g., eng+tur)

# Math rendering
math:
  mode: auto               # auto | katex | none

# Batch processing
batch:
  jobs: auto               # Number of parallel workers (auto = CPU cores)

# File patterns to ignore
ignore:
  - "**/node_modules/**"
  - "**/.git/**"
  - "**/.*"
  - "**/*.tmp"
  - "**/*.temp"
  - "**/*.bak"
  - "**/*.backup"

# Template settings
template:
  default: "default"       # Default template name
  custom_dir: "templates"  # Directory for custom templates

# Validation settings
validation:
  enabled: true            # Enable file validation
  strict: false            # Strict validation mode
```

## 🚀 Performance Features

### Parallel Processing
- **Multi-threaded**: Uses all CPU cores by default
- **Configurable**: Set number of worker threads
- **Progress Tracking**: Real-time progress bars
- **Error Handling**: Continue processing on individual file errors

### Memory Management
- **Streaming**: Process large files without loading entirely into memory
- **Efficient**: Optimized for memory usage
- **Caching**: Smart caching for repeated operations

### Watch Mode
- **File Monitoring**: Automatically detect file changes
- **Incremental**: Only process changed files
- **Real-time**: Immediate conversion on file save

## 🔧 External Dependencies

### Required for Full Functionality
- **Pandoc**: Universal document converter
- **LibreOffice**: Office document processing
- **Tesseract**: OCR engine
- **Poppler**: PDF text extraction (pdfimages, pdftotext)

### Installation Commands

#### macOS (Homebrew)
```bash
brew install pandoc libreoffice tesseract poppler
```

#### Ubuntu/Debian
```bash
sudo apt-get install pandoc libreoffice tesseract-ocr poppler-utils
```

#### Windows (Chocolatey)
```bash
choco install pandoc libreoffice tesseract poppler
```

## 📚 API Usage

### Library Integration
```rust
use c2md::{convert, Options};
use c2md::validator::validate_file;
use c2md::metadata::extract_metadata;

// Basic conversion
let options = Options::default();
let markdown = convert("document.pdf", options)?;

// File validation
let file_info = validate_file(Path::new("document.pdf"))?;

// Metadata extraction
let metadata = extract_metadata(Path::new("document.pdf"))?;
```

### Advanced Options
```rust
let mut options = Options::default();
options.title = Some("My Document".to_string());
options.author = Some("John Doe".to_string());
options.frontmatter = "yaml".to_string();
options.wrap = "hard".to_string();
options.width = 80;
options.tables = "grid".to_string();
options.ocr = true;
options.ocr_lang = Some("eng".to_string());

let markdown = convert("document.pdf", options)?;
```

## 🎯 Use Cases

### Academic Research
- Convert research papers to markdown for analysis
- Extract metadata for citation management
- OCR scanned documents for text analysis

### Content Management
- Convert documents for static site generators
- Batch process documentation
- Maintain document archives

### Data Processing
- Convert spreadsheets to markdown tables
- Process CSV data for analysis
- Extract text from images

### Publishing
- Convert manuscripts to markdown
- Process EPUB books for web publishing
- Generate documentation from various sources

This comprehensive feature set makes c2md a powerful tool for document conversion and processing workflows.