// Optional integration test for Gmail API interactions.
//
// This test is ignored by default to avoid network use in CI.
// To run locally, ensure you have valid OAuth client credentials and set up
// the configuration as required by `ClientConfig`.
//
// Example to run:
//   cargo test --test gmail_message_list_integration -- --ignored

use cull_gmail::{ClientConfig, GmailClient, MessageList, Result};

#[ignore]
#[tokio::test]
async fn list_first_page_of_messages_smoke_test() -> Result<()> {
    // Configure with your own credentials before running locally.
    let config = ClientConfig::builder()
        // .with_config_base(&cull_gmail::client_config::config_root::RootBase::Home)
        // .with_config_path(".cull-gmail")
        // .with_credential_file("client_secret.json")
        // Alternatively specify client_id/client_secret and related fields:
        // .with_client_id("<your-client-id>")
        // .with_client_secret("<your-client-secret>")
        .build();

    let mut client = GmailClient::new_with_config(config).await?;

    // Configure a conservative query to avoid heavy traffic
    client.set_query("in:inbox newer_than:30d");
    client.set_max_results(10);

    // Should complete without error; results may be empty depending on mailbox
    client.get_messages(1).await?;
    let _ids = client.message_ids();

    Ok(())
}
