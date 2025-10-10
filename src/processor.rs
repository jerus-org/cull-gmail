// use crate::EolRule;

use crate::{Delete, EolAction, Result, Trash, config::EolRule};

/// Rule processor
#[derive(Debug)]
pub struct Processor<'a> {
    credential_file: String,
    rule: &'a EolRule,
    execute: bool,
}

impl<'a> Processor<'a> {
    /// Initialise a new processor
    pub fn new(credential_file: &str, rule: &'a EolRule) -> Self {
        Processor {
            credential_file: credential_file.to_string(),
            rule,
            execute: false,
        }
    }

    /// The action set in the rule  
    pub fn action(&self) -> Option<EolAction> {
        self.rule.action()
    }

    /// Trash the messages
    pub async fn trash_messages(&self, label: &str) -> Result<()> {
        let mut messages_to_trash = Trash::new(&self.credential_file).await?;

        messages_to_trash
            .message_list()
            .add_labels(&self.credential_file, &[label.to_string()])
            .await?;

        messages_to_trash
            .message_list()
            .set_query(&self.rule.eol_query());

        messages_to_trash.run(0).await
    }

    /// Delete the messages
    pub async fn delete_messages(&self, label: &str) -> Result<()> {
        let mut messages_to_delete = Delete::new(&self.credential_file).await?;

        messages_to_delete
            .message_list()
            .add_labels(&self.credential_file, &[label.to_string()])
            .await?;

        messages_to_delete
            .message_list()
            .set_query(&self.rule.eol_query());

        messages_to_delete.prepare(0).await?;
        if self.execute {
            messages_to_delete.batch_delete().await
        } else {
            Ok(())
        }
    }
}
