use std::collections::HashMap;

use google_gmail1::{
    Gmail,
    api::Label,
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::TokioExecutor,
    },
    yup_oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod},
};

use crate::{Credential, Result};

/// Default for the maximum number of results to return on a page
pub const DEFAULT_MAX_RESULTS: &str = "10";

/// Struct to capture configuration for List API call.
pub struct Labels {
    hub: Gmail<HttpsConnector<HttpConnector>>,
    label_list: Vec<Label>,
    label_map: HashMap<String, String>,
}

impl std::fmt::Debug for Labels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Labels")
            .field("label_list", &self.label_list)
            .field("label_map", &self.label_map)
            .finish()
    }
}

impl Labels {
    /// Create a new List struct and add the Gmail api connection.
    pub async fn new(credential: &str, show: bool) -> Result<Self> {
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

        let call = hub.users().labels_list("me");
        let (_response, list) = call.doit().await.map_err(Box::new)?;

        let Some(label_list) = list.labels else {
            return Ok(Labels {
                hub,
                label_list: Vec::new(),
                label_map: HashMap::new(),
            });
        };

        let mut label_map = HashMap::new();
        for label in &label_list {
            if label.id.is_some() && label.name.is_some() {
                let name = label.name.clone().unwrap();
                let id = label.id.clone().unwrap();
                if show {
                    log::info!("{name}: {id}")
                }
                label_map.insert(name, id);
            }
        }

        Ok(Labels {
            hub,
            label_list,
            label_map,
        })
    }

    /// Return the id for the name from the labels map.
    /// Returns `None` if the name is not found in the map.
    pub fn get_label_id(&self, name: &str) -> Option<String> {
        self.label_map.get(name).cloned()
    }
}
