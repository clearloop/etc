//! File tree
use crate::{Error, Etc, FileSystem, Meta, Read, Write};
use std::{fs, path::PathBuf};

#[cfg(feature = "serde-tree")]
use serde::{Deserialize, Serialize};

/// Here are two file types in `Tree`
///
/// + Dir  - no contents, have children
/// + File - have contents, no children
#[cfg_attr(feature = "serde-tree", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Eq, PartialEq)]
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
        self.map(|t| {
            if t.path.is_file() {
                let stream = Etc::from(t.path.clone()).read();
                if let Ok(ctx) = stream {
                    t.content = Some(ctx);
                } else {
                    t.content = None;
                }
            }
        });
        Ok(())
    }

    /// Map the whole tree
    pub fn map<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Tree) + Sized + Copy,
    {
        f(self);
        if let Some(children) = &mut self.children {
            for t in children {
                t.map(f);
            }
        }
    }

    /// Redir tree path, just like `cp -r` in `unix`
    pub fn redir(&mut self, mut path: PathBuf) -> Result<(), Error> {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        path.push(Etc::from(self.path.clone()).name()?);
        self.path = path.clone();
        if let Some(children) = &mut self.children {
            for f in children {
                f.redir(path.clone())?;
            }
        }

        if let Some(content) = &self.content {
            Etc::from(self.path.clone()).write(&content)?;
        }

        Ok(())
    }

    /// Refresh children
    pub fn refresh(self) -> Result<Tree, Error> {
        Tree::batch(Etc::from(self.path))
    }
}

macro_rules! into {
    ([$($t:ty),+]) => {
        $(
            // Into `Vec<PathBuf>`
            impl Into<Vec<PathBuf>> for $t {
                fn into(self) -> Vec<PathBuf> {
                    let mut vp: Vec<PathBuf> = vec![];
                    vp.push(self.path.clone());
                    if let Some(children) = &self.children {
                        for f in children {
                            vp.append(&mut f.into());
                        }
                    }

                    vp
                }
            }

            // Into `Vec<tree>`
            impl Into<Vec<Tree>> for $t {
                fn into(self) -> Vec<Tree> {
                    let mut vp: Vec<Tree> = vec![];
                    let mut t = Tree::default();
                    t.path = self.path.clone();
                    t.content = self.content.clone();

                    vp.push(t);
                    if let Some(children) = &self.children {
                        for f in children {
                            vp.append(&mut f.into());
                        }
                    }

                    vp
                }
            }
        )+
    }
}

into!([&Tree, &mut Tree]);
