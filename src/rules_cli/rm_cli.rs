use clap::Parser;
use cull_gmail::{Config, Error};

#[derive(Debug, Parser)]
pub struct RmCli {}

impl RmCli {
    pub fn run(&self, _config: Config) -> Result<(), Error> {
        Ok(())
    }
}
