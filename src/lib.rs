//! # c2md - Universal Markdown Converter
//!
//! A fast, scriptable, and configurable CLI tool for converting various document formats to Markdown.
//!
//! ## Features
//!
//! - Convert PDF, Word (.doc/.docx), Excel (.xls/.xlsx), PowerPoint (.ppt/.pptx), RTF, TXT, HTML, EPUB, images (OCR), and more to clean Markdown
//! - Smart structure detection: headings, tables, lists, footnotes, links, images, code fences
//! - Image handling: keep as external refs, download & relink, or inline as data URIs
//! - OCR support for scanned PDFs/images using Tesseract
//! - Pluggable backends using best tool per format (Pandoc, LibreOffice headless, pdfminer, etc.)
//! - Config-first approach with per-project `c2md.yaml` for styles/paths/filters
//! - Batch & watch modes: convert single files, folders, or glob patterns
//! - Deterministic output with stable IDs, slugified headings, reproducible runs
//!
//! ## Example Usage
//!
//! ```rust
//! use c2md::{convert, Options};
//!
//! let options = Options::default();
//! let markdown = convert("input.pdf", options)?;
//! std::fs::write("output.md", markdown)?;
//! ```
//!
//! ## Supported Formats
//!
//! - **PDF**: Digital and scanned (with OCR)
//! - **Office**: Word (.doc/.docx), Excel (.xls/.xlsx), PowerPoint (.ppt/.pptx)
//! - **Text**: Plain text, RTF
//! - **Web**: HTML, EPUB
//! - **Data**: CSV
//! - **Images**: JPG, PNG, GIF, BMP, TIFF, WebP (with OCR)
//!
//! ## Dependencies
//!
//! For best results, install these optional dependencies:
//! - `pandoc` - Universal document converter
//! - `tesseract` - OCR engine
//! - `libreoffice` - Office document processing
//! - `poppler` - PDF text extraction

pub mod cli;
pub mod config;
pub mod converter;
pub mod error;
pub mod utils;
pub mod validator;
pub mod metadata;
pub mod template;

pub use config::Config;
pub use error::{C2mdError, Result};

/// Convert a single file to Markdown
pub fn convert(input_path: &str, options: Options) -> Result<String> {
    let path = std::path::Path::new(input_path);
    let config = Config::default();
    
    // Detect format and convert
    let format = utils::get_file_extension(path)?;
    
    match format.as_str() {
        "pdf" => converter::pdf::convert_pdf(path, &config, &options.to_args()),
        "doc" | "docx" => converter::office::convert_office(path, &config, &options.to_args()),
        "xls" | "xlsx" => converter::office::convert_excel(path, &config, &options.to_args()),
        "ppt" | "pptx" => converter::office::convert_powerpoint(path, &config, &options.to_args()),
        "rtf" => converter::rtf::convert_rtf(path, &config, &options.to_args()),
        "txt" => converter::text::convert_text(path, &config, &options.to_args()),
        "html" | "htm" => converter::html::convert_html(path, &config, &options.to_args()),
        "epub" => converter::epub::convert_epub(path, &config, &options.to_args()),
        "csv" => converter::csv::convert_csv(path, &config, &options.to_args()),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" => {
            converter::image::convert_image(path, &config, &options.to_args())
        }
        _ => Err(C2mdError::UnsupportedFormat(format)),
    }
}

/// Conversion options
#[derive(Debug, Clone)]
pub struct Options {
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub frontmatter: String,
    pub wrap: String,
    pub width: usize,
    pub tables: String,
    pub headings: String,
    pub slug: String,
    pub list_style: String,
    pub code_fence: String,
    pub images: String,
    pub ocr: bool,
    pub ocr_lang: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            date: None,
            frontmatter: "yaml".to_string(),
            wrap: "soft".to_string(),
            width: 100,
            tables: "grid".to_string(),
            headings: "atx".to_string(),
            slug: "github".to_string(),
            list_style: "dash".to_string(),
            code_fence: "```".to_string(),
            images: "keep".to_string(),
            ocr: false,
            ocr_lang: None,
        }
    }
}

impl Options {
    fn to_args(&self) -> cli::Args {
        cli::Args {
            inputs: vec![],
            output: None,
            out_dir: None,
            preserve_structure: false,
            from: None,
            to: "gfm".to_string(),
            encoding: None,
            headings: self.headings.clone(),
            slug: self.slug.clone(),
            wrap: self.wrap.clone(),
            width: self.width,
            tables: self.tables.clone(),
            list_style: self.list_style.clone(),
            code_fence: self.code_fence.clone(),
            frontmatter: self.frontmatter.clone(),
            title: self.title.clone(),
            author: self.author.clone(),
            date: self.date.clone(),
            images: self.images.clone(),
            assets_dir: None,
            image_max_width: None,
            ocr: self.ocr,
            ocr_lang: self.ocr_lang.clone(),
            pdf_layout: "auto".to_string(),
            libreoffice_bin: None,
            sheet: None,
            sheets: "all".to_string(),
            math: "auto".to_string(),
            math_block: "$$".to_string(),
            watch: false,
            jobs: None,
            dry_run: false,
            verbose: false,
            config: None,
            template: "default".to_string(),
            include_metadata: false,
            validate: false,
            metadata_only: false,
        }
    }
}