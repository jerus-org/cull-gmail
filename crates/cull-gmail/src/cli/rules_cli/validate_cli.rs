//! Validation subcommand for Gmail retention rules.
//!
//! Loads a rules file and checks each rule for correctness without
//! executing any actions or making API calls. Exits with a non-zero
//! status if any issues are found.

use clap::Parser;
use std::path::Path;

use cull_gmail::Rules;

use crate::Result;

/// Validate a rules file without executing any actions.
///
/// Checks each rule for:
/// - Non-empty label set
/// - Valid retention period (e.g. `d:30`, `m:6`, `y:2`)
/// - Valid action (`Trash` or `Delete`)
///
/// Also checks across rules for duplicate labels.
///
/// Exits 0 if all rules are valid, non-zero otherwise.
#[derive(Debug, Parser)]
pub struct ValidateCli {}

impl ValidateCli {
    pub fn run(&self, rules_path: Option<&Path>) -> Result<()> {
        let rules = Rules::load_from(rules_path)?;
        let issues = rules.validate();

        if issues.is_empty() {
            println!("Rules are valid.");
            Ok(())
        } else {
            for issue in &issues {
                eprintln!("{issue}");
            }
            Err(cull_gmail::Error::FileIo(format!(
                "{} validation issue(s) found",
                issues.len()
            )))
        }
    }
}
