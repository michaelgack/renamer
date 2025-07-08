use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum RenameError {
    #[error("I/O error for path {path:?}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Collision: destination {0:?} already exists.")]
    Collision(PathBuf),
    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),
}

pub enum RenameMode {
    Regex(Regex, String),
    Lowercase,
    Uppercase,
    Capitalize,
}

pub struct Config {
    pub mode: RenameMode,
    pub paths: Vec<PathBuf>,
    pub verbose: bool,
    pub dry_run: bool,
    pub force: bool,
    pub auto_number: bool,
}

pub fn run(config: &Config) -> Result<(), Vec<RenameError>> {
    let mut errors = Vec::new();

    for path in &config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        if let Err(e) = process_file(entry.path(), config) {
                            errors.push(e);
                        }
                    }
                }
                Err(e) => errors.push(RenameError::Walkdir(e)),
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn process_file(path: &Path, config: &Config) -> Result<(), RenameError> {
    let filename = match path.file_name().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => return Ok(()),
    };

    let new_filename = match &config.mode {
        RenameMode::Regex(re, replacement) => re.replace_all(filename, replacement).to_string(),
        RenameMode::Lowercase => filename.to_lowercase(),
        RenameMode::Uppercase => filename.to_uppercase(),
        RenameMode::Capitalize => {
            let mut c = filename.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
            }
        }
    };

    if new_filename != filename {
        let mut new_path = path.with_file_name(&new_filename);

        if new_path.exists() && !config.force {
            if config.auto_number {
                let stem = new_path.file_stem().unwrap().to_str().unwrap();
                let ext = new_path.extension().unwrap_or_default().to_str().unwrap();
                let mut i = 1;
                loop {
                    let numbered_filename = if ext.is_empty() {
                        format!("{}({})", stem, i)
                    } else {
                        format!("{}({}).{}", stem, i, ext)
                    };
                    let numbered_path = path.with_file_name(&numbered_filename);
                    if !numbered_path.exists() {
                        new_path = numbered_path;
                        break;
                    }
                    i += 1;
                }
            } else {
                return Err(RenameError::Collision(new_path));
            }
        }

        if config.dry_run {
            println!("[DRY RUN] Would rename {:?} to {:?}", path, new_path);
        } else {
            fs::rename(path, &new_path).map_err(|e| RenameError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;
        }
    } else if config.verbose {
        println!("No changes for {:?}", path);
    }

    Ok(())
}