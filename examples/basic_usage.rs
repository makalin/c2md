use c2md::{convert, Options};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic conversion
    let options = Options::default();
    let markdown = convert("example.pdf", options)?;
    println!("{}", markdown);
    
    // Advanced conversion with custom options
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
    std::fs::write("output.md", markdown)?;
    
    Ok(())
}