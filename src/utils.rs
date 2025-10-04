use crate::error::{C2mdError, Result};
use std::path::Path;
use slug::slugify;

pub fn slugify_heading(text: &str, strategy: &str) -> String {
    match strategy {
        "github" => slugify(text),
        "kebab" => slugify(text),
        "none" => text.to_string(),
        _ => slugify(text),
    }
}

pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' || c == '_' {
            c
        } else {
            '_'
        })
        .collect()
}

pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_file_extension(path: &Path) -> Result<String> {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .ok_or_else(|| C2mdError::Generic("Cannot determine file extension".to_string()))
}

pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(ext_str.as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp")
    } else {
        false
    }
}

pub fn is_office_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(ext_str.as_str(), "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx")
    } else {
        false
    }
}

pub fn is_text_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        matches!(ext_str.as_str(), "txt" | "md" | "rst" | "asciidoc")
    } else {
        false
    }
}

pub fn format_file_size(bytes: u64) -> String {
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

pub fn detect_encoding(content: &[u8]) -> Result<String> {
    // Simple encoding detection - in a real implementation, you'd use chardet or similar
    if content.starts_with(&[0xEF, 0xBB, 0xBF]) {
        Ok("utf-8".to_string())
    } else if content.starts_with(&[0xFF, 0xFE]) {
        Ok("utf-16le".to_string())
    } else if content.starts_with(&[0xFE, 0xFF]) {
        Ok("utf-16be".to_string())
    } else {
        // Try UTF-8 first
        match std::str::from_utf8(content) {
            Ok(_) => Ok("utf-8".to_string()),
            Err(_) => Ok("latin1".to_string()), // Fallback
        }
    }
}

pub fn normalize_path(path: &Path) -> Result<std::path::PathBuf> {
    Ok(path.canonicalize()?)
}

pub fn relative_path(_from: &Path, _to: &Path) -> Result<std::path::PathBuf> {
    // Simple relative path calculation
    // In a real implementation, you'd use pathdiff or similar
    Ok(std::path::PathBuf::from("."))
}