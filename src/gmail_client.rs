//! # Gmail Client Module
//!
//! This module provides the core Gmail API client functionality for the cull-gmail application.
//! The `GmailClient` struct manages Gmail API connections, authentication, and message operations.
//!
//! ## Overview
//!
//! The Gmail client provides:
//!
//! - Authenticated Gmail API access using OAuth2 flows
//! - Label management and mapping functionality
//! - Message list operations with filtering support
//! - Configuration-based setup with credential management
//! - Integration with Gmail's REST API via the `google-gmail1` crate
//!
//! ## Authentication
//!
//! The client uses OAuth2 authentication with the "installed application" flow,
//! requiring client credentials (client ID and secret) to be configured. Tokens
//! are automatically managed and persisted to disk for reuse.
//!
//! ## Configuration
//!
//! The client is configured using [`ClientConfig`] which specifies:
//! - OAuth2 credentials (client ID, client secret)
//! - Token persistence location
//! - Configuration file paths
//!
//! ## Error Handling
//!
//! All operations return `Result<T, Error>` where [`Error`] encompasses:
//! - Gmail API errors (network, authentication, quota)
//! - Configuration and credential errors
//! - I/O errors from file operations
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```rust,no_run
//! use cull_gmail::{ClientConfig, GmailClient};
//!
//! # async fn example() -> cull_gmail::Result<()> {
//! // Create configuration with OAuth2 credentials
//! let config = ClientConfig::builder()
//!     .with_client_id("your-client-id.googleusercontent.com")
//!     .with_client_secret("your-client-secret")
//!     .build();
//!
//! // Initialize Gmail client with authentication
//! let client = GmailClient::new_with_config(config).await?;
//!
//! // Display available labels
//! client.show_label();
//!
//! // Get label ID for a specific label name
//! if let Some(inbox_id) = client.get_label_id("INBOX") {
//!     println!("Inbox ID: {}", inbox_id);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Label Operations
//!
//! ```rust,no_run
//! use cull_gmail::{ClientConfig, GmailClient};
//!
//! # async fn example() -> cull_gmail::Result<()> {
//! # let config = ClientConfig::builder().build();
//! let client = GmailClient::new_with_config(config).await?;
//!
//! // Check if a label exists
//! match client.get_label_id("Important") {
//!     Some(id) => println!("Important label ID: {}", id),
//!     None => println!("Important label not found"),
//! }
//!
//! // List all available labels (logged to console)
//! client.show_label();
//! # Ok(())
//! # }
//! ```
//!
//! ## Thread Safety
//!
//! The Gmail client contains async operations and internal state. While individual
//! operations are thread-safe, the client itself should not be shared across
//! threads without proper synchronization.
//!
//! ## Rate Limits
//!
//! The Gmail API has usage quotas and rate limits. The client does not implement
//! automatic retry logic, so applications should handle rate limit errors appropriately.
//!
//! [`ClientConfig`]: crate::ClientConfig
//! [`Error`]: crate::Error

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

/// Default maximum number of results to return per page from Gmail API calls.
///
/// This constant defines the default page size for Gmail API list operations.
/// The value "200" represents a balance between API efficiency and memory usage.
///
/// Gmail API supports up to 500 results per page, but 200 provides good performance
/// while keeping response sizes manageable.
pub const DEFAULT_MAX_RESULTS: &str = "200";

/// Gmail API client providing authenticated access to Gmail operations.
///
/// `GmailClient` manages the connection to Gmail's REST API, handles OAuth2 authentication,
/// maintains label mappings, and provides methods for message list operations.
///
/// The client contains internal state for:
/// - Authentication credentials and tokens
/// - Label name-to-ID mappings
/// - Query filters and pagination settings  
/// - Retrieved message summaries
/// - Rule processing configuration
///
/// # Examples
///
/// ```rust,no_run
/// use cull_gmail::{ClientConfig, GmailClient};
///
/// # async fn example() -> cull_gmail::Result<()> {
/// let config = ClientConfig::builder()
///     .with_client_id("client-id")
///     .with_client_secret("client-secret")
///     .build();
///     
/// let mut client = GmailClient::new_with_config(config).await?;
/// client.show_label();
/// # Ok(())
/// # }
/// ```
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
        f.debug_struct("GmailClient")
            .field("label_map", &self.label_map)
            .field("max_results", &self.max_results)
            .field("label_ids", &self.label_ids)
            .field("query", &self.query)
            .field("messages_count", &self.messages.len())
            .field("execute", &self.execute)
            .finish_non_exhaustive()
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

    /// Creates a new Gmail client with the provided configuration.
    ///
    /// This method initializes a Gmail API client with OAuth2 authentication using the
    /// "installed application" flow. It sets up the HTTPS connector, authenticates
    /// using the provided credentials, and fetches the label mapping from Gmail.
    ///
    /// # Arguments
    ///
    /// * `config` - Client configuration containing OAuth2 credentials and settings
    ///
    /// # Returns
    ///
    /// Returns a configured `GmailClient` ready for API operations, or an error if:
    /// - Authentication fails (invalid credentials, network issues)
    /// - Gmail API is unreachable
    /// - Label fetching fails
    ///
    /// # Errors
    ///
    /// This method can fail with:
    /// - [`Error::GoogleGmail1`] - Gmail API errors during authentication or label fetch
    /// - Network connectivity issues during OAuth2 flow
    /// - [`Error::NoLabelsFound`] - If no labels exist in the mailbox (unusual)
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cull_gmail::{ClientConfig, GmailClient};
    ///
    /// # async fn example() -> cull_gmail::Result<()> {
    /// let config = ClientConfig::builder()
    ///     .with_client_id("123456789-abc.googleusercontent.com")
    ///     .with_client_secret("your-client-secret")
    ///     .build();
    ///
    /// let client = GmailClient::new_with_config(config).await?;
    /// println!("Gmail client initialized successfully");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This method contains `.unwrap()` calls for:
    /// - HTTPS connector building (should not fail with valid TLS setup)
    /// - Default max results parsing (hardcoded valid string)
    /// - OAuth2 authenticator building (should not fail with valid config)
    ///
    /// [`Error::GoogleGmail1`]: crate::Error::GoogleGmail1
    /// [`Error::NoLabelsFound`]: crate::Error::NoLabelsFound
    pub async fn new_with_config(config: ClientConfig) -> Result<Self> {
        let executor = TokioExecutor::new();
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build();

        let client = Client::builder(executor.clone()).build(connector.clone());
        log::trace!("file to persist tokens to `{}`", config.persist_path());

        let auth = InstalledFlowAuthenticator::with_client(
            config.secret().clone(),
            InstalledFlowReturnMethod::HTTPRedirect,
            Client::builder(executor).build(connector),
        )
        .persist_tokens_to_disk(config.persist_path())
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

    /// Fetches the label mapping from Gmail API.
    ///
    /// This method retrieves all labels from the user's Gmail account and creates
    /// a mapping from label names to their corresponding label IDs.
    ///
    /// # Arguments
    ///
    /// * `hub` - The Gmail API hub instance for making API calls
    ///
    /// # Returns
    ///
    /// Returns a `BTreeMap` containing label name to ID mappings, or an error if
    /// the API call fails or no labels are found.
    ///
    /// # Errors
    ///
    /// - [`Error::GoogleGmail1`] - Gmail API request failure
    /// - [`Error::NoLabelsFound`] - No labels exist in the mailbox
    ///
    /// [`Error::GoogleGmail1`]: crate::Error::GoogleGmail1
    /// [`Error::NoLabelsFound`]: crate::Error::NoLabelsFound
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

    /// Retrieves the Gmail label ID for a given label name.
    ///
    /// This method looks up a label name in the internal label mapping and returns
    /// the corresponding Gmail label ID if found.
    ///
    /// # Arguments
    ///
    /// * `name` - The label name to look up (case-sensitive)
    ///
    /// # Returns
    ///
    /// Returns `Some(String)` containing the label ID if the label exists,
    /// or `None` if the label name is not found.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::{ClientConfig, GmailClient};
    /// # async fn example(client: &GmailClient) {
    /// // Look up standard Gmail labels
    /// if let Some(inbox_id) = client.get_label_id("INBOX") {
    ///     println!("Inbox ID: {}", inbox_id);
    /// }
    ///
    /// // Look up custom labels
    /// match client.get_label_id("Important") {
    ///     Some(id) => println!("Found label ID: {}", id),
    ///     None => println!("Label 'Important' not found"),
    /// }
    /// # }
    /// ```
    pub fn get_label_id(&self, name: &str) -> Option<String> {
        self.label_map.get(name).cloned()
    }

    /// Displays all available labels and their IDs to the log.
    ///
    /// This method iterates through the internal label mapping and outputs each
    /// label name and its corresponding ID using the `log::info!` macro.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use cull_gmail::{ClientConfig, GmailClient};
    /// # async fn example() -> cull_gmail::Result<()> {
    /// # let config = ClientConfig::builder().build();
    /// let client = GmailClient::new_with_config(config).await?;
    ///
    /// // Display all labels (output goes to log)
    /// client.show_label();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Output example:
    /// ```text
    /// INFO: INBOX: Label_1
    /// INFO: SENT: Label_2
    /// INFO: Important: Label_3
    /// ```
    pub fn show_label(&self) {
        for (name, id) in self.label_map.iter() {
            log::info!("{name}: {id}")
        }
    }

    /// Returns a clone of the Gmail API hub for direct API access.
    ///
    /// This method provides access to the underlying Gmail API client hub,
    /// allowing for direct API operations not covered by the higher-level
    /// methods in this struct.
    ///
    /// # Returns
    ///
    /// A cloned `Gmail` hub instance configured with the same authentication
    /// and connectors as this client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # fn example() { }
    /// ```
    pub(crate) fn hub(&self) -> Gmail<HttpsConnector<HttpConnector>> {
        self.hub.clone()
    }
}
