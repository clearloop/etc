//! etc source

use crate::Meta;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// contains dir and file
pub struct Source<'s> {
    /// base directory
    pub base: &'s str,

    /// source namep
    pub name: &'s str,

    /// source tree
    pub tree: Rc<RefCell<HashMap<&'s str, Rc<Source<'s>>>>>,
}

impl<'m> Meta<'m> for Source<'m> {
    fn base(&'m self) -> &'m str {
        self.base
    }

    fn entry(&'m self, path: &'m str) -> Option<Rc<Source<'m>>> {
        let mut t = self.tree.borrow_mut();
        let r = t.remove(path)?;
        t.insert(path, r.clone());

        Some(r)
    }

    fn path(&'m self) -> &'m str {
        self.name
    }

    fn tree(&'m self) -> Rc<RefCell<HashMap<&'m str, Rc<Source<'m>>>>> {
        self.tree.clone()
    }
}
