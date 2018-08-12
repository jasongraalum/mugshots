extern crate mugshots;
use mugshots::library::{ImageLibrary};


#[test]
fn test_new_imagelibrary() {

    let new_library: ImageLibrary = ImageLibrary::create(
        "testLibrary".to_string(),
        "./tests/testLibraries".to_string(),
    ).unwrap();

    let name = "testLibrary".to_string() + ".mgst";
    assert_eq!(new_library.name, name);
}
