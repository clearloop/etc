//! meta data

use crate::Error;
use std::path::PathBuf;

/// meta data
pub trait Meta {
    /// real path
    fn real_path(&self) -> Result<PathBuf, Error>;

    /// base directory
    fn base(&self) -> Result<PathBuf, Error> {
        if let Some(path) = self.real_path()?.parent() {
            Ok(path.to_path_buf())
        } else {
            Err(Error::Custom(format!(
                "error: parse {} failed",
                self.real_path()?.to_string_lossy()
            )))
        }
    }

    /// current working directory
    fn name(&self) -> Result<String, Error> {
        if let Some(name) = self.real_path()?.file_name() {
            if let Ok(string) = name.to_os_string().into_string() {
                Ok(string)
            } else {
                Err(Error::Custom(format!(
                    "error: convert OsString {:?} failed",
                    name,
                )))
            }
        } else {
            Err(Error::Custom(format!(
                "error: parse {} failed",
                self.real_path()?.to_string_lossy()
            )))
        }
    }
}
