// use crate::EolRule;

use crate::{Delete, EolAction, Error, Result, Trash, config::EolRule};

/// Rule processor
#[derive(Debug)]
pub struct Processor<'a> {
    credential_file: String,
    rule: &'a EolRule,
    execute: bool,
}

/// Rule processor builder
#[derive(Debug)]
pub struct ProcessorBuilder<'a> {
    credential_file: String,
    rule: &'a EolRule,
    execute: bool,
}

impl<'a> ProcessorBuilder<'a> {
    /// Set the execute flag
    pub fn set_execute(&mut self, value: bool) -> &mut Self {
        self.execute = value;
        self
    }

    /// Build the Processor
    pub fn build(&'_ self) -> Processor<'_> {
        Processor {
            credential_file: self.credential_file.clone(),
            rule: self.rule,
            execute: self.execute,
        }
    }
}

impl<'a> Processor<'a> {
    /// Initialise a new processor
    pub fn builder(credential_file: &str, rule: &'a EolRule) -> ProcessorBuilder<'a> {
        ProcessorBuilder {
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

        if messages_to_trash.message_list().label_ids().is_empty() {
            return Err(Error::LabelNotFoundInMailbox(label.to_string()));
        }

        let Some(query) = self.rule.eol_query() else {
            return Err(Error::NoQueryStringCalculated(self.rule.id()));
        };
        messages_to_trash.message_list().set_query(&query);

        log::info!("{messages_to_trash:?}");
        log::info!("Ready to run");
        messages_to_trash.prepare(0).await?;
        if self.execute {
            log::info!("***executing final delete messages***");
            messages_to_trash.batch_trash().await
        } else {
            log::warn!("Execution stopped for dry run");
            Ok(())
        }
    }

    /// Delete the messages
    pub async fn delete_messages(&self, label: &str) -> Result<()> {
        let mut messages_to_delete = Delete::new(&self.credential_file).await?;

        messages_to_delete
            .message_list()
            .add_labels(&self.credential_file, &[label.to_string()])
            .await?;

        if messages_to_delete.message_list().label_ids().is_empty() {
            return Err(Error::LabelNotFoundInMailbox(label.to_string()));
        }

        let Some(query) = self.rule.eol_query() else {
            return Err(Error::NoQueryStringCalculated(self.rule.id()));
        };
        messages_to_delete.message_list().set_query(&query);

        log::info!("{messages_to_delete:?}");
        log::info!("Ready to run");
        messages_to_delete.prepare(0).await?;
        if self.execute {
            log::info!("***executing final delete messages***");
            messages_to_delete.batch_delete().await
        } else {
            log::warn!("Execution stopped for dry run");

            Ok(())
        }
    }
}
