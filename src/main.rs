use clap::Parser;
use std::process::Command;
use std::path::Path;
use std::env;
use serde_json::Value;

/// A simple app to generate GitHub permalinks for a file in a git repository.
#[derive(Parser)]
struct Cli {
    /// Path to the file within the repository
    file_path: String,
    /// Optional line number to include in the permalink
    line_number: Option<u32>,
}

fn main() {
    let args = Cli::parse();

    // Get the current working directory
    let current_dir = env::current_dir().expect("Failed to get current working directory");

    // Resolve the full path to the file
    let full_path = current_dir.join(&args.file_path);
    if !full_path.exists() {
        eprintln!("Error: File does not exist: {}", full_path.display());
        std::process::exit(1);
    }

    // Ensure the file is within a git repository
    let git_top_level = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .expect("Failed to execute git command");

    if !git_top_level.status.success() {
        eprintln!("Error: Not inside a git repository");
        std::process::exit(1);
    }

    let repo_root = Path::new(
        std::str::from_utf8(&git_top_level.stdout)
            .expect("Invalid UTF-8 in git output")
            .trim(),
    );

    if !full_path.starts_with(repo_root) {
        eprintln!("Error: File is not inside the git repository");
        std::process::exit(1);
    }

    // Get the relative path from the repository root
    let relative_path = full_path.strip_prefix(repo_root).expect("Failed to strip repo root");

    // Get the current commit hash
    let git_commit = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("Failed to get git commit hash");

    if !git_commit.status.success() {
        eprintln!("Error: Failed to determine git commit hash");
        std::process::exit(1);
    }

    let commit_hash = std::str::from_utf8(&git_commit.stdout)
        .expect("Invalid UTF-8 in git commit output")
        .trim();

    // Use the GitHub CLI to get the repository URL
    let repo_url_output = Command::new("gh")
        .arg("repo")
        .arg("view")
        .arg("--json")
        .arg("url")
        .output()
        .expect("Failed to get repository URL");

    if !repo_url_output.status.success() {
        eprintln!("Error: Failed to get GitHub repository URL. Is the GitHub CLI configured?");
        std::process::exit(1);
    }

    let repo_url_json = std::str::from_utf8(&repo_url_output.stdout)
        .expect("Invalid UTF-8 in repo URL output");

    let repo_url: String = match serde_json::from_str::<Value>(repo_url_json) {
        Ok(json) => json["url"].as_str().unwrap_or_default().to_string(),
        Err(_) => {
            eprintln!("Error: Failed to parse repository URL JSON");
            std::process::exit(1);
        }
    };

    // Construct the permalink with the commit hash
    let mut permalink = format!("{}/blob/{}/{}", repo_url, commit_hash, relative_path.display());

    // Append the line number if provided
    if let Some(line_number) = args.line_number {
        permalink.push_str(&format!("#L{}", line_number));
    }

    println!("{}", permalink);
}

