use google_gmail1::api::BatchDeleteMessagesRequest;

use crate::{MessageList, Result};

/// Struct for trashing messages
#[derive(Debug)]
pub struct Delete {
    message_list: MessageList,
}

impl Delete {
    /// Create a new Delete struct
    pub async fn new(credential: &str) -> Result<Self> {
        let message_list = MessageList::new(credential).await?;
        Ok(Delete { message_list })
    }

    /// Set the maximum results
    pub fn set_max_results(&mut self, value: u32) {
        self.message_list.set_max_results(value);
    }

    /// Report the maximum results value
    pub fn max_results(&self) -> u32 {
        self.message_list.max_results()
    }

    /// Add label to the labels collection
    pub async fn add_labels(&mut self, credential: &str, labels: &[String]) -> Result<()> {
        self.message_list.add_labels(credential, labels).await
    }

    /// Set the query string
    pub fn set_query(&mut self, query: &str) {
        self.message_list.set_query(query)
    }

    /// Run the trash cli
    pub async fn prepare(&mut self, pages: u32) -> Result<()> {
        self.message_list.run(pages).await
    }

    /// Run the batch delete on the selected messages
    pub async fn batch_delete(&self) -> Result<()> {
        let ids = Some(self.message_list.message_ids());

        let batch_request = BatchDeleteMessagesRequest { ids };

        log::trace!("{batch_request:#?}");

        let _res = self
            .message_list
            .hub()
            .users()
            .messages_batch_delete(batch_request, "me")
            .add_scope("https://mail.google.com/")
            .doit()
            .await
            .map_err(Box::new)?;

        for m in self.message_list.messages() {
            log::info!("Message with subject `{}` deleted.", m.subject());
        }

        Ok(())
    }
}
