use crate::{
    EolAction, Error, GmailClient, Result, config::EolRule, delete::Delete,
    message_list::MessageList, trash::Trash,
};

// /// Rule processor
// #[derive()]
// pub struct Processor<'a> {
//     client: GmailClient,
//     rule: &'a EolRule,
//     execute: bool,
// }

// impl<'a> fmt::Debug for Processor<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Processor")
//             .field("rule", &self.rule)
//             .field("execute", &self.execute)
//             .finish()
//     }
// }

// /// Rule processor builder
// #[derive()]
// pub struct ProcessorBuilder<'a> {
//     client: GmailClient,
//     rule: &'a EolRule,
//     execute: bool,
// }

// impl<'a> fmt::Debug for ProcessorBuilder<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("ProcessorBuilder")
//             .field("rule", &self.rule)
//             .field("execute", &self.execute)
//             .finish()
//     }
// }

// impl<'a> ProcessorBuilder<'a> {
//     /// Set the execute flag
//     pub fn set_execute(&mut self, value: bool) -> &mut Self {
//         self.execute = value;
//         self
//     }

//     /// Build the Processor
//     pub fn build(&'_ self) -> Processor<'_> {
//         Processor {
//             client: self.client.clone(),
//             rule: self.rule,
//             execute: self.execute,
//         }
//     }
// }

/// Rules processor to apply the configured rules to the mailbox.
pub trait RuleProcessor {
    /// Set the execute flag in the client
    fn set_execute(&mut self, value: bool);
    /// Delete messages
    fn delete_messages(
        &mut self,
        label: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Trash Messages
    fn trash_messages(
        &mut self,
        label: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Set rule to process
    fn set_rule(&mut self, action: EolRule);
    /// Report the action from the rule
    fn action(&self) -> Option<EolAction>;
}

impl RuleProcessor for GmailClient {
    // /// Initialise a new processor
    // pub fn builder(client: &GmailClient, rule: &'a EolRule) -> ProcessorBuilder<'a> {
    //     ProcessorBuilder {
    //         client: client.clone(),
    //         rule,
    //         execute: false,
    //     }
    // }

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

    /// Trash the messages
    async fn trash_messages(&mut self, label: &str) -> Result<()> {
        self.add_labels(&[label.to_string()]).await?;

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

    /// Delete the messages
    async fn delete_messages(&mut self, label: &str) -> Result<()> {
        self.add_labels(&[label.to_string()]).await?;

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
            self.batch_delete().await
        } else {
            log::warn!("Execution stopped for dry run");

            Ok(())
        }
    }
}
