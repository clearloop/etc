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
    pub fn new(root: &PathBuf) -> Result<Etc, Error> {
        if !root.exists() {
            fs::create_dir_all(root)?;
        }

        let mut perms = fs::metadata(root)?.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
        }

        Ok(Etc(root.to_owned()))
    }
}

impl Meta for Etc {
    fn real_path(&self) -> Result<PathBuf, Error> {
        Ok(self.0.to_owned())
    }
}

impl<'e> Meta for &Etc {
    fn real_path(&self) -> Result<PathBuf, Error> {
        Ok(self.0.to_owned())
    }
}

impl<'e> Meta for &mut Etc {
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
        self.name().unwrap_or_else(|_| "".to_string())
    }
}
