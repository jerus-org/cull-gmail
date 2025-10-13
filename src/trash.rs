use google_gmail1::api::BatchModifyMessagesRequest;

use crate::{GmailClient, MessageList, Result};

/// Struct for trashing messages
#[derive(Debug)]
pub struct Trash {
    message_list: MessageList,
}

impl Trash {
    /// Create a new Trash struct
    pub async fn new(client: &GmailClient) -> Result<Self> {
        let message_list = MessageList::new(client).await?;
        Ok(Trash { message_list })
    }

    /// return the message list struct
    pub fn message_list(&mut self) -> &mut MessageList {
        &mut self.message_list
    }

    /// Prepare the trash cli
    pub async fn prepare(&mut self, pages: u32) -> Result<()> {
        self.message_list.run(pages).await?;

        Ok(())
    }

    /// Move the messages to trash
    pub async fn batch_trash(&self) -> Result<()> {
        self.batch_move_to_trash().await
    }

    async fn batch_move_to_trash(&self) -> Result<()> {
        let add_label_ids = Some(Vec::from(["TRASH".to_string()]));
        let ids = Some(self.message_list.message_ids());
        let remove_label_ids = Some(self.message_list.label_ids());

        let batch_request = BatchModifyMessagesRequest {
            add_label_ids,
            ids,
            remove_label_ids,
        };

        log::trace!("{batch_request:#?}");

        let _res = self
            .message_list
            .hub()
            .users()
            .messages_batch_modify(batch_request, "me")
            .add_scope("https://www.googleapis.com/auth/gmail.modify")
            .doit()
            .await
            .map_err(Box::new)?;

        for m in self.message_list.messages() {
            log::info!("Message with subject `{}` moved to trash.", m.subject());
        }

        Ok(())
    }
}
