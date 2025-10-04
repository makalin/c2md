use c2md::{convert, Options};
use c2md::validator::validate_file;
use c2md::metadata::extract_metadata;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic conversion with enhanced options
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
    std::fs::write("output.md", markdown)?;
    
    // File validation
    let file_info = validate_file(Path::new("document.pdf"))?;
    println!("File validation: {:?}", file_info);
    
    // Metadata extraction
    let metadata = extract_metadata(Path::new("document.pdf"))?;
    println!("Document metadata: {:?}", metadata);
    
    // Advanced image processing with OCR
    let mut image_options = Options::default();
    image_options.ocr = true;
    image_options.ocr_lang = Some("eng+tur".to_string());
    image_options.images = "download".to_string();
    
    let image_markdown = convert("scanned_document.png", image_options)?;
    std::fs::write("image_output.md", image_markdown)?;
    
    // Office document conversion
    let mut office_options = Options::default();
    office_options.tables = "grid".to_string();
    office_options.frontmatter = "yaml".to_string();
    
    let office_markdown = convert("report.docx", office_options)?;
    std::fs::write("office_output.md", office_markdown)?;
    
    // EPUB conversion
    let epub_markdown = convert("book.epub", Options::default())?;
    std::fs::write("epub_output.md", epub_markdown)?;
    
    // CSV to markdown table
    let csv_markdown = convert("data.csv", Options::default())?;
    std::fs::write("csv_output.md", csv_markdown)?;
    
    println!("All conversions completed successfully!");
    Ok(())
}