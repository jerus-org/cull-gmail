use std::{env, fs, path::PathBuf};

use google_gmail1::yup_oauth2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Installed {
    pub(crate) client_id: String,
    pub(crate) project_id: Option<String>,
    pub(crate) auth_uri: String,
    pub(crate) token_uri: String,
    pub(crate) auth_provider_x509_cert_url: Option<String>,
    pub(crate) client_secret: String,
    pub(crate) redirect_uris: Vec<String>,
}

/// Struct for google credentials
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    installed: Option<Installed>,
}

impl Credential {
    /// Load the credential from a file.
    pub fn load_json_file(path: &str) -> Self {
        let home_dir = env::home_dir().unwrap();

        let path = PathBuf::new().join(home_dir).join(".config").join(path);
        let json_str = fs::read_to_string(path).expect("could not read path");

        serde_json::from_str(&json_str).expect("could not convert to struct")
    }
}

impl From<Credential> for yup_oauth2::ApplicationSecret {
    fn from(value: Credential) -> Self {
        let mut out = Self::default();
        if let Some(installed) = value.installed {
            out.client_id = installed.client_id;
            out.client_secret = installed.client_secret;
            out.token_uri = installed.token_uri;
            out.auth_uri = installed.auth_uri;
            out.redirect_uris = installed.redirect_uris;
            out.project_id = installed.project_id;
            // out.client_email = installed.client_email;
            out.auth_provider_x509_cert_url = installed.auth_provider_x509_cert_url;
            // out.client_x509_cert_url = installed.client_x509_cert_url;
        }
        out
    }
}
