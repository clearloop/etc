//! file system implementationOA

use crate::{source::EtcSource, Error, Source};
use std::{
    cell::RefCell,
    collections::HashMap,
    convert::AsRef,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

/// mock file system
pub trait FileSystem<'fs> {
    /// base directory
    fn base(&'fs self) -> &'fs str;

    /// entry of a file/dir under cwd
    fn entry(&'fs mut self, path: &'fs str) -> Option<Box<Source<'fs>>>;

    /// current working directory
    fn path(&'fs self) -> &'fs str;

    /// tree of current directory
    fn tree(&'fs self) -> Rc<RefCell<HashMap<&'fs str, Box<Source<'fs>>>>>;

    // /// sync target source to Etc
    // fn sync(src: P) -> Result<Etc, Error>
    // where
    //     P: AsRef<&'fs str> + AsRef<Path>,
    // {
    //     let src = fs::read_dir(src)?;
    //     let mut res = Source::default();
    //     src.collect::<PathBuf>().for_each(|path| {
    //         if (path.is_dir()) {
    //             res.mkdir(path);
    //         } else {
    //             // res.write(path);
    //         }
    //     })
    // }

    /// find source
    fn find(&'fs self, src: &'fs str) -> Option<Box<Source<'fs>>> {
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
                let res = t.remove(src);
                t.insert(k, res.clone().unwrap_or_default());

                return res;
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
        P: AsRef<&'fs str> + AsRef<Path>,
    {
        let mut dir = PathBuf::from(self.base());
        dir.push(path);

        // let tree = self.tree();
        // let mut t = tree.borrow_mut();

        fs::create_dir(dir)?;

        unimplemented!();
        // dir.read_dir()?.collect::<Vec<PathBuf>>().for_each(|x| {
        //     if x.is_dir() {
        //         x.mkdir(x.to_string());
        //     }
        //
        //     FileSystem::sync(x);
        // })
    }

    /// remove dir or file
    fn rm(&'fs mut self, path: &'fs str) -> Result<(), Error> {
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
