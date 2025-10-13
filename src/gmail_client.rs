use std::collections::BTreeMap;

use google_gmail1::{
    Gmail,
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::TokioExecutor,
    },
    yup_oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod},
};

use crate::{Credential, Error, MessageList, Result};

/// Default for the maximum number of results to return on a page
pub const DEFAULT_MAX_RESULTS: &str = "200";

/// Struct to capture configuration for List API call.
#[derive(Clone)]
pub struct GmailClient {
    hub: Gmail<HttpsConnector<HttpConnector>>,
    label_map: BTreeMap<String, String>,
}

impl std::fmt::Debug for GmailClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Labels")
            .field("label_map", &self.label_map)
            .finish()
    }
}

impl GmailClient {
    /// Create a new List struct and add the Gmail api connection.
    pub async fn new(credential: &str) -> Result<Self> {
        let (config_dir, secret) = {
            let config_dir = crate::utils::assure_config_dir_exists("~/.cull-gmail")?;

            let secret: ApplicationSecret = Credential::load_json_file(credential).into();
            (config_dir, secret)
        };

        let executor = TokioExecutor::new();
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build();

        let client = Client::builder(executor.clone()).build(connector.clone());

        let auth = InstalledFlowAuthenticator::with_client(
            secret,
            InstalledFlowReturnMethod::HTTPRedirect,
            Client::builder(executor).build(connector),
        )
        .persist_tokens_to_disk(format!("{config_dir}/gmail1"))
        .build()
        .await
        .unwrap();

        let hub = Gmail::new(client, auth);
        let label_map = GmailClient::get_label_map(&hub).await?;

        Ok(GmailClient { hub, label_map })
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

    /// Get the message list
    pub async fn get_message_list(&self) -> Result<MessageList> {
        MessageList::new(self).await
    }
}
