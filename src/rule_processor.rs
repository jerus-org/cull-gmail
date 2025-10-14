use google_gmail1::api::{BatchDeleteMessagesRequest, BatchModifyMessagesRequest};

use crate::{EolAction, Error, GmailClient, Result, config::EolRule, message_list::MessageList};

/// Rules processor to apply the configured rules to the mailbox.
pub trait RuleProcessor {
    /// Find the rule and the message for a specific label
    fn find_rule_and_messages_for_label(
        &mut self,
        label: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Set the execute flag in the client
    fn set_execute(&mut self, value: bool);
    // /// Delete messages
    // fn delete_messages(
    //     &mut self,
    //     label: &str,
    // ) -> impl std::future::Future<Output = Result<()>> + Send;
    // /// Trash Messages
    // fn trash_messages(
    //     &mut self,
    //     label: &str,
    // ) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Set rule to process
    fn set_rule(&mut self, action: EolRule);
    /// Report the action from the rule
    fn action(&self) -> Option<EolAction>;
    /// Prepare a list of messages to trash or delete
    fn prepare(&mut self, pages: u32) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Batch delete of messages
    fn batch_delete(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Batch trash
    fn batch_trash(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl RuleProcessor for GmailClient {
    /// Add Action to the Client for processing
    fn set_rule(&mut self, value: EolRule) {
        self.rule = Some(value);
    }

    /// Set the execute flag
    fn set_execute(&mut self, value: bool) {
        self.execute = value;
    }

    /// The action set in the rule  
    fn action(&self) -> Option<EolAction> {
        if let Some(rule) = &self.rule {
            return rule.action();
        }
        None
    }

    /// Find the rule and messages for the label
    async fn find_rule_and_messages_for_label(&mut self, label: &str) -> Result<()> {
        self.add_labels(&[label.to_string()])?;

        if self.label_ids().is_empty() {
            return Err(Error::LabelNotFoundInMailbox(label.to_string()));
        }

        let Some(rule) = &self.rule else {
            return Err(Error::RuleNotFound(0));
        };

        let Some(query) = rule.eol_query() else {
            return Err(Error::NoQueryStringCalculated(rule.id()));
        };
        self.set_query(&query);

        log::info!("{:?}", self.messages());
        log::info!("Ready to run");
        self.prepare(0).await?;
        if self.execute {
            log::info!("***executing final delete messages***");
            self.batch_trash().await
        } else {
            log::warn!("Execution stopped for dry run");
            Ok(())
        }
    }

    // /// Trash the messages
    // async fn trash_messages(&mut self, label: &str) -> Result<()> {
    //     self.add_labels(&[label.to_string()]).await?;

    //     if self.label_ids().is_empty() {
    //         return Err(Error::LabelNotFoundInMailbox(label.to_string()));
    //     }

    //     let Some(rule) = &self.rule else {
    //         return Err(Error::RuleNotFound(0));
    //     };

    //     let Some(query) = rule.eol_query() else {
    //         return Err(Error::NoQueryStringCalculated(rule.id()));
    //     };
    //     self.set_query(&query);

    //     log::info!("{:?}", self.messages());
    //     log::info!("Ready to run");
    //     self.prepare(0).await?;
    //     if self.execute {
    //         log::info!("***executing final delete messages***");
    //         self.batch_trash().await
    //     } else {
    //         log::warn!("Execution stopped for dry run");
    //         Ok(())
    //     }
    // }

    // /// Delete the messages
    // async fn delete_messages(&mut self, label: &str) -> Result<()> {
    //     self.add_labels(&[label.to_string()]).await?;

    //     if self.label_ids().is_empty() {
    //         return Err(Error::LabelNotFoundInMailbox(label.to_string()));
    //     }

    //     let Some(rule) = &self.rule else {
    //         return Err(Error::RuleNotFound(0));
    //     };

    //     let Some(query) = rule.eol_query() else {
    //         return Err(Error::NoQueryStringCalculated(rule.id()));
    //     };
    //     self.set_query(&query);

    //     log::info!("{:?}", self.messages());
    //     log::info!("Ready to run");
    //     self.prepare(0).await?;
    //     if self.execute {
    //         log::info!("***executing final delete messages***");
    //         self.batch_delete().await
    //     } else {
    //         log::warn!("Execution stopped for dry run");

    //         Ok(())
    //     }
    // }
    /// Prepare the message list for delete to be completed on execute by batch_delete
    async fn prepare(&mut self, pages: u32) -> Result<()> {
        self.get_messages(pages).await
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
    /// Move the messages to trash
    async fn batch_trash(&self) -> Result<()> {
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
            .add_scope("https://www.google.com/")
            .doit()
            .await
            .map_err(Box::new)?;

        for m in self.messages() {
            log::info!("Message with subject `{}` moved to trash.", m.subject());
        }

        Ok(())
    }
}
