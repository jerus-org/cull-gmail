use clap::Parser;
use cull_gmail::{Error, Labels, List};

/// Command line options for the list subcommand
#[derive(Debug, Parser)]
pub struct ListCli {
    /// Maximum results per page
    #[arg(short, long, default_value = cull_gmail::DEFAULT_MAX_RESULTS)]
    max_results: u32,
    /// Maximum number of pages (0=all)
    #[arg(short, long, default_value = "1")]
    pages: u32,
    /// Labels to filter the message list
    #[arg(short, long)]
    labels: Vec<String>,
}

impl ListCli {
    pub(crate) async fn run(&self, credential_file: &str) -> Result<(), Error> {
        let mut labels = Labels::new(credential_file).await?;
        labels.get_labels().await?;
        log::trace!("labels found and setup {labels:#?}");

        let mut list = List::new(credential_file).await?;
        log::debug!("labels from command line: {:?}", self.labels);

        for label in &self.labels {
            if let Some(l) = labels.get_label_id(label) {
                log::trace!("adding `{l}` id to labels");
                list.add_label(&l);
            }
        }

        log::trace!("Max results: `{}`", self.max_results);
        list.set_max_results(self.max_results);
        log::debug!("List max results set to {}", list.max_results());

        list.run(self.pages).await
    }
}
