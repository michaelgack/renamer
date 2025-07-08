use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The regex pattern to search for in filenames
    #[arg(short, long)]
    pattern: String,

    /// The replacement string
    #[arg(short, long)]
    replacement: String,

    /// The files or directories to process
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Perform a dry run without actually renaming files
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let re = Regex::new(&cli.pattern).context("Failed to compile regex pattern")?;

    if cli.dry_run {
        println!("*** DRY RUN MODE ENABLED ***");
    }

    for path in &cli.paths {
        process_path(path, &re, &cli.replacement, cli.verbose, cli.dry_run);
    }

    Ok(())
}

fn process_path(path: &Path, re: &Regex, replacement: &str, verbose: bool, dry_run: bool) {
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => process_path(&entry.path(), re, replacement, verbose, dry_run),
                        Err(e) => eprintln!("Error reading directory entry in {:?}: {}", path, e),
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory {:?}: {}", path, e),
        }
    } else if path.is_file() {
        if let Err(e) = process_file(path, re, replacement, verbose, dry_run) {
            eprintln!("Error processing file {:?}: {}", path, e);
        }
    } else if verbose {
        println!("Skipping non-file/directory: {:?}", path);
    }
}

fn process_file(path: &Path, re: &Regex, replacement: &str, verbose: bool, dry_run: bool) -> Result<()> {
    let filename = match path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => {
            if verbose {
                println!("Skipping path with no filename: {:?}", path);
            }
            return Ok(());
        }
    };

    let new_filename = re.replace_all(&filename, replacement);

    if new_filename != filename {
        let new_path = path.with_file_name(new_filename.as_ref());
        if dry_run {
            println!("[DRY RUN] Would rename {:?} to {:?}", path, new_path);
        } else {
            println!("Renaming {:?} to {:?}", path, new_path);
            fs::rename(path, &new_path)
                .with_context(|| format!("Failed to rename {:?} to {:?}", path, new_path))?;
        }
    } else if verbose {
        println!("No changes for {:?}", path);
    }

    Ok(())
}
