use std::collections::BTreeMap;

use google_gmail1::{
    Gmail,
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::TokioExecutor,
    },
    yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod},
};

mod message_summary;

pub(crate) use message_summary::MessageSummary;

use crate::{ClientConfig, Error, Result, rules::EolRule};

/// Default for the maximum number of results to return on a page
pub const DEFAULT_MAX_RESULTS: &str = "200";

/// Struct to capture configuration for List API call.
#[derive(Clone)]
pub struct GmailClient {
    hub: Gmail<HttpsConnector<HttpConnector>>,
    label_map: BTreeMap<String, String>,
    pub(crate) max_results: u32,
    pub(crate) label_ids: Vec<String>,
    pub(crate) query: String,
    pub(crate) messages: Vec<MessageSummary>,
    pub(crate) rule: Option<EolRule>,
    pub(crate) execute: bool,
}

impl std::fmt::Debug for GmailClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Labels")
            .field("label_map", &self.label_map)
            .finish()
    }
}

impl GmailClient {
    // /// Create a new Gmail Api connection and fetch label map using credential file.
    // pub async fn new_from_credential_file(credential_file: &str) -> Result<Self> {
    //     let (config_dir, secret) = {
    //         let config_dir = crate::utils::assure_config_dir_exists("~/.cull-gmail")?;

    //         let home_dir = env::home_dir().unwrap();

    //         let path = home_dir.join(".cull-gmail").join(credential_file);
    //         let json_str = fs::read_to_string(path).expect("could not read path");

    //         let console: ConsoleApplicationSecret =
    //             serde_json::from_str(&json_str).expect("could not convert to struct");

    //         let secret: ApplicationSecret = console.installed.unwrap();
    //         (config_dir, secret)
    //     };

    //     GmailClient::new_from_secret(secret, &config_dir).await
    // }

    /// Create a new List struct and add the Gmail api connection.
    pub async fn new_with_config(config: ClientConfig) -> Result<Self> {
        let executor = TokioExecutor::new();
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build();

        let client = Client::builder(executor.clone()).build(connector.clone());

        let auth = InstalledFlowAuthenticator::with_client(
            config.secret().clone(),
            InstalledFlowReturnMethod::HTTPRedirect,
            Client::builder(executor).build(connector),
        )
        .persist_tokens_to_disk(format!("{}/gmail1", config.config_root()))
        .build()
        .await
        .unwrap();

        let hub = Gmail::new(client, auth);
        let label_map = GmailClient::get_label_map(&hub).await?;

        Ok(GmailClient {
            hub,
            label_map,
            max_results: DEFAULT_MAX_RESULTS.parse::<u32>().unwrap(),
            label_ids: Vec::new(),
            query: String::new(),
            messages: Vec::new(),
            rule: None,
            execute: false,
        })
    }

    /// Create a new List struct and add the Gmail api connection.
    async fn get_label_map(
        hub: &Gmail<HttpsConnector<HttpConnector>>,
    ) -> Result<BTreeMap<String, String>> {
        let call = hub.users().labels_list("me");
        let (_response, list) = call
            .add_scope("https://mail.google.com/")
            .doit()
            .await
            .map_err(Box::new)?;

        let Some(label_list) = list.labels else {
            return Err(Error::NoLabelsFound);
        };

        let mut label_map = BTreeMap::new();
        for label in &label_list {
            if label.id.is_some() && label.name.is_some() {
                let name = label.name.clone().unwrap();
                let id = label.id.clone().unwrap();
                label_map.insert(name, id);
            }
        }

        Ok(label_map)
    }

    /// Return the id for the name from the labels map.
    /// Returns `None` if the name is not found in the map.
    pub fn get_label_id(&self, name: &str) -> Option<String> {
        self.label_map.get(name).cloned()
    }

    /// Show the label names and related id.
    pub fn show_label(&self) {
        for (name, id) in self.label_map.iter() {
            log::info!("{name}: {id}")
        }
    }

    /// Get the hub from the client
    pub(crate) fn hub(&self) -> Gmail<HttpsConnector<HttpConnector>> {
        self.hub.clone()
    }
}
