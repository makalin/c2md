use crate::cli::Args;
use crate::config::Config;
use crate::error::{C2mdError, Result};
use std::path::Path;

pub fn convert_rtf(path: &Path, config: &Config, args: &Args) -> Result<String> {
    // Placeholder for RTF conversion
    let mut markdown = String::new();
    
    // Add front matter
    if config.frontmatter != "none" {
        markdown.push_str("---\n");
        if let Some(title) = &args.title {
            markdown.push_str(&format!("title: {}\n", title));
        }
        if let Some(author) = &args.author {
            markdown.push_str(&format!("author: {}\n", author));
        }
        if let Some(date) = &args.date {
            markdown.push_str(&format!("date: {}\n", date));
        } else {
            markdown.push_str(&format!("date: {}\n", chrono::Utc::now().format("%Y-%m-%d")));
        }
        markdown.push_str("---\n\n");
    }
    
    markdown.push_str("# RTF Document\n\n");
    markdown.push_str("**Note**: RTF conversion requires external tools.\n\n");
    markdown.push_str("To enable RTF conversion, install:\n");
    markdown.push_str("- `pandoc` (for universal conversion)\n");
    markdown.push_str("- `unrtf` (for RTF-specific conversion)\n\n");
    
    markdown.push_str("## File Information\n\n");
    markdown.push_str(&format!("- **File**: {}\n", path.file_name().unwrap().to_string_lossy()));
    markdown.push_str("- **Format**: RTF Document\n");
    
    Ok(markdown)
}