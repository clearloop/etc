use crate::{Error, Meta};
use std::{
    convert::{From, Into},
    fs,
    path::PathBuf,
};

/// The main struct in etc
pub struct Etc<'e> {
    /// root directory
    pub root: &'e PathBuf,
}

impl<'e> Etc<'e> {
    /// abstract an etc
    pub fn new(root: &'e PathBuf) -> Result<Etc, Error> {
        fs::create_dir(root)?;

        Ok(Etc { root })
    }
}

impl<'m> Meta<'m> for Etc<'m> {
    fn real_path(&'m self) -> Result<PathBuf, Error> {
        Ok(self.root.to_owned())
    }
}

impl<'s> From<&'s PathBuf> for Etc<'s> {
    fn from(p: &'s PathBuf) -> Etc<'s> {
        Etc { root: p }
    }
}

impl<'s> Into<String> for Etc<'s> {
    fn into(self) -> String {
        self.root.to_string_lossy().into()
    }
}
