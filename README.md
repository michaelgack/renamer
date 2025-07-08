# File Renamer

A command-line tool for renaming files and directories using regular expressions.

## Description

This script allows you to rename files and directories by matching a regex pattern and replacing it with a specified string. It can process individual files or recursively traverse directories.

## Installation

1.  **Create a new Rust project:**

    ```bash
    cargo new renamer
    cd renamer
    ```

2.  **Add dependencies:**

    Add the following lines to your `Cargo.toml` file:

    ```toml
    [dependencies]
    anyhow = "1.0"
    clap = { version = "4.0", features = ["derive"] }
    regex = "1.5"
    ```

3.  **Add the code:**

    Copy the provided Rust code into `src/main.rs`.

## Usage

Run the script from your terminal using `cargo run`:

```bash
cargo run -- -p <pattern> -r <replacement> <path(s)>
```

### Arguments

*   `-p, --pattern <PATTERN>`: The regex pattern to search for in filenames.
*   `-r, --replacement <REPLACEMENT>`: The string to replace the matched pattern with.
*   `<PATHS>`: One or more files or directories to process.

## Examples

### Rename file extensions

To rename all `.txt` files to `.md` in the `my_docs` directory:

```bash
cargo run -- -p "\.txt$" -r ".md" my_docs
```

### Add a prefix to filenames

To add the prefix `new_` to all files in the current directory:

```bash
cargo run -- -p "^" -r "new_" .
```

### Replace spaces with underscores

To replace all spaces with underscores in filenames within the `project_files` directory:

```bash
cargo run -- -p " " -r "_" project_files
```
