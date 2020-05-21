//! File tree
use crate::{Error, Etc, FileSystem, Read};
use std::{fs, path::PathBuf};

#[cfg(feature = "serde-tree")]
use serde::{Deserialize, Serialize};

/// Here are two file types in `Tree`
///
/// + Dir  - no contents, have children
/// + File - have contents, no children
#[cfg_attr(feature = "serde-tree", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq)]
pub struct Tree {
    /// File path
    pub path: PathBuf,
    /// File content
    pub content: Option<Vec<u8>>,
    /// Children files
    pub children: Option<Vec<Tree>>,
}

impl Tree {
    /// Batch all files into a tree, we can use this
    /// to implement some pre build stuffs.
    pub fn batch<Fs>(src: Fs) -> Result<Tree, Error>
    where
        Fs: FileSystem,
    {
        let path = src.real_path()?;
        if path.is_file() {
            Ok(Tree {
                path,
                content: None,
                children: None,
            })
        } else {
            let mut files: Vec<Tree> = vec![];
            for f in fs::read_dir(&path)? {
                files.push(Tree::batch(Etc::from(f?.path()))?);
            }

            // Iter children
            let children = if !files.is_empty() {
                files.sort_by_key(|f| f.path.clone());
                Some(files)
            } else {
                None
            };

            Ok(Tree {
                path,
                content: None,
                children,
            })
        }
    }

    /// Load file contents
    pub fn load(&mut self) -> Result<(), Error> {
        if self.path.is_file() {
            self.content = Some(Etc::from(self.path.clone()).read()?);
        } else if let Some(children) = &mut self.children {
            for f in children {
                f.load()?;
            }
        }
        Ok(())
    }
}
