//! etc source

use crate::{Error, Meta};
use std::{cell::RefCell, collections::HashMap, path::PathBuf, rc::Rc};

/// contains dir and file
pub struct Source<'s> {
    /// source path
    pub path: &'s str,

    /// source tree
    pub tree: Rc<RefCell<HashMap<&'s str, Rc<Source<'s>>>>>,
}

impl<'m> Meta<'m> for Source<'m> {
    fn real_path(&'m self) -> Result<PathBuf, Error> {
        Ok(PathBuf::from(self.path))
    }
}
