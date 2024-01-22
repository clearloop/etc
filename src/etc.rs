//! etc source
use crate::{Error, Meta, Tree};
use std::{
    convert::From,
    fs,
    path::{Path, PathBuf},
};

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

/// contains dir and file
#[derive(Clone, Debug)]
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
            #[cfg(target_family = "unix")]
            perms.set_mode(0o755);
            #[cfg(target_family = "windows")]
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

impl<P> From<P> for Etc
where
    P: AsRef<Path> + Sized,
{
    fn from(p: P) -> Etc {
        Etc(p.as_ref().to_path_buf())
    }
}

impl From<Etc> for String {
    fn from(val: Etc) -> Self {
        val.name().unwrap_or_else(|_| "".to_string())
    }
}

impl From<Etc> for PathBuf {
    fn from(val: Etc) -> Self {
        val.0
    }
}
