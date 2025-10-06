use std::fmt::Debug;

use google_gmail1::{
    Gmail,
    api::ListMessagesResponse,
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
pub struct MessageList {
    hub: Gmail<HttpsConnector<HttpConnector>>,
    max_results: u32,
    label_ids: Vec<String>,
    message_ids: Vec<String>,
    query: String,
}

impl Debug for MessageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageList")
            .field("max_results", &self.max_results)
            .field("label_ids", &self.label_ids)
            .field("message_ids", &self.message_ids)
            .field("query", &self.query)
            .finish()
    }
}

impl MessageList {
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

        Ok(MessageList {
            hub: Gmail::new(client, auth),
            max_results: DEFAULT_MAX_RESULTS.parse::<u32>().unwrap(),
            label_ids: Vec::new(),
            message_ids: Vec::new(),
            query: String::new(),
        })
    }

    /// Set the maximum results
    pub fn set_max_results(&mut self, value: u32) {
        self.max_results = value;
    }

    /// Report the maximum results value
    pub fn max_results(&self) -> u32 {
        self.max_results
    }

    /// Add label to the labels collection
    pub fn add_labels(&mut self, label_ids: &[String]) {
        if !label_ids.is_empty() {
            for id in label_ids {
                self.label_ids.push(id.to_string())
            }
        }
    }

    /// Set the query string
    pub fn set_query(&mut self, query: &str) {
        self.query = query.to_string()
    }

    /// Get a reference to the message_ids
    pub fn message_ids(&self) -> Vec<String> {
        self.message_ids.clone()
    }

    /// Get a reference to the message_ids
    pub fn label_ids(&self) -> Vec<String> {
        self.label_ids.clone()
    }

    /// Get the hub
    pub fn hub(&self) -> Gmail<HttpsConnector<HttpConnector>> {
        self.hub.clone()
    }

    /// Run the Gmail api as configured
    pub async fn run(&mut self, pages: u32) -> Result<(), Error> {
        let list = self.messages_list(None).await?;
        match pages {
            1 => {}
            0 => {
                let mut list = list;
                let mut page = 1;
                loop {
                    page += 1;
                    log::debug!("Processing page #{page}");
                    if list.next_page_token.is_none() {
                        break;
                    }
                    list = self.messages_list(list.next_page_token).await?;
                    // self.log_message_subjects(&list).await?;
                }
            }
            _ => {
                let mut list = list;
                for page in 2..=pages {
                    log::debug!("Processing page #{page}");
                    if list.next_page_token.is_none() {
                        break;
                    }
                    list = self.messages_list(list.next_page_token).await?;
                    // self.log_message_subjects(&list).await?;
                }
            }
        }

        if log::max_level() >= log::Level::Info {
            self.log_message_subjects().await?;
        }

        Ok(())
    }

    async fn messages_list(
        &mut self,
        next_page_token: Option<String>,
    ) -> Result<ListMessagesResponse, Error> {
        let mut call = self
            .hub
            .users()
            .messages_list("me")
            .max_results(self.max_results);
        // Add any labels specified
        if !self.label_ids.is_empty() {
            log::debug!("Setting labels for list: {:#?}", self.label_ids);
            for id in self.label_ids.as_slice() {
                call = call.add_label_ids(id);
            }
        }
        // Add query
        if !self.query.is_empty() {
            log::debug!("Setting query string `{}`", self.query);
            call = call.q(&self.query);
        }
        // Add a page token if it exists
        if let Some(page_token) = next_page_token {
            log::debug!("Setting token for next page.");
            call = call.page_token(&page_token);
        }

        let (_response, list) = call.doit().await.map_err(Box::new)?;

        let mut list_ids: Vec<String> = list
            .clone()
            .messages
            .unwrap()
            .iter()
            .map(|item| item.id.clone().unwrap())
            .collect();
        self.message_ids.append(&mut list_ids);

        Ok(list)
    }

    async fn log_message_subjects(&self) -> Result<(), Error> {
        for id in &self.message_ids {
            log::trace!("{id}");
            let (_res, m) = self
                .hub
                .users()
                .messages_get("me", id)
                .add_scope("https://www.googleapis.com/auth/gmail.metadata")
                .format("metadata")
                .add_metadata_headers("subject")
                .doit()
                .await
                .map_err(Box::new)?;

            let mut subject = String::new();
            let Some(payload) = m.payload else { continue };
            let Some(headers) = payload.headers else {
                continue;
            };

            for header in headers {
                if header.name.is_some()
                    && header.name.unwrap() == "Subject"
                    && header.value.is_some()
                {
                    subject = header.value.unwrap();
                    break;
                } else {
                    continue;
                }
            }

            if subject.is_empty() {
                log::info!("***Email with no subject***");
            } else {
                log::info!("{subject:?}");
            }
        }

        Ok(())
    }
}
