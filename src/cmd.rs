//! This module exports shortcuts of `etc`
use crate::{Error, Etc, FileSystem, Meta};
use std::{
    env,
    path::{Path, PathBuf},
};

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

/// find file from upper directories till `/`
pub fn find_up(target: &str) -> Result<PathBuf, Error> {
    let mut cur = Etc::from(env::current_dir()?);
    while !cur.is_root() {
        cur = cur.base()?.into();
        if cur.ls()?.contains(&target.to_string()) {
            return Ok(cur.open(target)?.into());
        }
    }

    Err(Error::Custom(format!("Could not find file {}", target)))
}
