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
}

impl ClientConfig {
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
            let path = PathBuf::from(root).join(credential_file);
            let json_str = fs::read_to_string(path).expect("could not read path");

            let console: ConsoleApplicationSecret =
                serde_json::from_str(&json_str).expect("could not convert to struct");

            console.installed.unwrap()
        };

        Ok(ClientConfig {
            config_root,
            secret,
        })
    }

    /// Report a reference to the secret.
    pub fn secret(&self) -> &ApplicationSecret {
        &self.secret
    }

    /// Report a reference to the config root.
    pub fn config_root(&self) -> &ConfigRoot {
        &self.config_root
    }
}
