# cull-gmail Library Documentation

The `cull-gmail` library provides a Rust API for managing Gmail messages through the Gmail API. It enables programmatic email culling operations including authentication, message querying, filtering, and batch operations (trash/delete).

## Installation

Add the library to your `Cargo.toml`:

```toml
[dependencies]
cull-gmail = "0.0.12"
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

## Gmail Query Syntax

The library supports Gmail's search syntax for message queries:

```rust path=null start=null
// Date-based queries
client.set_query("older_than:1y");           // Older than 1 year
client.set_query("newer_than:30d");          // Newer than 30 days
client.set_query("after:2023/1/1");          // After specific date

// Label-based queries
client.set_query("label:promotions");        // Has promotions label
client.set_query("-label:important");        // Does NOT have important label

// Content queries
client.set_query("subject:newsletter");      // Subject contains "newsletter"
client.set_query("from:noreply@example.com"); // From specific sender

// Combined queries
client.set_query("label:promotions older_than:6m -is:starred");
```

## Performance & Limits

### Pagination

- Default page size: 200 messages
- Use `client.set_max_results(n)` to adjust
- Use `client.get_messages(0)` to get all pages
- Use `client.get_messages(n)` to limit to n pages

### Rate Limits

- The library uses the official `google-gmail1` crate
- Built-in retry logic for transient errors
- Respects Gmail API quotas and limits

### Batch Operations

- Batch delete/trash operations are more efficient than individual calls
- Operations are atomic - either all succeed or all fail

## Logging

The library uses the `log` crate for logging:

```rust path=null start=null
use env_logger;

// Initialize logging
env_logger::init();

# Set log level via environment variable
# RUST_LOG=cull_gmail=debug cargo run
```

Log levels:
- `error`: Critical errors
- `warn`: Warnings (e.g., missing labels, dry-run mode)
- `info`: General information (e.g., message subjects, action results)
- `debug`: Detailed operation info
- `trace`: Very detailed debugging info

## Security Considerations

### OAuth2 Token Storage
- Tokens are stored in `~/.cull-gmail/gmail1` by default
- Tokens are automatically refreshed when expired
- Revoke access in [Google Account settings](https://myaccount.google.com/permissions)

### Required Scopes
The library requires the `https://mail.google.com/` scope for full Gmail access.

### OAuth2 File Security
- Store OAuth2 credential files securely (not in version control)
- Use restrictive file permissions (600)
- Consider using environment variables in production

## Troubleshooting

### Authentication Issues
1. Verify OAuth2 credential file path and JSON format
2. Check OAuth2 client is configured for "Desktop Application"
3. Ensure redirect URI matches configuration
4. Clear token cache: `rm -rf ~/.cull-gmail/gmail1`

### No Messages Found
1. Verify label names exist: `client.show_label()`
2. Test query syntax in Gmail web interface
3. Check for typos in label names or query strings

### Rate Limiting
1. Reduce page size: `client.set_max_results(100)`
2. Add delays between operations
3. Check [Gmail API quotas](https://developers.google.com/gmail/api/reference/quota)

## See Also

- [CLI Documentation](main.md) - Complete guide to the command-line interface
- [Examples Directory](../examples/) - Additional code examples and sample configurations
- [API Documentation](https://docs.rs/cull-gmail) - Generated API reference
- [Repository](https://github.com/jerus-org/cull-gmail) - Source code and issue tracking
