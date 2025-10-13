use std::fmt::Debug;

use crate::{GmailClient, Result};

use google_gmail1::{
    Gmail, api::ListMessagesResponse, hyper_rustls::HttpsConnector,
    hyper_util::client::legacy::connect::HttpConnector,
};

mod message_summary;

use message_summary::MessageSummary;

use crate::utils::Elide;

/// Default for the maximum number of results to return on a page
pub const DEFAULT_MAX_RESULTS: &str = "200";

/// Struct to capture configuration for List API call.
pub struct MessageList {
    hub: Gmail<HttpsConnector<HttpConnector>>,
    max_results: u32,
    label_ids: Vec<String>,
    messages: Vec<MessageSummary>,
    query: String,
}

impl Debug for MessageList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageList")
            .field("max_results", &self.max_results)
            .field("label_ids", &self.label_ids)
            .field("messages", &self.messages)
            .field("query", &self.query)
            .finish()
    }
}

impl MessageList {
    /// Create a new List struct and add the Gmail api connection.
    pub async fn new(client: &GmailClient) -> Result<Self> {
        Ok(MessageList {
            hub: client.hub(),
            max_results: DEFAULT_MAX_RESULTS.parse::<u32>().unwrap(),
            label_ids: Vec::new(),
            messages: Vec::new(),
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
    pub async fn add_labels(&mut self, client: GmailClient, labels: &[String]) -> Result<()> {
        log::debug!("labels from command line: {labels:?}");
        let mut label_ids = Vec::new();
        for label in labels {
            if let Some(id) = client.get_label_id(label) {
                label_ids.push(id)
            }
        }
        self.add_labels_ids(label_ids.as_slice());
        Ok(())
    }

    /// Add label to the labels collection
    fn add_labels_ids(&mut self, label_ids: &[String]) {
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

    /// Get the summary of the messages
    pub(crate) fn messages(&self) -> &Vec<MessageSummary> {
        &self.messages
    }

    /// Get a reference to the message_ids
    pub fn message_ids(&self) -> Vec<String> {
        self.messages
            .iter()
            .map(|m| m.id().to_string())
            .collect::<Vec<_>>()
            .clone()
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
    pub async fn run(&mut self, pages: u32) -> Result<()> {
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
    ) -> Result<ListMessagesResponse> {
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
        log::trace!(
            "Estimated {} messages.",
            list.result_size_estimate.unwrap_or(0)
        );

        if list.result_size_estimate.unwrap_or(0) == 0 {
            log::warn!("Search returned no messages.");
            return Ok(list);
        }

        let mut list_ids = list
            .clone()
            .messages
            .unwrap()
            .iter()
            .flat_map(|item| item.id.as_ref().map(|id| MessageSummary::new(id)))
            .collect();
        self.messages.append(&mut list_ids);

        Ok(list)
    }

    async fn log_message_subjects(&mut self) -> Result<()> {
        for message in &mut self.messages {
            log::trace!("{}", message.id());
            let (_res, m) = self
                .hub
                .users()
                .messages_get("me", message.id())
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
                subject.elide(24);
                message.set_subject(&subject);
                log::info!("{subject:?}");
            }
        }

        Ok(())
    }
}
