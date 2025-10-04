use crate::cli::Args;
use crate::config::Config;
use crate::error::{C2mdError, Result};
use std::fs;
use std::path::Path;
use image::{DynamicImage, ImageFormat};
use base64::Engine;
use std::process::Command;

pub fn convert_image(path: &Path, config: &Config, args: &Args) -> Result<String> {
    let mut markdown = String::new();
    
    // Add front matter
    if config.frontmatter != "none" {
        markdown.push_str("---\n");
        if let Some(title) = &args.title {
            markdown.push_str(&format!("title: {}\n", title));
        }
        markdown.push_str("---\n\n");
    }
    
    // Handle image based on strategy
    match config.images.mode.as_str() {
        "keep" => {
            markdown.push_str(&format!("![Image]({})\n", path.display()));
        }
        "download" => {
            let asset_path = download_image(path, config, args)?;
            markdown.push_str(&format!("![Image]({})\n", asset_path.display()));
        }
        "inline" => {
            let data_uri = inline_image(path)?;
            markdown.push_str(&format!("![Image]({})\n", data_uri));
        }
        _ => {
            return Err(C2mdError::Generic("Invalid image mode".to_string()));
        }
    }
    
    // If OCR is enabled, try to extract text
    if args.ocr || config.ocr.enabled {
        match extract_text_from_image(path, &config.ocr.lang) {
            Ok(text) => {
                if !text.trim().is_empty() {
                    markdown.push_str("\n## Extracted Text\n\n");
                    markdown.push_str(&text);
                    markdown.push('\n');
                }
            }
            Err(e) => {
                // OCR failed, but continue with image
                tracing::warn!("OCR failed for {}: {}", path.display(), e);
            }
        }
    }
    
    // Add image metadata
    if let Ok(metadata) = extract_image_metadata(path) {
        markdown.push_str("\n## Image Metadata\n\n");
        markdown.push_str(&format!("- **File**: {}\n", path.file_name().unwrap().to_string_lossy()));
        markdown.push_str(&format!("- **Format**: {}\n", metadata.format));
        markdown.push_str(&format!("- **Dimensions**: {}x{}\n", metadata.width, metadata.height));
        if let Some(size) = metadata.file_size {
            markdown.push_str(&format!("- **File Size**: {}\n", format_file_size(size)));
        }
        markdown.push('\n');
    }
    
    Ok(markdown)
}

fn download_image(path: &Path, config: &Config, args: &Args) -> Result<std::path::PathBuf> {
    let default_assets_dir = std::path::PathBuf::from(&config.images.assets_dir);
    let assets_dir = args.assets_dir.as_ref()
        .unwrap_or(&default_assets_dir);
    
    // Create assets directory if it doesn't exist
    fs::create_dir_all(assets_dir)?;
    
    // Generate unique filename
    let stem = path.file_stem()
        .ok_or_else(|| C2mdError::Generic("Invalid file name".to_string()))?;
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("png");
    
    let filename = format!("{}.{}", stem.to_string_lossy(), ext);
    let asset_path = assets_dir.join(filename);
    
    // Copy image to assets directory
    fs::copy(path, &asset_path)?;
    
    // Optionally resize image
    if let Some(max_width) = args.image_max_width {
        resize_image(&asset_path, max_width)?;
    }
    
    Ok(asset_path)
}

fn inline_image(path: &Path) -> Result<String> {
    let image_data = fs::read(path)?;
    let mime_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();
    
    let encoded = base64::engine::general_purpose::STANDARD.encode(&image_data);
    Ok(format!("data:{};base64,{}", mime_type, encoded))
}

fn extract_text_from_image(path: &Path, lang: &str) -> Result<String> {
    // Check if tesseract is available
    if which::which("tesseract").is_err() {
        return Err(C2mdError::MissingDependency("Tesseract OCR not found. Please install tesseract-ocr".to_string()));
    }

    let output = Command::new("tesseract")
        .args(&[path.to_str().unwrap(), "stdout", "-l", lang])
        .output()?;

    if !output.status.success() {
        return Err(C2mdError::Ocr(format!("OCR failed: {}", String::from_utf8_lossy(&output.stderr))));
    }

    let text = String::from_utf8(output.stdout)?;
    
    // Clean up the OCR text
    let cleaned_text = clean_ocr_text(&text);
    Ok(cleaned_text)
}

fn clean_ocr_text(text: &str) -> String {
    let mut cleaned = text.to_string();
    
    // Remove excessive whitespace
    if let Ok(re) = regex::Regex::new(r"\n\s*\n\s*\n") {
        cleaned = re.replace_all(&cleaned, "\n\n").to_string();
    }
    
    // Fix common OCR errors
    if let Ok(re) = regex::Regex::new(r"([a-z])([A-Z])") {
        cleaned = re.replace_all(&cleaned, "$1 $2").to_string();
    }
    
    // Remove page numbers
    if let Ok(re) = regex::Regex::new(r"^\s*\d+\s*$") {
        cleaned = re.replace_all(&cleaned, "").to_string();
    }
    
    cleaned
}

fn resize_image(path: &Path, max_width: usize) -> Result<()> {
    let img = image::open(path)?;
    
    if img.width() > max_width as u32 {
        let new_height = (img.height() * max_width as u32) / img.width();
        let resized = img.resize(max_width as u32, new_height, image::imageops::FilterType::Lanczos3);
        resized.save(path)?;
    }
    
    Ok(())
}

#[derive(Debug)]
struct ImageMetadata {
    format: String,
    width: u32,
    height: u32,
    file_size: Option<u64>,
}

fn extract_image_metadata(path: &Path) -> Result<ImageMetadata> {
    let img = image::open(path)?;
    let file_size = fs::metadata(path).ok().map(|m| m.len());
    
    let format = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_uppercase();
    
    Ok(ImageMetadata {
        format,
        width: img.width(),
        height: img.height(),
        file_size,
    })
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