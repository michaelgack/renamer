# File Renamer

A command-line tool for renaming files and directories using regular expressions or common case conversions.

## Description

This script allows you to rename files and directories by either matching a regex pattern or by applying a case conversion (lowercase, uppercase, or capitalize). It can process individual files or recursively traverse directories.

## Installation

1.  **Install Rust:** If you don't have Rust, install it from [rust-lang.org](https://www.rust-lang.org/).
2.  **Compile the project:**
    ```bash
    git clone <repository_url>
    cd renamer
    cargo build --release
    ```
3.  **Install the command (Optional):** To make `renamer` available system-wide, move the executable to a directory in your `PATH`.
    ```bash
    # For the current user
    mv target/release/renamer ~/.local/bin/

    # Or for all users
    sudo mv target/release/renamer /usr/local/bin/
    ```

## Usage

The tool uses subcommands to determine the renaming operation.

```bash
renamer [COMMAND] [OPTIONS] [PATHS]...
```

### Commands

*   `regex`: Rename files using a regular expression.
*   `lowercase`: Convert filenames to lowercase.
*   `uppercase`: Convert filenames to uppercase.
*   `capitalize`: Capitalize filenames (e.g., `my file.txt` -> `My file.txt`).

### Shared Options

These options can be used with any of the commands.

*   `--verbose`, `-v`: Enable verbose output, showing which files are being processed.
*   `--dry-run`: Show what changes would be made without actually renaming any files.
*   `--force`: Forcefully overwrite existing files if a name collision occurs.
*   `--auto-number`: If a name collision occurs, automatically append a number to the new filename (e.g., `file(1).txt`). This cannot be used with `--force`.

## Examples

### Regex Rename

To replace all spaces with underscores in the `project_files` directory:

```bash
renamer regex --pattern " " --replacement "_" ./project_files
```

### Lowercase Conversion

To convert all filenames in the current directory to lowercase:

```bash
renamer lowercase .
```

### Uppercase with a Dry Run

To see what would happen when converting all `.jpg` files to uppercase, without actually changing them:

```bash
renamer uppercase --dry-run ./*.jpg
```

### Capitalize with Auto-Numbering

To capitalize all files in `~/Documents` and automatically handle any name collisions:

```bash
renamer capitalize --auto-number ~/Documents
```