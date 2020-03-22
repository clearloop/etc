//! file system implementationOA

use crate::{source::EtcSource, Error, Source};
use std::{
    cell::RefCell,
    collections::HashMap,
    convert::AsRef,
    fs,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
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
        P: AsRef<&'fs str>,
    {
        let mut dir = PathBuf::from(self.base());
        dir.push(path.as_ref());

        let tree = self.tree();
        let mut t = tree.borrow_mut();

        fs::create_dir(dir)?;
        t.insert(
            path.as_ref(),
            Box::new(Source {
                base: self.base(),
                name: path.as_ref(),
                stream: &[],
                tree: Rc::new(RefCell::new(HashMap::new())),
                ty: EtcSource::Dir,
            }),
        );

        Ok(())
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

    /// write stream into file
    fn write<B>(&'fs mut self, name: &'fs str, stream: B) -> Result<(), Error>
    where
        B: AsRef<&'fs [u8]>,
    {
        let mut src = PathBuf::from(self.base());
        src.push(name);

        if !src.exists() {
            File::create(&src)?;
        }

        let f = File::open(src)?;
        let mut writer = BufWriter::new(f);

        writer.write(stream.as_ref())?;
        Ok(())
    }
}
