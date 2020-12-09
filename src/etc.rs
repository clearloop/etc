//! etc source
use crate::{Error, Meta, Tree};
use std::{
    convert::{From, Into},
    fs,
    path::{Path, PathBuf},
};

/// contains dir and file
pub struct Etc(PathBuf);

impl Etc {
    /// Abstract an etc dir
    pub fn new<P>(root: P) -> Result<Etc, Error>
    where
        P: AsRef<Path> + Sized,
    {
        if !root.as_ref().exists() {
            fs::create_dir_all(&root)?;
        }

        let mut perms = fs::metadata(&root)?.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
        }

        Ok(Etc(root.as_ref().to_path_buf()))
    }

    /// Check if is `/`
    pub fn is_root(&self) -> bool {
        self.0 == PathBuf::from("/")
    }

    /// Convert `Etc` to `Tree`
    pub fn tree(self) -> Result<Tree, Error> {
        Tree::batch(self)
    }
}

impl Meta for Etc {
    fn real_path(&self) -> Result<PathBuf, Error> {
        Ok(self.0.to_owned())
    }
}

impl<'e> Meta for &Etc {
    fn real_path(&self) -> Result<PathBuf, Error> {
        Ok(self.0.to_owned())
    }
}

impl<'e> Meta for &mut Etc {
    fn real_path(&self) -> Result<PathBuf, Error> {
        Ok(self.0.to_owned())
    }
}

impl<P> From<P> for Etc
where
    P: AsRef<Path> + Sized,
{
    fn from(p: P) -> Etc {
        Etc(p.as_ref().to_path_buf())
    }
}

impl Into<String> for Etc {
    fn into(self) -> String {
        self.name().unwrap_or_else(|_| "".to_string())
    }
}

impl Into<PathBuf> for Etc {
    fn into(self) -> PathBuf {
        self.0
    }
}
