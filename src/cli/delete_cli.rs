use clap::Parser;
use cull_gmail::{Delete, GmailClient, Result};

/// Command line options for the list subcommand
#[derive(Debug, Parser)]
pub struct DeleteCli {
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
    /// Execute the delete command
    #[arg(short, long)]
    execute: bool,
}

impl DeleteCli {
    pub(crate) async fn run(&self, client: &GmailClient) -> Result<()> {
        let mut messages_to_delete = Delete::new(client).await?;

        if !self.labels.is_empty() {
            // add labels if any specified
            messages_to_delete
                .message_list()
                .add_labels(client, &self.labels)
                .await?;
        }

        if let Some(query) = self.query.as_ref() {
            messages_to_delete.message_list().set_query(query)
        }

        log::trace!("Max results: `{}`", self.max_results);
        messages_to_delete
            .message_list()
            .set_max_results(self.max_results);
        log::debug!(
            "List max results set to {}",
            messages_to_delete.message_list().max_results()
        );

        messages_to_delete.prepare(self.pages).await?;

        if self.execute {
            messages_to_delete.batch_delete().await?;
            log::info!("Messages deleted.");
        } else {
            log::info!("Messages not deleted.");
        }
        Ok(())
    }
}
