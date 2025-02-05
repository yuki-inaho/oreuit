# oreuit

- **oreuit** is a tool that generates a text report summarizing the file structure and contents within a specified directory. Inspired by [uithub](https://uithub.com/).
- It allows visualization of directory trees and bulk viewing of file contents for codebases without repositories, such as those not hosted on GitHub.

# Installation

## Requirements

- [Rust](https://www.rust-lang.org/) installed (version 1.70 or higher recommended)

## Installation Steps

1. **Clone the Repository or Download the Code**  
   Obtain the source code of this tool from GitHub or other sources.

  ```bash
  git clone git@github.com:yuki-inaho/uithub_like_text_generator.git
  cd uithub_like_text_generator
  ```

2. **Build**  
   Execute the following command within the project directory to create a release build.

   ```bash
   cargo build --release
   ```

3. **Place the Executable**  
   After the build completes, the executable will be generated at `./target/release/oreuit`.  
   Add it to your system's PATH or specify the execution path as needed.

# Usage

The basic usage involves specifying the target directory and options when running the tool. Below are the main command-line options and their descriptions.

## Available Options

- `-d, --directory <DIRECTORY>`  
  Specify the directory to explore. If not specified, the current directory (`.`) is used.

- `-e, --extensions <EXTENSIONS>`  
  Provide a comma-separated list of allowed file extensions. (Example: `-e .txt,.md,.py`)

  If left empty, the default list (`.txt`, `.md`, `.py`, `.js`, `.java`, `.cpp`, `.c`, `.cs`, `.rb`, `.go`, `.rs`) is used.

- `-i, --ignore-extensions <EXTENSIONS>`  
  Provide a comma-separated list of file extensions to ignore. (Example: `-i .lock,.md`)

  The default value is:

  ```
  .bin,.zip,.tar,.gz,.7z,.rar,.exe,.dll,.so,.dylib,.a,.lib,.obj,.o,.class,.jar,.war,.ear,.ipynb,.jpg,.jpeg,.png,.gif
  ```

- `-o, --output <OUTPUT>`  
  Specify the output file name. The default is `summary.txt`.

- `--max-size <MAX_SIZE>`  
  Specify the maximum file size (in bytes) for reading file contents. The default is `10485760` (10MB).

  ※ Files exceeding the specified size will have their content extraction skipped.

- `-c, --clipboard`  
  Instead of writing the output to a file, copy the results to the clipboard.

- `--ignore-dirs <DIRS>`  
  Provide a comma-separated list of directory names to ignore. (Example: `.git,node_modules,__pycache__,target`)

  The default value is:

  ```
  .git,.vscode,target,node_modules,__pycache__,.idea,build,dist
  ```

# Command Examples

- For example, `summary.txt_example` was generated using the following command:

```bash
./target/release/oreuit -d  . --ignore-dirs target,.git -i .lock,.md -e .toml,.rs
```
