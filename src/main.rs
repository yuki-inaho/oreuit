use clap::Parser;
use encoding_rs::SHIFT_JIS;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Tool to summarize directory structure and file contents
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "A tool to summarize directory structure and file contents (similar to uithub)"
)]
struct Args {
    /// Directory to explore (default is the current directory)
    #[clap(short = 'd', long = "directory", default_value = ".")]
    directory: String,

    /// Allowed file extensions (comma-separated, e.g., .txt,.md,.py). If empty, the default list is used
    #[clap(short = 'e', long = "extensions", default_value = "")]
    extensions: String,

    /// File extensions to ignore (comma-separated, e.g., .bin,.zip,...). If empty, no extensions are ignored
    #[clap(
        short = 'i',
        long = "ignore-extensions",
        default_value = ".bin,.zip,.tar,.gz,.7z,.rar,.exe,.dll,.so,.dylib,.a,.lib,.obj,.o,.class,.jar,.war,.ear,.ipynb,.jpg,.jpeg,.png,.gif"
    )]
    ignore_extensions: String,

    /// Output file name (default is summary.txt)
    #[clap(short = 'o', long = "output", default_value = "summary.txt")]
    output: String,

    /// Maximum file size to read (in bytes, default is 10485760 = 10MB)
    #[clap(long = "max-size", default_value = "10485760")]
    max_size: u64,

    /// Copy output to clipboard instead of writing to a file
    #[clap(short = 'c', long = "clipboard")]
    clipboard: bool,

    /// Directory names to ignore (comma-separated, e.g., .git,node_modules,__pycache__, etc.)
    #[clap(
        long = "ignore-dirs",
        default_value = ".git,.vscode,target,node_modules,__pycache__,.idea,build,dist"
    )]
    ignore_dirs: String,
}

/// Determines if a file is binary by checking for NUL bytes in the first 1024 bytes
fn is_binary(file_path: &Path) -> bool {
    if let Ok(mut file) = fs::File::open(file_path) {
        let mut buffer = [0u8; 1024];
        if let Ok(n) = file.read(&mut buffer) {
            return buffer[..n].iter().any(|&b| b == 0);
        }
    }
    true
}

/// Attempts to read a file as UTF-8, and if it fails, tries to decode using SHIFT_JIS.
/// If both attempts fail, returns "[Cannot decode file content]".
fn read_file_contents(file_path: &Path) -> String {
    match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(_) => match fs::read(file_path) {
            Ok(bytes) => {
                let (cow, _, had_errors) = SHIFT_JIS.decode(&bytes);
                if had_errors {
                    "[Cannot decode file content]".to_string()
                } else {
                    cow.into_owned()
                }
            }
            Err(_) => "[Cannot decode file content]".to_string(),
        },
    }
}

/// Recursively searches the specified directory and lists files that
/// - Match allowed extensions
/// - Do not have ignored extensions
/// Files within ignored directories are not searched.
fn collect_files(
    directory: &Path,
    allowed: &HashSet<String>,
    ignore: &HashSet<String>,
    ignore_dirs: &HashSet<String>,
) -> Vec<PathBuf> {
    // Use WalkDir's filter_entry to exclude ignored directories from the search
    let walker = WalkDir::new(directory).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            if let Some(name) = e.file_name().to_str() {
                return !ignore_dirs.contains(&name.to_string());
            }
        }
        true
    });

    let mut files = Vec::new();
    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_formatted = format!(".{}", ext.to_lowercase());
                // Check file extension filters
                if !allowed.is_empty() && !allowed.contains(&ext_formatted) {
                    continue;
                }
                if ignore.contains(&ext_formatted) {
                    continue;
                }
                files.push(path.to_path_buf());
            }
        }
    }
    // Sort file paths in relative path order
    files.sort_by(|a, b| {
        a.strip_prefix(directory)
            .unwrap_or(a)
            .cmp(b.strip_prefix(directory).unwrap_or(b))
    });
    files
}

/// Generates a tree structure of the specified directory.
/// Applies allowed and ignore filters, and excludes directories in ignore_dirs.
fn build_tree(
    directory: &Path,
    allowed: &HashSet<String>,
    ignore: &HashSet<String>,
    ignore_dirs: &HashSet<String>,
) -> String {
    let base = directory
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(".");
    let mut lines = vec![base.to_string()];
    build_tree_helper(directory, "", allowed, ignore, ignore_dirs, &mut lines);
    lines.join("\n")
}

/// Helper function that recursively traverses the directory structure and builds the tree string
fn build_tree_helper(
    path: &Path,
    prefix: &str,
    allowed: &HashSet<String>,
    ignore: &HashSet<String>,
    ignore_dirs: &HashSet<String>,
    lines: &mut Vec<String>,
) {
    // Read entries in the directory and sort them by name
    let mut entries: Vec<fs::DirEntry> = match fs::read_dir(path) {
        Ok(iter) => iter.filter_map(|e| e.ok()).collect(),
        Err(_) => return,
    };
    entries.sort_by_key(|e| e.file_name());

    // Separate directories and files
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for entry in entries {
        let entry_path = entry.path();
        let name = entry.file_name().into_string().unwrap_or_default();

        // Skip ignored directories
        if entry_path.is_dir() {
            if ignore_dirs.contains(&name) {
                continue;
            }
            dirs.push(entry);
        } else if entry_path.is_file() {
            // Apply file extension filters
            if let Some(ext_os) = entry_path.extension() {
                if let Some(ext) = ext_os.to_str() {
                    let ext_formatted = format!(".{}", ext.to_lowercase());
                    if !allowed.is_empty() && !allowed.contains(&ext_formatted) {
                        continue;
                    }
                    if ignore.contains(&ext_formatted) {
                        continue;
                    }
                }
            }
            files.push(entry);
        }
    }

    // Display directories first, then files
    let mut all_entries = Vec::new();
    for d in dirs {
        all_entries.push((d, true));
    }
    for f in files {
        all_entries.push((f, false));
    }

    let count = all_entries.len();
    for (i, (entry, is_dir)) in all_entries.into_iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let name = entry.file_name().into_string().unwrap_or_default();
        lines.push(format!("{}{}{}", prefix, connector, name));

        // If it's a directory, recurse into it
        if is_dir {
            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };
            build_tree_helper(
                &entry.path(),
                &new_prefix,
                allowed,
                ignore,
                ignore_dirs,
                lines,
            );
        }
    }
}

/// Generates text that combines the tree structure of the directory and
/// the relative paths and contents (or skip messages) of each file.
fn generate_output_text(
    directory: &Path,
    allowed: &HashSet<String>,
    ignore: &HashSet<String>,
    ignore_dirs: &HashSet<String>,
    max_size: u64,
) -> String {
    // Generate the tree structure of the directory
    let tree_text = build_tree(directory, allowed, ignore, ignore_dirs);
    // Retrieve contents of target files
    let files = collect_files(directory, allowed, ignore, ignore_dirs);
    let mut file_contents = String::new();
    for file in files {
        let relative_path = file
            .strip_prefix(directory)
            .unwrap_or(&file)
            .to_string_lossy();
        let header = format!(
            "--------------------------------------------------------------------------------\n{}:\n--------------------------------------------------------------------------------\n",
            relative_path
        );
        let size = fs::metadata(&file).map(|m| m.len()).unwrap_or(0);
        let content = if size > max_size {
            "[File size exceeds limit; skipped]\n".to_string()
        } else if is_binary(&file) {
            "[Binary file skipped]\n".to_string()
        } else {
            read_file_contents(&file)
        };
        file_contents.push_str(&header);
        file_contents.push_str(&content);
        file_contents.push_str("\n\n");
    }
    format!(
        "＜Directory Structure＞\n\n{}\n\n＜File Contents＞\n\n{}",
        tree_text, file_contents
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let dir = Path::new(&args.directory);

    // Default list of allowed file extensions (used when extensions are not specified)
    let default_allowed: HashSet<String> = [
        ".txt", ".md", ".py", ".js", ".java", ".cpp", ".c", ".cs", ".rb", ".go", ".rs",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    // Configure allowed file extensions (use default list if input is empty)
    let allowed: HashSet<String> = if args.extensions.trim().is_empty() {
        default_allowed
    } else {
        args.extensions
            .split(',')
            .filter_map(|s| {
                let s = s.trim().to_lowercase();
                if s.is_empty() {
                    None
                } else if s.starts_with('.') {
                    Some(s)
                } else {
                    Some(format!(".{}", s))
                }
            })
            .collect()
    };

    // Configure ignored file extensions
    let ignore: HashSet<String> = args
        .ignore_extensions
        .split(',')
        .filter_map(|s| {
            let s = s.trim().to_lowercase();
            if s.is_empty() {
                None
            } else if s.starts_with('.') {
                Some(s)
            } else {
                Some(format!(".{}", s))
            }
        })
        .collect();

    // Configure ignored directory names
    let ignore_dirs: HashSet<String> = args
        .ignore_dirs
        .split(',')
        .filter_map(|s| {
            let s = s.trim().to_string();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .collect();

    // Generate the output text
    let output_text = generate_output_text(dir, &allowed, &ignore, &ignore_dirs, args.max_size);

    // If copying to clipboard, use the arboard crate
    if args.clipboard {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => {
                clipboard.set_text(output_text)?;
                println!("Output content has been copied to the clipboard.");
            }
            Err(e) => {
                eprintln!("Failed to access the clipboard: {}", e);
            }
        }
    } else {
        fs::write(&args.output, output_text)?;
        println!("Output completed: {}", args.output);
    }
    Ok(())
}
