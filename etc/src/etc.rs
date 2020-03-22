use crate::{Error, Meta};
use std::{fs, path::PathBuf};

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
