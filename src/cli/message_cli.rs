use clap::Parser;
use cull_gmail::{GmailClient, MessageList, Result};

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
    pub(crate) async fn run(&self, client: &mut GmailClient) -> Result<()> {
        if !self.labels.is_empty() {
            client.add_labels(&self.labels).await?;
        }

        if let Some(query) = self.query.as_ref() {
            client.set_query(query)
        }

        log::trace!("Max results: `{}`", self.max_results);
        client.set_max_results(self.max_results);
        log::debug!("List max results set to {}", client.max_results());

        client.run(self.pages).await
    }
}
