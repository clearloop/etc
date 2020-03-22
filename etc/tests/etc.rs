use etc::{Etc, FileSystem};

#[test]
fn test_init() {
    // config root path
    let mut dir = dirs::home_dir().unwrap();
    dir.push(".etc");

    // generate ~/.etc dir
    let etc = Etc::new(&dir).unwrap();

    // check if root exits and remove it
    assert!(dir.exists());
    assert!(etc.rm(".etc").is_ok());

    // ~/.etc has been removed
    assert!(!dir.exists());
}
