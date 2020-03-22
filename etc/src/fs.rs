//! file system implementation

use crate::{Error, Meta, Source};
use std::{convert::AsRef, fs, path::PathBuf, rc::Rc};

/// mock file system
pub trait FileSystem<'fs>: Meta<'fs> {
    /// find source
    fn find(&'fs self, _src: &'fs str) -> Option<Rc<Source<'fs>>> {
        None
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
    fn rm(&'fs mut self, path: &'fs str) -> Result<(), Error> {
        let mut full = PathBuf::from(self.base()?);
        full.push(path);

        if full.is_dir() {
            fs::remove_dir(full)?;
        } else {
            fs::remove_file(full)?;
        }

        Ok(())
    }
}
