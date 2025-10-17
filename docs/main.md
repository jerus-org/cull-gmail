# cull-gmail CLI Documentation

A command-line program for managing Gmail messages using the Gmail API. The tool provides subcommands for label querying, message querying, rule configuration, and rule execution to trash/delete messages with built-in safety features like dry-run mode.

## Installation

### From Crates.io

```bash
cargo install cull-gmail
```

### From Source

```bash
git clone https://github.com/jerus-org/cull-gmail.git
cd cull-gmail
cargo install --path .
```

### Verify Installation

```bash
cull-gmail --version
```

## Authentication Setup

### 1. Google Cloud Console Setup

1. Visit the [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Enable the Gmail API:
   - Go to "APIs & Services" > "Library"
   - Search for "Gmail API" and enable it
4. Create OAuth2 credentials:
   - Go to "APIs & Services" > "Credentials"
   - Click "Create Credentials" > "OAuth client ID"
   - Choose "Desktop application"
   - Download the JSON file

### 2. Configure cull-gmail

1. Create the configuration directory:
   ```bash
   mkdir -p ~/.cull-gmail
   ```

2. Copy your credential file:
   ```bash
   cp ~/Downloads/client_secret_*.json ~/.cull-gmail/credential.json
   ```

3. Create configuration file `~/.cull-gmail/cull-gmail.toml`:
   ```toml
   credentials = "credential.json"
   config_root = "~/.cull-gmail"
   rules = "rules.toml"
   execute = false  # Start in dry-run mode
   ```

### 3. First Run Authentication

Run any command to trigger the OAuth flow:

```bash
cull-gmail labels
```

This will:
1. Open your browser for Google authentication
2. Prompt you to grant Gmail access
3. Save tokens to `~/.cull-gmail/gmail1/`

## Configuration

### Configuration File

**Location**: `~/.cull-gmail/cull-gmail.toml`

```toml
# OAuth2 credential file (relative to config_root)
credentials = "credential.json"

# Configuration directory
config_root = "~/.cull-gmail"

# Rules file
rules = "rules.toml"

# Default execution mode (false = dry-run, true = execute)
execute = false

# Alternative: Direct OAuth2 configuration
# client_id = "your-client-id.apps.googleusercontent.com"
# client_secret = "your-client-secret"
# token_uri = "https://oauth2.googleapis.com/token"
# auth_uri = "https://accounts.google.com/o/oauth2/auth"
```

### Environment Variables

Override any configuration setting:

```bash
export APP_CREDENTIALS="credential.json"
export APP_EXECUTE="true"
export APP_CLIENT_ID="your-client-id"
export APP_CLIENT_SECRET="your-client-secret"
export APP_CONFIG_ROOT="/custom/config/path"
```

## Command Structure

```bash
cull-gmail [OPTIONS] [COMMAND]
```

### Global Options

- `-v, --verbose...`: Increase logging verbosity (can be used multiple times)
- `-q, --quiet...`: Decrease logging verbosity
- `-h, --help`: Show help
- `-V, --version`: Show version

### Commands

- `labels`: List available Gmail labels
- `messages`: Query and operate on messages
- `rules`: Configure and run retention rules

## Command Reference

### Labels Command

List all labels in your Gmail account:

```bash
cull-gmail labels
```

**Example Output**:
```
INBOX: INBOX
IMPORTANT: IMPORTANT
CHAT: CHAT
SENT: SENT
DRAFT: DRAFT
promotions: Label_1234567890
old-emails: Label_0987654321
```

### Messages Command

Query and operate on Gmail messages.

#### Syntax

```bash
cull-gmail messages [OPTIONS] <ACTION>
```

#### Options

- `-l, --labels <LABELS>`: Filter by labels (can be used multiple times)
- `-m, --max-results <MAX_RESULTS>`: Maximum results per page [default: 200]
- `-p, --pages <PAGES>`: Maximum number of pages (0=all) [default: 1]
- `-Q, --query <QUERY>`: Gmail query string

#### Actions

- `list`: Display message information
- `trash`: Move messages to trash
- `delete`: Permanently delete messages

#### Examples

**List recent messages**:
```bash
cull-gmail messages -m 10 list
```

**List promotional emails older than 6 months**:
```bash
cull-gmail messages -Q "label:promotions older_than:6m" list
```

**Move old promotional emails to trash**:
```bash
cull-gmail messages -Q "label:promotions older_than:1y" trash
```

**Permanently delete very old messages**:
```bash
cull-gmail messages -Q "older_than:5y -label:important" delete
```

**Query with multiple labels**:
```bash
cull-gmail messages -l "promotions" -l "newsletters" -Q "older_than:3m" list
```

**Process all pages (not just first page)**:
```bash
cull-gmail messages -p 0 -Q "older_than:2y" list
```

### Rules Command

Manage retention rules for automated email lifecycle management.

#### Syntax

```bash
cull-gmail rules <SUBCOMMAND>
```

#### Subcommands

- `config`: Configure retention rules
- `run`: Execute configured rules

### Rules Config Command

Configure retention rules:

```bash
cull-gmail rules config <ACTION>
```

#### Config Actions

- `rules`: Manage rule definitions
- `label`: Add/remove labels from rules
- `action`: Set action (trash/delete) on rules

**Example Rules Configuration**:

Create/edit `~/.cull-gmail/rules.toml`:

```toml
[rules."1"]
id = 1
retention = { age = "y:1", generate_label = true }
labels = ["old-emails"]
action = "Trash"

[rules."2"]
id = 2
retention = { age = "m:6", generate_label = true }
labels = ["promotions", "newsletters"]
action = "Trash"

[rules."3"]
id = 3
retention = { age = "y:5", generate_label = true }
labels = ["archive"]
action = "Delete"
```

### Rules Run Command

Execute configured rules:

```bash
cull-gmail rules run [OPTIONS]
```

#### Options

- `-e, --execute`: Actually perform actions (without this, runs in dry-run mode)
- `-t, --skip-trash`: Skip rules with "trash" action
- `-d, --skip-delete`: Skip rules with "delete" action

#### Examples

**Dry-run all rules** (safe, no changes made):
```bash
cull-gmail rules run
```

**Execute all rules**:
```bash
cull-gmail rules run --execute
```

**Execute only delete rules**:
```bash
cull-gmail rules run --execute --skip-trash
```

**Execute only trash rules**:
```bash
cull-gmail rules run --execute --skip-delete
```

## Gmail Query Syntax

The `-Q, --query` option supports Gmail's powerful search syntax:

### Date Queries

```bash
# Relative dates
-Q "older_than:1y"        # Older than 1 year
-Q "newer_than:30d"       # Newer than 30 days
-Q "older_than:6m"        # Older than 6 months

# Absolute dates
-Q "after:2023/1/1"       # After January 1, 2023
-Q "before:2023/12/31"    # Before December 31, 2023
```

### Label Queries

```bash
# Has label
-Q "label:promotions"
-Q "label:important"

# Does NOT have label (note the minus sign)
-Q "-label:important"
-Q "-label:spam"
```

### Content Queries

```bash
# Subject line
-Q "subject:newsletter"
-Q "subject:(unsubscribe OR newsletter)"

# From/To
-Q "from:noreply@example.com"
-Q "to:me@example.com"

# Message content
-Q "unsubscribe"
-Q "has:attachment"
```

### Status Queries

```bash
# Read status
-Q "is:unread"
-Q "is:read"

# Star status
-Q "is:starred"
-Q "-is:starred"

# Size
-Q "size:larger_than:10M"
-Q "size:smaller_than:1M"
```

### Combined Queries

```bash
# Complex combinations
-Q "label:promotions older_than:6m -is:starred"
-Q "from:newsletters@example.com older_than:1y has:attachment"
-Q "subject:newsletter OR subject:promo older_than:3m"
```

## Common Workflows

### 1. Clean Up Promotional Emails

```bash
# Step 1: Preview what will be affected
cull-gmail messages -Q "label:promotions older_than:6m" list

# Step 2: Move to trash (can be recovered for 30 days)
cull-gmail messages -Q "label:promotions older_than:6m" trash
```

### 2. Archive Old Conversations

```bash
# Archive conversations older than 2 years (excluding starred)
cull-gmail messages -Q "older_than:2y -is:starred -label:important" trash
```

### 3. Delete Very Old Messages

```bash
# Permanently delete messages older than 5 years (be careful!)
cull-gmail messages -Q "older_than:5y -is:starred -label:important" delete
```

### 4. Rule-Based Automation

```bash
# Set up rules in ~/.cull-gmail/rules.toml, then:

# Preview what rules will do
cull-gmail rules run

# Execute rules
cull-gmail rules run --execute
```

### 5. Scheduled Cleanup

Add to your crontab for weekly cleanup:

```bash
# Edit crontab
crontab -e

# Add this line (runs every Sunday at 2 AM)
0 2 * * 0 /home/user/.cargo/bin/cull-gmail rules run --execute >> /var/log/cull-gmail.log 2>&1
```

## Safety Features

### Dry-Run Mode

- **Default behaviour**: All operations are dry-run unless explicitly executed
- **Messages**: Use `list` action to preview what would be affected
- **Rules**: Run without `--execute` flag to see what would happen

### Confirmation and Logging

- All operations are logged with detailed information
- Use `-v` for verbose logging to see exactly what's happening
- Check log output before running destructive operations

### Recoverable Operations

- **Trash**: Messages moved to trash can be recovered for 30 days
- **Delete**: Permanent deletion - cannot be undone

## Logging and Debugging

### Environment Variables

```bash
# Set log level
export RUST_LOG=cull_gmail=debug

# Enable all logging
export RUST_LOG=debug
```

### Verbosity Levels

```bash
# Quiet (errors only)
cull-gmail -q messages list

# Normal (default)
cull-gmail messages list

# Verbose (info level)
cull-gmail -v messages list

# Very verbose (debug level)
cull-gmail -vv messages list

# Maximum verbosity (trace level)
cull-gmail -vvv messages list
```

### Log Information

- **Error**: Critical issues
- **Warn**: Non-fatal issues, dry-run notifications
- **Info**: General operation info, message subjects, counts
- **Debug**: Detailed API calls, query strings
- **Trace**: Very detailed debugging information

## Troubleshooting

### Authentication Issues

**Problem**: "Authentication failed" or "Invalid credentials"

**Solutions**:
1. Verify credential file exists and is valid JSON
2. Check OAuth client is configured as "Desktop Application"
3. Clear token cache: `rm -rf ~/.cull-gmail/gmail1`
4. Re-run authentication: `cull-gmail labels`

**Problem**: "Access denied" or "Insufficient permissions"

**Solutions**:
1. Verify Gmail API is enabled in Google Cloud Console
2. Check OAuth scopes include Gmail access
3. Re-authenticate with proper permissions

### Query Issues

**Problem**: "No messages found" when you expect results

**Solutions**:
1. Test query in Gmail web interface first
2. Check label names: `cull-gmail labels`
3. Verify query syntax (no typos)
4. Use `-v` flag to see the actual query being sent

**Problem**: Query returns unexpected results

**Solutions**:
1. Use `messages list` to preview before `trash`/`delete`
2. Check for operator precedence in complex queries
3. Test simpler queries first, then combine

### Performance Issues

**Problem**: Operations are slow or timeout

**Solutions**:
1. Reduce page size: `-m 100`
2. Limit pages: `-p 5` instead of `-p 0`
3. Use more specific queries to reduce result sets
4. Check Gmail API quotas in Google Cloud Console

### Configuration Issues

**Problem**: "Configuration not found" or "Config parse error"

**Solutions**:
1. Verify config file path: `~/.cull-gmail/cull-gmail.toml`
2. Check TOML syntax
3. Ensure credential file path is correct
4. Use absolute paths if relative paths fail

## Exit Codes

- **0**: Success
- **101**: Error (check stderr for details)

## Examples

### Basic Message Management

```bash
# List all labels
cull-gmail labels

# List first 50 messages
cull-gmail messages -m 50 list

# List promotional emails from last year
cull-gmail messages -Q "label:promotions after:2023/1/1" list
```

### Batch Operations

```bash
# Move old promotional emails to trash
cull-gmail messages -Q "label:promotions older_than:1y" trash

# Permanently delete very old messages (careful!)
cull-gmail messages -Q "older_than:5y -is:starred" delete
```

### Rule-Based Management

```bash
# Preview all rules
cull-gmail rules run

# Execute only trash rules
cull-gmail rules run --execute --skip-delete

# Execute all rules
cull-gmail rules run --execute
```

## See Also

- [Library Documentation](lib.md) - Rust API reference and programming examples
- [API Documentation](https://docs.rs/cull-gmail) - Generated API reference
- [Repository](https://github.com/jerus-org/cull-gmail) - Source code, examples, and issue tracking
- [Gmail API Documentation](https://developers.google.com/gmail/api) - Google's official API docs
- [Gmail Search Operators](https://support.google.com/mail/answer/7190?hl=en) - Complete Gmail query syntax
