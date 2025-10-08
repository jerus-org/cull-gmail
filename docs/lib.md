## cull-gmail Library Documentation

The `cull-gmail` library provides types to enable the culling of emails using the Gmail API including the following steps:
- login to get authorization
- backup the mailbox 
- filtered lists of the contents
- move email matching a filtered list to trash

### Installation

Add the library to your program's `Cargo.toml` using `cargo add`:

```bash
$ cargo add cull-gmail
```

Or by configuring the dependencies manually in `Cargo.toml`:

```toml
[dependencies]
cull-gmail = "0.0.5"
```

