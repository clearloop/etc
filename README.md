# etc

[![etc](https://github.com/clearloop/etc/workflows/etc/badge.svg)](https://github.com/clearloop/etc)
[![crate](https://img.shields.io/crates/v/etc.svg)](https://crates.io/crates/etc)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/etc/)
[![downloads](https://img.shields.io/crates/d/etc.svg)](https://crates.io/crates/etc)
[![LICENSE](https://img.shields.io/crates/l/etc.svg)](https://choosealicense.com/licenses/mit/)

It's time to bundle etc for your awesome project!

```rust
use etc::{Etc, FileSystem, Read, Write};

fn main() {
    // config root path
    let mut dir = std::env::temp_dir();
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
```

## Loading dir with files' content into one struct!

```rust
use etc::{Etc, Tree, FileSystem, Write};

fn main() {
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
    let mut tree = Tree::batch(&etc).unwrap();
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
```

## LICENSE

MIT
