//! etc source

use crate::Meta;
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
}

impl<'m> Meta<'m> for Source<'m> {
    fn base(&'m self) -> &'m str {
        self.base
    }

    fn entry(&'m mut self, path: &'m str) -> Option<Box<Source<'m>>> {
        let mut t = self.tree.borrow_mut();
        let r = t.remove(path)?;
        t.insert(path, r.clone());

        Some(r)
    }

    fn path(&'m self) -> &'m str {
        self.name
    }

    fn tree(&'m self) -> Rc<RefCell<HashMap<&'m str, Box<Source<'m>>>>> {
        self.tree.clone()
    }
}
