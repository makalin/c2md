use crate::cli::Args;
use crate::config::Config;
use crate::error::{C2mdError, Result};
use std::fs;
use std::path::Path;

pub fn convert_html(path: &Path, config: &Config, args: &Args) -> Result<String> {
    let html_content = fs::read_to_string(path)?;
    
    // Clean HTML
    let clean_html = ammonia::clean(&html_content);
    
    // Convert to markdown using pulldown-cmark
    let markdown = html_to_markdown(&clean_html, config, args)?;
    
    Ok(markdown)
}

fn html_to_markdown(html: &str, config: &Config, args: &Args) -> Result<String> {
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
    
    // Simple HTML to Markdown conversion
    // This is a basic implementation - for production use, consider using pandoc
    let mut content = html.to_string();
    
    // Convert common HTML tags to Markdown
    content = content.replace("<h1>", "# ");
    content = content.replace("</h1>", "\n\n");
    content = content.replace("<h2>", "## ");
    content = content.replace("</h2>", "\n\n");
    content = content.replace("<h3>", "### ");
    content = content.replace("</h3>", "\n\n");
    content = content.replace("<h4>", "#### ");
    content = content.replace("</h4>", "\n\n");
    content = content.replace("<h5>", "##### ");
    content = content.replace("</h5>", "\n\n");
    content = content.replace("<h6>", "###### ");
    content = content.replace("</h6>", "\n\n");
    
    content = content.replace("<p>", "");
    content = content.replace("</p>", "\n\n");
    
    content = content.replace("<strong>", "**");
    content = content.replace("</strong>", "**");
    content = content.replace("<b>", "**");
    content = content.replace("</b>", "**");
    
    content = content.replace("<em>", "*");
    content = content.replace("</em>", "*");
    content = content.replace("<i>", "*");
    content = content.replace("</i>", "*");
    
    content = content.replace("<code>", "`");
    content = content.replace("</code>", "`");
    
    content = content.replace("<pre>", "```\n");
    content = content.replace("</pre>", "\n```\n");
    
    content = content.replace("<ul>", "");
    content = content.replace("</ul>", "\n");
    content = content.replace("<ol>", "");
    content = content.replace("</ol>", "\n");
    content = content.replace("<li>", "- ");
    content = content.replace("</li>", "\n");
    
    content = content.replace("<br>", "\n");
    content = content.replace("<br/>", "\n");
    content = content.replace("<br />", "\n");
    
    // Remove remaining HTML tags
    let re = regex::Regex::new(r"<[^>]*>")?;
    content = re.replace_all(&content, "").to_string();
    
    // Clean up whitespace
    let re = regex::Regex::new(r"\n\s*\n\s*\n")?;
    content = re.replace_all(&content, "\n\n").to_string();
    
    markdown.push_str(&content);
    
    // Apply wrapping if requested
    if config.wrap == "hard" {
        markdown = wrap_text(&markdown, config.width);
    }
    
    Ok(markdown)
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