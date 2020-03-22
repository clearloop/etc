//! meta data

use crate::Source;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// meta data
pub trait Meta<'m> {
    /// base directory
    fn base(&'m self) -> &'m str;

    /// entry of a file/dir under cwd
    fn entry(&'m mut self, path: &'m str) -> Option<Box<Source<'m>>>;

    /// current working directory
    fn path(&'m self) -> &'m str;

    /// tree of current directory
    fn tree(&'m self) -> Rc<RefCell<HashMap<&'m str, Box<Source<'m>>>>>;
}
