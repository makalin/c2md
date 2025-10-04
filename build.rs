fn main() {
    // Add build-time information
    println!("cargo:rustc-env=BUILD_DATE={}", chrono::Utc::now().format("%Y-%m-%d"));
    println!("cargo:rustc-env=GIT_HASH={}", get_git_hash());
}

fn get_git_hash() -> String {
    use std::process::Command;
    
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();
    
    match output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "unknown".to_string()
            }
        }
        Err(_) => "unknown".to_string(),
    }
}