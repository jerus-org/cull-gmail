use cull_gmail::{GmailClient, MessageList, Result};

use crate::{delete_cli::DeleteCli, message_cli::MessageCli};

pub trait Message {
    fn set_parameters(&self, client: &mut GmailClient) -> Result<()>;
}

impl Message for MessageCli {
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
}

impl Message for DeleteCli {
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
}
