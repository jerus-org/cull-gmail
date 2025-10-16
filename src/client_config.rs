use std::{fs, path::PathBuf};

use config::Config;
use google_gmail1::yup_oauth2::{ApplicationSecret, ConsoleApplicationSecret};

use crate::Result;

mod config_root;

use config_root::ConfigRoot;

/// Configuration for the gmail client
#[derive(Debug)]
pub struct ClientConfig {
    secret: ApplicationSecret,
    config_root: ConfigRoot,
    persist_path: String,
}

impl ClientConfig {
    /// Config Builder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Create new configuration from configuration
    pub fn new_from_configuration(configs: Config) -> Result<Self> {
        let root = configs.get_string("config_root")?;
        let config_root = ConfigRoot::parse(&root);

        let secret = if let Ok(client_id) = configs.get_string("client_id")
            && let Ok(client_secret) = configs.get_string("client_secret")
            && let Ok(token_uri) = configs.get_string("token_uri")
            && let Ok(auth_uri) = configs.get_string("auth_uri")
        {
            ApplicationSecret {
                client_id,
                client_secret,
                token_uri,
                auth_uri,
                project_id: None,
                redirect_uris: Vec::new(),
                client_email: None,
                auth_provider_x509_cert_url: None,
                client_x509_cert_url: None,
            }
        } else {
            let credential_file = configs.get_string("credential_file")?;
            log::info!("root: {config_root}");
            let path = config_root.full_path().join(credential_file);
            log::info!("path: {}", path.display());
            let json_str = fs::read_to_string(path).expect("could not read path");

            let console: ConsoleApplicationSecret =
                serde_json::from_str(&json_str).expect("could not convert to struct");

            console.installed.unwrap()
        };

        let persist_path = format!("{}/gmail1", config_root.full_path().display());

        Ok(ClientConfig {
            config_root,
            secret,
            persist_path,
        })
    }

    /// Report a reference to the secret.
    pub fn secret(&self) -> &ApplicationSecret {
        &self.secret
    }

    /// Report a reference to the full path to the file to persist tokens
    pub fn persist_path(&self) -> &str {
        &self.persist_path
    }

    /// Report a reference to the config root.
    pub fn config_root(&self) -> &ConfigRoot {
        &self.config_root
    }

    /// Report a reference to the config root.
    pub fn full_path(&self) -> String {
        self.config_root.full_path().display().to_string()
    }
}

#[derive(Debug)]
pub struct ConfigBuilder {
    secret: ApplicationSecret,
    config_root: ConfigRoot,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        let secret = ApplicationSecret {
            auth_uri: "https;://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            ..Default::default()
        };

        Self {
            secret,
            config_root: Default::default(),
        }
    }
}

impl ConfigBuilder {
    pub fn with_config_base(&mut self, value: &config_root::RootBase) -> &mut Self {
        self.config_root.set_root_base(value);
        self
    }

    pub fn with_config_path(&mut self, value: &str) -> &mut Self {
        self.config_root.set_path(value);
        self
    }

    pub fn with_credential_file(&mut self, credential_file: &str) -> &mut Self {
        let path = PathBuf::from(self.config_root.to_string()).join(credential_file);
        log::info!("path: {}", path.display());
        let json_str = fs::read_to_string(path).expect("could not read path");

        let console: ConsoleApplicationSecret =
            serde_json::from_str(&json_str).expect("could not convert to struct");

        self.secret = console.installed.unwrap();
        self
    }

    pub fn with_client_id(&mut self, value: &str) -> &mut Self {
        self.secret.client_id = value.to_string();
        self
    }

    pub fn with_client_secret(&mut self, value: &str) -> &mut Self {
        self.secret.client_secret = value.to_string();
        self
    }

    pub fn with_token_uri(&mut self, value: &str) -> &mut Self {
        self.secret.token_uri = value.to_string();
        self
    }

    pub fn with_auth_uri(&mut self, value: &str) -> &mut Self {
        self.secret.auth_uri = value.to_string();
        self
    }

    pub fn add_redirect_uri(&mut self, value: &str) -> &mut Self {
        self.secret.redirect_uris.push(value.to_string());
        self
    }

    pub fn with_project_id(&mut self, value: &str) -> &mut Self {
        self.secret.project_id = Some(value.to_string());
        self
    }

    pub fn with_client_email(&mut self, value: &str) -> &mut Self {
        self.secret.client_email = Some(value.to_string());
        self
    }
    pub fn with_auth_provider_x509_cert_url(&mut self, value: &str) -> &mut Self {
        self.secret.auth_provider_x509_cert_url = Some(value.to_string());
        self
    }
    pub fn with_client_x509_cert_url(&mut self, value: &str) -> &mut Self {
        self.secret.client_x509_cert_url = Some(value.to_string());
        self
    }

    fn full_path(&self) -> String {
        self.config_root.full_path().display().to_string()
    }

    pub fn build(&self) -> ClientConfig {
        let persist_path = format!("{}/gmail1", self.full_path());

        ClientConfig {
            secret: self.secret.clone(),
            config_root: self.config_root.clone(),
            persist_path,
        }
    }
}
