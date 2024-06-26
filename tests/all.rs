use etc::{Etc, FileSystem, Read, Tree, Write};
use std::{env, iter::FromIterator, path::PathBuf};

#[test]
fn test_init() {
    // config root path
    let mut dir = env::temp_dir();
    dir.push(".etc.init");

    // generate `/.etc.init` dir
    let etc = Etc::new(&dir).unwrap();

    // check if root exits and remove it
    assert!(dir.exists());
    assert!(etc.drain().is_ok());

    // `/.etc.init` has been removed
    assert!(!dir.exists());
}

#[test]
fn test_io() {
    // config root path
    let mut dir = env::temp_dir();
    dir.push(".etc.io");

    // generate `/.etc.io` dir
    let etc = Etc::new(&dir).unwrap();
    let hello = etc.open("hello.md").unwrap();

    // input and output
    assert!(hello.write(b"hello, world!\n").is_ok());
    assert_eq!(hello.read().unwrap(), b"hello, world!\n");

    // remove hello.md
    assert!(etc.rm("hello.md").is_ok());

    // hello.md has been removed
    let mut hello_md = dir.clone();
    hello_md.push("hello.md");
    assert!(!hello_md.exists());

    // remove all
    assert!(etc.drain().is_ok());
    assert!(!dir.exists());
}

#[test]
fn test_batch() {
    // config root path
    let mut dir = env::temp_dir();
    dir.push(".etc.batch");

    // generate dir
    let etc = Etc::new(&dir).unwrap();
    etc.mkdir("a/b/c").unwrap();
    etc.mkdir("a/c/b").unwrap();

    assert_eq!(
        Tree::batch(etc.clone()).unwrap(),
        Tree {
            path: PathBuf::from(&dir),
            content: None,
            children: Some(vec![Tree {
                path: PathBuf::from_iter(&[&dir, &PathBuf::from("a")]),
                content: None,
                children: Some(vec![
                    Tree {
                        path: PathBuf::from_iter(&[&dir, &PathBuf::from("a/b")]),
                        content: None,
                        children: Some(vec![Tree {
                            path: PathBuf::from_iter(&[&dir, &PathBuf::from("a/b/c")]),
                            content: None,
                            children: None,
                        }]),
                    },
                    Tree {
                        path: PathBuf::from_iter(&[&dir, &PathBuf::from("a/c")]),
                        content: None,
                        children: Some(vec![Tree {
                            path: PathBuf::from_iter(&[&dir, &PathBuf::from("a/c/b")]),
                            content: None,
                            children: None,
                        }]),
                    },
                ])
            }]),
        }
    );

    // remove all
    assert!(etc.drain().is_ok());
    assert!(!dir.exists());
}

#[test]
fn test_load() {
    // config root path
    let mut dir = env::temp_dir();
    dir.push(".etc.load");

    // write files
    let etc = Etc::new(&dir).unwrap();
    let amd = etc.open("mds/a.md").unwrap();
    let bmd = etc.open("mds/b.md").unwrap();
    assert!(amd.write(b"# hello").is_ok());
    assert!(bmd.write(b"# world").is_ok());

    // batch and load
    let mut tree = Tree::batch(etc.clone()).unwrap();
    assert!(tree.load().is_ok());
    assert_eq!(
        tree,
        Tree {
            path: PathBuf::from(&dir),
            content: None,
            children: Some(vec![Tree {
                path: PathBuf::from_iter(&[&dir, &PathBuf::from("mds")]),
                content: None,
                children: Some(vec![
                    Tree {
                        path: PathBuf::from_iter(&[&dir, &PathBuf::from("mds/a.md")]),
                        content: Some(b"# hello".to_vec()),
                        children: None,
                    },
                    Tree {
                        path: PathBuf::from_iter(&[&dir, &PathBuf::from("mds/b.md")]),
                        content: Some(b"# world".to_vec()),
                        children: None,
                    }
                ])
            }]),
        }
    );

    // remove all
    assert!(etc.drain().is_ok());
    assert!(!dir.exists());
}

#[test]
fn test_find_up() {
    assert!(etc::find_up("Cargo.toml").is_ok());
}
