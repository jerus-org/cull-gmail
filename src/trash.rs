use google_gmail1::api::BatchModifyMessagesRequest;

use crate::{Error, MessageList};

/// Struct for trashing messages
#[derive(Debug)]
pub struct Trash {
    message_list: MessageList,
}

impl Trash {
    /// Create a new Trash struct
    pub async fn new(credential: &str) -> Result<Self, Error> {
        let message_list = MessageList::new(credential).await?;
        Ok(Trash { message_list })
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
    pub fn add_labels(&mut self, label_ids: &[String]) {
        self.message_list.add_labels(label_ids)
    }

    /// Set the query string
    pub fn set_query(&mut self, query: &str) {
        self.message_list.set_query(query)
    }

    /// Run the trash cli
    pub async fn run(&mut self, pages: u32) -> Result<(), Error> {
        self.message_list.run(pages).await?;
        self.batch_move_to_trash().await?;

        Ok(())
    }

    async fn batch_move_to_trash(&self) -> Result<(), Error> {
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
            log::info!("Message with subject `{}` move to trash.", m.subject());
        }

        Ok(())
    }
}
