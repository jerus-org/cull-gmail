mod message_age;

pub use message_age::MessageAge;

/// Define retention period and flag to indicate if label should be generated
#[derive(Debug)]
pub struct Retention {
    age: MessageAge,
    generate_label: bool,
}

impl Default for Retention {
    fn default() -> Self {
        Self {
            age: MessageAge::Years(5),
            generate_label: true,
        }
    }
}

impl Retention {
    pub(crate) fn new(age: MessageAge, generate_label: bool) -> Self {
        Retention {
            age,
            generate_label,
        }
    }

    pub(crate) fn age(&self) -> &MessageAge {
        &self.age
    }

    pub(crate) fn generate_label(&self) -> bool {
        self.generate_label
    }
}
