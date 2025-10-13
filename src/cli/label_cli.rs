use clap::Parser;
use cull_gmail::{Error, GmailClient};

#[derive(Debug, Parser)]
pub struct LabelCli {}

impl LabelCli {
    pub async fn run(&self, client: GmailClient) -> Result<(), Error> {
        client.show_label();
        Ok(())
    }
}
