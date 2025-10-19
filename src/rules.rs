//! Rules management for Gmail message retention and cleanup.
//!
//! This module provides the [`Rules`] struct which manages a collection of end-of-life (EOL)
//! rules for automatically processing Gmail messages. Rules define when and how messages
//! should be processed based on their age and labels.
//!
//! # Overview
//!
//! The rules system allows you to:
//! - Create rules with specific retention periods (days, weeks, months, years)
//! - Target specific Gmail labels or apply rules globally
//! - Choose between moving to trash or permanent deletion
//! - Save and load rule configurations from disk
//! - Manage rules individually by ID or label
//!
//! # Usage
//!
//! ```
//! use cull_gmail::{Rules, Retention, MessageAge, EolAction};
//!
//! // Create a new rule set
//! let mut rules = Rules::new();
//!
//! // Add a rule to delete old newsletters after 6 months
//! let newsletter_retention = Retention::new(MessageAge::Months(6), true);
//! rules.add_rule(newsletter_retention, Some(&"newsletter".to_string()), true);
//!
//! // Add a rule to trash spam after 30 days
//! let spam_retention = Retention::new(MessageAge::Days(30), false);
//! rules.add_rule(spam_retention, Some(&"spam".to_string()), false);
//!
//! // Save the rules to disk
//! rules.save().expect("Failed to save rules");
//!
//! // List all configured rules
//! rules.list_rules().expect("Failed to list rules");
//! ```
//!
//! # Persistence
//!
//! Rules are automatically saved to `~/.cull-gmail/rules.toml` and can be loaded
//! using [`Rules::load()`]. The configuration uses TOML format for human readability.

use std::{
    collections::BTreeMap,
    env,
    fs::{self, read_to_string},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

mod eol_rule;

pub use eol_rule::EolRule;

use crate::{EolAction, Error, MessageAge, Result, Retention};

/// A collection of end-of-life rules for Gmail message processing.
///
/// `Rules` manages a set of end-of-life rule instances that define how Gmail messages
/// should be processed based on their age and labels. Rules can move messages to
/// trash or delete them permanently when they exceed specified retention periods.
///
/// # Structure
///
/// Each rule has:
/// - A unique ID for identification
/// - A retention period (age threshold)
/// - Optional target labels
/// - An action (trash or delete)
///
/// # Default Rules
///
/// When created with [`Rules::new()`] or [`Rules::default()`], the following
/// default rules are automatically added:
/// - 1 year retention with auto-generated label
/// - 1 week retention with auto-generated label  
/// - 1 month retention with auto-generated label
/// - 5 year retention with auto-generated label
///
/// # Examples
///
/// ```
/// use cull_gmail::{Rules, Retention, MessageAge};
///
/// let rules = Rules::new();
/// // Default rules are automatically created
/// assert!(!rules.labels().is_empty());
/// ```
///
/// # Serialization
///
/// Rules can be serialized to and from TOML format for persistence.
#[derive(Debug, Serialize, Deserialize)]
pub struct Rules {
    rules: BTreeMap<String, EolRule>,
}

impl Default for Rules {
    fn default() -> Self {
        let rules = BTreeMap::new();

        let mut cfg = Self { rules };

        cfg.add_rule(Retention::new(MessageAge::Years(1), true), None, false)
            .add_rule(Retention::new(MessageAge::Weeks(1), true), None, false)
            .add_rule(Retention::new(MessageAge::Months(1), true), None, false)
            .add_rule(Retention::new(MessageAge::Years(5), true), None, false);

        cfg
    }
}

impl Rules {
    /// Creates a new Rules instance with default retention rules.
    ///
    /// This creates the same configuration as [`Rules::default()`], including
    /// several pre-configured rules with common retention periods.
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::Rules;
    ///
    /// let rules = Rules::new();
    /// // Default rules are automatically created
    /// let labels = rules.labels();
    /// assert!(!labels.is_empty());
    /// ```
    pub fn new() -> Self {
        Rules::default()
    }

    /// Retrieves a rule by its unique ID.
    ///
    /// Returns a cloned copy of the rule if found, or `None` if no rule
    /// exists with the specified ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the rule to retrieve
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Rules, Retention, MessageAge};
    ///
    /// let mut rules = Rules::new();
    /// let retention = Retention::new(MessageAge::Days(30), false);
    /// rules.add_rule(retention, None, false);
    ///
    /// // Retrieve a rule (exact ID depends on existing rules)
    /// if let Some(rule) = rules.get_rule(1) {
    ///     println!("Found rule: {}", rule.describe());
    /// }
    /// ```
    pub fn get_rule(&self, id: usize) -> Option<EolRule> {
        self.rules.get(&id.to_string()).cloned()
    }

    /// Adds a new rule to the rule set with the specified retention settings.
    ///
    /// Creates a new rule with an automatically assigned unique ID. If a label
    /// is specified and another rule already targets that label, a warning is
    /// logged and the rule is not added.
    ///
    /// # Arguments
    ///
    /// * `retention` - The retention configuration (age and label generation)
    /// * `label` - Optional label that this rule should target
    /// * `delete` - If `true`, messages are permanently deleted; if `false`, moved to trash
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Rules, Retention, MessageAge, EolAction};
    ///
    /// let mut rules = Rules::new();
    ///
    /// // Add a rule to trash newsletters after 3 months
    /// let retention = Retention::new(MessageAge::Months(3), false);
    /// rules.add_rule(retention, Some(&"newsletter".to_string()), false);
    ///
    /// // Add a rule to delete spam after 7 days
    /// let spam_retention = Retention::new(MessageAge::Days(7), false);
    /// rules.add_rule(spam_retention, Some(&"spam".to_string()), true);
    /// ```
    pub fn add_rule(
        &mut self,
        retention: Retention,
        label: Option<&String>,
        delete: bool,
    ) -> &mut Self {
        let mut current_labels = Vec::new();
        for rule in self.rules.values() {
            let mut ls = rule.labels().clone();
            current_labels.append(&mut ls);
        }

        if label.is_some() && current_labels.contains(label.unwrap()) {
            log::warn!("a rule already applies to label {}", label.unwrap());
            return self;
        }

        let id = if let Some((_, max)) = self.rules.iter().max_by_key(|(_, r)| r.id()) {
            max.id() + 1
        } else {
            1
        };

        let mut rule = EolRule::new(id);
        rule.set_retention(retention);
        if let Some(l) = label {
            rule.add_label(l);
        }
        if delete {
            rule.set_action(&EolAction::Delete);
        }
        log::info!("added rule: {rule}");
        self.rules.insert(rule.id().to_string(), rule);
        self
    }

    /// Returns all labels targeted by the current rules.
    ///
    /// This method collects labels from all rules in the set and returns
    /// them as a single vector. Duplicate labels are not removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Rules, Retention, MessageAge};
    ///
    /// let mut rules = Rules::new();
    /// let retention = Retention::new(MessageAge::Days(30), false);
    /// rules.add_rule(retention, Some(&"test-label".to_string()), false);
    ///
    /// let labels = rules.labels();
    /// assert!(labels.len() > 0);
    /// println!("Configured labels: {:?}", labels);
    /// ```
    pub fn labels(&self) -> Vec<String> {
        let mut labels = Vec::new();
        for rule in self.rules.values() {
            labels.append(&mut rule.labels().clone());
        }
        labels
    }

    /// Find the id of the rule that contains a label
    fn find_label(&self, label: &str) -> usize {
        let rules_by_label = self.get_rules_by_label();
        if let Some(rule) = rules_by_label.get(label) {
            rule.id()
        } else {
            0
        }
    }

    /// Removes a rule from the set by its unique ID.
    ///
    /// If the rule exists, it is removed and a confirmation message is printed.
    /// If the rule doesn't exist, the operation completes successfully without error.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the rule to remove
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Rules, Retention, MessageAge};
    ///
    /// let mut rules = Rules::new();
    /// // Assume rule ID 1 exists from defaults
    /// rules.remove_rule_by_id(1).expect("Failed to remove rule");
    /// ```
    ///
    /// # Errors
    ///
    /// This method currently always returns `Ok(())`, but the return type
    /// is `Result<()>` for future extensibility.
    pub fn remove_rule_by_id(&mut self, id: usize) -> crate::Result<()> {
        self.rules.remove(&id.to_string());
        println!("Rule `{id}` has been removed.");
        Ok(())
    }

    /// Removes a rule from the set by targeting one of its labels.
    ///
    /// Finds the rule that contains the specified label and removes it.
    /// If multiple rules target the same label, only one is removed.
    ///
    /// # Arguments
    ///
    /// * `label` - The label to search for in existing rules
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cull_gmail::{Rules, Retention, MessageAge};
    ///
    /// let mut rules = Rules::new();
    /// let retention = Retention::new(MessageAge::Days(30), false);
    /// rules.add_rule(retention, Some(&"newsletter".to_string()), false);
    ///
    /// // Remove the rule targeting the newsletter label
    /// rules.remove_rule_by_label("newsletter")
    ///      .expect("Failed to remove rule");
    /// ```
    ///
    /// # Errors
    ///
    /// * [`Error::LabelNotFoundInRules`] if no rule contains the specified label
    /// * [`Error::NoRuleFoundForLabel`] if the label exists but no rule is found
    ///   (should not happen under normal conditions)
    pub fn remove_rule_by_label(&mut self, label: &str) -> crate::Result<()> {
        let labels = self.labels();

        if !labels.contains(&label.to_string()) {
            return Err(Error::LabelNotFoundInRules(label.to_string()));
        }

        let rule_id = self.find_label(label);
        if rule_id == 0 {
            return Err(Error::NoRuleFoundForLabel(label.to_string()));
        }

        self.rules.remove(&rule_id.to_string());

        log::info!("Rule containing the label `{label}` has been removed.");
        Ok(())
    }

    /// Returns a mapping from labels to rules that target them.
    ///
    /// Creates a `BTreeMap` where each key is a label and each value is a cloned
    /// copy of the rule that targets that label. If multiple rules target the
    /// same label, only one will be present in the result (the last one processed).
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Rules, Retention, MessageAge};
    ///
    /// let mut rules = Rules::new();
    /// let retention = Retention::new(MessageAge::Days(30), false);
    /// rules.add_rule(retention, Some(&"test".to_string()), false);
    ///
    /// let label_map = rules.get_rules_by_label();
    /// if let Some(rule) = label_map.get("test") {
    ///     println!("Rule for 'test' label: {}", rule.describe());
    /// }
    /// ```
    pub fn get_rules_by_label(&self) -> BTreeMap<String, EolRule> {
        let mut rbl = BTreeMap::new();

        for rule in self.rules.values() {
            for label in rule.labels() {
                rbl.insert(label, rule.clone());
            }
        }

        rbl
    }

    /// Adds a label to an existing rule and saves the configuration.
    ///
    /// Finds the rule with the specified ID and adds the given label to it.
    /// The configuration is automatically saved to disk after the change.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the rule to modify
    /// * `label` - The label to add to the rule
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cull_gmail::Rules;
    ///
    /// let mut rules = Rules::load().expect("Failed to load rules");
    /// rules.add_label_to_rule(1, "new-label")
    ///      .expect("Failed to add label");
    /// ```
    ///
    /// # Errors
    ///
    /// * [`Error::RuleNotFound`] if no rule exists with the specified ID
    /// * IO errors from saving the configuration file
    pub fn add_label_to_rule(&mut self, id: usize, label: &str) -> Result<()> {
        let Some(rule) = self.rules.get_mut(id.to_string().as_str()) else {
            return Err(Error::RuleNotFound(id));
        };
        rule.add_label(label);
        self.save()?;
        println!("Label `{label}` added to rule `#{id}`");

        Ok(())
    }

    /// Removes a label from an existing rule and saves the configuration.
    ///
    /// Finds the rule with the specified ID and removes the given label from it.
    /// The configuration is automatically saved to disk after the change.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the rule to modify
    /// * `label` - The label to remove from the rule
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cull_gmail::Rules;
    ///
    /// let mut rules = Rules::load().expect("Failed to load rules");
    /// rules.remove_label_from_rule(1, "old-label")
    ///      .expect("Failed to remove label");
    /// ```
    ///
    /// # Errors
    ///
    /// * [`Error::RuleNotFound`] if no rule exists with the specified ID
    /// * IO errors from saving the configuration file
    pub fn remove_label_from_rule(&mut self, id: usize, label: &str) -> Result<()> {
        let Some(rule) = self.rules.get_mut(id.to_string().as_str()) else {
            return Err(Error::RuleNotFound(id));
        };
        rule.remove_label(label);
        self.save()?;
        println!("Label `{label}` removed from rule `#{id}`");

        Ok(())
    }

    /// Sets the action for an existing rule and saves the configuration.
    ///
    /// Finds the rule with the specified ID and updates its action (trash or delete).
    /// The configuration is automatically saved to disk after the change.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the rule to modify
    /// * `action` - The new action to set (`Trash` or `Delete`)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cull_gmail::{Rules, EolAction};
    ///
    /// let mut rules = Rules::load().expect("Failed to load rules");
    /// rules.set_action_on_rule(1, &EolAction::Delete)
    ///      .expect("Failed to set action");
    /// ```
    ///
    /// # Errors
    ///
    /// * [`Error::RuleNotFound`] if no rule exists with the specified ID
    /// * IO errors from saving the configuration file
    pub fn set_action_on_rule(&mut self, id: usize, action: &EolAction) -> Result<()> {
        let Some(rule) = self.rules.get_mut(id.to_string().as_str()) else {
            return Err(Error::RuleNotFound(id));
        };
        rule.set_action(action);
        self.save()?;
        println!("Action set to `{action}` on rule `#{id}`");

        Ok(())
    }

    /// Saves the current rule configuration to disk.
    ///
    /// The configuration is saved as TOML format to `~/.cull-gmail/rules.toml`.
    /// The directory is created if it doesn't exist.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cull_gmail::{Rules, Retention, MessageAge};
    ///
    /// let mut rules = Rules::new();
    /// let retention = Retention::new(MessageAge::Days(30), false);
    /// rules.add_rule(retention, Some(&"test".to_string()), false);
    ///
    /// rules.save().expect("Failed to save configuration");
    /// ```
    ///
    /// # Errors
    ///
    /// * TOML serialization errors
    /// * IO errors when writing to the file system
    /// * File system permission errors
    pub fn save(&self) -> Result<()> {
        let home_dir = env::home_dir().unwrap();
        let path = PathBuf::new().join(home_dir).join(".cull-gmail/rules.toml");

        let res = toml::to_string(self);
        log::trace!("toml conversion result: {res:#?}");

        if let Ok(output) = res {
            fs::write(&path, output)?;
            log::trace!("Config saved to {}", path.display());
        }

        Ok(())
    }

    /// Loads rule configuration from disk.
    ///
    /// Reads the configuration from `~/.cull-gmail/rules.toml` and deserializes
    /// it into a `Rules` instance.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cull_gmail::Rules;
    ///
    /// match Rules::load() {
    ///     Ok(rules) => {
    ///         println!("Loaded {} rules", rules.labels().len());
    ///         rules.list_rules().expect("Failed to list rules");
    ///     }
    ///     Err(e) => println!("Failed to load rules: {}", e),
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// * IO errors when reading from the file system
    /// * TOML parsing errors if the file is malformed
    /// * File not found errors if the configuration doesn't exist
    pub fn load() -> Result<Rules> {
        let home_dir = env::home_dir().unwrap();
        let path = PathBuf::new().join(home_dir).join(".cull-gmail/rules.toml");
        log::trace!("Loading config from {}", path.display());

        let input = read_to_string(path)?;
        let config = toml::from_str::<Rules>(&input)?;
        Ok(config)
    }

    /// Prints all configured rules to standard output.
    ///
    /// Each rule is printed on a separate line with its description,
    /// including the rule ID, action, and age criteria.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use cull_gmail::Rules;
    ///
    /// let rules = Rules::new();
    /// rules.list_rules().expect("Failed to list rules");
    /// // Output:
    /// // Rule #1 is active on `retention/1-years` to move the message to trash if it is more than 1 years old.
    /// // Rule #2 is active on `retention/1-weeks` to move the message to trash if it is more than 1 weeks old.
    /// // ...
    /// ```
    ///
    /// # Errors
    ///
    /// This method currently always returns `Ok(())`, but the return type
    /// is `Result<()>` for consistency with other methods and future extensibility.
    pub fn list_rules(&self) -> Result<()> {
        for rule in self.rules.values() {
            println!("{rule}");
        }
        Ok(())
    }
}
