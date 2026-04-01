use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::time::{SystemTime, UNIX_EPOCH};

struct TestTempDir {
    path: PathBuf,
}

impl TestTempDir {
    fn new(name: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = env::temp_dir().join(format!(
            "oreuit_integration_{}_{}_{}",
            name,
            process::id(),
            unique
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for TestTempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("config_roundtrip_case")
}

fn oreuit_bin() -> &'static str {
    env!("CARGO_BIN_EXE_oreuit")
}

fn generate_default_config(path: &Path) {
    let output = Command::new(oreuit_bin())
        .arg("--generate-config")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "generate-config failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::write(path, output.stdout).unwrap();
}

#[test]
fn generate_config_round_trips() {
    let temp_dir = TestTempDir::new("generate_config_round_trips");
    let config_path = temp_dir.path.join("generated_default.toml");
    let output_path = temp_dir.path.join("from_config_summary.txt");

    generate_default_config(&config_path);

    let status = Command::new(oreuit_bin())
        .args([
            "--config",
            config_path.to_str().unwrap(),
            "-d",
            fixture_dir().to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .status()
        .unwrap();

    assert!(status.success(), "config path execution failed");
    assert!(output_path.exists(), "summary output was not generated");

    let summary = fs::read_to_string(&output_path).unwrap();
    assert!(summary.contains("＜Directory Structure＞"));
    assert!(summary.contains("sample.msg"));
}

#[test]
fn config_mode_matches_default_output() {
    let temp_dir = TestTempDir::new("config_mode_matches_default_output");
    let config_path = temp_dir.path.join("generated_default.toml");
    let default_output_path = temp_dir.path.join("default_summary.txt");
    let config_output_path = temp_dir.path.join("from_config_summary.txt");
    let fixture_dir = fixture_dir();

    generate_default_config(&config_path);

    let default_status = Command::new(oreuit_bin())
        .args([
            "-d",
            fixture_dir.to_str().unwrap(),
            "-o",
            default_output_path.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(default_status.success(), "default CLI execution failed");

    let config_status = Command::new(oreuit_bin())
        .args([
            "--config",
            config_path.to_str().unwrap(),
            "-d",
            fixture_dir.to_str().unwrap(),
            "-o",
            config_output_path.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(config_status.success(), "config CLI execution failed");

    let default_summary = fs::read_to_string(&default_output_path).unwrap();
    let config_summary = fs::read_to_string(&config_output_path).unwrap();

    assert_eq!(default_summary, config_summary);
}
