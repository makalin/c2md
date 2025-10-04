use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "c2md",
    version = env!("CARGO_PKG_VERSION"),
    about = "Universal Markdown Converter (CLI)",
    long_about = "Convert PDF, Word (.doc/.docx), Excel (.xls/.xlsx), PowerPoint (.ppt/.pptx), RTF, TXT, HTML, EPUB, images (OCR), and more to clean Markdown."
)]
pub struct Args {
    /// Input file(s) or glob patterns
    #[arg(required = true)]
    pub inputs: Vec<String>,

    /// Output file (single input) or map to stdout (default)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Write outputs into DIR (batch mode)
    #[arg(long)]
    pub out_dir: Option<PathBuf>,

    /// Mirror input tree under --out-dir
    #[arg(long)]
    pub preserve_structure: bool,

    /// Force input format (auto-detected by default)
    #[arg(long)]
    pub from: Option<String>,

    /// Markdown flavor: md|gfm|commonmark (default: gfm)
    #[arg(long, default_value = "gfm")]
    pub to: String,

    /// Override input text encoding
    #[arg(long)]
    pub encoding: Option<String>,

    /// Heading style (default: atx)
    #[arg(long, default_value = "atx")]
    pub headings: String,

    /// Heading slug strategy
    #[arg(long, default_value = "github")]
    pub slug: String,

    /// Line wrapping (default: soft)
    #[arg(long, default_value = "soft")]
    pub wrap: String,

    /// Line width for wrapping
    #[arg(long, default_value = "100")]
    pub width: usize,

    /// Table style (auto→best-fit)
    #[arg(long, default_value = "auto")]
    pub tables: String,

    /// Unordered list bullet
    #[arg(long, default_value = "dash")]
    pub list_style: String,

    /// Fence token (default: ```)
    #[arg(long, default_value = "```")]
    pub code_fence: String,

    /// Emit front matter (default: yaml)
    #[arg(long, default_value = "yaml")]
    pub frontmatter: String,

    /// Override document title
    #[arg(long)]
    pub title: Option<String>,

    /// One or more authors
    #[arg(long)]
    pub author: Option<String>,

    /// Override date
    #[arg(long)]
    pub date: Option<String>,

    /// Image strategy (default: keep)
    #[arg(long, default_value = "keep")]
    pub images: String,

    /// Where to put downloaded images/files
    #[arg(long)]
    pub assets_dir: Option<PathBuf>,

    /// Add width hints in HTML wrapper if needed
    #[arg(long)]
    pub image_max_width: Option<usize>,

    /// Force OCR pass
    #[arg(long)]
    pub ocr: bool,

    /// OCR language codes (e.g., eng+tur)
    #[arg(long)]
    pub ocr_lang: Option<String>,

    /// Choose PDF extractor (keeps columns & lists)
    #[arg(long, default_value = "auto")]
    pub pdf_layout: String,

    /// Custom LibreOffice binary path
    #[arg(long)]
    pub libreoffice_bin: Option<PathBuf>,

    /// Only one sheet
    #[arg(long)]
    pub sheet: Option<String>,

    /// Select sheets for xlsx/csv
    #[arg(long, default_value = "all")]
    pub sheets: String,

    /// Convert equations to $...$ or leave
    #[arg(long, default_value = "auto")]
    pub math: String,

    /// Block math delimiters
    #[arg(long, default_value = "$$")]
    pub math_block: String,

    /// Rebuild on file changes
    #[arg(long)]
    pub watch: bool,

    /// Parallel workers (default: CPU cores)
    #[arg(long)]
    pub jobs: Option<usize>,

    /// Show plan without writing
    #[arg(long)]
    pub dry_run: bool,

    /// More logs
    #[arg(short, long)]
    pub verbose: bool,

    /// Use a specific config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Template to use for output formatting
    #[arg(long, default_value = "default")]
    pub template: String,

    /// Include metadata in output
    #[arg(long)]
    pub include_metadata: bool,

    /// Validate files before conversion
    #[arg(long)]
    pub validate: bool,

    /// Extract metadata only (no conversion)
    #[arg(long)]
    pub metadata_only: bool,
}

impl Clone for Args {
    fn clone(&self) -> Self {
        Self {
            inputs: self.inputs.clone(),
            output: self.output.clone(),
            out_dir: self.out_dir.clone(),
            preserve_structure: self.preserve_structure,
            from: self.from.clone(),
            to: self.to.clone(),
            encoding: self.encoding.clone(),
            headings: self.headings.clone(),
            slug: self.slug.clone(),
            wrap: self.wrap.clone(),
            width: self.width,
            tables: self.tables.clone(),
            list_style: self.list_style.clone(),
            code_fence: self.code_fence.clone(),
            frontmatter: self.frontmatter.clone(),
            title: self.title.clone(),
            author: self.author.clone(),
            date: self.date.clone(),
            images: self.images.clone(),
            assets_dir: self.assets_dir.clone(),
            image_max_width: self.image_max_width,
            ocr: self.ocr,
            ocr_lang: self.ocr_lang.clone(),
            pdf_layout: self.pdf_layout.clone(),
            libreoffice_bin: self.libreoffice_bin.clone(),
            sheet: self.sheet.clone(),
            sheets: self.sheets.clone(),
            math: self.math.clone(),
            math_block: self.math_block.clone(),
            watch: self.watch,
            jobs: self.jobs,
            dry_run: self.dry_run,
            verbose: self.verbose,
            config: self.config.clone(),
            template: self.template.clone(),
            include_metadata: self.include_metadata,
            validate: self.validate,
            metadata_only: self.metadata_only,
        }
    }
}