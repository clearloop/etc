//! File tree
use crate::{Error, Etc, FileSystem, Meta, Read, Write};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Here are two file types in `Tree`
///
/// + Dir  - no contents, have children
/// + File - have contents, no children
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
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
    pub fn redir<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path> + Sized,
    {
        if !path.as_ref().exists() {
            fs::create_dir_all(&path)?;
        }

        let mut buf = path.as_ref().to_path_buf();
        buf.push(Etc::from(self.path.clone()).name()?);
        if let Some(children) = &mut self.children {
            for f in children {
                f.redir(&buf)?;
            }
        }

        if let Some(content) = &self.content {
            Etc::from(buf).write(content)?;
        }

        Ok(())
    }

    /// Refresh children
    pub fn refresh(self) -> Result<Tree, Error> {
        Tree::batch(Etc::from(self.path))
    }
}

impl From<Tree> for Vec<Tree> {
    fn from(mut tree: Tree) -> Vec<Tree> {
        let mut vp: Vec<Tree> = vec![];
        let children = tree.children.take();
        vp.push(tree);

        if let Some(children) = children {
            for tree in children {
                vp.append(&mut tree.clone().into());
            }
        }

        vp
    }
}

impl From<Tree> for Vec<PathBuf> {
    fn from(tree: Tree) -> Self {
        let mut vp: Vec<PathBuf> = vec![];
        vp.push(tree.path.clone());

        if let Some(children) = tree.children {
            for f in children {
                vp.append(&mut f.into());
            }
        }

        vp
    }
}
