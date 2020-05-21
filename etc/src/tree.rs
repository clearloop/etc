//! File tree
use crate::{Error, Etc, FileSystem};
use std::{fs, path::PathBuf};

#[cfg(feature = "serde-tree")]
use serde::{Deserialize, Serialize};

/// Here are two file types in `Tree`
///
/// + Dir  - no contents, have children
/// + File - have contents, no children
#[cfg_attr(feature = "serde-tree", derive(Serialize, Deserialize))]
pub struct Tree {
    /// File path
    pub path: PathBuf,
    /// File content
    pub content: Option<String>,
    /// Children files
    pub children: Option<Vec<Box<Tree>>>,
}

impl Tree {
    /// Batch all files into a tree, we can use this
    /// to implement some pre build stuffs.
    pub fn batch<Fs>(src: Fs) -> Result<Tree, Error>
    where
        Fs: FileSystem,
    {
        let path = PathBuf::from(src.real_path()?);
        if path.is_file() {
            Ok(Tree {
                path,
                content: None,
                children: None,
            })
        } else {
            let mut children: Vec<Box<Tree>> = vec![];
            for f in fs::read_dir(&path)? {
                children.push(Box::new(Tree::batch(Etc::from(f?.path()))?));
            }

            Ok(Tree {
                path,
                content: None,
                children: Some(children),
            })
        }
    }
}
