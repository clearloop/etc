//! etc source

use crate::FileSystem;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// etc source enum
#[derive(Clone)]
pub enum EtcSource {
    /// etc dir
    Dir,
    /// etc file
    File,
}

impl Default for EtcSource {
    fn default() -> EtcSource {
        EtcSource::File
    }
}

/// contains dir and file
#[derive(Clone, Default)]
pub struct Source<'s> {
    /// base directory
    pub base: &'s str,

    /// source namep
    pub name: &'s str,

    /// source tree
    pub tree: Rc<RefCell<HashMap<&'s str, Box<Source<'s>>>>>,

    /// source type
    pub ty: EtcSource,

    /// source stream
    pub stream: &'s [u8],
}

impl<'fs> FileSystem<'fs> for Source<'fs> {
    fn base(&'fs self) -> &'fs str {
        self.base
    }

    fn entry(&'fs mut self, path: &'fs str) -> Option<Box<Source<'fs>>> {
        let mut t = self.tree.borrow_mut();
        let r = t.remove(path)?;
        t.insert(path, r.clone());

        Some(r)
    }

    fn path(&'fs self) -> &'fs str {
        self.name
    }

    fn tree(&'fs self) -> Rc<RefCell<HashMap<&'fs str, Box<Source<'fs>>>>> {
        self.tree.clone()
    }
}
