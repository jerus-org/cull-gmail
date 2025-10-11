use std::{env, fs, io};

use crate::{Error, Result};

pub(crate) fn assure_config_dir_exists(dir: &str) -> Result<String> {
    let trdir = dir.trim();
    if trdir.is_empty() {
        return Err(Error::DirectoryUnset);
    }

    let expanded_config_dir = if trdir.as_bytes()[0] == b'~' {
        match env::var("HOME")
            .ok()
            .or_else(|| env::var("UserProfile").ok())
        {
            None => {
                return Err(Error::HomeExpansionFailed(trdir.to_string()));
            }
            Some(mut user) => {
                user.push_str(&trdir[1..]);
                user
            }
        }
    } else {
        trdir.to_string()
    };

    if let Err(err) = fs::create_dir(&expanded_config_dir) {
        if err.kind() != io::ErrorKind::AlreadyExists {
            return Err(Error::DirectoryCreationFailed((
                expanded_config_dir,
                Box::new(err),
            )));
        }
    }

    Ok(expanded_config_dir)
}

pub(crate) trait Elide {
    fn elide(&mut self, to: usize) -> &mut Self;
}

impl Elide for String {
    fn elide(&mut self, to: usize) -> &mut Self {
        if self.len() <= to {
            self
        } else {
            let mut range = to - 4;
            while !self.is_char_boundary(range) {
                range -= 1;
            }
            self.replace_range(range.., " ...");
            self
        }
    }
}

fn get_start_boundary(string: String, mut start: usize) -> usize {
    while !string.is_char_boundary(start) {
        start -= 1;
    }

    start
}
