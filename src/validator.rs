use crate::error::{C2mdError, Result};
use std::path::Path;
use std::fs;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: std::path::PathBuf,
    pub size: u64,
    pub format: String,
    pub mime_type: Option<String>,
    pub is_valid: bool,
    pub error: Option<String>,
}

pub fn validate_file(path: &Path) -> Result<FileInfo> {
    let metadata = fs::metadata(path)?;
    let size = metadata.len();
    
    // Detect format
    let format = detect_format(path)?;
    
    // Detect MIME type
    let mime_type = detect_mime_type(path)?;
    
    // Validate file
    let (is_valid, error) = validate_format(path, &format)?;
    
    Ok(FileInfo {
        path: path.to_path_buf(),
        size,
        format,
        mime_type,
        is_valid,
        error,
    })
}

pub fn detect_format(path: &Path) -> Result<String> {
    // First try extension
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        match ext_str.as_str() {
            "pdf" => return Ok("pdf".to_string()),
            "doc" | "docx" => return Ok("office".to_string()),
            "xls" | "xlsx" => return Ok("excel".to_string()),
            "ppt" | "pptx" => return Ok("powerpoint".to_string()),
            "rtf" => return Ok("rtf".to_string()),
            "txt" => return Ok("text".to_string()),
            "html" | "htm" => return Ok("html".to_string()),
            "epub" => return Ok("epub".to_string()),
            "csv" => return Ok("csv".to_string()),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" => return Ok("image".to_string()),
            _ => {}
        }
    }
    
    // Try magic number detection
    detect_by_magic_number(path)
}

fn detect_by_magic_number(path: &Path) -> Result<String> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0; 16];
    file.read_exact(&mut buffer)?;
    
    // Check magic numbers
    if buffer.starts_with(b"%PDF") {
        return Ok("pdf".to_string());
    }
    
    if buffer.starts_with(b"\x89PNG") {
        return Ok("image".to_string());
    }
    
    if buffer.starts_with(b"\xFF\xD8\xFF") {
        return Ok("image".to_string());
    }
    
    if buffer.starts_with(b"GIF87a") || buffer.starts_with(b"GIF89a") {
        return Ok("image".to_string());
    }
    
    if buffer.starts_with(b"BM") {
        return Ok("image".to_string());
    }
    
    if buffer.starts_with(b"{\\rtf") {
        return Ok("rtf".to_string());
    }
    
    // Default to text if we can't determine
    Ok("text".to_string())
}

fn detect_mime_type(path: &Path) -> Result<Option<String>> {
    let mime_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();
    
    if mime_type == "application/octet-stream" {
        Ok(None)
    } else {
        Ok(Some(mime_type))
    }
}

fn validate_format(path: &Path, format: &str) -> Result<(bool, Option<String>)> {
    match format {
        "pdf" => validate_pdf(path),
        "office" | "excel" | "powerpoint" => validate_office(path),
        "rtf" => validate_rtf(path),
        "html" => validate_html(path),
        "epub" => validate_epub(path),
        "csv" => validate_csv(path),
        "image" => validate_image(path),
        "text" => validate_text(path),
        _ => Ok((true, None)), // Unknown format, assume valid
    }
}

fn validate_pdf(path: &Path) -> Result<(bool, Option<String>)> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    
    if buffer.starts_with(b"%PDF") {
        Ok((true, None))
    } else {
        Ok((false, Some("Invalid PDF file".to_string())))
    }
}

fn validate_office(path: &Path) -> Result<(bool, Option<String>)> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    
    if buffer.starts_with(b"PK\x03\x04") {
        Ok((true, None))
    } else {
        Ok((false, Some("Invalid Office document".to_string())))
    }
}

fn validate_rtf(path: &Path) -> Result<(bool, Option<String>)> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0; 5];
    file.read_exact(&mut buffer)?;
    
    if buffer.starts_with(b"{\\rtf") {
        Ok((true, None))
    } else {
        Ok((false, Some("Invalid RTF file".to_string())))
    }
}

fn validate_html(path: &Path) -> Result<(bool, Option<String>)> {
    let content = fs::read_to_string(path)?;
    
    // Check for HTML tags
    let re = regex::Regex::new(r"<[^>]+>")?;
    if re.is_match(&content) {
        Ok((true, None))
    } else {
        Ok((false, Some("No HTML tags found".to_string())))
    }
}

fn validate_epub(path: &Path) -> Result<(bool, Option<String>)> {
    // For now, just check if it's a ZIP file
    let mut file = fs::File::open(path)?;
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer)?;
    
    if buffer.starts_with(b"PK\x03\x04") {
        Ok((true, None))
    } else {
        Ok((false, Some("Invalid EPUB file".to_string())))
    }
}

fn validate_csv(path: &Path) -> Result<(bool, Option<String>)> {
    let content = fs::read_to_string(path)?;
    
    // Check for CSV structure (comma-separated values)
    let lines: Vec<&str> = content.lines().take(3).collect();
    if lines.is_empty() {
        return Ok((false, Some("Empty CSV file".to_string())));
    }
    
    let first_line = lines[0];
    if first_line.contains(',') {
        Ok((true, None))
    } else {
        Ok((false, Some("No comma separators found".to_string())))
    }
}

fn validate_image(path: &Path) -> Result<(bool, Option<String>)> {
    match image::open(path) {
        Ok(_) => Ok((true, None)),
        Err(e) => Ok((false, Some(format!("Invalid image: {}", e)))),
    }
}

fn validate_text(path: &Path) -> Result<(bool, Option<String>)> {
    // Text files are always valid
    Ok((true, None))
}

pub fn get_file_stats(path: &Path) -> Result<FileStats> {
    let metadata = fs::metadata(path)?;
    let size = metadata.len();
    let modified = metadata.modified()?;
    
    Ok(FileStats {
        size,
        modified: chrono::DateTime::<chrono::Utc>::from(modified),
        is_readable: true,
        is_writable: metadata.permissions().readonly(),
    })
}

#[derive(Debug, Clone)]
pub struct FileStats {
    pub size: u64,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub is_readable: bool,
    pub is_writable: bool,
}