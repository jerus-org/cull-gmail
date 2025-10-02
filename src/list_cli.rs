use clap::Parser;
use cull_gmail::{Error, List};

/// Command line options for the list subcommand
#[derive(Debug, Parser)]
pub struct ListCli {
    #[arg(short, long, default_value = cull_gmail::DEFAULT_MAX_RESULTS)]
    max_results: u32,
}

impl ListCli {
    pub(crate) async fn run(&self, credential_file: &str) -> Result<(), Error> {
        log::debug!("Max results: `{}`", self.max_results);

        let mut list = List::new(credential_file).await?;
        list.set_max_results(self.max_results);
        log::debug!("List max results set to {}", list.max_results());

        list.run().await
    }
}
