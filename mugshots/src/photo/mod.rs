//
// Process a image file to add it to mugshots
// Build the metadata
//

pub mod image_hash;

use self::image_hash::ImageHash;

use std::io;
use std::env;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, SeekFrom};
use std::fs::File;
use std::path::Path;

use image;
use image::{GenericImage, ImageError};


use chrono::{DateTime, Utc};

pub struct ImageData {
    filename: String,
    hash: ImageHash,
    thumb_hash: ImageHash,
    image_format: ::image::ImageFormat,
    added_ts: DateTime<Utc>,
    last_mod_ts: DateTime<Utc>,
    dim: (u32, u32),
}

impl ImageData {
    pub fn load_file<'b>(filename: &str) -> io::Result<ImageData> {
        // Check file meta data for readable file

        let file_metadata = ::std::fs::metadata(&filename)?;

        if file_metadata.is_file() != true {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("{} is not an file", &filename),
            ));
        }

        //
        // Check for valid image type
        // If valid, generate thumbnail and hashes
        //
        // Open image file
        let mut image_f = File::open(&filename)?;

        //
        // read the first 200 bytes and "guess" the type
        // then rewind the file
        //
        let mut buffer = [0; 200];
        &image_f.read(&mut buffer);
        let image_type_result = ::image::guess_format(&buffer);
        let image_type = match image_type_result {
            Ok(image_type) => image_type,
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid filename: {}", &filename),
                ));
            }
        };

        // Rewind and check for success
        let _ = image_f.seek(SeekFrom::Start(0))?;


        //
        // Get base file name from input file
        //
        let base_filename_option = Path::new(&filename).file_name();
        let base_filename: String = match base_filename_option {
            Some(bfn) => {
                bfn.to_str().unwrap().to_string()
            }
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Invalid filename: {}", &filename),
                ));
            }
        };

        // Now check that we successfully checked the type
        // With good image type, generate the file hash
        let image_hash = ImageHash::new(image_f)?;
        //let hash_val = Sha512::digest_reader(&mut image_f)?;

        //
        // Create ImageData struct
        let mut image_data = ImageData {
            filename: base_filename,
            hash: image_hash,
            image_format: image_type,
            thumb_hash: ImageHash::nil(),
            added_ts: Utc::now(),
            last_mod_ts: Utc::now(),
            dim: (0, 0),
        };

        let image_result = image::open(Path::new(&filename));
        let image = match image_result {
            Err(err) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Unable to open {} as image.", &filename),
                ));
            },
            Ok(image) => image
        };
        image_data.dim = image.dimensions();

        return Ok(image_data);
    }

    fn update_mod_time(&mut self)
    {
        self.last_mod_ts = Utc::now();
    }
}

//
// Combined all tests to remove overhead of opening the image file multiple times
// Todo is to optimize opening image file - do we need to read the whole image into memory
// if we only need the meta data??
//
#[test]
fn test_imagedata_construction() {
    let test_image_file = "test/jpg/test1.jpg".to_string();
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
    println!("last_mod_ts = {}", &new_image.added_ts);
    println!("now = {}\n", dt_utc);
    assert!(dt_utc > new_image.added_ts);

    new_image.update_mod_time();

    println!("\nImage Struct: update mode time error");
    println!("last_mod_ts = {}", &new_image.last_mod_ts);
    println!("now = {}\n", dt_utc);
    assert!(dt_utc < new_image.last_mod_ts);

    assert_eq!(new_image.dim, (3264, 2448));
}


