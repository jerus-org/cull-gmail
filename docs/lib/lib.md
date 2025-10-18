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

```rust
# // This is a compile-only test since it requires OAuth credentials
use cull_gmail::{ClientConfig, GmailClient, MessageList, Result};

// Example of how to set up the client (requires valid OAuth credentials)
async fn setup_client() -> Result<GmailClient> {
    let config = ClientConfig::builder()
        .with_client_id("your-client-id")
        .with_client_secret("your-client-secret")
        .build();
    
    let mut client = GmailClient::new_with_config(config).await?;
    
    // Configure message listing
    client.set_max_results(10);
    // client.get_messages(1).await?;
    // client.log_messages().await?;
    
    Ok(client)
}

fn main() {}
```

## Core Types

### GmailClient

The main client for interacting with Gmail API:

```rust
# use cull_gmail::Result;
# use cull_gmail::{GmailClient, MessageList, ClientConfig}; 
#
# async fn example() -> Result<()> {
# let config = ClientConfig::builder().with_client_id("test").with_client_secret("test").build();

// Create client with configuration
let mut client = GmailClient::new_with_config(config).await?;

// Query messages with Gmail search syntax
client.set_query("older_than:1y label:promotions");
client.add_labels(&["INBOX".to_string()])?;
client.set_max_results(200);

// Get messages (0 = all pages, 1 = first page only)
// client.get_messages(0).await?;

// Access message data
let messages = client.messages();
let message_ids = client.message_ids();
# Ok(())
# }
# fn main() {}
```

### ClientConfig

Handles authentication and configuration:

```rust
use cull_gmail::ClientConfig;

// From individual OAuth2 parameters (recommended for doctests)
let config = ClientConfig::builder()
    .with_client_id("your-client-id")
    .with_client_secret("your-client-secret")
    .with_auth_uri("https://accounts.google.com/o/oauth2/auth")
    .with_token_uri("https://oauth2.googleapis.com/token")
    .add_redirect_uri("http://localhost:8080")
    .build();

// Configuration with file paths (requires actual files)
// let config = ClientConfig::builder()
//     .with_credential_file("path/to/credential.json")
//     .with_config_path(".cull-gmail")
//     .build();
```

### Rules and Retention Policies

Define automated message lifecycle rules:

```rust 
use cull_gmail::{Rules, Retention, MessageAge, EolAction};

# use cull_gmail::Result;
# fn main() -> Result<()> {

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
// rules.save()?;

// Load existing rules
// let loaded_rules = Rules::load()?;
# Ok(())
# }
```

### Message Operations

Batch operations on messages:

```rust
# use cull_gmail::{RuleProcessor, EolAction, GmailClient, Rules, ClientConfig, MessageAge, Retention, Result, MessageList};
# 
# async fn example() -> Result<()> {
# let config = ClientConfig::builder().with_client_id("test").with_client_secret("test").build();
# let mut client = GmailClient::new_with_config(config).await?;
# let mut rules = Rules::new();
# rules.add_rule(Retention::new(MessageAge::Years(1), true), Some(&"test".to_string()), false);
use cull_gmail::{RuleProcessor, EolAction};

// Set up rule and dry-run mode
client.set_execute(false); // Dry run - no actual changes
let rule = rules.get_rule(1).unwrap();
client.set_rule(rule);

// Find messages matching rule for a label would require network access
// client.find_rule_and_messages_for_label("promotions").await?;

// Check what action would be performed
if let Some(action) = client.action() {
    match action {
        EolAction::Trash => println!("Would move {} messages to trash", client.messages().len()),
        EolAction::Delete => println!("Would delete {} messages permanently", client.messages().len()),
    }
}

// Execute operations (commented out for doctest)
// client.set_execute(true);
// match client.action() {
//     Some(EolAction::Trash) => client.batch_trash().await?,
//     Some(EolAction::Delete) => client.batch_delete().await?,
//     None => println!("No action specified"),
// }
# Ok(())
# }
# fn main() {}
```

## Configuration

### OAuth2 Setup

1. Create OAuth2 credentials in [Google Cloud Console](https://console.cloud.google.com/)
2. Download the credential JSON file
3. Configure the client:

```rust
use cull_gmail::ClientConfig;

// Build config with OAuth parameters (recommended for tests)
let config = ClientConfig::builder()
    .with_client_id("your-client-id")
    .with_client_secret("your-client-secret")
    .build();
    
// Or from credential file (requires actual file)
// let config = ClientConfig::builder()
//     .with_credential_file("path/to/credential.json")
//     .build();
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

```rust
#[tokio::main]
async fn main() -> cull_gmail::Result<()> {
    // Your code here
    Ok(())
}
```

