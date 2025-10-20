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

#![warn(missing_docs)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use crate::{GmailClient, MessageSummary, Result};

use google_gmail1::{
    Gmail,
    api::{ListMessagesResponse, Message as GmailMessage},
    hyper_rustls::HttpsConnector,
    hyper_util::client::legacy::connect::HttpConnector,
};

/// A trait for interacting with Gmail message lists, providing methods for
/// retrieving, filtering, and managing collections of Gmail messages.
///
/// This trait abstracts the core operations needed to work with Gmail message lists,
/// including pagination, filtering by labels and queries, and configuring result limits.
///
/// # Examples
///
/// ```rust,no_run
/// use cull_gmail::{MessageList, GmailClient, ClientConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ClientConfig::builder().build();
/// let mut client = GmailClient::new_with_config(config).await?;
///
/// // Set search parameters
/// client.set_query("is:unread");
/// client.set_max_results(100);
///
/// // Retrieve first page of messages
/// client.get_messages(1).await?;
/// # Ok(())
/// # }
/// ```
pub trait MessageList {
    /// Fetches detailed metadata for stored messages and logs their subjects and dates.
    ///
    /// This method retrieves the subject line and date for each message currently
    /// stored in the message list and outputs them to the log.
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` on success, or an error if the Gmail API request fails.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The Gmail API is unreachable
    /// - Authentication credentials are invalid or expired
    /// - Network connectivity issues occur
    /// - Individual message retrieval fails
    fn log_messages(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Retrieves a list of messages from Gmail based on current filter settings.
    ///
    /// This method calls the Gmail API to get a page of messages matching the
    /// configured query and label filters. Retrieved message IDs are stored
    /// internally for further operations.
    ///
    /// # Arguments
    ///
    /// * `next_page_token` - Optional token for pagination. Use `None` for the first page,
    ///   or the token from a previous response to get subsequent pages.
    ///
    /// # Returns
    ///
    /// Returns the raw `ListMessagesResponse` from the Gmail API, which contains
    /// message metadata and pagination tokens.
    ///
    /// # Errors
    ///
    /// This method can fail if:
    /// - The Gmail API request fails
    /// - Authentication is invalid
    /// - The query syntax is malformed
    /// - Network issues prevent the API call
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::{MessageList, GmailClient, ClientConfig};
    /// # async fn example(mut client: impl MessageList) -> cull_gmail::Result<()> {
    /// // Get the first page of results
    /// let response = client.list_messages(None).await?;
    ///
    /// // Get the next page if available
    /// if let Some(token) = response.next_page_token {
    ///     let next_page = client.list_messages(Some(token)).await?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn list_messages(
        &mut self,
        next_page_token: Option<String>,
    ) -> impl std::future::Future<Output = Result<ListMessagesResponse>> + Send;

    /// Retrieves multiple pages of messages based on the specified page limit.
    ///
    /// This method handles pagination automatically, fetching the specified number
    /// of pages or all available pages if `pages` is 0.
    ///
    /// # Arguments
    ///
    /// * `pages` - Number of pages to retrieve:
    ///   - `0`: Fetch all available pages
    ///   - `1`: Fetch only the first page
    ///   - `n > 1`: Fetch exactly `n` pages or until no more pages are available
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` on success. All retrieved messages are stored internally
    /// and can be accessed via `messages()`.
    ///
    /// # Errors
    ///
    /// This method can fail if any individual page request fails. See `list_messages`
    /// for specific error conditions.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::{MessageList, GmailClient, ClientConfig};
    /// # async fn example(mut client: impl MessageList) -> cull_gmail::Result<()> {
    /// // Get all available pages
    /// client.get_messages(0).await?;
    ///
    /// // Get exactly 3 pages
    /// client.get_messages(3).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn get_messages(&mut self, pages: u32) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Returns a reference to the Gmail API hub for direct API access.
    ///
    /// This method provides access to the underlying Gmail API client for
    /// advanced operations not covered by this trait.
    ///
    /// # Returns
    ///
    /// A cloned `Gmail` hub instance configured with the appropriate connectors.
    fn hub(&self) -> Gmail<HttpsConnector<HttpConnector>>;

    /// Returns the list of label IDs currently configured for message filtering.
    ///
    /// # Returns
    ///
    /// A vector of Gmail label ID strings. These IDs are used to filter
    /// messages during API calls.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::MessageList;
    /// # fn example(client: impl MessageList) {
    /// let labels = client.label_ids();
    /// println!("Filtering by {} labels", labels.len());
    /// # }
    /// ```
    fn label_ids(&self) -> Vec<String>;

    /// Returns a list of message IDs for all currently stored messages.
    ///
    /// # Returns
    ///
    /// A vector of Gmail message ID strings. These IDs can be used for
    /// further Gmail API operations on specific messages.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::MessageList;
    /// # fn example(client: impl MessageList) {
    /// let message_ids = client.message_ids();
    /// println!("Found {} messages", message_ids.len());
    /// # }
    /// ```
    fn message_ids(&self) -> Vec<String>;

    /// Returns a reference to the collection of message summaries.
    ///
    /// This method provides access to all message summaries currently stored,
    /// including any metadata that has been fetched.
    ///
    /// # Returns
    ///
    /// A reference to a vector of `MessageSummary` objects containing
    /// message IDs and any retrieved metadata.
    fn messages(&self) -> &Vec<MessageSummary>;

    /// Sets the search query string for filtering messages.
    ///
    /// This method configures the Gmail search query that will be used in
    /// subsequent API calls. The query uses Gmail's search syntax.
    ///
    /// # Arguments
    ///
    /// * `query` - A Gmail search query string (e.g., "is:unread", "from:example@gmail.com")
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::MessageList;
    /// # fn example(mut client: impl MessageList) {
    /// client.set_query("is:unread older_than:30d");
    /// client.set_query("from:noreply@example.com");
    /// # }
    /// ```
    fn set_query(&mut self, query: &str);

    /// Adds Gmail label IDs to the current filter list.
    ///
    /// This method appends the provided label IDs to the existing list of
    /// labels used for filtering messages. Messages must match ALL specified labels.
    ///
    /// # Arguments
    ///
    /// * `label_ids` - A slice of Gmail label ID strings to add to the filter
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::MessageList;
    /// # fn example(mut client: impl MessageList) {
    /// let label_ids = vec!["Label_1".to_string(), "Label_2".to_string()];
    /// client.add_labels_ids(&label_ids);
    /// # }
    /// ```
    fn add_labels_ids(&mut self, label_ids: &[String]);

    /// Adds Gmail labels by name to the current filter list.
    ///
    /// This method resolves label names to their corresponding IDs and adds them
    /// to the filter list. This is more convenient than using `add_labels_ids`
    /// when you know the label names but not their IDs.
    ///
    /// # Arguments
    ///
    /// * `labels` - A slice of Gmail label name strings (e.g., "INBOX", "SPAM")
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` on success, or an error if label name resolution fails.
    ///
    /// # Errors
    ///
    /// This method can fail if label name to ID resolution is not available
    /// or if the underlying label ID mapping is not accessible.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::MessageList;
    /// # async fn example(mut client: impl MessageList) -> cull_gmail::Result<()> {
    /// let labels = vec!["INBOX".to_string(), "IMPORTANT".to_string()];
    /// client.add_labels(&labels)?;
    /// # Ok(())
    /// # }
    /// ```
    fn add_labels(&mut self, labels: &[String]) -> Result<()>;

    /// Returns the current maximum results limit per API request.
    ///
    /// # Returns
    ///
    /// The maximum number of messages to retrieve in a single API call.
    /// Default is typically 200.
    fn max_results(&self) -> u32;

    /// Sets the maximum number of results to return per API request.
    ///
    /// This controls how many messages are retrieved in each page when calling
    /// the Gmail API. Larger values reduce the number of API calls needed but
    /// increase memory usage and response time.
    ///
    /// # Arguments
    ///
    /// * `value` - Maximum results per page (typically 1-500, Gmail API limits apply)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::MessageList;
    /// # fn example(mut client: impl MessageList) {
    /// client.set_max_results(100);  // Retrieve 100 messages per page
    /// client.set_max_results(500);  // Retrieve 500 messages per page (maximum)
    /// # }
    /// ```
    fn set_max_results(&mut self, value: u32);
}

/// Abstraction for Gmail API calls used by MessageList.
pub(crate) trait GmailService {
    /// Fetch a page of messages using current filters.
    async fn list_messages_page(
        &self,
        label_ids: &[String],
        query: &str,
        max_results: u32,
        page_token: Option<String>,
    ) -> Result<ListMessagesResponse>;

    /// Fetch minimal metadata for a message (subject, date, etc.).
    async fn get_message_metadata(&self, message_id: &str) -> Result<GmailMessage>;
}

impl GmailClient {
    /// Append any message IDs from a ListMessagesResponse into the provided messages vector.
    fn append_list_to_messages(out: &mut Vec<MessageSummary>, list: &ListMessagesResponse) {
        if let Some(msgs) = &list.messages {
            let mut list_ids: Vec<MessageSummary> = msgs
                .iter()
                .flat_map(|item| item.id.as_deref().map(MessageSummary::new))
                .collect();
            out.append(&mut list_ids);
        }
    }
}

impl GmailService for GmailClient {
    async fn list_messages_page(
        &self,
        label_ids: &[String],
        query: &str,
        max_results: u32,
        page_token: Option<String>,
    ) -> Result<ListMessagesResponse> {
        let hub = self.hub();
        let mut call = hub.users().messages_list("me").max_results(max_results);
        if !label_ids.is_empty() {
            for id in label_ids {
                call = call.add_label_ids(id);
            }
        }
        if !query.is_empty() {
            call = call.q(query);
        }
        if let Some(token) = page_token {
            call = call.page_token(&token);
        }
        let (_response, list) = call.doit().await.map_err(Box::new)?;
        Ok(list)
    }

    async fn get_message_metadata(&self, message_id: &str) -> Result<GmailMessage> {
        let hub = self.hub();
        let (_res, m) = hub
            .users()
            .messages_get("me", message_id)
            .add_scope("https://mail.google.com/")
            .format("metadata")
            .add_metadata_headers("subject")
            .add_metadata_headers("date")
            .doit()
            .await
            .map_err(Box::new)?;
        Ok(m)
    }
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
            self.label_ids.extend(label_ids.iter().cloned());
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
        self.messages.iter().map(|m| m.id().to_string()).collect()
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
        if !self.label_ids.is_empty() {
            log::debug!("Setting labels for list: {:#?}", self.label_ids);
        }
        if !self.query.is_empty() {
            log::debug!("Setting query string `{}`", self.query);
        }
        if next_page_token.is_some() {
            log::debug!("Setting token for next page.");
        }

        let list = self
            .list_messages_page(
                &self.label_ids,
                &self.query,
                self.max_results,
                next_page_token,
            )
            .await?;
        log::trace!(
            "Estimated {} messages.",
            list.result_size_estimate.unwrap_or(0)
        );

        if list.result_size_estimate.unwrap_or(0) == 0 {
            log::warn!("Search returned no messages.");
            return Ok(list);
        }

        Self::append_list_to_messages(&mut self.messages, &list);

        Ok(list)
    }

    async fn log_messages(&mut self) -> Result<()> {
        for i in 0..self.messages.len() {
            let id = self.messages[i].id().to_string();
            log::trace!("{id}");
            let m = self.get_message_metadata(&id).await?;
            let message = &mut self.messages[i];

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

#[cfg(test)]
mod tests {
    use super::*;

    struct MockList {
        label_ids: Vec<String>,
        query: String,
        max_results: u32,
        messages: Vec<MessageSummary>,
    }

    impl MockList {
        fn new() -> Self {
            Self {
                label_ids: vec![],
                query: String::new(),
                max_results: 200,
                messages: vec![],
            }
        }

        fn push_msg(&mut self, id: &str) {
            self.messages.push(MessageSummary::new(id));
        }
    }

    impl MessageList for MockList {
        async fn log_messages(&mut self) -> Result<()> {
            Ok(())
        }
        async fn list_messages(
            &mut self,
            _next_page_token: Option<String>,
        ) -> Result<ListMessagesResponse> {
            Ok(ListMessagesResponse::default())
        }
        async fn get_messages(&mut self, _pages: u32) -> Result<()> {
            Ok(())
        }
        fn hub(&self) -> Gmail<HttpsConnector<HttpConnector>> {
            panic!("not used in tests")
        }
        fn label_ids(&self) -> Vec<String> {
            self.label_ids.clone()
        }
        fn message_ids(&self) -> Vec<String> {
            self.messages.iter().map(|m| m.id().to_string()).collect()
        }
        fn messages(&self) -> &Vec<MessageSummary> {
            &self.messages
        }
        fn set_query(&mut self, query: &str) {
            self.query = query.to_string();
        }
        fn add_labels_ids(&mut self, label_ids: &[String]) {
            self.label_ids.extend_from_slice(label_ids);
        }
        fn add_labels(&mut self, _labels: &[String]) -> Result<()> {
            Ok(())
        }
        fn max_results(&self) -> u32 {
            self.max_results
        }
        fn set_max_results(&mut self, value: u32) {
            self.max_results = value;
        }
    }

    use std::collections::HashMap;
    use std::sync::Mutex;

    struct TestClient {
        label_ids: Vec<String>,
        query: String,
        max_results: u32,
        messages: Vec<MessageSummary>,
        pages: Mutex<HashMap<Option<String>, ListMessagesResponse>>,
    }

    impl TestClient {
        fn with_pages(map: HashMap<Option<String>, ListMessagesResponse>) -> Self {
            Self {
                label_ids: vec![],
                query: String::new(),
                max_results: 200,
                messages: vec![],
                pages: Mutex::new(map),
            }
        }
    }

    impl super::GmailService for TestClient {
        async fn list_messages_page(
            &self,
            _label_ids: &[String],
            _query: &str,
            _max_results: u32,
            page_token: Option<String>,
        ) -> Result<ListMessagesResponse> {
            let map = self.pages.lock().unwrap();
            Ok(map
                .get(&page_token)
                .cloned()
                .unwrap_or_else(ListMessagesResponse::default))
        }

        async fn get_message_metadata(&self, _message_id: &str) -> Result<GmailMessage> {
            Ok(GmailMessage::default())
        }
    }

    impl MessageList for TestClient {
        fn set_max_results(&mut self, value: u32) {
            self.max_results = value;
        }
        fn max_results(&self) -> u32 {
            self.max_results
        }
        fn add_labels(&mut self, _labels: &[String]) -> Result<()> {
            Ok(())
        }
        fn add_labels_ids(&mut self, label_ids: &[String]) {
            self.label_ids.extend_from_slice(label_ids);
        }
        fn set_query(&mut self, query: &str) {
            self.query = query.to_string();
        }
        fn messages(&self) -> &Vec<MessageSummary> {
            &self.messages
        }
        fn message_ids(&self) -> Vec<String> {
            self.messages.iter().map(|m| m.id().to_string()).collect()
        }
        fn label_ids(&self) -> Vec<String> {
            self.label_ids.clone()
        }
        fn hub(&self) -> Gmail<HttpsConnector<HttpConnector>> {
            unimplemented!("not used in tests")
        }
        async fn get_messages(&mut self, pages: u32) -> Result<()> {
            let mut list = self.list_messages(None).await?;
            match pages {
                1 => {}
                0 => loop {
                    if list.next_page_token.is_none() {
                        break;
                    }
                    list = self.list_messages(list.next_page_token).await?;
                },
                _ => {
                    for _page in 2..=pages {
                        if list.next_page_token.is_none() {
                            break;
                        }
                        list = self.list_messages(list.next_page_token).await?;
                    }
                }
            }
            Ok(())
        }
        async fn list_messages(
            &mut self,
            next_page_token: Option<String>,
        ) -> Result<ListMessagesResponse> {
            let list = self
                .list_messages_page(
                    &self.label_ids,
                    &self.query,
                    self.max_results,
                    next_page_token,
                )
                .await?;

            if list.result_size_estimate.unwrap_or(0) == 0 {
                return Ok(list);
            }

            if let Some(msgs) = &list.messages {
                let mut list_ids: Vec<MessageSummary> = msgs
                    .iter()
                    .flat_map(|item| item.id.as_deref().map(MessageSummary::new))
                    .collect();
                self.messages.append(&mut list_ids);
            }

            Ok(list)
        }
        async fn log_messages(&mut self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn set_query_updates_state() {
        let mut ml = MockList::new();
        ml.set_query("from:noreply@example.com");
        // not directly accessible; rely on behavior by calling again
        ml.set_query("is:unread");
        assert_eq!(ml.query, "is:unread");
    }

    #[test]
    fn add_label_ids_accumulates() {
        let mut ml = MockList::new();
        ml.add_labels_ids(&["Label_1".into()]);
        ml.add_labels_ids(&["Label_2".into(), "Label_3".into()]);
        assert_eq!(ml.label_ids, vec!["Label_1", "Label_2", "Label_3"]);
    }

    #[test]
    fn max_results_get_set() {
        let mut ml = MockList::new();
        assert_eq!(ml.max_results(), 200);
        ml.set_max_results(123);
        assert_eq!(ml.max_results(), 123);
    }

    #[test]
    fn message_ids_maps_from_messages() {
        let mut ml = MockList::new();
        ml.push_msg("abc");
        ml.push_msg("def");
        assert_eq!(ml.message_ids(), vec!["abc", "def"]);
        assert_eq!(ml.messages().len(), 2);
    }

    #[test]
    fn append_list_to_messages_extracts_ids() {
        use google_gmail1::api::Message;
        let mut out = Vec::<MessageSummary>::new();
        let list = ListMessagesResponse {
            messages: Some(vec![
                Message {
                    id: Some("m1".into()),
                    ..Default::default()
                },
                Message {
                    id: None,
                    ..Default::default()
                },
                Message {
                    id: Some("m2".into()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        GmailClient::append_list_to_messages(&mut out, &list);
        let ids: Vec<_> = out.iter().map(|m| m.id().to_string()).collect();
        assert_eq!(ids, vec!["m1", "m2"]);
    }

    #[tokio::test]
    async fn list_messages_across_pages_collects_ids() {
        use google_gmail1::api::Message;
        let page1 = ListMessagesResponse {
            messages: Some(vec![
                Message {
                    id: Some("a".into()),
                    ..Default::default()
                },
                Message {
                    id: Some("b".into()),
                    ..Default::default()
                },
            ]),
            next_page_token: Some("t2".into()),
            result_size_estimate: Some(2),
        };
        let page2 = ListMessagesResponse {
            messages: Some(vec![Message {
                id: Some("c".into()),
                ..Default::default()
            }]),
            next_page_token: None,
            result_size_estimate: Some(1),
        };
        let mut map = HashMap::new();
        map.insert(None, page1);
        map.insert(Some("t2".into()), page2);

        let mut client = TestClient::with_pages(map);
        client.set_max_results(2);
        client.set_query("in:inbox");

        client.get_messages(0).await.unwrap();
        assert_eq!(client.message_ids(), vec!["a", "b", "c"]);
    }
}
