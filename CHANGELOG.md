# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of c2md - Universal Markdown Converter
- Support for PDF conversion (digital and scanned with OCR)
- Support for Office documents (Word, Excel, PowerPoint)
- Support for text formats (RTF, TXT, HTML, EPUB)
- Support for data formats (CSV)
- Support for image formats (JPG, PNG, GIF, BMP, TIFF, WebP) with OCR
- Configurable output formatting (tables, headings, lists, code fences)
- Front matter support (YAML, JSON)
- Image handling strategies (keep, download, inline)
- Batch processing with parallel workers
- Watch mode for automatic re-conversion
- Comprehensive configuration system via `c2md.yaml`
- CLI interface with extensive options
- Library API for programmatic usage
- Comprehensive test suite
- Documentation and examples

### Features
- Smart structure detection for headings, tables, lists, footnotes, links, images, code fences
- Deterministic output with stable IDs and slugified headings
- Pluggable backends using best tool per format (Pandoc, LibreOffice, pdfminer, Tesseract)
- Config-first approach with per-project settings
- Cross-platform support (macOS, Linux, Windows)

### Dependencies
- Pandoc for universal document conversion
- LibreOffice for Office document processing
- Tesseract for OCR capabilities
- Poppler for PDF text extraction
- Various Rust crates for core functionality

## [0.1.0] - 2024-01-01

### Added
- Initial implementation
- Basic CLI interface
- Core conversion engine
- Configuration system
- Error handling
- Logging system
- Test framework