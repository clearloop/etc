//! file system implementation

use crate::{Error, Meta, Source};
use std::{convert::AsRef, fs, ops::FnOnce, path::PathBuf};

/// mock file system
pub trait FileSystem<'fs>: Meta<'fs> {
    /// opens a file in write-only mode.
    fn open(&'fs self, name: &'fs str) -> Result<Source, Error> {
        let mut path = self.real_path()?;
        path.push(name);

        Ok(path.into())
    }

    /// remove current dir or file
    fn drain(&'fs self) -> Result<(), Error> {
        let path = self.real_path()?;

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    /// entry of a file
    fn entry<F>(&'fs self, name: &'fs str, f: F) -> Result<(), Error>
    where
        F: FnOnce(Source),
    {
        let mut path = self.real_path()?;
        path.push(name);

        f(path.into());
        Ok(())
    }

    /// find source
    fn find(&'fs self, src: &'fs str) -> Result<PathBuf, Error> {
        for f in fs::read_dir(self.real_path()?)? {
            let path = f?.path();
            if let Some(s) = path.file_name() {
                if src == s {
                    return Ok(path);
                } else {
                    if path.is_dir() {
                        let source: Source = path.into();
                        let res = FileSystem::find(&source, src);
                        if res.is_ok() {
                            return res;
                        }
                    }
                }
            }
        }

        Err(Error::Custom(format!("error: {} not found", src)))
    }

    /// list sources
    fn ls(&'fs self) -> Result<Vec<String>, Error> {
        let mut res = vec![];
        for f in fs::read_dir(self.real_path()?)? {
            if let Some(name) = f?.path().file_name() {
                if let Ok(string) = name.to_os_string().into_string() {
                    res.push(string);
                } else {
                    return Err(Error::Custom(format!(
                        "error: convert OsString {:?} failed",
                        name,
                    )));
                }
            } else {
                return Err(Error::Custom(format!(
                    "error: ls {} failed",
                    self.real_path()?.to_string_lossy()
                )));
            }
        }

        Ok(res)
    }

    /// create dir under root
    fn mkdir<P>(&'fs self, path: P) -> Result<(), Error>
    where
        P: AsRef<&'fs str>,
    {
        let mut dir = self.base()?;
        dir.push(path.as_ref());
        Ok(fs::create_dir(dir.clone())?)
    }

    /// remove dir or file
    fn rm(&'fs self, path: &'fs str) -> Result<(), Error> {
        let base = self.real_path();

        // file doesn't exist, so don't need to remove
        if base.is_err() {
            return Ok(());
        }

        let mut full = PathBuf::from(base?);
        full.push(path);

        if full.is_dir() {
            fs::remove_dir_all(full)?;
        } else {
            fs::remove_file(full)?;
        }

        Ok(())
    }
}

impl<'m, T> FileSystem<'m> for T where T: Meta<'m> {}
