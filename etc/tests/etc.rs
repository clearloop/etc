use etc::{Etc, FileSystem, Read, Write};
use std::env;

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

    // input and out put
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
