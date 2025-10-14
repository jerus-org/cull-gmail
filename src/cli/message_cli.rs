use clap::Parser;
use cull_gmail::{GmailClient, MessageList, Result};

use crate::message_trait::Message;

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
        self.set_parameters(client)?;

        client.get_messages(self.pages).await
    }

    pub(crate) fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub(crate) fn query(&self) -> &Option<String> {
        &self.query
    }

    pub(crate) fn max_results(&self) -> u32 {
        self.max_results
    }
}
