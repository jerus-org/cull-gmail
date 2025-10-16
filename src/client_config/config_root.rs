use std::{env, fmt::Display};

#[derive(Debug, Default)]
pub enum ConfigRoot {
    #[default]
    None,
    Crate(String),
    Home(String),
    Root(String),
}

impl Display for ConfigRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigRoot::None => write!(f, ""),
            ConfigRoot::Crate(path) => write!(f, "{path}"),
            ConfigRoot::Home(path) => {
                let pb = path.trim_start_matches("/");
                write!(f, "{}/{}", env::home_dir().unwrap().display(), pb)
            }
            ConfigRoot::Root(path) => {
                let pb = path.trim_start_matches("/");
                write!(f, "/{pb}")
            }
        }
    }
}

impl ConfigRoot {
    pub fn parse(s: &str) -> Self {
        if !s.is_empty() {
            match s.chars().nth(0) {
                Some('h') => ConfigRoot::Home(s.to_string()),
                Some('r') => ConfigRoot::Root(s.to_string()),
                Some('c') => ConfigRoot::Crate(s.to_string()),
                Some(_) => ConfigRoot::Crate(s.to_string()),
                None => ConfigRoot::None,
            }
        } else {
            ConfigRoot::None
        }
    }
}
