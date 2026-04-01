use std::process::Command;

fn oreuit_bin() -> &'static str {
    env!("CARGO_BIN_EXE_oreuit")
}

#[test]
fn short_help_points_to_long_help() {
    let output = Command::new(oreuit_bin()).arg("-h").output().unwrap();

    assert!(output.status.success(), "short help failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Run `oreuit --help`"));
    assert!(stdout.contains("Generate a text snapshot of directory trees and file contents."));
    assert!(!stdout.contains("Selection precedence (highest first):"));
    assert!(!stdout.contains("Default ignored directories:"));
}

#[test]
fn long_help_describes_behavior_and_defaults() {
    let output = Command::new(oreuit_bin()).arg("--help").output().unwrap();

    assert!(output.status.success(), "long help failed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Selection precedence (highest first):"));
    assert!(stdout.contains("match basenames only, not relative paths"));
    assert!(stdout.contains("all non-ignored extensions and all extensionless files are eligible"));
    assert!(stdout.contains("These options still work with `--config`"));
    assert!(stdout.contains("[File size exceeds limit; skipped]"));
    assert!(stdout.contains("[Binary file skipped]"));
    assert!(stdout.contains("[Cannot decode file content]"));
    assert!(stdout.contains("UTF-8 first, then falls back to Shift_JIS"));
    assert!(stdout.contains("Default allowed extensions:"));
    assert!(stdout.contains("Default ignored directories:"));
}
