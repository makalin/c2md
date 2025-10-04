use crate::cli::Args;
use crate::config::Config;
use crate::error::{C2mdError, Result};
use std::fs;
use std::path::Path;
use regex::Regex;
use chrono::{DateTime, Utc};

pub fn convert_text(path: &Path, config: &Config, args: &Args) -> Result<String> {
    let content = fs::read_to_string(path)?;
    
    let mut markdown = String::new();
    
    // Add front matter
    if config.frontmatter != "none" {
        markdown.push_str("---\n");
        if let Some(title) = &args.title {
            markdown.push_str(&format!("title: {}\n", title));
        } else {
            // Try to extract title from content
            if let Some(title) = extract_title_from_text(&content) {
                markdown.push_str(&format!("title: {}\n", title));
            }
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
    
    // Convert plain text to markdown
    let processed_content = text_to_markdown(&content, config, args)?;
    markdown.push_str(&processed_content);
    
    Ok(markdown)
}

fn extract_title_from_text(text: &str) -> Option<String> {
    // Try to find title in first few lines
    let lines: Vec<&str> = text.lines().take(5).collect();
    
    for line in lines {
        let trimmed = line.trim();
        if !trimmed.is_empty() && trimmed.len() < 100 {
            // Check if it looks like a title
            if is_title_candidate(trimmed) {
                return Some(trimmed.to_string());
            }
        }
    }
    
    None
}

fn is_title_candidate(line: &str) -> bool {
    // Simple heuristics for title detection
    line.len() > 5 && line.len() < 80 && 
    !line.contains(".") && 
    !line.contains(",") &&
    !line.starts_with(" ") &&
    !line.ends_with(" ")
}

fn text_to_markdown(text: &str, config: &Config, args: &Args) -> Result<String> {
    let mut markdown = String::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut in_list = false;
    let mut in_code = false;
    let mut in_quote = false;
    let mut current_paragraph = String::new();
    
    for line in lines {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            // End current block
            if in_list {
                markdown.push('\n');
                in_list = false;
            }
            if in_code {
                markdown.push('\n');
                in_code = false;
            }
            if in_quote {
                markdown.push('\n');
                in_quote = false;
            }
            if !current_paragraph.is_empty() {
                markdown.push_str(&current_paragraph);
                markdown.push_str("\n\n");
                current_paragraph.clear();
            }
            continue;
        }
        
        // Detect headings (lines that are all caps or start with numbers)
        if is_heading(trimmed) {
            end_current_block(&mut markdown, &mut current_paragraph, &mut in_list, &mut in_code, &mut in_quote);
            let level = heading_level(trimmed);
            markdown.push_str(&format!("{} {}\n\n", "#".repeat(level), trimmed));
            continue;
        }
        
        // Detect lists
        if trimmed.starts_with("•") || trimmed.starts_with("-") || trimmed.starts_with("*") {
            end_current_block(&mut markdown, &mut current_paragraph, &mut in_list, &mut in_code, &mut in_quote);
            if !in_list {
                markdown.push('\n');
                in_list = true;
            }
            let item = trimmed.trim_start_matches(|c| c == '•' || c == '-' || c == '*').trim();
            let bullet = "-"; // Default to dash for lists
            markdown.push_str(&format!("{} {}\n", bullet, item));
            continue;
        }
        
        // Detect numbered lists
        if let Ok(re) = Regex::new(r"^\d+\.\s+(.+)") {
            if let Some(caps) = re.captures(trimmed) {
                end_current_block(&mut markdown, &mut current_paragraph, &mut in_list, &mut in_code, &mut in_quote);
                if !in_list {
                    markdown.push('\n');
                    in_list = true;
                }
                let item = caps.get(1).unwrap().as_str();
                markdown.push_str(&format!("1. {}\n", item));
                continue;
            }
        }
        
        // Detect code blocks
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            end_current_block(&mut markdown, &mut current_paragraph, &mut in_list, &mut in_code, &mut in_quote);
            in_code = !in_code;
            markdown.push_str(&format!("{}\n", trimmed));
            continue;
        }
        
        // Detect quotes
        if trimmed.starts_with(">") {
            end_current_block(&mut markdown, &mut current_paragraph, &mut in_list, &mut in_code, &mut in_quote);
            if !in_quote {
                markdown.push('\n');
                in_quote = true;
            }
            let quote_text = trimmed.trim_start_matches('>').trim();
            markdown.push_str(&format!("> {}\n", quote_text));
            continue;
        }
        
        // Detect inline code
        if trimmed.contains('`') {
            end_current_block(&mut markdown, &mut current_paragraph, &mut in_list, &mut in_code, &mut in_quote);
            markdown.push_str(&format!("{}\n", line));
            continue;
        }
        
        // Regular paragraph
        if in_list || in_code || in_quote {
            end_current_block(&mut markdown, &mut current_paragraph, &mut in_list, &mut in_code, &mut in_quote);
        }
        
        // Add line to current paragraph
        if !current_paragraph.is_empty() {
            current_paragraph.push(' ');
        }
        current_paragraph.push_str(line);
    }
    
    // Add any remaining content
    if !current_paragraph.is_empty() {
        markdown.push_str(&current_paragraph);
    }
    
    // Apply wrapping if requested
    if config.wrap == "hard" {
        markdown = wrap_text(&markdown, config.width);
    }
    
    Ok(markdown)
}

fn end_current_block(
    markdown: &mut String,
    current_paragraph: &mut String,
    in_list: &mut bool,
    in_code: &mut bool,
    in_quote: &mut bool,
) {
    if *in_list {
        markdown.push('\n');
        *in_list = false;
    }
    if *in_code {
        markdown.push('\n');
        *in_code = false;
    }
    if *in_quote {
        markdown.push('\n');
        *in_quote = false;
    }
    if !current_paragraph.is_empty() {
        markdown.push_str(current_paragraph);
        markdown.push_str("\n\n");
        current_paragraph.clear();
    }
}

fn is_heading(line: &str) -> bool {
    // Enhanced heuristics for headings
    line.len() < 100 && (
        line.chars().all(|c| c.is_uppercase() || c.is_whitespace() || c.is_ascii_punctuation()) ||
        line.starts_with(char::is_numeric) ||
        line.contains("CHAPTER") ||
        line.contains("SECTION") ||
        line.contains("PART") ||
        line.contains("INTRODUCTION") ||
        line.contains("CONCLUSION") ||
        line.contains("ABSTRACT") ||
        line.contains("SUMMARY") ||
        line.contains("APPENDIX")
    )
}

fn heading_level(line: &str) -> usize {
    if line.len() < 20 {
        1
    } else if line.len() < 40 {
        2
    } else if line.len() < 60 {
        3
    } else {
        4
    }
}

fn wrap_text(text: &str, width: usize) -> String {
    let mut result = String::new();
    
    for line in text.lines() {
        if line.len() <= width {
            result.push_str(line);
            result.push('\n');
        } else {
            let words: Vec<&str> = line.split_whitespace().collect();
            let mut current_line = String::new();
            
            for word in words {
                if current_line.len() + word.len() + 1 <= width {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                } else {
                    if !current_line.is_empty() {
                        result.push_str(&current_line);
                        result.push('\n');
                    }
                    current_line = word.to_string();
                }
            }
            
            if !current_line.is_empty() {
                result.push_str(&current_line);
                result.push('\n');
            }
        }
    }
    
    result
}