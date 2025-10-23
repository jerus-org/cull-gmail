//! Integration tests for the init CLI command.

use assert_cmd::prelude::*;
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

/// Creates a mock OAuth2 credential file with test data.
///
/// This helper function creates a valid OAuth2 credential JSON file
/// suitable for testing credential file handling without using real credentials.
fn create_mock_credential_file(credential_file: &ChildPath) {
    credential_file
        .write_str(
            r#"{
        "installed": {
            "client_id": "test-client-id.googleusercontent.com",
            "client_secret": "test-client-secret",
            "auth_uri": "https://accounts.google.com/o/oauth2/auth",
            "token_uri": "https://oauth2.googleapis.com/token",
            "redirect_uris": ["http://localhost"]
        }
    }"#,
        )
        .unwrap();
}

/// Helper to run init command with config and rules directories.
///
/// This helper reduces duplication when testing init with separate directories.
fn run_init_with_dirs(
    config_dir: &std::path::Path,
    rules_dir: &std::path::Path,
    dry_run: bool,
) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    let config_arg = format!("c:{}", config_dir.to_string_lossy());
    let rules_arg = format!("c:{}", rules_dir.to_string_lossy());

    cmd.args([
        "init",
        "--config-dir",
        &config_arg,
        "--rules-dir",
        &rules_arg,
    ]);

    if dry_run {
        cmd.arg("--dry-run");
    }

    cmd.assert()
}

/// Helper to run init command with credential file.
///
/// This helper reduces duplication when testing init with credential files.
fn run_init_with_credential(
    config_dir: &std::path::Path,
    credential_path: &std::path::Path,
    dry_run: bool,
) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    let config_arg = format!("c:{}", config_dir.to_string_lossy());

    cmd.args([
        "init",
        "--config-dir",
        &config_arg,
        "--credential-file",
        credential_path.to_str().unwrap(),
    ]);

    if dry_run {
        cmd.arg("--dry-run");
    }

    cmd.assert()
}

#[test]
fn test_init_help() {
    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args(["init", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Initialize cull-gmail configuration",
        ))
        .stdout(predicate::str::contains("--config-dir"))
        .stdout(predicate::str::contains("--credential-file"))
        .stdout(predicate::str::contains("--dry-run"))
        .stdout(predicate::str::contains("--interactive"))
        .stdout(predicate::str::contains("--force"));
}

#[test]
fn test_init_dry_run_new_setup() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
        "--dry-run",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN: No changes will be made"))
        .stdout(predicate::str::contains("Planned operations:"))
        .stdout(predicate::str::contains("Create directory:"))
        .stdout(predicate::str::contains("Write file:"))
        .stdout(predicate::str::contains("cull-gmail.toml"))
        .stdout(predicate::str::contains("rules.toml"))
        .stdout(predicate::str::contains("Ensure token directory:"))
        .stdout(predicate::str::contains("gmail1"))
        .stdout(predicate::str::contains("OAuth2 authentication skipped"))
        .stdout(predicate::str::contains(
            "To apply these changes, run without --dry-run",
        ));

    // Verify no files were actually created
    assert!(!config_dir.exists());

    temp_dir.close().unwrap();
}

#[test]
fn test_init_with_separate_rules_directory() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("config");
    let rules_dir = temp_dir.path().join("rules");

    run_init_with_dirs(&config_dir, &rules_dir, false)
        .success()
        .stdout(predicate::str::contains(
            "Initialization completed successfully!",
        ));

    // Verify config directory was created
    assert!(config_dir.exists());
    assert!(config_dir.join("cull-gmail.toml").exists());
    assert!(config_dir.join("gmail1").exists());

    // Verify rules directory was created separately
    assert!(rules_dir.exists());
    assert!(rules_dir.join("rules.toml").exists());

    // Verify rules.toml is NOT in config directory
    assert!(!config_dir.join("rules.toml").exists());

    // Verify config file references the correct rules path
    let config_content = std::fs::read_to_string(config_dir.join("cull-gmail.toml")).unwrap();
    let rules_path = rules_dir.join("rules.toml");
    assert!(config_content.contains(&rules_path.to_string_lossy().to_string()));

    temp_dir.close().unwrap();
}

#[test]
fn test_init_rules_dir_dry_run() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("config");
    let rules_dir = temp_dir.path().join("rules");

    run_init_with_dirs(&config_dir, &rules_dir, true)
        .success()
        .stdout(predicate::str::contains("DRY RUN: No changes will be made"))
        .stdout(predicate::str::contains("Create directory:"))
        .stdout(predicate::str::contains("rules.toml"));

    // Verify no directories were created in dry-run
    assert!(!config_dir.exists());
    assert!(!rules_dir.exists());

    temp_dir.close().unwrap();
}

#[test]
fn test_init_dry_run_with_credential_file() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");
    let credential_file = temp_dir.child("credential.json");

    // Create a mock credential file
    create_mock_credential_file(&credential_file);

    run_init_with_credential(&config_dir, credential_file.path(), true)
        .success()
        .stdout(predicate::str::contains("DRY RUN: No changes will be made"))
        .stdout(predicate::str::contains("Planned operations:"))
        .stdout(predicate::str::contains("Copy file:"))
        .stdout(predicate::str::contains("credential.json"))
        .stdout(predicate::str::contains("OAuth2 authentication would open"));

    // Verify no files were actually created
    assert!(!config_dir.exists());

    temp_dir.close().unwrap();
}

#[test]
fn test_init_actual_execution() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Initialization completed successfully!",
        ))
        .stdout(predicate::str::contains("Configuration directory:"))
        .stdout(predicate::str::contains("Files created:"))
        .stdout(predicate::str::contains("cull-gmail.toml"))
        .stdout(predicate::str::contains("rules.toml"))
        .stdout(predicate::str::contains("Next steps:"));

    // Verify files were actually created
    assert!(config_dir.exists());
    assert!(config_dir.join("cull-gmail.toml").exists());
    assert!(config_dir.join("rules.toml").exists());
    assert!(config_dir.join("gmail1").exists());

    // Verify file contents
    let config_content = std::fs::read_to_string(config_dir.join("cull-gmail.toml")).unwrap();
    assert!(config_content.contains("credential_file = \"credential.json\""));
    assert!(config_content.contains("execute = false"));

    let rules_content = std::fs::read_to_string(config_dir.join("rules.toml")).unwrap();
    assert!(rules_content.contains("# Example rules for cull-gmail"));

    temp_dir.close().unwrap();
}

#[test]
fn test_init_force_overwrite() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");

    // Create config directory and file first
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("cull-gmail.toml"), "old config").unwrap();

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
        "--force",
        "--dry-run",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN: No changes will be made"))
        .stdout(predicate::str::contains("(with backup)"));

    temp_dir.close().unwrap();
}

#[test]
fn test_init_existing_config_no_force_fails() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");

    // Create config directory and file first
    std::fs::create_dir_all(&config_dir).unwrap();
    std::fs::write(config_dir.join("cull-gmail.toml"), "existing config").unwrap();

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("already exists"))
        .stderr(predicate::str::contains("--force"))
        .stderr(predicate::str::contains("--interactive"));

    temp_dir.close().unwrap();
}

#[test]
fn test_init_with_credential_file_copy() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");
    let credential_file = temp_dir.child("source_credential.json");

    // Create a mock credential file
    create_mock_credential_file(&credential_file);

    // This will fail at OAuth step, but we can check that files were created correctly
    let _output = run_init_with_credential(&config_dir, credential_file.path(), false)
        .get_output()
        .clone();

    // Verify files were created up to the OAuth step
    assert!(config_dir.exists());
    assert!(config_dir.join("cull-gmail.toml").exists());
    assert!(config_dir.join("rules.toml").exists());
    assert!(config_dir.join("credential.json").exists());
    assert!(config_dir.join("gmail1").exists());

    // Verify credential file was copied
    let copied_credential_content =
        std::fs::read_to_string(config_dir.join("credential.json")).unwrap();
    assert!(copied_credential_content.contains("test-client-id.googleusercontent.com"));

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // Verify credential file has secure permissions
        let metadata = std::fs::metadata(config_dir.join("credential.json")).unwrap();
        let permissions = metadata.permissions();
        assert_eq!(permissions.mode() & 0o777, 0o600);
    }

    temp_dir.close().unwrap();
}

#[test]
fn test_init_invalid_credential_file() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");
    let credential_file = temp_dir.child("invalid_credential.json");

    // Create an invalid credential file
    credential_file.write_str("invalid json content").unwrap();

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
        "--credential-file",
        credential_file.path().to_str().unwrap(),
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid credential file format"));

    temp_dir.close().unwrap();
}

#[test]
fn test_init_nonexistent_credential_file() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");
    let nonexistent_file = temp_dir.path().join("nonexistent.json");

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
        "--credential-file",
        nonexistent_file.to_str().unwrap(),
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("not found"));

    temp_dir.close().unwrap();
}

// This test would require real Gmail credentials and should be ignored by default
#[test]
#[ignore = "requires real Gmail OAuth2 credentials"]
fn test_init_oauth_integration() {
    // This test should only run when CULL_GMAIL_TEST_CREDENTIAL_FILE is set
    let credential_file = std::env::var("CULL_GMAIL_TEST_CREDENTIAL_FILE")
        .expect("CULL_GMAIL_TEST_CREDENTIAL_FILE must be set for OAuth integration test");

    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
        "--credential-file",
        &credential_file,
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "OAuth2 authentication successful!",
        ))
        .stdout(predicate::str::contains("gmail1/ (OAuth2 token cache)"));

    // Verify token files were created
    assert!(config_dir.join("gmail1").exists());

    // Check if there are token-related files in the gmail1 directory
    let gmail_dir_contents = std::fs::read_dir(config_dir.join("gmail1")).unwrap();
    let has_token_files = gmail_dir_contents.count() > 0;
    assert!(
        has_token_files,
        "Expected token files to be created in gmail1 directory"
    );

    temp_dir.close().unwrap();
}

#[test]
fn test_init_with_skip_rules_dry_run_shows_skipped() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--skip-rules",
        "--dry-run",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("DRY RUN: No changes will be made"))
        .stdout(predicate::str::contains("Planned operations:"))
        .stdout(predicate::str::contains("cull-gmail.toml"))
        .stdout(predicate::str::contains(
            "rules.toml: skipped (per --skip-rules flag)",
        ))
        .stdout(predicate::str::contains(
            "The rules file path is configured in cull-gmail.toml",
        ))
        .stdout(predicate::str::contains(
            "Expected to be provided externally",
        ));

    // Verify no files were actually created
    assert!(!config_dir.exists());

    temp_dir.close().unwrap();
}

#[test]
fn test_init_with_skip_rules_creates_config_but_not_rules() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("test-config");

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--skip-rules",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "Initialization completed successfully!",
        ))
        .stdout(predicate::str::contains(
            "rules.toml (SKIPPED - expected to be provided externally)",
        ));

    // Verify config directory was created
    assert!(config_dir.exists());
    assert!(config_dir.join("cull-gmail.toml").exists());
    assert!(config_dir.join("gmail1").exists());

    // Verify rules.toml was NOT created
    assert!(!config_dir.join("rules.toml").exists());

    // Verify config file contains skip-rules comment
    let config_content = std::fs::read_to_string(config_dir.join("cull-gmail.toml")).unwrap();
    assert!(config_content.contains("NOTE: rules.toml creation was skipped via --skip-rules flag"));
    assert!(config_content.contains("expected to be provided externally"));
    assert!(config_content.contains("rules = \"rules.toml\""));

    temp_dir.close().unwrap();
}

#[test]
fn test_init_with_skip_rules_and_rules_dir_creates_dir_only() {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let config_dir = temp_dir.path().join("config");
    let rules_dir = temp_dir.path().join("rules");

    let mut cmd = Command::cargo_bin("cull-gmail").unwrap();
    cmd.args([
        "init",
        "--skip-rules",
        "--config-dir",
        &format!("c:{}", config_dir.to_string_lossy()),
        "--rules-dir",
        &format!("c:{}", rules_dir.to_string_lossy()),
    ]);

    cmd.assert().success().stdout(predicate::str::contains(
        "Initialization completed successfully!",
    ));

    // Verify config directory was created
    assert!(config_dir.exists());
    assert!(config_dir.join("cull-gmail.toml").exists());

    // Verify rules directory was created
    assert!(rules_dir.exists());

    // Verify rules.toml was NOT created in either directory
    assert!(!config_dir.join("rules.toml").exists());
    assert!(!rules_dir.join("rules.toml").exists());

    // Verify config file references the correct rules path
    let config_content = std::fs::read_to_string(config_dir.join("cull-gmail.toml")).unwrap();
    let expected_rules_path = rules_dir.join("rules.toml");
    assert!(config_content.contains(&expected_rules_path.to_string_lossy().to_string()));
    assert!(config_content.contains("NOTE: rules.toml creation was skipped via --skip-rules flag"));

    temp_dir.close().unwrap();
}
