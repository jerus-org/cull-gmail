use std::{
    collections::BTreeMap,
    env,
    fs::{self, read_to_string},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

mod eol_rule;

use eol_rule::EolRule;

use crate::{Error, MessageAge, Result, Retention, eol_cmd::EolAction};

/// Configuration file for the program
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    credentials: Option<String>,
    rules: BTreeMap<String, EolRule>,
}

impl Default for Config {
    fn default() -> Self {
        let rules = BTreeMap::new();

        let mut cfg = Self {
            credentials: Some("credential.json".to_string()),
            rules,
        };

        cfg.add_rule(Retention::new(MessageAge::Years(1), true), None, false)
            .add_rule(Retention::new(MessageAge::Weeks(1), true), None, false)
            .add_rule(Retention::new(MessageAge::Months(1), true), None, false)
            .add_rule(Retention::new(MessageAge::Years(5), true), None, false);

        cfg
    }
}

impl Config {
    /// Create a new configuration file
    pub fn new() -> Self {
        Config::default()
    }

    /// Set a name for the credentials file
    pub fn set_credentials(&mut self, file_name: &str) -> &mut Self {
        self.credentials = Some(file_name.to_string());
        self
    }

    /// Get the contents of an existing rule
    pub fn get_rule(&self, id: usize) -> Option<EolRule> {
        self.rules.get(&id.to_string()).cloned()
    }

    /// Add a new rule to the rule set by setting the retention age
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
            rule.set_command(EolAction::Delete);
        }
        log::info!("added rule: {rule}");
        self.rules.insert(rule.id().to_string(), rule);
        self
    }

    /// Get the labels from the rules
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

    /// Remove a rule by the ID specified
    pub fn remove_rule_by_id(&mut self, id: usize) -> crate::Result<()> {
        self.rules.remove(&id.to_string());
        println!("Rule `{id}` has been removed.");
        Ok(())
    }

    /// Remove a rule by the Label specified
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

    fn get_rules_by_label(&self) -> BTreeMap<String, EolRule> {
        let mut rbl = BTreeMap::new();

        for rule in self.rules.values() {
            for label in rule.labels() {
                rbl.insert(label, rule.clone());
            }
        }

        rbl
    }

    /// Add a label to the rule identified by the id
    pub fn add_label_to_rule(&mut self, id: usize, label: &str) -> Result<()> {
        let Some(rule) = self.rules.get_mut(id.to_string().as_str()) else {
            return Err(Error::RuleNotFound(id));
        };
        rule.add_label(label);
        self.save()?;
        println!("Label `{label}` added to rule `#{id}`");

        Ok(())
    }

    /// Remove a label from the rule identified by the id
    pub fn remove_label_from_rule(&mut self, id: usize, label: &str) -> Result<()> {
        let Some(rule) = self.rules.get_mut(id.to_string().as_str()) else {
            return Err(Error::RuleNotFound(id));
        };
        rule.remove_label(label);
        self.save()?;
        println!("Label `{label}` removed from rule `#{id}`");

        Ok(())
    }

    /// Save the current configuration to the file
    pub fn save(&self) -> Result<()> {
        let home_dir = env::home_dir().unwrap();
        let path = PathBuf::new()
            .join(home_dir)
            .join(".cull-gmail/cull-gmail.toml");

        let res = toml::to_string(self);
        log::trace!("toml conversion result: {res:#?}");

        if let Ok(output) = res {
            fs::write(&path, output)?;
            log::trace!("Config saved to {}", path.display());
        }

        Ok(())
    }

    /// Load the current configuration
    pub fn load() -> Result<Config> {
        let home_dir = env::home_dir().unwrap();
        let path = PathBuf::new()
            .join(home_dir)
            .join(".cull-gmail/cull-gmail.toml");
        log::trace!("Loading config from {}", path.display());

        let input = read_to_string(path)?;
        let config = toml::from_str::<Config>(&input)?;
        Ok(config)
    }

    /// Return the credential file name
    pub fn credential_file(&self) -> &str {
        if let Some(file) = &self.credentials {
            file
        } else {
            ""
        }
    }

    /// List the end of life rules set in the configuration
    pub fn list_rules(&self) -> Result<()> {
        for rule in self.rules.values() {
            println!("{rule}");
        }
        Ok(())
    }
}
