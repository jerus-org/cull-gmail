# cull-gmail Library Documentation

The `cull-gmail` library provides a Rust API for managing Gmail messages through the Gmail API. It enables programmatic email culling operations including authentication, message querying, filtering, and batch operations (trash/delete).

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
cull-gmail = "0.0.10"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

Here's a minimal example to get started:

```rust path=null start=null
use cull_gmail::{ClientConfig, GmailClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from file or environment
    let config = ClientConfig::builder()
        .with_credential_file("credential.json")
        .build();
    
    // Create Gmail client and authenticate
    let mut client = GmailClient::new_with_config(config).await?;
    
    // List first 10 messages
    client.set_max_results(10);
    client.get_messages(1).await?;
    client.log_messages().await?;
    
    Ok(())
}
```

## Core Types

### GmailClient

The main client for interacting with Gmail API:

```rust path=null start=null
use cull_gmail::{GmailClient, MessageList};

// Create client with configuration
let mut client = GmailClient::new_with_config(config).await?;

// Query messages with Gmail search syntax
client.set_query("older_than:1y label:promotions");
client.add_labels(&["INBOX".to_string()])?;
client.set_max_results(200);

// Get messages (0 = all pages, 1 = first page only)
client.get_messages(0).await?;

// Access message data
let messages = client.messages();
let message_ids = client.message_ids();
```

### ClientConfig

Handles authentication and configuration:

```rust path=null start=null
use cull_gmail::ClientConfig;

// From credential file
let config = ClientConfig::builder()
    .with_credential_file("path/to/credential.json")
    .with_config_path(".cull-gmail")
    .build();

// From individual OAuth2 parameters
let config = ClientConfig::builder()
    .with_client_id("your-client-id")
    .with_client_secret("your-client-secret")
    .with_auth_uri("https://accounts.google.com/o/oauth2/auth")
    .with_token_uri("https://oauth2.googleapis.com/token")
    .add_redirect_uri("http://localhost:8080")
    .build();
```

### Rules and Retention Policies

Define automated message lifecycle rules:

```rust path=null start=null
use cull_gmail::{Rules, Retention, MessageAge, EolAction};

// Create a rule set
let mut rules = Rules::new();

// Add retention rules
rules.add_rule(
    Retention::new(MessageAge::Years(1), true),
    Some(&"old-emails".to_string()),
    false // false = trash, true = delete
);

rules.add_rule(
    Retention::new(MessageAge::Months(6), true),
    Some(&"promotions".to_string()),
    false
);

// Save rules to file
rules.save()?;

// Load existing rules
let loaded_rules = Rules::load()?;
```

### Message Operations

Batch operations on messages:

```rust path=null start=null
use cull_gmail::{RuleProcessor, EolAction};

// Set up rule and dry-run mode
client.set_execute(false); // Dry run - no actual changes
let rule = rules.get_rule(1).unwrap();
client.set_rule(rule);

// Find messages matching rule for a label
client.find_rule_and_messages_for_label("promotions").await?;

// Check what action would be performed
if let Some(action) = client.action() {
    match action {
        EolAction::Trash => println!("Would move {} messages to trash", client.messages().len()),
        EolAction::Delete => println!("Would delete {} messages permanently", client.messages().len()),
    }
}

// Execute for real
client.set_execute(true);
match client.action() {
    Some(EolAction::Trash) => client.batch_trash().await?,
    Some(EolAction::Delete) => client.batch_delete().await?,
    None => println!("No action specified"),
}
```

## Configuration

### OAuth2 Setup

1. Create OAuth2 credentials in [Google Cloud Console](https://console.cloud.google.com/)
2. Download the credential JSON file
3. Configure the client:

```rust path=null start=null
let config = ClientConfig::builder()
    .with_credential_file("path/to/credential.json")
    .build();
```

### Configuration File

The library supports TOML configuration files (default: `~/.cull-gmail/cull-gmail.toml`):

```toml
credentials = "credential.json"
config_root = "~/.cull-gmail"
rules = "rules.toml"
execute = false

# Alternative: direct OAuth2 parameters
# client_id = "your-client-id"
# client_secret = "your-client-secret"
# token_uri = "https://oauth2.googleapis.com/token"
# auth_uri = "https://accounts.google.com/o/oauth2/auth"
```

### Environment Variables

Override configuration with environment variables:

```bash
export APP_CREDENTIALS="/path/to/credential.json"
export APP_EXECUTE="true"
export APP_CLIENT_ID="your-client-id"
export APP_CLIENT_SECRET="your-client-secret"
```

## Error Handling

The library uses a comprehensive error type:

```rust path=null start=null
use cull_gmail::{Error, Result};

match client.get_messages(1).await {
    Ok(_) => println!("Success!"),
    Err(Error::NoLabelsFound) => println!("No labels found in mailbox"),
    Err(Error::LabelNotFoundInMailbox(label)) => println!("Label '{}' not found", label),
    Err(Error::GoogleGmail1(e)) => println!("Gmail API error: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

Common error types:
- `NoLabelsFound`: Mailbox has no labels
- `LabelNotFoundInMailbox(String)`: Specific label not found
- `RuleNotFound(usize)`: Rule ID doesn't exist
- `GoogleGmail1(Box<google_gmail1::Error>)`: Gmail API errors
- `StdIO(std::io::Error)`: File I/O errors
- `Config(config::ConfigError)`: Configuration errors

## Async Runtime

The library requires an async runtime (Tokio recommended):

```toml
[dependencies]
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }
```

```rust path=null start=null
#[tokio::main]
async fn main() -> cull_gmail::Result<()> {
    // Your code here
    Ok(())
}
```

