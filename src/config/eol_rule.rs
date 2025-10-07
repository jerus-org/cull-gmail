use serde::{Deserialize, Serialize};

use crate::{Retention, eol_cmd::EolCmd};

/// End of life rules
#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct EolRule {
    id: usize,
    retention: String,
    labels: Vec<String>,
    query: Option<String>,
    command: String,
}

impl EolRule {
    pub(crate) fn new(id: usize) -> Self {
        EolRule {
            id,
            command: EolCmd::Trash.to_string(),
            ..Default::default()
        }
    }

    pub(crate) fn set_retention(&mut self, value: Retention) -> &mut Self {
        self.retention = value.age().to_string();
        if value.generate_label() {
            self.add_label(&value.age().label());
        }
        self
    }

    pub(crate) fn add_label(&mut self, value: &str) -> &mut Self {
        self.labels.push(value.to_string());
        self
    }

    pub(crate) fn id(&self) -> usize {
        self.id
    }
}
