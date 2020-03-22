//! file system implementation

use crate::{Error, Meta, Source};
use std::{cell::RefCell, collections::HashMap, convert::AsRef, fs, path::PathBuf, rc::Rc};

/// mock file system
pub trait FileSystem<'fs>: Meta<'fs> {
    /// find source
    fn find(&'fs self, src: &'fs str) -> Option<Rc<Source<'fs>>> {
        let tree = self.tree();
        let mut t = tree.borrow_mut();

        if t.is_empty() {
            return None;
        }

        if t.contains_key(src) {
            return t.remove(src);
        }

        for k in t.clone().keys() {
            if k == &src {
                if let Some(v) = t.remove(src) {
                    t.insert(k, v.clone());
                    return Some(v);
                }
            }
        }

        None
    }

    /// list sources
    fn ls(&'fs self) -> Vec<&'fs str> {
        let mut res = vec![];
        self.tree().borrow().keys().for_each(|&k| {
            res.push(k);
        });

        res
    }

    /// create dir under root
    fn mkdir<P>(&'fs self, path: P) -> Result<(), Error>
    where
        P: AsRef<&'fs str>,
    {
        let mut dir = PathBuf::from(self.base());
        dir.push(path.as_ref());

        let tree = self.tree();
        let mut t = tree.borrow_mut();

        fs::create_dir(dir)?;
        t.insert(
            path.as_ref(),
            Rc::new(Source {
                base: self.base(),
                name: path.as_ref(),
                tree: Rc::new(RefCell::new(HashMap::new())),
            }),
        );

        Ok(())
    }

    /// remove dir or file
    fn rm(&'fs mut self, path: &'fs str) -> Result<(), Error> {
        let mut full = PathBuf::from(self.base());
        full.push(path);

        if let Some(src) = self.entry(path) {
            if full.is_dir() {
                fs::remove_dir(src.name)?;
            } else {
                fs::remove_file(src.name)?;
            }

            Ok(())
        } else {
            Err(Error::Custom(format!(
                "error: {} doesn't exist",
                full.to_str().unwrap_or(path)
            )))
        }
    }
}
