#[derive(Debug, Clone)]
pub struct MessageSummary {
    id: String,
    subject: Option<String>,
}

impl MessageSummary {
    pub(crate) fn new(id: &str) -> Self {
        MessageSummary {
            id: id.to_string(),
            subject: None,
        }
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn set_subject(&mut self, subject: &str) {
        self.subject = Some(subject.to_string())
    }

    pub(crate) fn subject(&self) -> &str {
        if let Some(s) = &self.subject {
            s
        } else {
            "*** No Subject for Message ***"
        }
    }
}
