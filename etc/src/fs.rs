//! file system implementation

use crate::{source::EtcSource, Error, Source};
use std::{
    convert::AsRef,
    fs,
    path::{Path, PathBuf},
};

/// mock file system
pub trait FileSystem<'fs> {
    fn path(&self) -> &PathBuf;
    fn entry(&self, path: &str) -> Option<Box<Source>>;

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
