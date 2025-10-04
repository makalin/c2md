use thiserror::Error;

#[derive(Error, Debug)]
pub enum C2mdError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Conversion error: {0}")]
    Conversion(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Missing dependency: {0}")]
    MissingDependency(String),

    #[error("OCR error: {0}")]
    Ocr(String),

    #[error("Pandoc error: {0}")]
    Pandoc(String),

    #[error("LibreOffice error: {0}")]
    LibreOffice(String),

    #[error("PDF processing error: {0}")]
    Pdf(String),

    #[error("Image processing error: {0}")]
    Image(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_yaml::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Glob error: {0}")]
    Glob(#[from] glob::GlobError),

    #[error("Pattern error: {0}")]
    Pattern(#[from] glob::PatternError),

    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),

    #[error("Tempfile error: {0}")]
    Tempfile(#[from] tempfile::PersistError),


    #[error("Notify error: {0}")]
    Notify(#[from] notify::Error),

    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Generic error: {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, C2mdError>;