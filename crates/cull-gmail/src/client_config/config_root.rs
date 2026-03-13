use std::{env, fmt::Display, path::PathBuf};

use lazy_regex::{Lazy, Regex, lazy_regex};

static ROOT_CONFIG: Lazy<Regex> = lazy_regex!(r"^(?P<class>[hrc]):(?P<path>.+)$");

#[derive(Debug, Default, Clone)]
pub enum RootBase {
    #[default]
    None,
    Crate,
    Home,
    Root,
}

impl Display for RootBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RootBase::None => write!(f, ""),
            RootBase::Crate => write!(f, "c:"),
            RootBase::Home => write!(f, "h:"),
            RootBase::Root => write!(f, "r:"),
        }
    }
}

impl RootBase {
    fn path(&self) -> PathBuf {
        match self {
            RootBase::None => PathBuf::new(),
            RootBase::Crate => PathBuf::new(),
            RootBase::Home => env::home_dir().unwrap_or_default(),
            RootBase::Root => PathBuf::from("/"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConfigRoot {
    root: RootBase,
    path: String,
}

impl Display for ConfigRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.root, self.path)
    }
}

impl ConfigRoot {
    pub fn parse(s: &str) -> Self {
        log::debug!("parsing the string `{s}`");
        let Some(captures) = ROOT_CONFIG.captures(s) else {
            return ConfigRoot {
                root: RootBase::None,
                path: "".to_string(),
            };
        };
        log::debug!("found captures `{captures:?}`");

        let path = String::from(if let Some(p) = captures.name("path") {
            p.as_str()
        } else {
            ""
        });
        log::debug!("set the path to `{path}`");

        let class = if let Some(c) = captures.name("class") {
            c.as_str()
        } else {
            ""
        };
        log::debug!("found the class `{class:?}`");

        let root = match class {
            "c" => RootBase::Crate,
            "h" => RootBase::Home,
            "r" => RootBase::Root,
            "" => RootBase::None,
            _ => unreachable!(),
        };

        ConfigRoot { root, path }
    }

    pub fn set_root_base(&mut self, root: &RootBase) {
        self.root = root.to_owned();
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }

    pub fn full_path(&self) -> PathBuf {
        self.root.path().join(&self.path)
    }
}

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

        let output = ConfigRoot::parse(&input);
        log::debug!("Output set to: `{output:?}`");

        assert_eq!(expected, output.full_path().to_string_lossy().to_string());
    }

    #[test]
    fn test_parse_to_root() {
        get_test_logger();
        let input = "r:.cull-gmail".to_string();
        log::debug!("Input set to: `{input}`");
        let dir_part = input[2..].to_string();

        let expected = format!("/{dir_part}");

        let output = ConfigRoot::parse(&input);
        log::debug!("Output set to: `{output:?}`");

        assert_eq!(expected, output.full_path().to_string_lossy().to_string());
    }

    #[test]
    fn test_parse_to_crate() {
        get_test_logger();
        let input = "c:.cull-gmail".to_string();
        log::debug!("Input set to: `{input}`");
        let expected = input[2..].to_string();

        let output = ConfigRoot::parse(&input);
        log::debug!("Output set to: `{output:?}`");

        assert_eq!(expected, output.full_path().to_string_lossy().to_string());
    }

    #[test]
    fn test_parse_to_none() {
        get_test_logger();
        let input = ".cull-gmail".to_string();
        log::debug!("Input set to: `{input}`");

        let expected = "".to_string();

        let output = ConfigRoot::parse(&input);
        log::debug!("Output set to: `{output:?}`");

        assert_eq!(expected, output.full_path().to_string_lossy().to_string());
    }
}
