use google_gmail1::{
    Error, Gmail, hyper_rustls::HttpsConnector, hyper_util::client::legacy::connect::HttpConnector,
};

/// Struct to capture configuration for List API call.
pub struct List {
    hub: Gmail<HttpsConnector<HttpConnector>>,
}

impl List {
    /// Create a new List struct and add the Gmail api connection.
    pub fn new(hub: Gmail<HttpsConnector<HttpConnector>>) -> Self {
        List { hub }
    }

    /// Run the Gmail api as configured
    pub async fn run(&self) {
        let result = self.hub.users().messages_list("me").doit().await;

        match result {
            Err(e) => match e {
                // The Error enum provides details about what exactly happened.
                // You can also just use its `Debug`, `Display` or `Error` traits
                Error::HttpError(_)
                | Error::Io(_)
                | Error::MissingAPIKey
                | Error::MissingToken(_)
                | Error::Cancelled
                | Error::UploadSizeLimitExceeded(_, _)
                | Error::Failure(_)
                | Error::BadRequest(_)
                | Error::FieldClash(_)
                | Error::JsonDecodeError(_, _) => println!("{e}"),
            },
            Ok(res) => println!("Success: {res:?}"),
        }
    }
}
