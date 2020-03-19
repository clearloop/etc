use crate::{Error, EtcSource, Source};
use std::{collections::HashMap, fs, path::PathBuf};

/// The main struct in etc
pub struct Etc<'e> {
    /// root directory
    pub root: &'e str,

    /// source tree
    pub tree: HashMap<&'e str, Source<'e>>,
}

impl<'e> Etc<'e> {
    /// abstract an etc in memory
    pub fn new(&self, root: &'e str) -> Etc {
        Etc {
            root,
            tree: HashMap::new(),
        }
    }

    /// init etc to the root path
    pub fn init(&self) -> Result<(), Error> {
        let dir = PathBuf::from(self.root);
        if dir.exists() {
            if dir.is_file() {
                Err(Error::Custom(format!(
                    "error: {} exists",
                    &dir.to_string_lossy()
                )))
            } else {
                Ok(())
            }
        } else {
            Ok(fs::create_dir(&dir)?)
        }
    }

    /// generate etc dir
    pub fn dir(name: &'e str) -> Source<'e> {
        Source {
            name,
            stream: &[],
            srcty: EtcSource::Dir,
            tree: HashMap::new(),
        }
    }

    /// generate etc file
    pub fn file<S>(name: &'e str, stream: S) -> Source<'e>
    where
        S: AsRef<&'e [u8]>,
    {
        Source {
            name,
            stream: stream.as_ref(),
            srcty: EtcSource::File,
            tree: HashMap::new(),
        }
    }
}
