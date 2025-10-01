use google_gmail1::{
    common::Client,
    hyper_rustls::{self, HttpsConnector},
    hyper_util::{self, client::legacy::connect::HttpConnector},
    yup_oauth2::{self, ApplicationSecret, authenticator::Authenticator},
};

/// Get auth for gmail
pub async fn get_auth(secret: ApplicationSecret) -> Authenticator<HttpsConnector<HttpConnector>> {
    yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap()
}

/// Get client for gmail
pub fn get_client() -> Client<HttpsConnector<HttpConnector>> {
    hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new()).build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_or_http()
            .enable_http1()
            .build(),
    )
}
