pub mod pdf;
pub mod office;
pub mod image;
pub mod text;
pub mod html;
pub mod epub;
pub mod csv;
pub mod rtf;

use crate::cli::Args;
use crate::config::Config;
use crate::error::{C2mdError, Result};
use crate::validator::{validate_file, FileInfo};
use crate::metadata::{extract_metadata, format_metadata_as_markdown};
use crate::template::{TemplateEngine, TemplateContext, create_template_context};
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use glob::glob;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use tracing::{info, error, warn};
use std::collections::HashMap;

pub struct Converter {
    config: Config,
    args: Args,
    template_engine: TemplateEngine,
}

impl Converter {
    pub fn new(config: Config, args: Args) -> Result<Self> {
        let template_engine = TemplateEngine::new()?;
        Ok(Self { config, args, template_engine })
    }

    pub async fn process(&self) -> Result<()> {
        let input_paths = self.collect_input_paths()?;
        
        if input_paths.is_empty() {
            return Err(C2mdError::Generic("No input files found".to_string()));
        }

        info!("Found {} input files", input_paths.len());

        if self.args.dry_run {
            self.show_plan(&input_paths)?;
            return Ok(());
        }

        if self.args.watch {
            self.watch_mode(&input_paths).await?;
        } else {
            self.batch_convert(&input_paths).await?;
        }

        Ok(())
    }

    fn collect_input_paths(&self) -> Result<Vec<std::path::PathBuf>> {
        let mut paths = Vec::new();

        for input in &self.args.inputs {
            if input.contains('*') || input.contains('?') {
                // Handle glob patterns
                for entry in glob(input)? {
                    let path = entry?;
                    if self.should_process_path(&path) {
                        paths.push(path);
                    }
                }
            } else {
                let path = std::path::PathBuf::from(input);
                if path.is_dir() {
                    // Recursively scan directory
                    for entry in WalkDir::new(&path) {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_file() && self.should_process_path(path) {
                            paths.push(path.to_path_buf());
                        }
                    }
                } else if path.is_file() {
                    paths.push(path);
                }
            }
        }

        Ok(paths)
    }

    fn should_process_path(&self, path: &Path) -> bool {
        // Check ignore patterns
        let path_str = path.to_string_lossy();
        for pattern in &self.config.ignore {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&path_str))
                .unwrap_or(false)
            {
                return false;
            }
        }

        // Check file extension
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            matches!(
                ext_str.as_str(),
                "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" |
                "rtf" | "txt" | "html" | "htm" | "epub" | "csv" |
                "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp"
            )
        } else {
            false
        }
    }

    fn show_plan(&self, paths: &[std::path::PathBuf]) -> Result<()> {
        println!("Conversion plan:");
        for path in paths {
            let output_path = self.determine_output_path(path)?;
            println!("  {} -> {}", path.display(), output_path.display());
        }
        Ok(())
    }

    fn determine_output_path(&self, input_path: &Path) -> Result<std::path::PathBuf> {
        if let Some(output) = &self.args.output {
            Ok(output.clone())
        } else if let Some(out_dir) = &self.args.out_dir {
            let mut output_path = out_dir.clone();
            
            if self.args.preserve_structure {
                // Preserve directory structure
                if let Some(parent) = input_path.parent() {
                    output_path = output_path.join(parent);
                }
            }
            
            let stem = input_path.file_stem()
                .ok_or_else(|| C2mdError::Generic("Invalid file name".to_string()))?;
            output_path.push(format!("{}.md", stem.to_string_lossy()));
            Ok(output_path)
        } else {
            // Default to stdout (represented as a special path)
            Ok(std::path::PathBuf::from("-"))
        }
    }

    async fn batch_convert(&self, paths: &[std::path::PathBuf]) -> Result<()> {
        let _jobs = self.args.jobs.unwrap_or_else(|| num_cpus::get());
        let pb = ProgressBar::new(paths.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        let results: Vec<Result<()>> = paths
            .par_iter()
            .with_max_len(1)
            .map(|path| {
                let result = self.convert_single_file(path);
                pb.inc(1);
                pb.set_message(format!("Processing {}", path.file_name().unwrap().to_string_lossy()));
                result
            })
            .collect();

        pb.finish_with_message("Conversion completed");

        let mut errors = 0;
        for (i, result) in results.into_iter().enumerate() {
            if let Err(e) = result {
                error!("Failed to convert {}: {}", paths[i].display(), e);
                errors += 1;
            }
        }

        if errors > 0 {
            warn!("{} files failed to convert", errors);
        }

        Ok(())
    }

    async fn watch_mode(&self, initial_paths: &[std::path::PathBuf]) -> Result<()> {
        use notify::{Watcher, RecursiveMode, Event, EventKind};
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)?;

        // Watch all directories containing input files
        let mut watched_dirs = std::collections::HashSet::new();
        for path in initial_paths {
            if let Some(parent) = path.parent() {
                watched_dirs.insert(parent.to_path_buf());
            }
        }

        for dir in watched_dirs {
            watcher.watch(&dir, RecursiveMode::Recursive)?;
            info!("Watching directory: {}", dir.display());
        }

        info!("Watch mode enabled. Press Ctrl+C to stop.");

        loop {
        match rx.recv() {
            Ok(Ok(Event { kind: EventKind::Modify(_), paths, .. })) => {
                    for path in paths {
                        if self.should_process_path(&path) {
                            info!("File changed: {}", path.display());
                            if let Err(e) = self.convert_single_file(&path) {
                                error!("Failed to convert {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            Ok(Ok(_)) => {} // Ignore other events
            Ok(Err(e)) => {
                error!("Watch error: {}", e);
                break;
            }
            Err(e) => {
                error!("Channel error: {}", e);
                break;
            }
            }
        }

        Ok(())
    }

    fn convert_single_file(&self, input_path: &Path) -> Result<()> {
        let output_path = self.determine_output_path(input_path)?;
        
        // Validate file first
        let file_info = validate_file(input_path)?;
        if !file_info.is_valid {
            return Err(C2mdError::Generic(format!("Invalid file: {}", file_info.error.unwrap_or("Unknown error".to_string()))));
        }
        
        // Extract metadata
        let metadata = extract_metadata(input_path)?;
        
        // Detect file format
        let format = self.detect_format(input_path)?;
        
        // Convert based on format
        let mut markdown = match format.as_str() {
            "pdf" => self.convert_pdf(input_path)?,
            "doc" | "docx" => self.convert_office(input_path)?,
            "xls" | "xlsx" => self.convert_excel(input_path)?,
            "ppt" | "pptx" => self.convert_powerpoint(input_path)?,
            "rtf" => self.convert_rtf(input_path)?,
            "txt" => self.convert_text(input_path)?,
            "html" | "htm" => self.convert_html(input_path)?,
            "epub" => self.convert_epub(input_path)?,
            "csv" => self.convert_csv(input_path)?,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" => {
                self.convert_image(input_path)?
            }
            _ => return Err(C2mdError::UnsupportedFormat(format)),
        };

        // Apply template if requested
        if self.args.frontmatter != "none" {
            let template_context = create_template_context(
                self.args.title.clone(),
                self.args.author.clone(),
                self.args.date.clone(),
                markdown,
                Some(self.metadata_to_hashmap(&metadata)),
                Some(self.file_info_to_template_info(&file_info)),
            );
            
            let template_name = self.get_template_name();
            markdown = self.template_engine.render(&template_name, &template_context)?;
        }

        // Write output
        if output_path.to_string_lossy() == "-" {
            print!("{}", markdown);
        } else {
            // Create output directory if needed
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&output_path, markdown)?;
            info!("Converted: {} -> {}", input_path.display(), output_path.display());
        }

        Ok(())
    }

    fn detect_format(&self, path: &Path) -> Result<String> {
        if let Some(ext) = path.extension() {
            Ok(ext.to_string_lossy().to_lowercase())
        } else {
            Err(C2mdError::Generic("Cannot detect file format".to_string()))
        }
    }

    fn convert_pdf(&self, path: &Path) -> Result<String> {
        pdf::convert_pdf(path, &self.config, &self.args)
    }

    fn convert_office(&self, path: &Path) -> Result<String> {
        office::convert_office(path, &self.config, &self.args)
    }

    fn convert_excel(&self, path: &Path) -> Result<String> {
        office::convert_excel(path, &self.config, &self.args)
    }

    fn convert_powerpoint(&self, path: &Path) -> Result<String> {
        office::convert_powerpoint(path, &self.config, &self.args)
    }

    fn convert_rtf(&self, path: &Path) -> Result<String> {
        rtf::convert_rtf(path, &self.config, &self.args)
    }

    fn convert_text(&self, path: &Path) -> Result<String> {
        text::convert_text(path, &self.config, &self.args)
    }

    fn convert_html(&self, path: &Path) -> Result<String> {
        html::convert_html(path, &self.config, &self.args)
    }

    fn convert_epub(&self, path: &Path) -> Result<String> {
        epub::convert_epub(path, &self.config, &self.args)
    }

    fn convert_csv(&self, path: &Path) -> Result<String> {
        csv::convert_csv(path, &self.config, &self.args)
    }

    fn convert_image(&self, path: &Path) -> Result<String> {
        image::convert_image(path, &self.config, &self.args)
    }
    
    fn metadata_to_hashmap(&self, metadata: &crate::metadata::DocumentMetadata) -> HashMap<String, String> {
        let mut map = HashMap::new();
        
        if let Some(title) = &metadata.title {
            map.insert("title".to_string(), title.clone());
        }
        if let Some(author) = &metadata.author {
            map.insert("author".to_string(), author.clone());
        }
        if let Some(subject) = &metadata.subject {
            map.insert("subject".to_string(), subject.clone());
        }
        if let Some(keywords) = &metadata.keywords {
            map.insert("keywords".to_string(), keywords.clone());
        }
        if let Some(creator) = &metadata.creator {
            map.insert("creator".to_string(), creator.clone());
        }
        if let Some(producer) = &metadata.producer {
            map.insert("producer".to_string(), producer.clone());
        }
        if let Some(language) = &metadata.language {
            map.insert("language".to_string(), language.clone());
        }
        if let Some(page_count) = &metadata.page_count {
            map.insert("pages".to_string(), page_count.to_string());
        }
        if let Some(word_count) = &metadata.word_count {
            map.insert("words".to_string(), word_count.to_string());
        }
        if let Some(character_count) = &metadata.character_count {
            map.insert("characters".to_string(), character_count.to_string());
        }
        
        map.insert("file_size".to_string(), format_file_size(metadata.file_size));
        map.insert("format".to_string(), metadata.format.clone());
        
        map
    }
    
    fn file_info_to_template_info(&self, file_info: &FileInfo) -> crate::template::FileInfo {
        crate::template::FileInfo {
            name: file_info.path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            size: file_info.size,
            format: file_info.format.clone(),
            modified: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        }
    }
    
    fn get_template_name(&self) -> String {
        // For now, use default template
        // In the future, this could be configurable
        "default".to_string()
    }
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