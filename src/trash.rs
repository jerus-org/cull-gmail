use google_gmail1::api::BatchModifyMessagesRequest;

use crate::{GmailClient, Result, message_list::MessageList};

// /// Struct for trashing messages
// #[derive(Debug)]
// pub struct Trash {
//     message_list: MessageList,
// }

/// Methods for GmailClient to batch move messages to trash
pub trait Trash {
    /// Batch move to trash
    fn batch_move_to_trash(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Batch trash
    fn batch_trash(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl Trash for GmailClient {
    /// Move the messages to trash
    async fn batch_trash(&self) -> Result<()> {
        self.batch_move_to_trash().await
    }

    async fn batch_move_to_trash(&self) -> Result<()> {
        let add_label_ids = Some(Vec::from(["TRASH".to_string()]));
        let ids = Some(self.message_ids());
        let remove_label_ids = Some(self.label_ids());

        let batch_request = BatchModifyMessagesRequest {
            add_label_ids,
            ids,
            remove_label_ids,
        };

        log::trace!("{batch_request:#?}");

        let _res = self
            .hub()
            .users()
            .messages_batch_modify(batch_request, "me")
            .add_scope("https://www.googleapis.com/auth/gmail.modify")
            .doit()
            .await
            .map_err(Box::new)?;

        for m in self.messages() {
            log::info!("Message with subject `{}` moved to trash.", m.subject());
        }

        Ok(())
    }
}
