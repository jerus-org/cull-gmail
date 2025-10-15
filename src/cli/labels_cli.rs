use clap::Parser;
use cull_gmail::{Error, GmailClient};

#[derive(Debug, Parser)]
pub struct LabelsCli {}

impl LabelsCli {
    pub async fn run(&self, client: GmailClient) -> Result<(), Error> {
        client.show_label();
        Ok(())
    }
}
