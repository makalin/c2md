# c2md API Documentation

## Library Usage

The `c2md` crate provides both a CLI tool and a library API for programmatic document conversion.

### Basic Usage

```rust
use c2md::{convert, Options};

let options = Options::default();
let markdown = convert("document.pdf", options)?;
std::fs::write("output.md", markdown)?;
```

### Advanced Usage

```rust
use c2md::{convert, Options};

let mut options = Options::default();
options.title = Some("My Document".to_string());
options.author = Some("John Doe".to_string());
options.frontmatter = "yaml".to_string();
options.wrap = "hard".to_string();
options.width = 80;
options.tables = "grid".to_string();
options.ocr = true;
options.ocr_lang = Some("eng".to_string());

let markdown = convert("scanned_document.pdf", options)?;
```

## Options Reference

### Options Struct

```rust
pub struct Options {
    pub title: Option<String>,           // Document title
    pub author: Option<String>,         // Document author
    pub date: Option<String>,           // Document date
    pub frontmatter: String,            // Front matter format: "yaml" | "json" | "none"
    pub wrap: String,                   // Text wrapping: "none" | "soft" | "hard"
    pub width: usize,                  // Line width for hard wrapping
    pub tables: String,                // Table style: "simple" | "grid" | "pipe" | "auto"
    pub headings: String,              // Heading style: "atx" | "setext"
    pub slug: String,                  // Heading slug strategy: "github" | "kebab" | "none"
    pub list_style: String,            // List bullet style: "dash" | "asterisk"
    pub code_fence: String,            // Code fence token: "```" | "~~~"
    pub images: String,                // Image strategy: "keep" | "download" | "inline"
    pub ocr: bool,                     // Enable OCR
    pub ocr_lang: Option<String>,      // OCR language codes
}
```

## Error Handling

All functions return `Result<T, C2mdError>` where `C2mdError` includes:

- `Io` - File I/O errors
- `Config` - Configuration errors
- `Conversion` - Document conversion errors
- `UnsupportedFormat` - Unsupported file format
- `MissingDependency` - Required external tool not found
- `Ocr` - OCR processing errors
- `Pandoc` - Pandoc conversion errors
- `LibreOffice` - LibreOffice conversion errors
- `Pdf` - PDF processing errors
- `Image` - Image processing errors
- `Network` - Network/HTTP errors
- `Serialization` - YAML/JSON parsing errors

## Supported Formats

### Input Formats

| Format | Extension | Notes |
|--------|-----------|-------|
| PDF | `.pdf` | Digital and scanned (OCR) |
| Word | `.doc`, `.docx` | Via LibreOffice or Pandoc |
| Excel | `.xls`, `.xlsx` | Tables converted to Markdown |
| PowerPoint | `.ppt`, `.pptx` | Via LibreOffice |
| RTF | `.rtf` | Via Pandoc |
| Plain Text | `.txt` | Smart formatting |
| HTML | `.html`, `.htm` | Via Pandoc |
| EPUB | `.epub` | E-book format |
| CSV | `.csv` | Tabular data |
| Images | `.jpg`, `.png`, `.gif`, `.bmp`, `.tiff`, `.webp` | With OCR |

### Output Format

- **Markdown**: CommonMark/GFM compatible
- **Front Matter**: YAML or JSON metadata
- **Tables**: Multiple styles (simple, grid, pipe)
- **Images**: External links, downloaded assets, or inline data URIs

## Configuration

The library respects the same configuration system as the CLI tool. See the main README for configuration file format.

## Examples

### Convert a PDF with OCR

```rust
use c2md::{convert, Options};

let mut options = Options::default();
options.ocr = true;
options.ocr_lang = Some("eng+tur".to_string());

let markdown = convert("multilingual_document.pdf", options)?;
```

### Convert Office Documents

```rust
use c2md::{convert, Options};

let mut options = Options::default();
options.tables = "grid".to_string();
options.frontmatter = "yaml".to_string();

let markdown = convert("report.docx", options)?;
```

### Convert Images with Text Extraction

```rust
use c2md::{convert, Options};

let mut options = Options::default();
options.ocr = true;
options.images = "inline".to_string();

let markdown = convert("scanned_page.png", options)?;
```

### Batch Processing

```rust
use std::fs;
use walkdir::WalkDir;

let mut options = Options::default();
options.frontmatter = "yaml".to_string();

for entry in WalkDir::new("documents/") {
    let entry = entry?;
    let path = entry.path();
    
    if path.is_file() {
        let markdown = convert(path.to_str().unwrap(), options.clone())?;
        let output_path = path.with_extension("md");
        fs::write(output_path, markdown)?;
    }
}
```