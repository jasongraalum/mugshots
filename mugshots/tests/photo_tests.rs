///
/// Copyright © 2018 Jason Graalum
///
extern crate mugshots;
extern crate chrono;
use mugshots::photo::ImageData;

use mugshots::photo::image_hash::ImageHash;

use chrono::{DateTime, Utc};

//
// Combined all tests to remove overhead of opening the image file multiple times
// Todo is to optimize opening image file - do we need to read the whole image into memory
// if we only need the meta data??
//
#[test]
fn test_imagedata_construction() {
    let test_image_file = "tests/jpg/test1.jpg".to_string();
    let new_image_result = ImageData::load_file(&test_image_file);

    let mut new_image: ImageData = match new_image_result {
        Ok(new_image) => new_image,
        Err(_) => {
            assert!(false);
            return; }
    };

    //
    // Need to split the arrays in to two x32 as Eq is not implements for [u8; 64]
    // and I'm too lazy to create it(yet)!
    //

    let good_hash_vals : [u8;32] = [
        224, 209,  64,  59,  83, 122, 245, 252, 117,   6, 236, 165,  71, 218,  72, 180,
        229, 239,  21, 153, 138, 223, 173, 212,  76, 104, 217,  18, 173,  87,  26,   8
    ];
    let good_hash = ImageHash::load(&good_hash_vals);
    assert_eq!(good_hash, new_image.hash);

    assert_eq!(&new_image.filename, "test1.jpg");

    let dt_utc: DateTime<Utc> = Utc::now();

    println!("\nImage Struct: added_ts error");
    println!("last_mod_ts = {}", *new_image.get_added_ts());
    println!("now = {}\n", dt_utc);
    assert!(dt_utc > *new_image.get_added_ts());

    new_image.update_mod_time();

    println!("\nImage Struct: update mode time error");
    println!("last_mod_ts = {}", *new_image.get_last_mod_ts());
    println!("now = {}\n", dt_utc);
    assert!(dt_utc < *new_image.get_last_mod_ts());

    assert_eq!(new_image.get_dim(), &(3264, 2448));

    new_image.add_tag("test tag".to_string());
}


