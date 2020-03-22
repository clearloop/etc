use crate::{Error, Meta, Source};
use std::{cell::RefCell, collections::HashMap, fs, path::PathBuf, rc::Rc};

/// The main struct in etc
pub struct Etc<'e> {
    /// root directory
    pub root: &'e PathBuf,

    /// source tree
    pub tree: Rc<RefCell<HashMap<&'e str, Rc<Source<'e>>>>>,
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

impl<'m> Meta<'m> for Etc<'m> {
    fn base(&'m self) -> &'m str {
        self.root.as_os_str().to_str().unwrap_or_default()
    }

    fn entry(&'m self, path: &'m str) -> Option<Rc<Source<'m>>> {
        let mut t = self.tree.borrow_mut();
        let r = t.remove(path)?;
        t.insert(path, r.clone());

        Some(r)
    }

    fn path(&'m self) -> &'m str {
        self.root
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
    }

    fn tree(&'m self) -> Rc<RefCell<HashMap<&'m str, Rc<Source<'m>>>>> {
        self.tree.clone()
    }
}
