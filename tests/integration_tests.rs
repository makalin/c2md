use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("c2md").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("c2md"));
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("c2md").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Convert PDF, Word"));
}

#[test]
fn test_text_conversion() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.txt");
    let output_file = temp_dir.path().join("test.md");
    
    std::fs::write(&input_file, "Hello, World!\nThis is a test document.").unwrap();
    
    let mut cmd = Command::cargo_bin("c2md").unwrap();
    cmd.arg(input_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap());
    
    cmd.assert().success();
    
    let output = std::fs::read_to_string(&output_file).unwrap();
    assert!(output.contains("Hello, World!"));
    assert!(output.contains("This is a test document"));
}

#[test]
fn test_csv_conversion() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.csv");
    let output_file = temp_dir.path().join("test.md");
    
    let csv_content = "Name,Age,City\nJohn,25,New York\nJane,30,London";
    std::fs::write(&input_file, csv_content).unwrap();
    
    let mut cmd = Command::cargo_bin("c2md").unwrap();
    cmd.arg(input_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap());
    
    cmd.assert().success();
    
    let output = std::fs::read_to_string(&output_file).unwrap();
    assert!(output.contains("Name"));
    assert!(output.contains("Age"));
    assert!(output.contains("City"));
}

#[test]
fn test_dry_run() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.txt");
    
    std::fs::write(&input_file, "Test content").unwrap();
    
    let mut cmd = Command::cargo_bin("c2md").unwrap();
    cmd.arg(input_file.to_str().unwrap())
        .arg("--dry-run");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Conversion plan"));
}

#[test]
fn test_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("test_config.yaml");
    let input_file = temp_dir.path().join("test.txt");
    let output_file = temp_dir.path().join("test.md");
    
    let config_content = r#"
to: gfm
wrap: hard
width: 80
frontmatter: yaml
slug: github
tables: grid
images:
  mode: download
  assets_dir: assets
pdf:
  layout: smart
ocr:
  enabled: false
  lang: eng
math:
  mode: auto
batch:
  jobs: auto
ignore:
  - "**/node_modules/**"
  - "**/.git/**"
"#;
    
    std::fs::write(&config_file, config_content).unwrap();
    std::fs::write(&input_file, "This is a very long line that should be wrapped according to the configuration settings").unwrap();
    
    let mut cmd = Command::cargo_bin("c2md").unwrap();
    cmd.arg(input_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .arg("-c")
        .arg(config_file.to_str().unwrap());
    
    cmd.assert().success();
    
    let output = std::fs::read_to_string(&output_file).unwrap();
    assert!(output.contains("---"));
    assert!(output.contains("title:"));
}

#[test]
fn test_batch_conversion() {
    let temp_dir = TempDir::new().unwrap();
    let input_dir = temp_dir.path().join("input");
    let output_dir = temp_dir.path().join("output");
    
    std::fs::create_dir(&input_dir).unwrap();
    std::fs::create_dir(&output_dir).unwrap();
    
    let file1 = input_dir.join("test1.txt");
    let file2 = input_dir.join("test2.txt");
    
    std::fs::write(&file1, "Content 1").unwrap();
    std::fs::write(&file2, "Content 2").unwrap();
    
    let mut cmd = Command::cargo_bin("c2md").unwrap();
    cmd.arg(input_dir.to_str().unwrap())
        .arg("--out-dir")
        .arg(output_dir.to_str().unwrap());
    
    cmd.assert().success();
    
    // Check that output files were created
    assert!(output_dir.join("test1.md").exists());
    assert!(output_dir.join("test2.md").exists());
}

#[test]
fn test_frontmatter_options() {
    let temp_dir = TempDir::new().unwrap();
    let input_file = temp_dir.path().join("test.txt");
    let output_file = temp_dir.path().join("test.md");
    
    std::fs::write(&input_file, "Test content").unwrap();
    
    let mut cmd = Command::cargo_bin("c2md").unwrap();
    cmd.arg(input_file.to_str().unwrap())
        .arg("-o")
        .arg(output_file.to_str().unwrap())
        .arg("--title")
        .arg("Test Document")
        .arg("--author")
        .arg("Test Author")
        .arg("--date")
        .arg("2024-01-01");
    
    cmd.assert().success();
    
    let output = std::fs::read_to_string(&output_file).unwrap();
    assert!(output.contains("title: Test Document"));
    assert!(output.contains("author: Test Author"));
    assert!(output.contains("date: 2024-01-01"));
}