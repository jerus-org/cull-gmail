use crate::{GmailClient, MessageSummary, Result};

use google_gmail1::{
    Gmail, api::ListMessagesResponse, hyper_rustls::HttpsConnector,
    hyper_util::client::legacy::connect::HttpConnector,
};

use crate::utils::Elide;

/// Methods to select lists of messages from the mailbox
pub trait MessageList {
    /// Log the initial characters of the subjects of the message in the list
    fn log_message_subjects(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
    /// List of messages
    fn messages_list(
        &mut self,
        next_page_token: Option<String>,
    ) -> impl std::future::Future<Output = Result<ListMessagesResponse>> + Send;
    /// Run something
    fn run(&mut self, pages: u32) -> impl std::future::Future<Output = Result<()>> + Send;
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
    fn add_labels(
        &mut self,
        labels: &[String],
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Report the max results value set
    fn max_results(&self) -> u32;
    /// Set the max_results value
    fn set_max_results(&mut self, value: u32);
}

impl MessageList for GmailClient {
    // /// Create a new List struct and add the Gmail api connection.
    // async fn new(client: &GmailClient) -> Result<Self> {
    //     Ok(MessageList {
    //         hub: client.hub(),
    //         max_results: DEFAULT_MAX_RESULTS.parse::<u32>().unwrap(),
    //         label_ids: Vec::new(),
    //         messages: Vec::new(),
    //         query: String::new(),
    //     })
    // }

    /// Set the maximum results
    fn set_max_results(&mut self, value: u32) {
        self.max_results = value;
    }

    /// Report the maximum results value
    fn max_results(&self) -> u32 {
        self.max_results
    }

    /// Add label to the labels collection
    async fn add_labels(&mut self, labels: &[String]) -> Result<()> {
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
    async fn run(&mut self, pages: u32) -> Result<()> {
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
        let hub = self.hub();
        for message in &mut self.messages {
            log::trace!("{}", message.id());
            let (_res, m) = hub
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
