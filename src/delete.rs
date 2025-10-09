use google_gmail1::api::BatchDeleteMessagesRequest;

use crate::{MessageList, Result};

/// Struct for deleting messages
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

    /// return the message list struct
    pub fn message_list(&mut self) -> &mut MessageList {
        &mut self.message_list
    }

    /// Prepare the message list for delete to be completed on execute by batch_delete
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
