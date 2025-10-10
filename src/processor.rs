// use crate::EolRule;

use crate::{Delete, Result, Trash, config::EolRule};

/// Rule processor
#[derive(Debug)]
pub struct Processor {
    credential_file: String,
    rule: EolRule,
    execute: bool,
}

impl Processor {
    fn new(credential_file: String, rule: EolRule) -> Self {
        Processor {
            credential_file,
            rule,
            execute: false,
        }
    }

    async fn trash_messages(&self, label: &str) -> Result<()> {
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

    async fn delete_messages(&self, label: &str) -> Result<()> {
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
