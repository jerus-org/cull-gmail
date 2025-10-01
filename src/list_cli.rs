use clap::Parser;
use cull_gmail::List;
use google_gmail1::{
    Gmail, hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector,
};

/// Command line options for the list subcommand
#[derive(Debug, Parser)]
pub struct ListCli {}

impl ListCli {
    pub(crate) async fn run(&self, hub: Gmail<HttpsConnector<HttpConnector>>) {
        let list = List::new(hub);
        list.run().await;
    }
}
