//! file system implementation

use crate::{source::EtcSource, Error, Source};
use std::{
    collections::HashMap,
    convert::AsRef,
    fs,
    path::{Path, PathBuf},
};

/// mock file system
pub trait FileSystem<'fs> {
    fn entry(&self, path: &str) -> Option<Box<Source>>;
    fn path(&self) -> &PathBuf;
    fn tree(&self) -> HashMap<&'fs str, Box<Source<'fs>>>;

    /// find source
    fn find(&self, src: &'fs str) -> Option<Box<Source<'fs>>> {
        let mut t = self.tree();

        if t.is_empty() {
            return None;
        }

        if t.contains_key(src) {
            return t.remove(src);
        }

        for k in t.clone().keys() {
            if k == &src {
                let res = t.remove(src);
                t.insert(k, res.clone().unwrap_or_default());

                return res;
            }
        }

        None
    }

    /// list sources
    fn ls(&self) -> Vec<&'fs str> {
        let mut res = vec![];
        self.tree().keys().for_each(|&k| {
            res.push(k);
        });

        res
    }

    /// create dir under root
    fn mkdir<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<&'fs str> + AsRef<Path>,
    {
        let mut dir = self.path().clone();
        dir.push(path);

        Ok(fs::create_dir(dir)?)
    }

    /// remove dir or file
    fn rm(&mut self, path: &str) -> Result<(), Error> {
        if let Some(src) = self.entry(path) {
            match src.ty {
                EtcSource::Dir => {
                    fs::remove_dir(src.name)?;
                }
                EtcSource::File => {
                    fs::remove_file(src.name)?;
                }
            }
        }

        Ok(())
    }
}
