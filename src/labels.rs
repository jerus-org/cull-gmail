use std::collections::HashMap;

use google_gmail1::{
    Gmail,
    api::{Label, ListLabelsResponse},
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::TokioExecutor,
    },
    yup_oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod},
};

use crate::{Credential, Error};

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
    pub async fn new(credential: &str) -> Result<Self, Error> {
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

        Ok(Labels {
            hub: Gmail::new(client, auth),
            label_list: Vec::new(),
            label_map: HashMap::new(),
        })
    }

    /// Get the labels for the authorised account
    pub async fn get_labels(&mut self) -> Result<(), Error> {
        let call = self.hub.users().labels_list("me");
        let (_response, list) = call.doit().await.map_err(Box::new)?;

        self.log_label_names(&list).await?;

        if let Some(labels) = list.labels {
            let mut label_map = HashMap::new();
            for label in &labels {
                if label.id.is_some() && label.name.is_some() {
                    let label = label.clone();
                    label_map.insert(label.name.unwrap(), label.id.unwrap());
                }
            }
            self.label_list = labels;
            self.label_map = label_map;
        }

        Ok(())
    }

    async fn log_label_names(&self, list: &ListLabelsResponse) -> Result<(), Error> {
        if let Some(labels) = &list.labels {
            for label in labels {
                if let Some(name) = &label.name {
                    log::info!("{name}");
                } else {
                    log::warn!("No name for label {:?}", label.id);
                }
            }
        }

        Ok(())
    }

    /// Return the id for the name from the labels map.
    /// Returns `None` if the name is not found in the map.
    pub fn get_label_id(&self, name: &str) -> Option<String> {
        self.label_map.get(name).cloned()
    }
}
