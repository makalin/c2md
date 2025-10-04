#!/bin/bash

# c2md Installation Script
# This script installs c2md globally using cargo

set -e

echo "🚀 Installing c2md Universal Markdown Converter..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo is not installed."
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if we're in the c2md directory
if [ ! -f "Cargo.toml" ] || [ ! -f "src/main.rs" ]; then
    echo "❌ Error: Please run this script from the c2md project directory."
    exit 1
fi

echo "📦 Building and installing c2md..."

# Install c2md globally
cargo install --path .

if [ $? -eq 0 ]; then
    echo "✅ c2md installed successfully!"
    echo ""
    echo "🎉 You can now use 'c2md' from any directory!"
    echo ""
    echo "📖 Quick start:"
    echo "  c2md --help                    # Show help"
    echo "  c2md document.pdf              # Convert PDF to markdown"
    echo "  c2md *.txt --template academic # Convert with academic template"
    echo "  c2md file.docx --validate      # Convert with validation"
    echo ""
    echo "🔧 Advanced features:"
    echo "  c2md --template report         # Use report template"
    echo "  c2md --include-metadata       # Include document metadata"
    echo "  c2md --metadata-only          # Extract metadata only"
    echo "  c2md --validate               # Validate files before conversion"
    echo ""
    echo "📚 For more information, visit: https://github.com/makalin/c2md"
else
    echo "❌ Installation failed. Please check the error messages above."
    exit 1
fi