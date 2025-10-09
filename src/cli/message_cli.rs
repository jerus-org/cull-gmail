use clap::Parser;
use cull_gmail::{MessageList, Result};

/// Command line options for the list subcommand
#[derive(Debug, Parser)]
pub struct MessageCli {
    /// Maximum results per page
    #[arg(short, long, default_value = cull_gmail::DEFAULT_MAX_RESULTS)]
    max_results: u32,
    /// Maximum number of pages (0=all)
    #[arg(short, long, default_value = "1")]
    pages: u32,
    /// Labels to filter the message list
    #[arg(short, long)]
    labels: Vec<String>,
    /// Query string to select messages to list
    #[arg(short = 'Q', long)]
    query: Option<String>,
}

impl MessageCli {
    pub(crate) async fn run(&self, credential_file: &str) -> Result<()> {
        let mut list = MessageList::new(credential_file).await?;

        if !self.labels.is_empty() {
            list.add_labels(credential_file, &self.labels).await?;
        }

        if let Some(query) = self.query.as_ref() {
            list.set_query(query)
        }

        log::trace!("Max results: `{}`", self.max_results);
        list.set_max_results(self.max_results);
        log::debug!("List max results set to {}", list.max_results());

        list.run(self.pages).await
    }
}
