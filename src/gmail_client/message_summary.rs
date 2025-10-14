#[derive(Debug, Clone)]
pub struct MessageSummary {
    id: String,
    date: Option<String>,
    subject: Option<String>,
}

impl MessageSummary {
    pub(crate) fn new(id: &str) -> Self {
        MessageSummary {
            id: id.to_string(),
            date: None,
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

    pub(crate) fn set_date(&mut self, date: &str) {
        self.date = Some(date.to_string())
    }

    pub(crate) fn date(&self) -> &str {
        if let Some(d) = &self.date {
            d
        } else {
            "*** No Date for Message ***"
        }
    }

    pub(crate) fn list_date_and_subject(&self) -> String {
        let Some(date) = self.date.as_ref() else {
            return "***invalid date or subject***".to_string();
        };

        let Some(subject) = self.subject.as_ref() else {
            return "***invalid date or subject***".to_string();
        };
        let s = date[5..16].to_string();
        let s = format!("{s}: {subject}");
        s
    }
}
