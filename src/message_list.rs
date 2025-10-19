//! # Message List Module
//!
//! This module provides the `MessageList` trait for interacting with Gmail message lists.
//! The trait defines methods for retrieving, filtering, and managing collections of Gmail messages.
//!
//! ## Overview
//!
//! The `MessageList` trait provides:
//!
//! - Message list retrieval with pagination support
//! - Label and query-based filtering
//! - Message metadata fetching and logging
//! - Configuration of result limits and query parameters
//!
//! ## Error Handling
//!
//! All asynchronous methods return `Result<T>` where errors may include:
//! - Gmail API communication errors
//! - Authentication failures
//! - Network connectivity issues
//! - Invalid query parameters
//!
//! ## Threading
//!
//! All async methods in this trait are `Send` compatible, allowing them to be used
//! across thread boundaries in concurrent contexts.
//!
//! ## Example
//!
//! ```rust,no_run
//! use cull_gmail::{GmailClient, MessageList, ClientConfig};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with proper configuration (credentials required)
//!     let config = ClientConfig::builder()
//!         .with_client_id("your-client-id")
//!         .with_client_secret("your-client-secret")
//!         .build();
//!     let mut client = GmailClient::new_with_config(config).await?;
//!     
//!     // Configure search parameters
//!     client.set_query("is:unread");
//!     client.set_max_results(50);
//!     
//!     // Retrieve messages from Gmail
//!     client.get_messages(1).await?;
//!     
//!     // Access retrieved message summaries
//!     let messages = client.messages();
//!     println!("Found {} messages", messages.len());
//!     
//!     Ok(())
//! }
//! ```

use crate::{GmailClient, MessageSummary, Result};

use google_gmail1::{
    Gmail, api::ListMessagesResponse, hyper_rustls::HttpsConnector,
    hyper_util::client::legacy::connect::HttpConnector,
};

/// Methods to select lists of messages from the mailbox
pub trait MessageList {
    /// Log the initial characters of the subjects of the message in the list
    fn log_messages(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
    /// List of messages
    fn list_messages(
        &mut self,
        next_page_token: Option<String>,
    ) -> impl std::future::Future<Output = Result<ListMessagesResponse>> + Send;
    /// Run something
    fn get_messages(&mut self, pages: u32) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Return the gmail hub
    fn hub(&self) -> Gmail<HttpsConnector<HttpConnector>>;
    /// Return the list of label_ids
    fn label_ids(&self) -> Vec<String>;
    /// Return the list of message ids
    fn message_ids(&self) -> Vec<String>;
    /// Return a summary of the messages (id and summary)
    fn messages(&self) -> &Vec<MessageSummary>;
    /// Set the query for the message list
    fn set_query(&mut self, query: &str);
    /// Add label ids to the list of labels for the message list
    fn add_labels_ids(&mut self, label_ids: &[String]);
    /// Add labels to the list of labels for the message list
    fn add_labels(&mut self, labels: &[String]) -> Result<()>;
    /// Report the max results value set
    fn max_results(&self) -> u32;
    /// Set the max_results value
    fn set_max_results(&mut self, value: u32);
}

impl MessageList for GmailClient {
    /// Set the maximum results
    fn set_max_results(&mut self, value: u32) {
        self.max_results = value;
    }

    /// Report the maximum results value
    fn max_results(&self) -> u32 {
        self.max_results
    }

    /// Add label to the labels collection
    fn add_labels(&mut self, labels: &[String]) -> Result<()> {
        log::debug!("labels from command line: {labels:?}");
        let mut label_ids = Vec::new();
        for label in labels {
            if let Some(id) = self.get_label_id(label) {
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
    fn set_query(&mut self, query: &str) {
        self.query = query.to_string()
    }

    /// Get the summary of the messages
    fn messages(&self) -> &Vec<MessageSummary> {
        &self.messages
    }

    /// Get a reference to the message_ids
    fn message_ids(&self) -> Vec<String> {
        self.messages
            .iter()
            .map(|m| m.id().to_string())
            .collect::<Vec<_>>()
            .clone()
    }

    /// Get a reference to the message_ids
    fn label_ids(&self) -> Vec<String> {
        self.label_ids.clone()
    }

    /// Get the hub
    fn hub(&self) -> Gmail<HttpsConnector<HttpConnector>> {
        self.hub().clone()
    }

    /// Run the Gmail api as configured
    async fn get_messages(&mut self, pages: u32) -> Result<()> {
        let list = self.list_messages(None).await?;
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
                    list = self.list_messages(list.next_page_token).await?;
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
                    list = self.list_messages(list.next_page_token).await?;
                    // self.log_message_subjects(&list).await?;
                }
            }
        }

        Ok(())
    }

    async fn list_messages(
        &mut self,
        next_page_token: Option<String>,
    ) -> Result<ListMessagesResponse> {
        let hub = self.hub();
        let mut call = hub
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
        // Add a page token
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

    async fn log_messages(&mut self) -> Result<()> {
        let hub = self.hub();
        for message in &mut self.messages {
            log::trace!("{}", message.id());
            let (_res, m) = hub
                .users()
                .messages_get("me", message.id())
                .add_scope("https://mail.google.com/")
                .format("metadata")
                .add_metadata_headers("subject")
                .add_metadata_headers("date")
                .doit()
                .await
                .map_err(Box::new)?;

            let Some(payload) = m.payload else { continue };
            let Some(headers) = payload.headers else {
                continue;
            };

            for header in headers {
                if let Some(name) = header.name {
                    match name.to_lowercase().as_str() {
                        "subject" => message.set_subject(header.value),
                        "date" => message.set_date(header.value),
                        _ => {}
                    }
                } else {
                    continue;
                }
            }

            log::info!("{}", message.list_date_and_subject());
        }

        Ok(())
    }
}
