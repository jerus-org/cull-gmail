// use crate::EolRule;

use std::fmt;

use crate::{Delete, EolAction, Error, GmailClient, Result, Trash, config::EolRule};

/// Rule processor
#[derive()]
pub struct Processor<'a> {
    client: GmailClient,
    rule: &'a EolRule,
    execute: bool,
}

impl<'a> fmt::Debug for Processor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Processor")
            .field("rule", &self.rule)
            .field("execute", &self.execute)
            .finish()
    }
}

/// Rule processor builder
#[derive()]
pub struct ProcessorBuilder<'a> {
    client: GmailClient,
    rule: &'a EolRule,
    execute: bool,
}

impl<'a> fmt::Debug for ProcessorBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcessorBuilder")
            .field("rule", &self.rule)
            .field("execute", &self.execute)
            .finish()
    }
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
            client: self.client.clone(),
            rule: self.rule,
            execute: self.execute,
        }
    }
}

impl<'a> Processor<'a> {
    /// Initialise a new processor
    pub fn builder(client: &GmailClient, rule: &'a EolRule) -> ProcessorBuilder<'a> {
        ProcessorBuilder {
            client: client.clone(),
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
        let mut messages_to_trash = Trash::new(&self.client).await?;
        messages_to_trash
            .message_list()
            .add_labels(&self.client, &[label.to_string()])
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
        let mut messages_to_delete = Delete::new(&self.client).await?;

        messages_to_delete
            .message_list()
            .add_labels(&self.client, &[label.to_string()])
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
