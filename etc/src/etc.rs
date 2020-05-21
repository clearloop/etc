//! etc source
use crate::{Error, Meta};
use std::{
    convert::{From, Into},
    fs,
    path::PathBuf,
};

/// contains dir and file
pub struct Etc(PathBuf);

impl Etc {
    /// Abstract an etc dir
    pub fn new<'e>(root: &'e PathBuf) -> Result<Etc, Error> {
        if !root.exists() {
            fs::create_dir(root)?;
        }

        Ok(Etc(root.to_owned()))
    }
}

impl Meta for Etc {
    fn real_path(&self) -> Result<PathBuf, Error> {
        Ok(self.0.to_owned())
    }
}

impl From<PathBuf> for Etc {
    fn from(p: PathBuf) -> Etc {
        Etc(p)
    }
}

impl Into<String> for Etc {
    fn into(self) -> String {
        self.name().unwrap_or("".to_string())
    }
}
