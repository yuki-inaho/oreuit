use clap::Parser;
use encoding_rs::SHIFT_JIS;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
#[macro_use]
extern crate lazy_static;

use walkdir::WalkDir;

/// Tool to summarize directory structure and file contents
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "A tool to summarize directory structure and file contents (similar to uithub)"
)]
struct Args {
    /// Directories to explore (comma-separated, default is the current directory)
    #[clap(short = 'd', long = "directory", default_value = ".")]
    directories: String,

    /// Allowed file extensions (comma-separated, e.g., .txt,.md,.py).
    /// Prefix with '+,' to ADD to the default list (e.g., +,.json,.vue).
    /// If not specified, the default list is used.
    #[clap(short = 'e', long = "extensions")]
    extensions: Option<String>,

    /// File extensions to ignore (comma-separated, e.g., .bin,.zip,...). If empty, no extensions are ignored
    #[clap(
        short = 'i',
        long = "ignore-extensions",
        default_value = ".bin,.zip,.tar,.gz,.7z,.rar,.exe,.dll,.so,.dylib,.a,.lib,.obj,.o,.class,.jar,.war,.ear,.ipynb,.jpg,.jpeg,.png,.gif"
    )]
    ignore_extensions: String,

    /// Filenames to ignore (comma-separated, e.g., setup.py,config.toml). Overrides allowed extensions but not whitelist.
    #[clap(long = "ignore-files", default_value = "")]
    ignore_files: String,

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
    /// Prefix with '+,' to ADD to the default list (e.g., +,my_temp,build2).
    #[clap(long = "ignore-dirs")]
    ignore_dirs: Option<String>,

    /// Filenames to whitelist (comma-separated, e.g., Dockerfile,Makefile). These are always included.
    #[clap(
        short = 'w',
        long = "whitelist-filenames",
        default_value = "Dockerfile,Makefile,justfile"
    )]
    whitelist_filenames: String,
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
/// - Match allowed extensions OR are whitelisted filenames
/// - Do not have ignored extensions
/// - Are not ignored filenames
/// Files within ignored directories are not searched.
fn collect_files(
    directory: &Path,
    allowed: &HashSet<String>,
    ignore_exts: &HashSet<String>,
    ignore_dirs: &HashSet<String>,
    whitelist_filenames: &HashSet<String>,
    ignore_files: &HashSet<String>,
) -> Vec<PathBuf> {
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
            let file_name_os = entry.file_name();
            let file_name = file_name_os.to_string_lossy();
            if whitelist_filenames.contains(file_name.as_ref()) {
                files.push(path.to_path_buf());
                continue;
            }
            if ignore_files.contains(file_name.as_ref()) {
                continue;
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext_formatted = format!(".{}", ext.to_lowercase());
                if ignore_exts.contains(&ext_formatted) {
                    continue;
                }
                if !allowed.is_empty() && !allowed.contains(&ext_formatted) {
                    continue;
                }
            } else {
                let allowed_no_ext = [
                    "Makefile", "Dockerfile", "LICENSE", "README", ".gitignore", ".gitattributes", "justfile"
                ];
                if !allowed.is_empty() && !allowed_no_ext.contains(&file_name.as_ref()) {
                    continue;
                }
            }
            files.push(path.to_path_buf());
        }
    }
    files.sort();
    files
}

/// Generates a tree structure of the specified directory.
fn build_tree(
    directory: &Path,
    allowed: &HashSet<String>,
    ignore_exts: &HashSet<String>,
    ignore_dirs: &HashSet<String>,
    whitelist_filenames: &HashSet<String>,
    ignore_files: &HashSet<String>,
) -> String {
    let base_name = match directory.file_name().and_then(|s| s.to_str()) {
        Some(s) => s.to_string(),
        None => directory.to_string_lossy().into_owned(),
    };
    let mut lines = vec![base_name];
    build_tree_helper(
        directory,
        "",
        allowed,
        ignore_exts,
        ignore_dirs,
        whitelist_filenames,
        ignore_files,
        &mut lines,
    );
    lines.join("\n")
}

/// Helper function that recursively traverses the directory structure and builds the tree string
fn build_tree_helper(
    path: &Path,
    prefix: &str,
    allowed: &HashSet<String>,
    ignore_exts: &HashSet<String>,
    ignore_dirs: &HashSet<String>,
    whitelist_filenames: &HashSet<String>,
    ignore_files: &HashSet<String>,
    lines: &mut Vec<String>,
) {
    let mut entries: Vec<fs::DirEntry> = match fs::read_dir(path) {
        Ok(iter) => iter.filter_map(|e| e.ok()).collect(),
        Err(_) => return,
    };
    entries.sort_by_key(|e| e.file_name());
    let mut filtered_entries = Vec::new();
    for entry in entries {
        let entry_path = entry.path();
        let file_name_os = entry.file_name();
        let name_buf = file_name_os.to_string_lossy().to_string();
        let name = &name_buf;
        if entry_path.is_dir() {
            if ignore_dirs.contains(name) {
                continue;
            }
            filtered_entries.push((entry, true));
        } else if entry_path.is_file() {
            if whitelist_filenames.contains(name) {
                filtered_entries.push((entry, false));
                continue;
            }
            if ignore_files.contains(name) {
                continue;
            }
            if let Some(ext) = entry_path.extension().and_then(|e| e.to_str()) {
                let ext_formatted = format!(".{}", ext.to_lowercase());
                if ignore_exts.contains(&ext_formatted) {
                    continue;
                }
                if !allowed.is_empty() && !allowed.contains(&ext_formatted) {
                    continue;
                }
            } else {
                let allowed_no_ext = [
                    "Makefile", "Dockerfile", "LICENSE", "README", ".gitignore", ".gitattributes", "justfile"
                ];
                if !allowed.is_empty() && !allowed_no_ext.contains(&name.as_ref()) {
                    continue;
                }
            }
            filtered_entries.push((entry, false));
        }
    }
    let count = filtered_entries.len();
    for (i, (entry, is_dir)) in filtered_entries.into_iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let name_buf = entry.file_name().to_string_lossy().to_string();
        let name = &name_buf;
        lines.push(format!("{}{}{}", prefix, connector, name));
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
                ignore_exts,
                ignore_dirs,
                whitelist_filenames,
                ignore_files,
                lines,
            );
        }
    }
}

lazy_static! {
    static ref DEFAULT_ALLOWED_EXTENSIONS: HashSet<String> = [
        ".txt", ".md", ".py", ".js", ".java", ".cpp", ".c", ".cs", ".rb", ".go", ".rs", ".hpp",
        ".ts", ".tsx", ".d.ts", ".jsx", ".toml",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    static ref DEFAULT_IGNORE_DIRS: HashSet<String> = [
        ".git",
        ".vscode",
        "target",
        "node_modules",
        "__pycache__",
        ".idea",
        "build",
        "dist",
        ".ruff_cache",
        ".cache",
        ".tox",
        ".nox",
        ".pytest_cache",
        "htmlcov",
        "instance",
        ".env",
        ".venv",
        "env",
        "venv",
        "ENV",
        "site",
        ".mypy_cache",
        "debug",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let directories: Vec<PathBuf> = args
        .directories
        .split(',')
        .filter_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                None
            } else {
                let path = PathBuf::from(s);
                if !path.exists() {
                    eprintln!("Warning: Directory not found, skipping: {}", path.display());
                    None
                } else if !path.is_dir() {
                    eprintln!(
                        "Warning: Path is not a directory, skipping: {}",
                        path.display()
                    );
                    None
                } else {
                    Some(path)
                }
            }
        })
        .collect();

    if directories.is_empty() {
        eprintln!("Error: No valid directories specified or found.");
        return Ok(());
    }

    let allowed: HashSet<String> = match &args.extensions {
        None => DEFAULT_ALLOWED_EXTENSIONS.clone(),
        Some(val) => {
            let val = val.trim();
            if val.is_empty() {
                DEFAULT_ALLOWED_EXTENSIONS.clone()
            } else if val.starts_with("+,") {
                let mut set = DEFAULT_ALLOWED_EXTENSIONS.clone();
                for s in val.trim_start_matches("+,").split(',') {
                    let s = s.trim().to_lowercase();
                    if s.is_empty() {
                        continue;
                    }
                    if s.starts_with('.') {
                        set.insert(s);
                    } else {
                        set.insert(format!(".{}", s));
                    }
                }
                set
            } else {
                val.split(',')
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
            }
        }
    };

    let ignore_exts: HashSet<String> = args
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

    let ignore_dirs: HashSet<String> = match &args.ignore_dirs {
        None => DEFAULT_IGNORE_DIRS.clone(),
        Some(val) => {
            let val = val.trim();
            if val.is_empty() {
                DEFAULT_IGNORE_DIRS.clone()
            } else if val.starts_with("+,") {
                let mut set = DEFAULT_IGNORE_DIRS.clone();
                for s in val.trim_start_matches("+,").split(',') {
                    let s = s.trim().to_string();
                    if s.is_empty() {
                        continue;
                    }
                    set.insert(s);
                }
                set
            } else {
                val.split(',')
                    .filter_map(|s| {
                        let s = s.trim().to_string();
                        if s.is_empty() {
                            None
                        } else {
                            Some(s)
                        }
                    })
                    .collect()
            }
        }
    };

    let whitelist_filenames: HashSet<String> = args
        .whitelist_filenames
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

    let ignore_files: HashSet<String> = args
        .ignore_files
        .split(',')
        .filter_map(|s| {
            let s = s.trim().to_string();
            if s.is_empty() { None } else { Some(s) }
        })
        .collect();

    let mut all_tree_text = String::new();
    let mut all_file_contents = String::new();

    for dir in &directories {
        let dir_name_for_header = match dir.file_name().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => dir.to_string_lossy().into_owned(),
        };

        let tree_text = build_tree(
            dir,
            &allowed,
            &ignore_exts,
            &ignore_dirs,
            &whitelist_filenames,
            &ignore_files,
        );

        all_tree_text.push_str(&format!(
            "=== Tree for {} ===\n{}\n\n",
            dir_name_for_header, tree_text
        ));

        let files = collect_files(
            dir,
            &allowed,
            &ignore_exts,
            &ignore_dirs,
            &whitelist_filenames,
            &ignore_files,
        );

        for file in files {
            let relative_path = file.strip_prefix(dir).unwrap_or(&file).to_string_lossy();

            let header = format!(
                "--------------------------------------------------------------------------------\n{} (in {}):\n--------------------------------------------------------------------------------\n",
                relative_path, dir_name_for_header
            );
            let size = fs::metadata(&file).map(|m| m.len()).unwrap_or(0);
            let content = if size > args.max_size {
                "[File size exceeds limit; skipped]\n".to_string()
            } else if is_binary(&file) {
                "[Binary file skipped]\n".to_string()
            } else {
                read_file_contents(&file)
            };
            all_file_contents.push_str(&header);
            all_file_contents.push_str(&content);
            all_file_contents.push_str("\n\n");
        }
    }

    if !all_tree_text.is_empty() {
        all_tree_text.pop();
        all_tree_text.pop();
    }
    if !all_file_contents.is_empty() {
        all_file_contents.pop();
        all_file_contents.pop();
    }

    let output_text = format!(
        "＜Directory Structure＞\n\n{}\n\n＜File Contents＞\n\n{}",
        all_tree_text, all_file_contents
    );

    if args.clipboard {
        #[cfg(feature = "clipboard")]
        {
            // Assumes arboard is set as optional = true and configured in features in Cargo.toml
            match arboard::Clipboard::new() {
                Ok(mut clipboard) => {
                    clipboard.set_text(output_text)?;
                    println!("Output content has been copied to the clipboard.");
                }
                Err(e) => {
                    eprintln!(
                        "Failed to access the clipboard: {}. Try writing to a file instead.",
                        e
                    );
                }
            }
        }
        #[cfg(not(feature = "clipboard"))]
        {
            eprintln!("Clipboard feature is not enabled. Please compile with '--features clipboard' or use the -o option to write to a file.");
        }
    } else {
        fs::write(&args.output, output_text)?;
        println!("Output completed: {}", args.output);
    }
    Ok(())
}
