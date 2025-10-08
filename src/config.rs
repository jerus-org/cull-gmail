use std::{
    collections::BTreeMap,
    env,
    fs::{self, read_to_string},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

mod eol_rule;

use eol_rule::EolRule;

use crate::{Error, MessageAge, Retention, eol_cmd::EolAction};

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

    /// Add a new rule to the rule set by setting the retention age
    pub fn add_rule(
        &mut self,
        retention: Retention,
        label: Option<&String>,
        delete: bool,
    ) -> &mut Self {
        if self.rules.contains_key(&retention.age().to_string()) && label.is_none() {
            log::warn!("rule already exists");
            return self;
        }

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
        self.rules.insert(rule.retention().to_string(), rule);
        self
    }

    /// Save the current configuration to the file
    pub fn save(&self) -> Result<(), Error> {
        let home_dir = env::home_dir().unwrap();
        let path = PathBuf::new()
            .join(home_dir)
            .join(".cull-gmail/cull-gmail.toml");

        if let Ok(output) = toml::to_string(self) {
            fs::write(path, output)?;
        }

        Ok(())
    }

    /// Load the current configuration
    pub fn load() -> Result<Config, Error> {
        let home_dir = env::home_dir().unwrap();
        let path = PathBuf::new()
            .join(home_dir)
            .join(".cull-gmail/cull-gmail.toml");

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
    pub fn list_rules(&self) -> Result<(), Error> {
        for rule in self.rules.values() {
            println!("{rule}");
        }
        Ok(())
    }
}
