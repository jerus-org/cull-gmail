use clap::Parser;
use cull_gmail::{Error, List};

/// Command line options for the list subcommand
#[derive(Debug, Parser)]
pub struct ListCli {}

impl ListCli {
    pub(crate) async fn run(&self, credential_file: &str) -> Result<(), Error> {
        let list = List::new(credential_file).await?;
        list.run().await
    }
}
