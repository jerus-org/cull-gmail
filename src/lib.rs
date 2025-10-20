#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]
#![doc = include_str!("../docs/lib/lib.md")]

mod client_config;
mod eol_action;
mod error;
mod gmail_client;
mod message_list;
mod retention;
mod rule_processor;
mod rules;
#[cfg(test)]
pub(crate) mod test_utils;

pub(crate) mod utils;

pub use gmail_client::DEFAULT_MAX_RESULTS;

pub use client_config::ClientConfig;
pub use gmail_client::GmailClient;
pub(crate) use gmail_client::MessageSummary;
pub use retention::Retention;
pub use rules::Rules;

pub use eol_action::EolAction;
pub use error::Error;
pub use retention::MessageAge;

pub use message_list::MessageList;
pub use rule_processor::RuleProcessor;

/// Type alias for result with crate Error
pub type Result<O> = std::result::Result<O, Error>;
