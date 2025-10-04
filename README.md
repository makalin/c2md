# c2md — Universal Markdown Converter (CLI)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](#license)
![platforms](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-informational)
![lang](https://img.shields.io/badge/impl-Rust%20CLI-blue)
[![Build](https://img.shields.io/github/actions/workflow/status/makalin/c2md/ci.yml?label=build)](./.github/workflows/ci.yml)

Convert **PDF, Word (.doc/.docx), Excel (.xls/.xlsx), PowerPoint (.ppt/.pptx), RTF, TXT, HTML, EPUB, images (OCR),** and more to clean **Markdown**.
Fast, scriptable, and configurable for terminal workflows.

---

## Highlights

* 🔁 **Many in → one out:** Normalizes dozens of formats to Markdown (CommonMark/GFM).
* 🧠 **Smart structure:** Headings, tables, lists, footnotes, links, images, code fences.
* 🖼️ **Image handling:** Keep as external refs, download & relink, or inline as data URIs.
* 🔤 **OCR when needed:** Extract text from scanned PDFs/images (Tesseract).
* 🧩 **Pluggable backends:** Uses best tool per format (Pandoc, LibreOffice headless, pdfminer, unrtf, csv, etc.).
* ⚙️ **Config-first:** Per-project `c2md.yaml` for styles/paths/filters.
* 🚀 **Batch & watch:** Convert single files, folders, or glob patterns; optional `--watch`.
* 🧪 **Deterministic output:** Stable IDs, slugified headings, reproducible runs.

---

## Install

> Prebuilt binaries: see **Releases**.

**Homebrew (macOS/Linux):**

```bash
brew tap you/tap
brew install c2md
```

**Cargo (Rust toolchain):**

```bash
cargo install c2md
```

**Scoop (Windows):**

```powershell
scoop bucket add you-bucket https://github.com/makalin/scoop-bucket
scoop install c2md
```

> Optional deps for best results: `pandoc`, `tesseract` (OCR), `libreoffice` (office → intermediary), `poppler` (`pdftotext`/`pdfimages`).

---

## Quick start

```bash
# Single file → Markdown (stdout)
c2md report.pdf

# Save to file
c2md contract.docx -o contract.md

# Batch convert a folder (recursively) preserving tree
c2md docs/**/*.pdf --out-dir md/ --preserve-structure

# Convert spreadsheets to tables, one sheet per section
c2md finance.xlsx -o finance.md --sheets all --tables grid

# OCR a scanned PDF in Turkish
c2md scan.pdf -o scan.md --ocr --ocr-lang tur

# Download images next to the MD and relink
c2md slides.pptx -o slides.md --images download --assets-dir assets/slides

# Watch a directory and rebuild on change
c2md src_docs/ --watch --out-dir md/
```

---

## Usage

````text
c2md <input...> [options]

Inputs:
  File(s) or globs. Directories are scanned recursively unless --no-recursive.

Core options:
  -o, --output <FILE>              Output file (single input) or map to stdout (default).
      --out-dir <DIR>              Write outputs into DIR (batch mode).
      --preserve-structure         Mirror input tree under --out-dir.
      --from <fmt>                 Force input format (auto-detected by default).
      --to <fmt>                   Markdown flavor: md|gfm|commonmark (default: gfm)
      --encoding <enc>             Override input text encoding.

Structure & style:
      --headings <atx|setext>      Heading style (default: atx).
      --slug <github|kebab|none>   Heading slug strategy.
      --wrap <none|soft|hard>      Line wrapping (default: soft), --width <n>.
      --tables <simple|grid|pipe>  Table style (auto→best-fit).
      --list-style <dash|asterisk> Unordered list bullet.
      --code-fence <```|~~~>       Fence token (default: ```).

Metadata:
      --frontmatter <yaml|json|none>   Emit front matter (default: yaml).
      --title <string>                 Override document title.
      --author <string>                One or more authors.
      --date <YYYY-MM-DD|now>          Override date.

Images & assets:
      --images <keep|download|inline>  Strategy (default: keep).
      --assets-dir <DIR>               Where to put downloaded images/files.
      --image-max-width <px>           Add width hints in HTML wrapper if needed.

PDF & OCR:
      --ocr                            Force OCR pass.
      --ocr-lang <codes>               e.g., eng+tur.
      --pdf-layout <auto|raw|smart>    Choose extractor (keeps columns & lists).

Office docs:
      --libreoffice-bin <path>         Custom soffice path.
      --sheet <name|idx>               Only one sheet.
      --sheets <all|names...>          Select sheets for xlsx/csv.

Math:
      --math <auto|katex|none>         Convert equations to $...$ or leave.
      --math-block <$$|\\[\\]>         Block math delimiters.

Batch / perf:
      --watch                          Rebuild on file changes.
      --jobs <N>                       Parallel workers (default: CPU cores).
      --dry-run                        Show plan without writing.
      --verbose                        More logs.

General:
  -c, --config <FILE>              Use a specific config file.
      --version
  -h, --help
````

---

## Configuration (`c2md.yaml`)

```yaml
to: gfm
wrap: soft
width: 100
frontmatter: yaml
slug: github
tables: grid
images:
  mode: download         # keep | download | inline
  assets_dir: assets
pdf:
  layout: smart          # auto | raw | smart
ocr:
  enabled: false
  lang: eng
math:
  mode: auto             # auto | katex | none
batch:
  jobs: auto
ignore:
  - "**/node_modules/**"
  - "**/.git/**"
```

Use `c2md --config ./.config/c2md.yaml`.

---

## How it works (backends)

c2md orchestrates specialized converters and normalizes their output:

* **Office**: LibreOffice (headless) → intermediary (HTML) → Pandoc → Markdown
* **PDF (digital)**: Poppler/pdfminer → structural heuristics → Markdown
* **PDF (scanned) & images**: Tesseract OCR → text blocks → Markdown
* **RTF/HTML/TXT/CSV/EPUB**: Pandoc/format-specific parsers → Markdown

You can pin/override backends via flags or config.

---

## Examples

**Full website dump → MD with assets**

```bash
c2md site.html --images download --assets-dir md_assets -o site.md
```

**Large archive**

```bash
fd -e pdf -e docx -a source/ | xargs -P 8 -n 1 c2md --out-dir build/md --preserve-structure
```

**Sheet selection**

```bash
c2md budget.xlsx -o budget.md --sheet "2025-Q3"
```

**Front matter with metadata**

```bash
c2md report.docx -o report.md --frontmatter yaml --title "Q4 Report" --author "M. Akalın"
```

---

## Exit codes

* `0` success
* `1` recoverable conversion error(s) (some files failed)
* `2` hard failure (bad args, missing backends)

---

## Tips

* Install `pandoc`, `tesseract`, `libreoffice`, `poppler` for best coverage.
* Use `--wrap hard --width 80` for Git diffs that like fixed columns.
* Prefer `--images download` when moving Markdown to static site builders.

---

## Roadmap

* Native readers for DOCX/XLSX/PPTX (fewer external deps)
* Embedded fonts/table styling hints → MD+HTML hybrid
* Language-aware hyphenation/line-break heuristics
* Incremental cache for unchanged pages
* Plugins: custom pre/post filters in JS/TS or WASM

---

## Programmatic API (optional)

Rust crate exposes a library:

```rust
use c2md::{convert, Options};

let md = convert("input.pdf", Options::default())?;
std::fs::write("out.md", md)?;
```

---

## FAQ

**Q:** Do I need LibreOffice/Pandoc?
**A:** Not for all formats, but they improve accuracy/coverage.

**Q:** Will layout match pixel-perfect?
**A:** No. c2md prioritizes *semantic* Markdown, not WYSIWYG.

**Q:** Private PDFs?
**A:** Runs locally. No network calls.

---

## License

MIT © Mehmet T. AKALIN

---

## Acknowledgements

Powered by the open-source ecosystems of **Pandoc**, **Poppler**, **pdfminer**, **Tesseract**, and **LibreOffice**.
