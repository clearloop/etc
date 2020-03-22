use etc::Etc;

#[test]
fn test_init() {
    // config root path
    let mut dir = dirs::home_dir().unwrap();
    dir.push(".etc");

    // generate ~/.etc dir
    Etc::new(&dir).unwrap();

    // check if root exits and remove it
    assert!(dir.exists());
    assert!(::std::fs::remove_dir(dir).is_ok());
}
