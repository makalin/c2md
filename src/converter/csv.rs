use crate::cli::Args;
use crate::config::Config;
use crate::error::{C2mdError, Result};
use std::fs;
use std::path::Path;
use csv::ReaderBuilder;

pub fn convert_csv(path: &Path, config: &Config, args: &Args) -> Result<String> {
    let mut markdown = String::new();
    
    // Add front matter
    if config.frontmatter != "none" {
        markdown.push_str("---\n");
        if let Some(title) = &args.title {
            markdown.push_str(&format!("title: {}\n", title));
        }
        markdown.push_str("---\n\n");
    }
    
    // Read CSV file
    let content = fs::read_to_string(path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());
    
    let headers = reader.headers()?.clone();
    let mut rows = Vec::new();
    
    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        rows.push(row);
    }
    
    // Convert to markdown table
    let header_vec: Vec<&str> = headers.iter().collect();
    markdown.push_str(&format_table(&header_vec, &rows, &config.tables));
    
    Ok(markdown)
}

fn format_table(headers: &[&str], rows: &[Vec<String>], style: &str) -> String {
    match style {
        "grid" => format_grid_table(headers, rows),
        "pipe" => format_pipe_table(headers, rows),
        _ => format_simple_table(headers, rows),
    }
}

fn format_grid_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut result = String::new();
    
    // Header row
    result.push('|');
    for header in headers {
        result.push_str(&format!(" {} |", header));
    }
    result.push('\n');
    
    // Separator row
    result.push('|');
    for _ in headers {
        result.push_str(" --- |");
    }
    result.push('\n');
    
    // Data rows
    for row in rows {
        result.push('|');
        for cell in row {
            result.push_str(&format!(" {} |", cell));
        }
        result.push('\n');
    }
    
    result
}

fn format_pipe_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut result = String::new();
    
    // Header row
    result.push('|');
    for header in headers {
        result.push_str(&format!(" {} |", header));
    }
    result.push('\n');
    
    // Separator row
    result.push('|');
    for _ in headers {
        result.push_str(" --- |");
    }
    result.push('\n');
    
    // Data rows
    for row in rows {
        result.push('|');
        for cell in row {
            result.push_str(&format!(" {} |", cell));
        }
        result.push('\n');
    }
    
    result
}

fn format_simple_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let mut result = String::new();
    
    // Header row
    for (i, header) in headers.iter().enumerate() {
        if i > 0 {
            result.push_str(" | ");
        }
        result.push_str(header);
    }
    result.push('\n');
    
    // Separator row
    for (i, _) in headers.iter().enumerate() {
        if i > 0 {
            result.push_str(" | ");
        }
        result.push_str("---");
    }
    result.push('\n');
    
    // Data rows
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                result.push_str(" | ");
            }
            result.push_str(cell);
        }
        result.push('\n');
    }
    
    result
}