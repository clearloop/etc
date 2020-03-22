use crate::{Error, FileSystem, Source};
use std::{cell::RefCell, collections::HashMap, fs, path::PathBuf, rc::Rc};

/// The main struct in etc
pub struct Etc<'e> {
    /// root directory
    pub root: &'e PathBuf,

    /// source tree
    pub tree: Rc<RefCell<HashMap<&'e str, Box<Source<'e>>>>>,
}

impl<'e> Etc<'e> {
    /// abstract an etc
    pub fn new(root: &'e PathBuf) -> Result<Etc, Error> {
        fs::create_dir(root)?;

        Ok(Etc {
            root,
            tree: Rc::new(RefCell::new(HashMap::new())),
        })
    }
}

impl<'fs> FileSystem<'fs> for Etc<'fs> {
    fn base(&'fs self) -> &'fs str {
        self.root.as_os_str().to_str().unwrap_or_default()
    }

    fn entry(&'fs mut self, path: &'fs str) -> Option<Box<Source<'fs>>> {
        let mut t = self.tree.borrow_mut();
        let r = t.remove(path)?;
        t.insert(path, r.clone());

        Some(r)
    }

    fn path(&'fs self) -> &'fs str {
        self.root
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }

    fn tree(&'fs self) -> Rc<RefCell<HashMap<&'fs str, Box<Source<'fs>>>>> {
        self.tree.clone()
    }
}
