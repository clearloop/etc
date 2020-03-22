//! etc source

use crate::{Error, Meta};
use std::{
    convert::{From, Into},
    path::PathBuf,
};

/// contains dir and file
pub struct Source(PathBuf);

impl<'m> Meta<'m> for Source {
    fn real_path(&'m self) -> Result<PathBuf, Error> {
        Ok(self.0.to_owned())
    }
}

impl<'s> From<PathBuf> for Source {
    fn from(p: PathBuf) -> Source {
        Source(p)
    }
}

impl<'s> Into<String> for Source {
    fn into(self) -> String {
        self.name().unwrap_or("".to_string())
    }
}
