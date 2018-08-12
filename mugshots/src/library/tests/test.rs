#[test]
fn test_new_imagelibrary() {
    let new_library: ImageLibrary = ImageLibrary::create(
        "testLibrary".to_string(),
        "./test/testLibraries".to_string(),
    ).unwrap();
    let name = "testLibrary.".to_string() + LIB_EXTENSION;
    assert_eq!(new_library.name, name);
}
