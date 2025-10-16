use std::{env, fmt::Display};

use lazy_regex::{Lazy, Regex, lazy_regex};

static ROOT_CONFIG: Lazy<Regex> = lazy_regex!(r"^(?P<class>[hrc]):(?P<path>.+)$");

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
        log::debug!("parsing the string `{s}`");
        let Some(captures) = ROOT_CONFIG.captures(s) else {
            return ConfigRoot::None;
        };
        log::debug!("found captures `{captures:?}`");

        let path = String::from(if let Some(p) = captures.name("path") {
            p.as_str()
        } else {
            ""
        });
        log::debug!("set the path to `{path}`");

        let Some(class) = captures.name("class") else {
            return ConfigRoot::None;
        };
        log::debug!("found the class `{class:?}`");

        match class.as_str() {
            "h" => ConfigRoot::Home(path),
            "r" => ConfigRoot::Root(path),
            "c" => ConfigRoot::Crate(path),
            _ => unreachable!(),
        }
    }
}

enum ConfigRootError {}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    use crate::test_utils::get_test_logger;

    #[test]
    fn test_parse_to_home() {
        get_test_logger();
        let input = "h:.cull-gmail".to_string();
        log::debug!("Input set to: `{input}`");
        let dir_part = input[2..].to_string();

        let user_home = env::home_dir().unwrap();

        let expected = user_home.join(dir_part).display().to_string();

        assert_eq!(expected, ConfigRoot::parse(&input).to_string());
    }

    #[test]
    fn test_parse_to_root() {
        get_test_logger();
        let input = "r:.cull-gmail".to_string();
        log::debug!("Input set to: `{input}`");
        let dir_part = input[2..].to_string();

        let expected = format!("/{dir_part}");

        assert_eq!(expected, ConfigRoot::parse(&input).to_string());
    }

    #[test]
    fn test_parse_to_crate() {
        get_test_logger();
        let input = "c:.cull-gmail".to_string();
        log::debug!("Input set to: `{input}`");
        let expected = input[2..].to_string();

        assert_eq!(expected, ConfigRoot::parse(&input).to_string());
    }

    #[test]
    fn test_parse_to_none() {
        get_test_logger();
        let input = ".cull-gmail".to_string();
        log::debug!("Input set to: `{input}`");

        let expected = "".to_string();

        assert_eq!(expected, ConfigRoot::parse(&input).to_string());
    }
}
