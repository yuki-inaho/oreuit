# oreuit

- **oreuit** is a tool that generates a text report summarizing the file structure and contents within specified director**ies**. Inspired by [uithub](https://uithub.com/).
- It allows visualization of directory trees and bulk viewing of file contents for codebases without repositories, such as those not hosted on GitHub.

# Installation

## Requirements

- [Rust](https://www.rust-lang.org/) installed (version 1.70 or higher recommended)

## Installation Steps

1.  **Clone the Repository or Download the Code**
    Obtain the source code of this tool from GitHub or other sources.

    ```bash
    git clone https://github.com/yuki-inaho/uithub_like_text_generator.git # Use HTTPS or SSH URL as appropriate
    cd uithub_like_text_generator
    ```

2.  **Build**
    Execute the following command within the project directory to create a release build.

    ```bash
    # Standard build (without clipboard support)
    cargo build --release

    # Build with clipboard support (if you need the -c option)
    # cargo build --release --features clipboard
    ```
    If you plan to use the `-c, --clipboard` option, uncomment and run the second build command.

3.  **Place the Executable**
    After the build completes, the executable will be generated at `./target/release/oreuit`.
    Add it to your system's PATH or specify the execution path as needed.

# Usage

The basic usage involves specifying the target directory/directories and options when running the tool. Below are the main command-line options and their descriptions.

## Available Options

-   `-d, --directory <DIRECTORIES>`
    Specify the director**y or directories (comma-separated)** to explore. If not specified, the current directory (`.`) is used. (Example: `-d src,tests`)

-   `-e, --extensions <EXTENSIONS>`
    Provide a comma-separated list of allowed file extensions. (Example: `-e .txt,.md,.py`)
    If left empty, the default list (`.txt`, `.md`, `.py`, `.js`, `.java`, `.cpp`, `.c`, `.cs`, `.rb`, `.go`, `.rs`, `.hpp`) is used.

-   `-i, --ignore-extensions <EXTENSIONS>`
    Provide a comma-separated list of file extensions to ignore. (Example: `-i .lock,.md`)
    The default value is:
    ```
    .bin,.zip,.tar,.gz,.7z,.rar,.exe,.dll,.so,.dylib,.a,.lib,.obj,.o,.class,.jar,.war,.ear,.ipynb,.jpg,.jpeg,.png,.gif
    ```

-   `-o, --output <OUTPUT>`
    Specify the output file name. The default is `summary.txt`.

-   `-c, --clipboard`
    Instead of writing the output to a file, copy the results to the clipboard.
    **Note:** Requires the tool to be built with the `clipboard` feature enabled (see Installation section).

-   `--ignore-dirs <DIRS>`
    Provide a comma-separated list of directory names to ignore.
    (Example: `.git,node_modules,__pycache__,target`)
    The default value is:
    ```
    .git,.vscode,target,node_modules,__pycache__,.idea,build,dist
    ```

-   `--max-size <MAX_SIZE>`
    Specify the maximum file size (in bytes) for reading file contents. The default is `10485760` (10MB).
    Files exceeding the specified size will have their content extraction skipped.

-   `-w, --whitelist-filenames <FILENAMES>`
    Specify a comma-separated list of filenames that are always included, regardless of their file extension or location. (Example: `Dockerfile,Makefile`)

---

## Command and Output Examples

Example command:

```bash
./target/release/oreuit -d src -o summary_src.txt --ignore-dirs ".git,target"
```

This command scans the src directory, ignores .git and target subdirectories, and writes the output to summary_src.txt.