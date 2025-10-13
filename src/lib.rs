#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]
#![doc = include_str!("../docs/lib.md")]

mod config;
mod credential;
mod delete;
mod eol_action;
mod error;
mod gmail_client;
mod message_list;
mod processor;
mod retention;
mod trash;

pub(crate) mod utils;

pub use config::Config;
pub use credential::Credential;
pub use eol_action::EolAction;
pub use error::Error;
pub use gmail_client::DEFAULT_MAX_RESULTS;
pub use gmail_client::GmailClient;
pub(crate) use gmail_client::MessageSummary;
pub use processor::Processor;
pub use retention::MessageAge;
pub use retention::Retention;
pub use trash::Trash;

/// Type alias for result with crate Error
pub type Result<O> = std::result::Result<O, Error>;
