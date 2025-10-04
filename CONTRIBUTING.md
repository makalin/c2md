# Contributing to c2md

Thank you for your interest in contributing to c2md! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- Git
- Optional dependencies for full functionality:
  - Pandoc
  - LibreOffice
  - Tesseract OCR
  - Poppler

### Building from Source

```bash
# Clone the repository
git clone https://github.com/makalin/c2md.git
cd c2md

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI
cargo run -- --help
```

### Running Tests

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration_tests

# Run with verbose output
cargo test -- --nocapture
```

## Contributing Guidelines

### Code Style

- Follow Rust's official style guidelines
- Use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Write comprehensive tests for new features
- Document public APIs with rustdoc comments

### Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Maintenance tasks

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run formatting (`cargo fmt`)
7. Run linting (`cargo clippy`)
8. Commit your changes (`git commit -m 'feat: add amazing feature'`)
9. Push to your branch (`git push origin feature/amazing-feature`)
10. Open a Pull Request

### Testing

- Write unit tests for individual functions
- Write integration tests for CLI functionality
- Test with various file formats and edge cases
- Ensure tests pass on all supported platforms

### Documentation

- Update README.md for user-facing changes
- Update API documentation for library changes
- Add examples for new features
- Update CHANGELOG.md for significant changes

## Architecture

### Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library entry point
├── cli.rs               # CLI argument parsing
├── config.rs            # Configuration management
├── error.rs             # Error types
├── utils.rs             # Utility functions
└── converter/            # Format-specific converters
    ├── mod.rs
    ├── pdf.rs
    ├── office.rs
    ├── image.rs
    ├── text.rs
    ├── html.rs
    ├── epub.rs
    ├── csv.rs
    └── rtf.rs
```

### Adding New Formats

To add support for a new file format:

1. Create a new module in `src/converter/`
2. Implement the conversion logic
3. Add the format to the main converter logic in `src/converter/mod.rs`
4. Add tests for the new format
5. Update documentation

### Configuration

The configuration system uses YAML files and supports:
- Default configuration in `Config::default()`
- File-based configuration via `c2md.yaml`
- CLI argument overrides

## Issue Reporting

When reporting issues:

1. Use the issue templates
2. Provide detailed reproduction steps
3. Include system information (OS, Rust version, etc.)
4. Attach sample files if relevant
5. Check existing issues first

## Feature Requests

For feature requests:

1. Check existing issues and discussions
2. Provide clear use case description
3. Consider implementation complexity
4. Discuss with maintainers if needed

## License

By contributing to c2md, you agree that your contributions will be licensed under the MIT License.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Getting Help

- Check the documentation
- Search existing issues
- Join discussions
- Contact maintainers if needed

Thank you for contributing to c2md!