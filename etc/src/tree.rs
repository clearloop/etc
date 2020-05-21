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
            let mut files: Vec<Box<Tree>> = vec![];
            for f in fs::read_dir(&path)? {
                files.push(Box::new(Tree::batch(Etc::from(f?.path()))?));
            }

            // Iter children
            let mut children: Option<Vec<Box<Tree>>> = None;
            if files.len() > 0 {
                if cfg!(target_family = "unix") {
                    files.reverse();
                }
                children = Some(files);
            }

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
        } else {
            if let Some(children) = &mut self.children {
                for f in children {
                    f.load()?;
                }
            }
        }

        Ok(())
    }
}
