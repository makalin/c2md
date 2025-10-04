# c2md Installation Guide

This guide shows you how to install `c2md` so you can use it from any directory on your system.

## 🚀 Quick Installation

### Method 1: Using the Installation Script (Recommended)

```bash
# Navigate to the c2md project directory
cd /path/to/c2md

# Run the installation script
./install.sh
```

### Method 2: Using Cargo Install

```bash
# Navigate to the c2md project directory
cd /path/to/c2md

# Install globally
cargo install --path .
```

### Method 3: From Source

```bash
# Clone the repository
git clone https://github.com/makalin/c2md.git
cd c2md

# Install globally
cargo install --path .
```

## ✅ Verify Installation

After installation, verify that `c2md` is available globally:

```bash
# Check if c2md is in your PATH
which c2md

# Check version
c2md --version

# Test from any directory
cd ~
c2md --help
```

## 🔧 Prerequisites

- **Rust**: Install Rust from [rustup.rs](https://rustup.rs/)
- **Cargo**: Comes with Rust installation

## 📍 Installation Location

The `c2md` binary will be installed to:
- **macOS/Linux**: `~/.cargo/bin/c2md`
- **Windows**: `%USERPROFILE%\.cargo\bin\c2md.exe`

Make sure `~/.cargo/bin` is in your `PATH` environment variable.

## 🎯 Usage Examples

Once installed, you can use `c2md` from any directory:

```bash
# Basic conversion
c2md document.pdf

# Convert with academic template
c2md report.docx --template academic

# Include metadata
c2md file.txt --include-metadata

# Validate before conversion
c2md document.pdf --validate

# Extract metadata only
c2md file.docx --metadata-only

# Batch conversion
c2md documents/ --out-dir converted/

# Watch mode for automatic conversion
c2md documents/ --watch
```

## 🔄 Updating c2md

To update to the latest version:

```bash
# If installed from source
cd /path/to/c2md
git pull
cargo install --path .

# If installed from crates.io (when available)
cargo install --upgrade c2md
```

## 🗑️ Uninstalling

To remove `c2md`:

```bash
cargo uninstall c2md
```

## 🐛 Troubleshooting

### Command Not Found

If `c2md` is not found after installation:

1. **Check PATH**: Ensure `~/.cargo/bin` is in your PATH
   ```bash
   echo $PATH | grep cargo
   ```

2. **Add to PATH**: Add this to your shell profile (`.bashrc`, `.zshrc`, etc.)
   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

3. **Restart Terminal**: Close and reopen your terminal

### Permission Issues

If you get permission errors:

```bash
# Make sure you have write permissions to ~/.cargo/bin
ls -la ~/.cargo/bin/

# If needed, create the directory
mkdir -p ~/.cargo/bin
```

### Build Errors

If installation fails with build errors:

1. **Update Rust**: `rustup update`
2. **Check Dependencies**: Ensure all system dependencies are installed
3. **Clean Build**: `cargo clean && cargo install --path .`

## 🌟 Advanced Features

After installation, explore these advanced features:

- **Templates**: `--template academic|report|minimal|default`
- **Metadata**: `--include-metadata`, `--metadata-only`
- **Validation**: `--validate`
- **OCR**: `--ocr`, `--ocr-lang eng`
- **Batch Processing**: `--jobs 8`, `--watch`
- **Configuration**: `--config config.yaml`

## 📚 External Dependencies

For full functionality, install these optional dependencies:

### macOS (Homebrew)
```bash
brew install pandoc libreoffice tesseract poppler
```

### Ubuntu/Debian
```bash
sudo apt-get install pandoc libreoffice tesseract-ocr poppler-utils
```

### Windows (Chocolatey)
```bash
choco install pandoc libreoffice tesseract poppler
```

## 🎉 Success!

Once installed, `c2md` is ready to use from any directory on your system!

For more information, see the [README.md](README.md) and [FEATURES.md](FEATURES.md) files.