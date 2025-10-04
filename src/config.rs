use crate::error::{C2mdError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub to: String,
    pub wrap: String,
    pub width: usize,
    pub frontmatter: String,
    pub slug: String,
    pub tables: String,
    pub images: ImageConfig,
    pub pdf: PdfConfig,
    pub ocr: OcrConfig,
    pub math: MathConfig,
    pub batch: BatchConfig,
    pub ignore: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub mode: String,
    pub assets_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfConfig {
    pub layout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    pub enabled: bool,
    pub lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathConfig {
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConfig {
    pub jobs: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            to: "gfm".to_string(),
            wrap: "soft".to_string(),
            width: 100,
            frontmatter: "yaml".to_string(),
            slug: "github".to_string(),
            tables: "grid".to_string(),
            images: ImageConfig {
                mode: "download".to_string(),
                assets_dir: "assets".to_string(),
            },
            pdf: PdfConfig {
                layout: "smart".to_string(),
            },
            ocr: OcrConfig {
                enabled: false,
                lang: "eng".to_string(),
            },
            math: MathConfig {
                mode: "auto".to_string(),
            },
            batch: BatchConfig {
                jobs: "auto".to_string(),
            },
            ignore: vec![
                "**/node_modules/**".to_string(),
                "**/.git/**".to_string(),
            ],
        }
    }
}

impl Config {
    pub fn load(config_path: &Option<PathBuf>) -> Result<Self> {
        let mut config = Self::default();

        if let Some(path) = config_path {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                let file_config: Config = serde_yaml::from_str(&content)?;
                config = file_config;
            }
        } else {
            // Try to load from default locations
            let default_paths = vec![
                PathBuf::from("c2md.yaml"),
                PathBuf::from("c2md.yml"),
                PathBuf::from(".c2md.yaml"),
                PathBuf::from(".c2md.yml"),
            ];

            for path in default_paths {
                if path.exists() {
                    let content = fs::read_to_string(&path)?;
                    let file_config: Config = serde_yaml::from_str(&content)?;
                    config = file_config;
                    break;
                }
            }
        }

        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = serde_yaml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}