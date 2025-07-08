use clap::{Parser, Subcommand};
use regex::Regex;
use std::path::PathBuf;
use std::process::ExitCode;

// Use the library crate
use renamer::{run, Config, RenameMode};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Rename files using a regular expression
    Regex {
        /// The regex pattern to search for
        #[arg(short, long)]
        pattern: String,

        /// The replacement string
        #[arg(short, long)]
        replacement: String,

        #[command(flatten)]
        args: SharedArgs,
    },
    /// Convert filenames to lowercase
    Lowercase {
        #[command(flatten)]
        args: SharedArgs,
    },
    /// Convert filenames to uppercase
    Uppercase {
        #[command(flatten)]
        args: SharedArgs,
    },
    /// Capitalize filenames
    Capitalize {
        #[command(flatten)]
        args: SharedArgs,
    },
}

#[derive(Parser)]
struct SharedArgs {
    /// The files or directories to process
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Perform a dry run without actually renaming files
    #[arg(long)]
    dry_run: bool,

    /// Overwrite existing files
    #[arg(long)]
    force: bool,

    /// Automatically number files to resolve conflicts
    #[arg(long, conflicts_with = "force")]
    auto_number: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let (args, mode) = match cli.command {
        Commands::Regex {
            pattern,
            replacement,
            args,
        } => {
            let re = match Regex::new(&pattern) {
                Ok(re) => re,
                Err(e) => {
                    eprintln!("Invalid regex: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            (args, RenameMode::Regex(re, replacement))
        }
        Commands::Lowercase { args } => (args, RenameMode::Lowercase),
        Commands::Uppercase { args } => (args, RenameMode::Uppercase),
        Commands::Capitalize { args } => (args, RenameMode::Capitalize),
    };

    let config = Config {
        mode,
        paths: args.paths,
        verbose: args.verbose,
        dry_run: args.dry_run,
        force: args.force,
        auto_number: args.auto_number,
    };

    if let Err(errors) = run(&config) {
        for error in errors {
            eprintln!("Error: {}", error);
        }
        std::process::exit(1);
    }

    ExitCode::SUCCESS
}