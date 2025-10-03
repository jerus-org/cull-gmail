#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(rustdoc_missing_doc_code_examples))]
#![cfg_attr(docsrs, warn(rustdoc::invalid_codeblock_attributes))]
#![doc = include_str!("../docs/lib.md")]

mod credential;
mod error;
mod labels;
mod list;
pub(crate) mod utils;

pub use credential::Credential;
pub use error::Error;
pub use labels::Labels;
pub use list::DEFAULT_MAX_RESULTS;
pub use list::List;
