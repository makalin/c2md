use crate::error::{C2mdError, Result};
use handlebars::{Handlebars, RenderError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateContext {
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub content: String,
    pub metadata: Option<HashMap<String, String>>,
    pub file_info: Option<FileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub format: String,
    pub modified: String,
}

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();
        
        // Register built-in templates
        Self::register_builtin_templates(&mut handlebars)?;
        
        // Register helper functions
        Self::register_helpers(&mut handlebars)?;
        
        Ok(Self { handlebars })
    }
    
    fn register_builtin_templates(handlebars: &mut Handlebars) -> Result<()> {
        // Default template
        let default_template = r#"---
{{#if title}}title: {{title}}{{/if}}
{{#if author}}author: {{author}}{{/if}}
{{#if date}}date: {{date}}{{/if}}
---

{{content}}

{{#if metadata}}
## Metadata
{{#each metadata}}
- **{{@key}}**: {{this}}
{{/each}}
{{/if}}

{{#if file_info}}
## File Information
- **Name**: {{file_info.name}}
- **Size**: {{file_info.size}}
- **Format**: {{file_info.format}}
- **Modified**: {{file_info.modified}}
{{/if}}
"#;
        
        handlebars.register_template_string("default", default_template)
            .map_err(|e| C2mdError::Generic(format!("Template error: {}", e)))?;
        
        // Minimal template
        let minimal_template = r#"{{content}}"#;
        handlebars.register_template_string("minimal", minimal_template)
            .map_err(|e| C2mdError::Generic(format!("Template error: {}", e)))?;
        
        // Academic template
        let academic_template = r#"---
title: {{#if title}}{{title}}{{else}}Untitled Document{{/if}}
author: {{#if author}}{{author}}{{else}}Unknown{{/if}}
date: {{#if date}}{{date}}{{else}}{{now}}{{/if}}
abstract: |
  This document was converted from {{#if file_info}}{{file_info.format}}{{else}}unknown format{{/if}}.
---

# {{#if title}}{{title}}{{else}}Untitled Document{{/if}}

{{#if author}}
**Author**: {{author}}
{{/if}}

{{#if date}}
**Date**: {{date}}
{{/if}}

---

{{content}}

---

{{#if metadata}}
## Document Metadata
{{#each metadata}}
- **{{@key}}**: {{this}}
{{/each}}
{{/if}}
"#;
        
        handlebars.register_template_string("academic", academic_template)
            .map_err(|e| C2mdError::Generic(format!("Template error: {}", e)))?;
        
        Ok(())
    }
    
    fn register_helpers(handlebars: &mut Handlebars) -> Result<()> {
        // Format file size helper
        handlebars.register_helper("format_size", Box::new(format_size_helper));
        
        // Current date helper
        handlebars.register_helper("now", Box::new(now_helper));
        
        // Word count helper
        handlebars.register_helper("word_count", Box::new(word_count_helper));
        
        Ok(())
    }
    
    pub fn render(&self, template_name: &str, context: &TemplateContext) -> Result<String> {
        self.handlebars.render(template_name, context)
            .map_err(|e| C2mdError::Generic(format!("Template rendering error: {}", e)))
    }
    
    pub fn load_template_from_file(&mut self, name: &str, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path)?;
        self.handlebars.register_template_string(name, content)
            .map_err(|e| C2mdError::Generic(format!("Template error: {}", e)))?;
        Ok(())
    }
    
    pub fn list_templates(&self) -> Vec<String> {
        self.handlebars.get_templates().keys().cloned().collect()
    }
}

fn format_size_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> std::result::Result<(), RenderError> {
    if let Some(size) = h.param(0).and_then(|v| v.value().as_u64()) {
        let formatted = format_file_size(size);
        out.write(&formatted)?;
    }
    Ok(())
}

fn now_helper(
    _: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> std::result::Result<(), RenderError> {
    let now = chrono::Utc::now().format("%Y-%m-%d");
    out.write(&now.to_string())?;
    Ok(())
}

fn word_count_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> std::result::Result<(), RenderError> {
    if let Some(text) = h.param(0).and_then(|v| v.value().as_str()) {
        let count = text.split_whitespace().count();
        out.write(&count.to_string())?;
    }
    Ok(())
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

pub fn create_template_context(
    title: Option<String>,
    author: Option<String>,
    date: Option<String>,
    content: String,
    metadata: Option<HashMap<String, String>>,
    file_info: Option<FileInfo>,
) -> TemplateContext {
    TemplateContext {
        title,
        author,
        date,
        content,
        metadata,
        file_info,
    }
}

pub fn load_custom_templates(engine: &mut TemplateEngine, template_dir: &Path) -> Result<()> {
    if !template_dir.exists() {
        return Ok(());
    }
    
    for entry in fs::read_dir(template_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "hbs" || extension == "handlebars" {
                    if let Some(stem) = path.file_stem() {
                        let template_name = stem.to_string_lossy().to_string();
                        engine.load_template_from_file(&template_name, &path)?;
                    }
                }
            }
        }
    }
    
    Ok(())
}