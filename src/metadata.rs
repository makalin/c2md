use crate::error::{C2mdError, Result};
use std::path::Path;
use std::fs;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
    pub modification_date: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub page_count: Option<u32>,
    pub word_count: Option<u32>,
    pub character_count: Option<u32>,
    pub file_size: u64,
    pub format: String,
}

pub fn extract_metadata(path: &Path) -> Result<DocumentMetadata> {
    let metadata = fs::metadata(path)?;
    let file_size = metadata.len();
    let format = detect_format(path)?;
    
    let mut doc_metadata = DocumentMetadata {
        title: None,
        author: None,
        subject: None,
        keywords: None,
        creator: None,
        producer: None,
        creation_date: None,
        modification_date: Some(DateTime::<Utc>::from(metadata.modified()?)),
        language: None,
        page_count: None,
        word_count: None,
        character_count: None,
        file_size,
        format: format.clone(),
    };
    
    // Extract format-specific metadata
    match format.as_str() {
        "pdf" => extract_pdf_metadata(path, &mut doc_metadata)?,
        "office" => extract_office_metadata(path, &mut doc_metadata)?,
        "excel" => extract_excel_metadata(path, &mut doc_metadata)?,
        "powerpoint" => extract_powerpoint_metadata(path, &mut doc_metadata)?,
        "epub" => extract_epub_metadata(path, &mut doc_metadata)?,
        "html" => extract_html_metadata(path, &mut doc_metadata)?,
        "text" => extract_text_metadata(path, &mut doc_metadata)?,
        _ => {}
    }
    
    Ok(doc_metadata)
}

fn detect_format(path: &Path) -> Result<String> {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        match ext_str.as_str() {
            "pdf" => return Ok("pdf".to_string()),
            "doc" | "docx" => return Ok("office".to_string()),
            "xls" | "xlsx" => return Ok("excel".to_string()),
            "ppt" | "pptx" => return Ok("powerpoint".to_string()),
            "epub" => return Ok("epub".to_string()),
            "html" | "htm" => return Ok("html".to_string()),
            "txt" => return Ok("text".to_string()),
            _ => {}
        }
    }
    Ok("unknown".to_string())
}

fn extract_pdf_metadata(path: &Path, metadata: &mut DocumentMetadata) -> Result<()> {
    // Placeholder for PDF metadata extraction
    metadata.title = Some("PDF Document".to_string());
    metadata.author = Some("Unknown".to_string());
    Ok(())
}

fn extract_office_metadata(path: &Path, metadata: &mut DocumentMetadata) -> Result<()> {
    // Placeholder for Office metadata extraction
    metadata.title = Some("Office Document".to_string());
    metadata.author = Some("Unknown".to_string());
    Ok(())
}

fn extract_excel_metadata(path: &Path, metadata: &mut DocumentMetadata) -> Result<()> {
    // Placeholder for Excel metadata extraction
    metadata.title = Some("Excel Spreadsheet".to_string());
    Ok(())
}

fn extract_powerpoint_metadata(path: &Path, metadata: &mut DocumentMetadata) -> Result<()> {
    // Placeholder for PowerPoint metadata extraction
    metadata.title = Some("PowerPoint Presentation".to_string());
    metadata.author = Some("Unknown".to_string());
    Ok(())
}

fn extract_epub_metadata(path: &Path, metadata: &mut DocumentMetadata) -> Result<()> {
    // Placeholder for EPUB metadata extraction
    metadata.title = Some("EPUB Book".to_string());
    metadata.author = Some("Unknown".to_string());
    Ok(())
}

fn extract_html_metadata(path: &Path, metadata: &mut DocumentMetadata) -> Result<()> {
    let content = fs::read_to_string(path)?;
    
    // Extract title from HTML
    if let Some(title) = extract_html_tag(&content, "title") {
        metadata.title = Some(title);
    }
    
    // Extract meta tags
    if let Some(author) = extract_html_meta(&content, "author") {
        metadata.author = Some(author);
    }
    if let Some(description) = extract_html_meta(&content, "description") {
        metadata.subject = Some(description);
    }
    if let Some(keywords) = extract_html_meta(&content, "keywords") {
        metadata.keywords = Some(keywords);
    }
    
    // Count words and characters
    let text_content = strip_html_tags(&content);
    metadata.word_count = Some(count_words(&text_content));
    metadata.character_count = Some(text_content.len() as u32);
    
    Ok(())
}

fn extract_text_metadata(path: &Path, metadata: &mut DocumentMetadata) -> Result<()> {
    let content = fs::read_to_string(path)?;
    
    // Try to extract title from first line
    if let Some(first_line) = content.lines().next() {
        let trimmed = first_line.trim();
        if trimmed.len() < 100 && !trimmed.is_empty() {
            metadata.title = Some(trimmed.to_string());
        }
    }
    
    // Count words and characters
    metadata.word_count = Some(count_words(&content));
    metadata.character_count = Some(content.len() as u32);
    
    Ok(())
}

fn extract_html_tag(html: &str, tag: &str) -> Option<String> {
    let pattern = format!("<{}[^>]*>([^<]+)</{}>", tag, tag);
    if let Ok(re) = regex::Regex::new(&pattern) {
        if let Some(caps) = re.captures(html) {
            return Some(caps.get(1)?.as_str().trim().to_string());
        }
    }
    None
}

fn extract_html_meta(html: &str, name: &str) -> Option<String> {
    let pattern = format!(r#"<meta[^>]*name=["']{}["'][^>]*content=["']([^"']+)["'][^>]*>"#, name);
    if let Ok(re) = regex::Regex::new(&pattern) {
        if let Some(caps) = re.captures(html) {
            return Some(caps.get(1)?.as_str().to_string());
        }
    }
    None
}

fn strip_html_tags(html: &str) -> String {
    let re = regex::Regex::new(r"<[^>]*>").unwrap_or_else(|_| regex::Regex::new("").unwrap());
    re.replace_all(html, "").to_string()
}

fn count_words(text: &str) -> u32 {
    text.split_whitespace().count() as u32
}

pub fn format_metadata_as_markdown(metadata: &DocumentMetadata) -> String {
    let mut markdown = String::new();
    
    markdown.push_str("## Document Metadata\n\n");
    
    if let Some(title) = &metadata.title {
        markdown.push_str(&format!("- **Title**: {}\n", title));
    }
    
    if let Some(author) = &metadata.author {
        markdown.push_str(&format!("- **Author**: {}\n", author));
    }
    
    if let Some(subject) = &metadata.subject {
        markdown.push_str(&format!("- **Subject**: {}\n", subject));
    }
    
    if let Some(keywords) = &metadata.keywords {
        markdown.push_str(&format!("- **Keywords**: {}\n", keywords));
    }
    
    if let Some(creator) = &metadata.creator {
        markdown.push_str(&format!("- **Creator**: {}\n", creator));
    }
    
    if let Some(producer) = &metadata.producer {
        markdown.push_str(&format!("- **Producer**: {}\n", producer));
    }
    
    if let Some(creation_date) = &metadata.creation_date {
        markdown.push_str(&format!("- **Created**: {}\n", creation_date.format("%Y-%m-%d %H:%M:%S UTC")));
    }
    
    if let Some(modification_date) = &metadata.modification_date {
        markdown.push_str(&format!("- **Modified**: {}\n", modification_date.format("%Y-%m-%d %H:%M:%S UTC")));
    }
    
    if let Some(language) = &metadata.language {
        markdown.push_str(&format!("- **Language**: {}\n", language));
    }
    
    if let Some(page_count) = &metadata.page_count {
        markdown.push_str(&format!("- **Pages**: {}\n", page_count));
    }
    
    if let Some(word_count) = &metadata.word_count {
        markdown.push_str(&format!("- **Words**: {}\n", word_count));
    }
    
    if let Some(character_count) = &metadata.character_count {
        markdown.push_str(&format!("- **Characters**: {}\n", character_count));
    }
    
    markdown.push_str(&format!("- **File Size**: {}\n", format_file_size(metadata.file_size)));
    markdown.push_str(&format!("- **Format**: {}\n", metadata.format));
    
    markdown
}

fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}