use clap::{Parser, Subcommand};
use cull_gmail::{GmailClient, MessageList, Result, RuleProcessor};

#[derive(Debug, Subcommand)]
enum MessageAction {
    List,
    Trash,
    Delete,
}

/// Command line options for the list subcommand
#[derive(Debug, Parser)]
pub struct MessagesCli {
    /// Maximum results per page
    #[arg(short, long,display_order = 1, help_heading = "Config", default_value = cull_gmail::DEFAULT_MAX_RESULTS)]
    max_results: u32,
    /// Maximum number of pages (0=all)
    #[arg(
        short,
        long,
        display_order = 1,
        help_heading = "Config",
        default_value = "1"
    )]
    pages: u32,
    /// Labels to filter the message list
    #[arg(short, long, display_order = 1, help_heading = "Config")]
    labels: Vec<String>,
    /// Query string to select messages to list
    #[arg(short = 'Q', long, display_order = 1, help_heading = "Config")]
    query: Option<String>,
    /// Action: what to do with the message list
    #[command(subcommand)]
    action: MessageAction,
}

impl MessagesCli {
    pub(crate) async fn run(&self, client: &mut GmailClient) -> Result<()> {
        self.set_parameters(client)?;

        client.get_messages(self.pages).await?;

        match self.action {
            MessageAction::List => {
                if log::max_level() >= log::Level::Info {
                    client.log_messages().await
                } else {
                    Ok(())
                }
            }
            MessageAction::Trash => client.batch_trash().await,
            MessageAction::Delete => client.batch_delete().await,
        }

        // Ok(())
    }

    fn set_parameters(&self, client: &mut GmailClient) -> Result<()> {
        if !self.labels().is_empty() {
            client.add_labels(self.labels())?;
        }

        if let Some(query) = self.query().as_ref() {
            client.set_query(query)
        }

        log::trace!("Max results: `{}`", self.max_results());
        client.set_max_results(self.max_results());
        log::debug!("List max results set to {}", client.max_results());

        Ok(())
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
