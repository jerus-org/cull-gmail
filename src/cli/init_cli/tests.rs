//! Unit tests for init CLI functionality.

#[cfg(test)]
mod unit_tests {
    use super::super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    /// Test helper to create a mock credential file
    fn create_mock_credential_file(dir: &Path) -> std::io::Result<()> {
        let credential_content = r#"{
            "installed": {
                "client_id": "test-client-id.googleusercontent.com",
                "client_secret": "test-client-secret",
                "auth_uri": "https://accounts.google.com/o/oauth2/auth",
                "token_uri": "https://oauth2.googleapis.com/token",
                "redirect_uris": ["http://localhost"]
            }
        }"#;
        fs::write(dir.join("credential.json"), credential_content)
    }

    #[test]
    fn test_parse_config_root_home() {
        let result = parse_config_root("h:.test-config");
        let home = env::home_dir().unwrap_or_default();
        assert_eq!(result, home.join(".test-config"));
    }

    #[test]
    fn test_parse_config_root_current() {
        let result = parse_config_root("c:.test-config");
        let current = env::current_dir().unwrap_or_default();
        assert_eq!(result, current.join(".test-config"));
    }

    #[test]
    fn test_parse_config_root_root() {
        let result = parse_config_root("r:etc/cull-gmail");
        assert_eq!(result, std::path::PathBuf::from("/etc/cull-gmail"));
    }

    #[test]
    fn test_parse_config_root_no_prefix() {
        let result = parse_config_root("/absolute/path");
        assert_eq!(result, std::path::PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_init_defaults() {
        assert_eq!(InitDefaults::credential_filename(), "credential.json");
        assert_eq!(InitDefaults::config_filename(), "cull-gmail.toml");
        assert_eq!(InitDefaults::rules_filename(), "rules.toml");
        assert_eq!(InitDefaults::token_dir_name(), "gmail1");

        // Test that config content contains expected keys
        let config_content = InitDefaults::CONFIG_FILE_CONTENT;
        assert!(config_content.contains("credential_file = \"credential.json\""));
        assert!(config_content.contains("config_root = \"h:.cull-gmail\""));
        assert!(config_content.contains("execute = false"));

        // Test that rules content is a valid template
        let rules_content = InitDefaults::RULES_FILE_CONTENT;
        assert!(rules_content.contains("# Example rules for cull-gmail"));
        assert!(rules_content.contains("older_than:30d"));
    }

    #[test]
    fn test_validate_credential_file_success() {
        let temp_dir = TempDir::new().unwrap();
        create_mock_credential_file(temp_dir.path()).unwrap();

        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: false,
            dry_run: false,
            interactive: false,
        };

        let credential_path = temp_dir.path().join("credential.json");
        let result = init_cli.validate_credential_file(&credential_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_credential_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: false,
            dry_run: false,
            interactive: false,
        };

        let nonexistent_path = temp_dir.path().join("nonexistent.json");
        let result = init_cli.validate_credential_file(&nonexistent_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_validate_credential_file_invalid_json() {
        let temp_dir = TempDir::new().unwrap();
        let credential_path = temp_dir.path().join("invalid.json");
        fs::write(&credential_path, "invalid json content").unwrap();

        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: false,
            dry_run: false,
            interactive: false,
        };

        let result = init_cli.validate_credential_file(&credential_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid credential file format")
        );
    }

    #[test]
    fn test_plan_operations_new_setup() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("new-config");

        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: false,
            dry_run: false,
            interactive: false,
        };

        let operations = init_cli.plan_operations(&config_path, None).unwrap();

        // Should have: CreateDir, WriteFile (config), WriteFile (rules), EnsureTokenDir
        assert_eq!(operations.len(), 4);

        match &operations[0] {
            Operation::CreateDir { path, .. } => {
                assert_eq!(path, &config_path);
            }
            _ => panic!("Expected CreateDir operation"),
        }

        match &operations[1] {
            Operation::WriteFile { path, contents, .. } => {
                assert_eq!(path, &config_path.join("cull-gmail.toml"));
                assert!(contents.contains("credential_file = \"credential.json\""));
            }
            _ => panic!("Expected WriteFile operation for config"),
        }

        match &operations[2] {
            Operation::WriteFile { path, contents, .. } => {
                assert_eq!(path, &config_path.join("rules.toml"));
                assert!(contents.contains("# Example rules for cull-gmail"));
            }
            _ => panic!("Expected WriteFile operation for rules"),
        }

        match &operations[3] {
            Operation::EnsureTokenDir { path, .. } => {
                assert_eq!(path, &config_path.join("gmail1"));
            }
            _ => panic!("Expected EnsureTokenDir operation"),
        }
    }

    #[test]
    fn test_plan_operations_with_credential_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("new-config");
        let cred_path = temp_dir.path().join("cred.json");
        create_mock_credential_file(temp_dir.path()).unwrap();
        fs::rename(temp_dir.path().join("credential.json"), &cred_path).unwrap();

        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: false,
            dry_run: false,
            interactive: false,
        };

        let operations = init_cli
            .plan_operations(&config_path, Some(&cred_path))
            .unwrap();

        // Should have: CreateDir, CopyFile (credential), WriteFile (config), WriteFile (rules), EnsureTokenDir, RunOAuth2
        assert_eq!(operations.len(), 6);

        // Check that CopyFile operation exists
        let copy_op = operations
            .iter()
            .find(|op| matches!(op, Operation::CopyFile { .. }));
        assert!(copy_op.is_some());

        // Check that RunOAuth2 operation exists
        let oauth_op = operations
            .iter()
            .find(|op| matches!(op, Operation::RunOAuth2 { .. }));
        assert!(oauth_op.is_some());
    }

    #[test]
    fn test_plan_operations_existing_config_no_force() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("existing-config");
        fs::create_dir_all(&config_path).unwrap();

        // Create existing config file
        fs::write(config_path.join("cull-gmail.toml"), "existing config").unwrap();

        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: false,
            dry_run: false,
            interactive: false,
        };

        let result = init_cli.plan_operations(&config_path, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));
    }

    #[test]
    fn test_plan_operations_existing_config_with_force() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("existing-config");
        fs::create_dir_all(&config_path).unwrap();

        // Create existing config file
        fs::write(config_path.join("cull-gmail.toml"), "existing config").unwrap();
        fs::write(config_path.join("rules.toml"), "existing rules").unwrap();

        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: true,
            dry_run: false,
            interactive: false,
        };

        let operations = init_cli.plan_operations(&config_path, None).unwrap();

        // Should succeed and plan backup operations
        let config_op = operations.iter().find(|op| {
            if let Operation::WriteFile {
                path,
                backup_if_exists,
                ..
            } = op
            {
                path.file_name().unwrap() == "cull-gmail.toml" && *backup_if_exists
            } else {
                false
            }
        });
        assert!(config_op.is_some());
    }

    #[test]
    fn test_create_backup() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: false,
            dry_run: false,
            interactive: false,
        };

        let result = init_cli.create_backup(&test_file);
        assert!(result.is_ok());

        // Check that a backup file was created
        let backup_files: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("test.bak-") {
                    Some(name)
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(backup_files.len(), 1);

        // Verify backup content
        let backup_path = temp_dir.path().join(&backup_files[0]);
        let backup_content = fs::read_to_string(backup_path).unwrap();
        assert_eq!(backup_content, "test content");
    }

    #[cfg(unix)]
    #[test]
    fn test_set_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        let init_cli = InitCli {
            config_dir: "test".to_string(),
            credential_file: None,
            force: false,
            dry_run: false,
            interactive: false,
        };

        let result = init_cli.set_permissions(&test_file, 0o600);
        assert!(result.is_ok());

        let metadata = fs::metadata(&test_file).unwrap();
        let permissions = metadata.permissions();
        assert_eq!(permissions.mode() & 0o777, 0o600);
    }

    #[test]
    fn test_operation_display() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().join("test");

        let create_dir_op = Operation::CreateDir {
            path: temp_path.clone(),
            #[cfg(unix)]
            mode: Some(0o755),
        };
        assert_eq!(format!("{create_dir_op}"), format!("Create directory: {}", temp_path.display()));

        let copy_file_op = Operation::CopyFile {
            from: temp_path.clone(),
            to: temp_path.join("dest"),
            #[cfg(unix)]
            mode: Some(0o600),
            backup_if_exists: false,
        };
        assert_eq!(
            format!("{copy_file_op}"),
            format!("Copy file: {} → {}", temp_path.display(), temp_path.join("dest").display())
        );

        let write_file_op = Operation::WriteFile {
            path: temp_path.clone(),
            contents: "content".to_string(),
            #[cfg(unix)]
            mode: Some(0o644),
            backup_if_exists: false,
        };
        assert_eq!(format!("{write_file_op}"), format!("Write file: {}", temp_path.display()));

        let oauth_op = Operation::RunOAuth2 {
            config_root: "h:.config".to_string(),
            credential_file: Some("cred.json".to_string()),
        };
        assert_eq!(format!("{oauth_op}"), "Run OAuth2 authentication flow");
    }

    #[cfg(unix)]
    #[test]
    fn test_operation_get_mode() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().join("test");

        let create_dir_op = Operation::CreateDir {
            path: temp_path.clone(),
            mode: Some(0o755),
        };
        assert_eq!(create_dir_op.get_mode(), Some(0o755));

        let oauth_op = Operation::RunOAuth2 {
            config_root: "h:.config".to_string(),
            credential_file: Some("cred.json".to_string()),
        };
        assert_eq!(oauth_op.get_mode(), None);
    }
}
