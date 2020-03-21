use crate::{Error, Source};
use std::{collections::HashMap, fs, path::PathBuf};

/// The main struct in etc
pub struct Etc<'e> {
    /// root directory
    pub root: &'e PathBuf,

    /// source tree
    pub tree: HashMap<&'e str, Source<'e>>,
}

impl<'e> Etc<'e> {
    /// abstract an etc
    pub fn new(root: &'e PathBuf) -> Result<Etc, Error> {
        fs::create_dir(root)?;

        Ok(Etc {
            root,
            tree: HashMap::new(),
        })
    }
}
