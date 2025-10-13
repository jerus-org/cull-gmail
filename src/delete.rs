use google_gmail1::api::BatchDeleteMessagesRequest;

use crate::{GmailClient, Result, message_list::MessageList};

// /// Struct for deleting messages
// #[derive(Debug)]
// pub struct Delete {
//     message_list: MessageList,
// }

pub(crate) trait Delete {
    async fn batch_delete(&self) -> Result<()>;
    async fn prepare(&mut self, pages: u32) -> Result<()>;
}

impl Delete for GmailClient {
    /// Prepare the message list for delete to be completed on execute by batch_delete
    async fn prepare(&mut self, pages: u32) -> Result<()> {
        self.run(pages).await
    }

    /// Run the batch delete on the selected messages
    async fn batch_delete(&self) -> Result<()> {
        let ids = Some(self.message_ids());

        let batch_request = BatchDeleteMessagesRequest { ids };

        log::trace!("{batch_request:#?}");

        let _res = self
            .hub()
            .users()
            .messages_batch_delete(batch_request, "me")
            .add_scope("https://mail.google.com/")
            .doit()
            .await
            .map_err(Box::new)?;

        for m in self.messages() {
            log::info!("Message with subject `{}` deleted.", m.subject());
        }

        Ok(())
    }
}
