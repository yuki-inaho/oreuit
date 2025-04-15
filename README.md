# oreuit

- **oreuit** is a tool that generates a text report summarizing the file structure and contents within specified directories. Inspired by [uithub](https://uithub.com/).
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

- `-d, --directory <DIRECTORIES>`
  Specify the directory or directories (comma-separated) to explore. If not specified, the current directory (`.`) is used. (Example: `-d src,tests`)

- `-e, --extensions <EXTENSIONS>`
  Provide a comma-separated list of allowed file extensions.

  - If the value starts with `+,`, the specified extensions are **added** to the default list. (Example: `-e +,.json,.vue` adds `.json` and `.vue` to the default list.)
  - Otherwise, the specified list **overwrites** the default list. (Example: `-e .py,.js` uses only `.py` and `.js`.)
  - If not specified, the default list is used.
    **Default:** `.txt`, `.md`, `.py`, `.js`, `.java`, `.cpp`, `.c`, `.cs`, `.rb`, `.go`, `.rs`, `.hpp`, `.ts`, `.tsx`, `.d.ts`, `.jsx`, `.toml`
    **Note:** `.json` is not included by default. To include it, use `-e +,.json`.
    **Note:** Files without extensions (e.g., `.gitignore`, `Makefile`, `Dockerfile`, `LICENSE`, `README`, `.gitattributes`) are allowed by default unless explicitly ignored.

- `-i, --ignore-extensions <EXTENSIONS>`
  Provide a comma-separated list of file extensions to ignore. (Example: `-i .lock,.md`)
  The default value is:

  ```
  .bin,.zip,.tar,.gz,.7z,.rar,.exe,.dll,.so,.dylib,.a,.lib,.obj,.o,.class,.jar,.war,.ear,.ipynb,.jpg,.jpeg,.png,.gif
  ```

- `-o, --output <OUTPUT>`
  Specify the output file name. The default is `summary.txt`.

- `-c, --clipboard`
  Instead of writing the output to a file, copy the results to the clipboard.

  **Note:** Requires the tool to be built with the `clipboard` feature enabled (see Installation section).

- `--ignore-dirs <DIRS>`
  Provide a comma-separated list of directory names to ignore.

  - If the value starts with `+,`, the specified directories are **added** to the default ignore list. (Example: `--ignore-dirs +,my_temp,build2` adds `my_temp` and `build2` to the default ignore list.)
  - Otherwise, the specified list **overwrites** the default list.
  - If not specified, the default list is used.
    **Default:** `.git`, `.vscode`, `target`, `node_modules`, `__pycache__`, `.idea`, `build`, `dist`, `.ruff_cache`

- `--max-size <MAX_SIZE>`
  Specify the maximum file size (in bytes) for reading file contents. The default is `10485760` (10MB).
  Files exceeding the specified size will have their content extraction skipped.

- `-w, --whitelist-filenames <FILENAMES>`
  Specify a comma-separated list of filenames that are always included, regardless of their file extension or location. (Example: `Dockerfile,Makefile`)

---

## Command and Output Examples

Example commands:

```bash
# Scan the current directory, ignoring temp and adding .json files
./target/release/oreuit -d . --ignore-dirs +,temp -e +,.json -o summary.txt

# Scan the src directory, using only .py and .js extensions
./target/release/oreuit -d src -e .py,.js -o summary_src.txt

# Scan the current directory, ignoring only the 'build' directory (overwrites default ignore list)
./target/release/oreuit -d . --ignore-dirs build -o summary_build_only.txt
```
