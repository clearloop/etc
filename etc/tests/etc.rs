use etc::Etc;

#[test]
fn test_init() {
    // config root path
    let mut dir = dirs::home_dir().unwrap();
    dir.push(".etc");

    // generate ~/.etc dir
    let etc = Etc::new(&dir);
    assert!(etc.init().is_ok());

    // check if root exits and remove it
    assert!(dir.exists());
    assert!(fs::remove_dir(dir).is_ok());
}
