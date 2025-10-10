// use crate::EolRule;

use crate::{Result, Trash, config::EolRule};

/// Rule processor
#[derive(Debug)]
pub struct Processor {
    credential_file: String,
    rule: EolRule,
}

impl Processor {
    fn new(credential_file: String, rule: EolRule) -> Self {
        Processor {
            credential_file,
            rule,
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
}
