//! file system implementation
use crate::{Error, Etc, Meta};
use std::{convert::AsRef, fs, ops::FnOnce, path::PathBuf};

/// mock file system
pub trait FileSystem: Meta {
    /// remove current dir or file
    fn drain(&self) -> Result<(), Error> {
        let path = self.real_path()?;

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }

        Ok(())
    }

    /// opens a file in write-only mode.
    fn back(&self) -> Result<Etc, Error> {
        Ok(Etc::from(self.base()?))
    }

    /// entry of a file
    fn entry<F>(&self, name: &str, f: F) -> Result<(), Error>
    where
        F: FnOnce(Etc),
    {
        let mut path = self.real_path()?;
        path.push(name);

        f(path.into());
        Ok(())
    }

    /// find target
    fn find(&self, src: &str) -> Result<PathBuf, Error> {
        for f in fs::read_dir(self.real_path()?)? {
            let path = f?.path();
            if let Some(s) = path.file_name() {
                if src == s {
                    return Ok(path);
                } else if path.is_dir() {
                    let source: Etc = path.into();
                    let res = FileSystem::find(&source, src);
                    if res.is_ok() {
                        return res;
                    }
                }
            }
        }

        Err(Error::Custom(format!("error: {} not found", src)))
    }

    /// find all target
    fn find_all(&self, src: &str, res: &mut Vec<PathBuf>) -> Result<(), Error> {
        for f in fs::read_dir(self.real_path()?)? {
            let path = f?.path();
            if let Some(s) = path.file_name() {
                if src == s {
                    res.push(path);
                } else if path.is_dir() {
                    let source: Etc = path.into();
                    FileSystem::find_all(&source, src, res)?;
                }
            }
        }

        Ok(())
    }

    /// list sources
    fn ls(&self) -> Result<Vec<String>, Error> {
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
    fn mkdir<P>(&self, path: P) -> Result<(), Error>
    where
        P: AsRef<str>,
    {
        let mut dir = self.real_path()?;
        dir.push(path.as_ref());
        if !dir.exists() {
            Ok(fs::create_dir_all(dir)?)
        } else {
            Ok(())
        }
    }

    /// opens a file in write-only mode.
    fn open(&self, name: &str) -> Result<Etc, Error> {
        let mut path = self.real_path()?;
        path.push(name);

        Ok(path.into())
    }

    /// remove dir or file
    fn rm(&self, path: &str) -> Result<(), Error> {
        let base = self.real_path();

        // file doesn't exist, so don't need to remove
        if base.is_err() {
            return Ok(());
        }

        let mut full = base?;
        full.push(path);

        if full.is_dir() {
            fs::remove_dir_all(full)?;
        } else {
            fs::remove_file(full)?;
        }

        Ok(())
    }
}

impl<T> FileSystem for T where T: Meta {}
