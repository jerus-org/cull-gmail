use google_clis_common as common;
use google_gmail1::{
    Gmail,
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    hyper_util::{
        client::legacy::{Client, connect::HttpConnector},
        rt::TokioExecutor,
    },
    yup_oauth2::{ApplicationSecret, InstalledFlowAuthenticator, InstalledFlowReturnMethod},
};

use crate::{Credential, Error};

const DEFAULT_MAX_RESULTS: u32 = 10;

/// Struct to capture configuration for List API call.
pub struct List {
    hub: Gmail<HttpsConnector<HttpConnector>>,
    max_results: u32,
}

impl List {
    /// Create a new List struct and add the Gmail api connection.
    pub async fn new(credential: &str) -> Result<Self, Error> {
        let (config_dir, secret) = {
            let config_dir = match common::assure_config_dir_exists("~/.cull-gmail") {
                Err(e) => return Err(Error::InvalidOptionsError(e, 3)),
                Ok(p) => p,
            };

            let secret: ApplicationSecret = Credential::load_json_file(credential).into();
            (config_dir, secret)
        };

        let executor = TokioExecutor::new();
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build();

        let client = Client::builder(executor.clone()).build(connector.clone());

        let auth = InstalledFlowAuthenticator::with_client(
            secret,
            InstalledFlowReturnMethod::HTTPRedirect,
            Client::builder(executor).build(connector),
        )
        .persist_tokens_to_disk(format!("{config_dir}/gmail1"))
        .build()
        .await
        .unwrap();

        Ok(List {
            hub: Gmail::new(client, auth),
            max_results: DEFAULT_MAX_RESULTS,
        })
    }

    /// Run the Gmail api as configured
    pub async fn run(&self) -> Result<(), Error> {
        let (_response, list) = self
            .hub
            .users()
            .messages_list("me")
            .max_results(self.max_results)
            .doit()
            .await?;

        // println!("{list:#?}");
        if let Some(messages) = list.messages {
            for message in messages {
                if let Some(id) = message.id {
                    log::trace!("{id}");
                    let (_res, m) = self
                        .hub
                        .users()
                        .messages_get("me", &id)
                        .add_scope("https://www.googleapis.com/auth/gmail.metadata")
                        .format("metadata")
                        .add_metadata_headers("subject")
                        .doit()
                        .await?;

                    let mut subject = String::new();
                    if let Some(payload) = m.payload {
                        if let Some(headers) = payload.headers {
                            for header in headers {
                                if header.name.is_some()
                                    && header.name.unwrap() == "Subject"
                                    && header.value.is_some()
                                {
                                    subject = header.value.unwrap();
                                    break;
                                } else {
                                    continue;
                                }
                            }
                        }
                    }

                    log::info!("{subject:?}");
                }
            }
        }
        Ok(())
    }
}
