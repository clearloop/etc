//! This module exports shortcuts of `etc`
use crate::{Error, Etc};
use std::path::Path;

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
