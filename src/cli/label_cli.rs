use clap::Parser;
use cull_gmail::{Error, Labels};

#[derive(Debug, Parser)]
pub struct LabelCli {}

impl LabelCli {
    pub async fn run(&self, credential_file: &str) -> Result<(), Error> {
        let _ = Labels::new(credential_file, true).await?;
        Ok(())
    }
}
