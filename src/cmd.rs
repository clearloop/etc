//! This module exports shortcuts of `etc`
use crate::{Error, Etc, FileSystem};
use std::path::{Path, PathBuf};

/// unix `cp -r`
pub fn cp_r<P>(src: P, target: P) -> Result<(), Error>
where
    P: AsRef<Path> + Sized,
{
    let mut tree = Etc::from(src).tree()?;
    tree.load()?;

    if let Some(mut children) = tree.children {
        children.iter_mut().for_each(|t| t.redir(&target).unwrap());
    }
    Ok(())
}

/// unix `find <src> -name <target>`
pub fn find_all<P>(src: P, target: &str) -> Result<Vec<PathBuf>, Error>
where
    P: AsRef<Path> + Sized,
{
    let root = Etc::from(src);
    let mut res = vec![];
    root.find_all(target, &mut res)?;
    Ok(res.to_vec())
}
