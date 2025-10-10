use clap::Parser;
use cull_gmail::Result;

#[derive(Debug, Parser)]
pub struct RunCli {}

impl RunCli {
    pub async fn run(&self, _credential: &str) -> Result<()> {
        Ok(())
    }
}
