// Process a image file to add it to mugshots
// Build the metadata
//

//extern crate sha2;
//extern crate generic_array;
//extern crate chrono;
//extern crate uuid;
//
//extern crate image;

//use std::env;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, SeekFrom};
use std::fs::File;
//use std::path::Path;

//use image::*;
//use image::GenericImage;

use sha2::{Digest, Sha512};
//use chrono::prelude::*;
use std::ffi::OsString;

//use std::fmt;

//use uuid::Uuid;

pub struct ImageData {
    filename: OsString,
    hash: [u8; 64],
    thumbhash: [u8; 64],
    image_format: ::image::ImageFormat,
    //    timestamp: chrono::NaiveDateTime,
    //    last_mod: chrono::NaiveDateTime,
    //    xdim: i32,
    //    ydim: i32,
    filepath: OsString,
}

static DEFAULT_IMAGE: &'static str = "/tmp/test.jpg";

impl ImageData {
    pub fn load_default<'b>() -> io::Result<ImageData> {
        ImageData::load_file(DEFAULT_IMAGE)
    }

    pub fn load_file<'b>(filename: &str) -> io::Result<ImageData> {
        println!("Test file: {}", filename);
        // Check file meta data for readable file

        let file_metadata_result = ::std::fs::metadata(&filename);

        match file_metadata_result {
            Err(err) => {
                return Err(err);
            }
            Ok(file_metadata) => {
                if file_metadata.is_file() != true {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("{} is not an file", &filename),
                    ));
                }
            }
        }

        //
        // Check for valid image type
        // If valid, generate thumbnail and hashes
        //
        // Open image file
        let image_f_result = File::open(&filename);

        // Check for success
        match image_f_result {
            Ok(mut image_f) => {
                //
                // read the first 200 bytes and "guess" the type
                // then rewind the file
                //
                let mut buffer = [0; 200];
                &image_f.read(&mut buffer);
                let image_type_result = ::image::guess_format(&buffer);

                match image_f.seek(SeekFrom::Start(0)) {
                    Ok(_) => {}
                    Err(err) => return Err(err),
                };

                // Now check that we successfully checked the type
                match image_type_result {
                    Ok(image_type) => {
                        // With good image type, generate the file hash
                        let hash_result = Sha512::digest_reader(&mut image_f);

                        // Now check for a success hash result
                        match hash_result {
                            Err(err) => {
                                return Err(err);
                            }
                            Ok(hash_generic) => {
                                let mut hash_vals: [u8; 64] = [0; 64];
                                hash_vals.copy_from_slice(hash_generic.as_slice());
                                let image_data = ImageData {
                                    filename: OsString::from(&filename),
                                    hash: hash_vals,
                                    image_format: image_type,
                                    thumbhash: [0; 64],
                                    filepath: OsString::from("temp"),
                                };
                                return Ok(image_data);
                            }
                        }
                    }
                    Err(_) => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!("Could not open: {}", &filename),
                        ))
                    }
                }
            }
            Err(_) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Could not open: {}", &filename),
                ));
            }
        }
    }
}

#[test]
fn test_default_image() {
    let new_image = ImageData::load_file(DEFAULT_IMAGE);

    //
    // Need to split the arrays in to two x32 as Eq is not implements for [u8; 64]
    // and I'm too lazy to create it(yet)!
    //
    let hash = new_image.unwrap().hash;
    let hash_0_31 = &hash[0..32];
    let hash_32_63 = &hash[32..64];

    let bytes_0_31 = [
        64, 207, 131, 132, 162, 80, 98, 188, 207, 222, 91, 54, 229, 110, 198, 56, 46, 69, 51, 241,
        51, 5, 187, 46, 222, 88, 41, 119, 210, 189, 114, 199,
    ];

    let bytes_32_63 = [
        17, 38, 201, 142, 107, 40, 229, 31, 202, 211, 15, 78, 118, 243, 143, 138, 254, 42, 89, 227,
        188, 9, 89, 11, 167, 250, 38, 170, 116, 95, 205, 35,
    ];

    assert_eq!(hash_0_31, bytes_0_31);
    assert_eq!(hash_32_63, bytes_32_63);

    let new_image = ImageData::load_default();
    let hash = new_image.unwrap().hash;
    let hash_0_31 = &hash[0..32];
    let hash_32_63 = &hash[32..64];

    assert_eq!(hash_0_31, bytes_0_31);
    assert_eq!(hash_32_63, bytes_32_63);
}
