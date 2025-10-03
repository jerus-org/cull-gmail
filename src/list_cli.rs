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
    /// Query string to select messages to list
    #[arg(short = 'Q', long)]
    query: Option<String>,
}

impl ListCli {
    pub(crate) async fn run(&self, credential_file: &str) -> Result<(), Error> {
        let mut list = List::new(credential_file).await?;

        if !self.labels.is_empty() {
            // add labels if any specified
            let label_list = Labels::new(credential_file, false).await?;

            log::trace!("labels found and setup {label_list:#?}");
            log::debug!("labels from command line: {:?}", self.labels);
            let mut label_ids = Vec::new();
            for label in &self.labels {
                if let Some(id) = label_list.get_label_id(label) {
                    label_ids.push(id)
                }
            }
            list.add_labels(label_ids.as_slice());
        }

        if let Some(query) = self.query.as_ref() {
            list.set_query(query)
        }

        log::trace!("Max results: `{}`", self.max_results);
        list.set_max_results(self.max_results);
        log::debug!("List max results set to {}", list.max_results());

        list.run(self.pages).await
    }
}
