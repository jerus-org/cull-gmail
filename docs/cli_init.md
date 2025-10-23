# cull-gmail init Command

The `init` command provides guided setup for the cull-gmail application, creating the configuration directory, installing OAuth2 credentials, generating default configuration files, and completing the initial Gmail API authentication.

## Overview

The initialization process performs these steps:

1. **Configuration Directory**: Create or verify the configuration directory
2. **Credential Installation**: Copy and validate OAuth2 credential file (if provided)  
3. **Configuration Generation**: Create `cull-gmail.toml` with safe defaults
4. **Rules Template**: Generate `rules.toml` with example retention rules
5. **Token Directory**: Ensure OAuth2 token cache directory exists
6. **Authentication**: Complete OAuth2 flow to generate and persist tokens

## Command Syntax

```bash
cull-gmail init [OPTIONS]
```

## Options

### Required Configuration

- `--config-dir <DIR>`: Configuration directory path (default: `h:.cull-gmail`)
  - Supports path prefixes:
    - `h:path` - Relative to home directory
    - `c:path` - Relative to current directory  
    - `r:path` - Relative to filesystem root
    - `path` - Use path as-is

### OAuth2 Credentials

- `--credential-file <PATH>`: Path to OAuth2 credential JSON file
  - Should be downloaded from Google Cloud Console
  - Must be for Desktop application type
  - Will be copied to config directory as `credential.json`

### Rules Configuration

- `--rules-dir <DIR>`: Optional separate directory for rules.toml file
  - Useful for version controlling rules separately from credentials
  - Supports the same path prefixes as `--config-dir` (h:, c:, r:)
  - If not provided, rules.toml is created in the configuration directory

- `--skip-rules`: Skip rules.toml file creation
  - The rules.toml file will not be created during initialization
  - Useful for ephemeral compute environments where rules.toml is provided externally
  - The cull-gmail.toml will still reference the rules.toml path with a comment
  - If combined with `--rules-dir`, the directory is created but the file is not

### Execution Modes

- `--dry-run`: Preview all planned actions without making changes
  - Shows what files would be created
  - Displays OAuth2 authentication plan
  - Safe to run multiple times

- `--interactive` / `-i`: Enable interactive prompts and confirmations
  - Prompts for missing credential file path
  - Asks for confirmation before overwriting files
  - Recommended for first-time users

- `--force`: Overwrite existing files without prompting
  - Creates timestamped backups (e.g., `config.bak-20231201120000`)
  - Use with caution as it replaces existing configuration

## Usage Examples

### Basic Setup

```bash
# Basic initialization with default settings
cull-gmail init

# Custom configuration directory
cull-gmail init --config-dir /custom/path
```

### Interactive Setup

```bash
# Interactive setup (recommended for first use)
cull-gmail init --interactive

# Interactive with credential file
cull-gmail init --interactive --credential-file ~/Downloads/client_secret.json
```

### Planning and Preview

```bash
# Preview what would be created
cull-gmail init --dry-run

# Preview with specific options
cull-gmail init --config-dir ~/.cull-gmail --credential-file credentials.json --dry-run
```

### Force Overwrite

```bash
# Recreate configuration with backups
cull-gmail init --force

# Force with specific credential file
cull-gmail init --force --credential-file new_credentials.json
```

### Ephemeral Environments

```bash
# Skip rules.toml creation when it's provided externally
cull-gmail init --skip-rules

# Skip rules with dry-run to preview
cull-gmail init --skip-rules --dry-run

# Skip rules with custom rules directory
cull-gmail init --skip-rules --rules-dir /mnt/rules

# Skip rules with custom config directory (e.g., in a container)
cull-gmail init --skip-rules --config-dir /app/config
```

When using `--skip-rules`:

- The `rules.toml` file is **not created** during initialization
- The `cull-gmail.toml` file includes a comment indicating that `rules.toml` should be provided externally
- The `rules = "rules.toml"` line (or custom path if `--rules-dir` is used) remains in the configuration
- If `--rules-dir` is specified, the rules directory is still created, but without the `rules.toml` file
- This is ideal for:
  - Docker/Kubernetes environments where rules.toml is mounted as a volume
  - CI/CD pipelines that provide rules.toml from version control
  - Configuration management systems that supply rules.toml separately

## File Structure Created

The init command creates the following structure:

```
~/.cull-gmail/                  # Configuration directory
├── cull-gmail.toml             # Main configuration
├── rules.toml                  # Retention rules template  
├── credential.json             # OAuth2 credentials (if provided)
└── gmail1/                     # OAuth2 token cache
    └── [token files]           # Generated after OAuth2 flow
```

### Configuration File (`cull-gmail.toml`)

```toml
# cull-gmail configuration
# This file configures the cull-gmail application.

# OAuth2 credential file (relative to config_root)
credential_file = "credential.json"

# Configuration root directory  
config_root = "h:.cull-gmail"

# Rules configuration file
rules = "rules.toml"

# Default execution mode (false = dry-run, true = execute)
# Set to false for safety - you can override with --execute flag
execute = false

# Environment variable name for token cache (for ephemeral environments)
token_cache_env = "CULL_GMAIL_TOKEN_CACHE"
```

### Rules File (`rules.toml`)

```toml
# Example rules for cull-gmail
# Each rule targets a Gmail label and specifies an action.
# 
# Actions:
#   - "Trash" is recoverable (messages go to Trash folder ~30 days)
#   - "Delete" is irreversible (messages are permanently deleted)
#
# Time formats:
#   - "older_than:30d" (30 days)
#   - "older_than:6m" (6 months) 
#   - "older_than:2y" (2 years)
#
# Example rule for promotional emails:
# [[rules]]
# id = 1
# label = "Promotions"
# query = "category:promotions older_than:30d"
# action = "Trash"
#
# Example rule for old newsletters:
# [[rules]]
# id = 2
# label = "Updates"
# query = "category:updates older_than:90d"
# action = "Trash"
#
# Uncomment and modify the examples above to create your own rules.
# Run 'cull-gmail rules run --dry-run' to test rules before execution.
```

## OAuth2 Setup Guide

### 1. Google Cloud Console Setup

1. Visit [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Enable the Gmail API:
   - Go to "APIs & Services" > "Library"
   - Search for "Gmail API" and enable it
4. Create OAuth2 credentials:
   - Go to "APIs & Services" > "Credentials"
   - Click "Create Credentials" > "OAuth client ID"
   - Choose "Desktop application"
   - Download the JSON file

### 2. OAuth2 Authentication Flow

When you run `cull-gmail init` with a credential file:

1. The credential file is validated and copied securely
2. A web browser opens for Google authentication
3. You grant Gmail access permissions
4. Tokens are automatically saved to the `gmail1/` directory
5. Future runs use the cached tokens (no browser needed)

### 3. Troubleshooting OAuth2

**Error: Invalid credential file format**
- Ensure you downloaded the JSON file for "Desktop application"
- Mobile or web application credentials won't work

**Error: Gmail API not enabled**
- Return to Google Cloud Console
- Enable the Gmail API for your project

**Error: OAuth2 authentication failed**
- Check your internet connection
- Verify the credential file is not corrupted
- Try re-downloading credentials from Google Cloud Console

**Error: Redirect URI mismatch**
- Desktop application credentials should work automatically
- If issues persist, check the redirect URIs in Google Cloud Console

## Security Considerations

### File Permissions

The init command sets secure permissions on created files:

- **Configuration files**: `0644` (owner read/write, others read)
- **Credential files**: `0600` (owner read/write only)  
- **Token directory**: `0700` (owner access only)

### Backup Safety

When using `--force` or accepting overwrites in `--interactive` mode:

- Existing files are backed up with timestamps
- Backup format: `filename.bak-YYYYmmddHHMMSS`
- Original files are preserved until manually removed

### Credential Handling

- Credential files are validated before copying
- Files are copied with restricted permissions
- OAuth2 tokens are stored securely in the token cache directory

## Next Steps After Initialization

After successful initialization:

```bash
# 1. Test Gmail connection
cull-gmail labels

# 2. Review the rules template
cull-gmail rules run --dry-run

# 3. Customize rules.toml as needed
# Edit ~/.cull-gmail/rules.toml

# 4. Test your rules safely
cull-gmail rules run --dry-run

# 5. Execute rules for real (when ready)
cull-gmail rules run --execute
```

## Error Handling

### Common Error Scenarios

**Configuration Already Exists**
```bash
# Error message shown
File I/O error: Configuration file already exists: ~/.cull-gmail/cull-gmail.toml
Use --force to overwrite or --interactive for prompts

# Solutions
cull-gmail init --force                    # Overwrite with backup
cull-gmail init --interactive             # Prompt for each conflict
```

**Missing Credential File**
```bash
# Error message shown  
File I/O error: Credential file not found: /path/to/file.json

# Solution
# Ensure the file path is correct and the file exists
cull-gmail init --credential-file /correct/path/to/file.json
```

**Permission Errors**
```bash
# Error message shown
File I/O error: Failed to create directory: Permission denied

# Solutions
# Ensure you have write permission to the target directory
# Or choose a different config directory
cull-gmail init --config-dir ~/my-cull-gmail
```

## Integration with Other Commands

The `init` command integrates seamlessly with other cull-gmail features:

### Token Export/Import

```bash
# After initialization, export tokens for ephemeral environments
cull-gmail token export > my-tokens.env

# In another environment, set the token cache
export CULL_GMAIL_TOKEN_CACHE="$(cat my-tokens.env)"
cull-gmail labels  # Uses imported tokens
```

### Rules Management

```bash
# After initialization, manage rules
cull-gmail rules config rules add
cull-gmail rules config label add 1 "old-emails"
cull-gmail rules config action 1 trash
```

### Message Operations

```bash
# After initialization, work with messages
cull-gmail messages -l "Promotions" list
cull-gmail messages -Q "older_than:30d" trash --dry-run
```

This comprehensive setup makes cull-gmail ready for automated Gmail message management with full OAuth2 authentication and secure configuration handling.