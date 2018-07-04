//
// Process a image file to add it to mugshots
// Build the metadata
//


//use std::env;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, SeekFrom};
use std::fs::File;
use std::path::Path;

use image;
use image::{GenericImage,ImageError};

use sha2::{Digest, Sha512};
//use chrono::prelude::*;
use chrono::{DateTime, Utc};
//use std::ffi::OsString;

//use std::fmt;

//use uuid::Uuid;

pub struct ImageData {
    filename: String,
    hash: [u8; 64],
    thumbhash: [u8; 64],
    image_format: ::image::ImageFormat,
    added_ts: DateTime<Utc>,
    last_mod_ts: DateTime<Utc>,
    dim: (u32, u32),
    //    ts: chrono::NaiveDateTime,
    //    ydim: i32,
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

                // Rewind and check for success
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
                                //
                                // Copy hash result from it's GenericArray of type <T: N>
                                // to an array of u8
                                //
                                let mut hash_vals: [u8; 64] = [0; 64];
                                hash_vals.copy_from_slice(hash_generic.as_slice());

                                //
                                // Get basefile name from input file
                                //
                                let base_filename_option = Path::new(&filename).file_name();
                                let base_filename: String = match base_filename_option {
                                    Some(bfn) => {
                                        bfn.to_str().unwrap().to_string()
                                    },
                                    None =>  {
                                        return Err(Error::new(
                                                ErrorKind::InvalidInput,
                                                format!("Invalid filename: {}", &filename),
                                                ));
                                    },
                                };

                                // 
                                // Create ImageData struct
                                let mut image_data = ImageData {
                                    filename: base_filename, 
                                    hash: hash_vals,
                                    image_format: image_type,
                                    thumbhash: [0; 64],
                                    added_ts: Utc::now(),
                                    last_mod_ts: Utc::now(),
                                    dim: (0,0),
                                };

                                let image_result = image::open(Path::new(&filename));
                                match image_result {
                                    Ok(image) => {
                                        image_data.dim = image.dimensions();
                                    },
                                    Err(err) => {
                                        return Err(Error::new(ErrorKind::InvalidData,format!("{}",err)));
                                    }
                                }
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
    let new_image_result = ImageData::load_file(DEFAULT_IMAGE);

    let mut new_image : ImageData = match new_image_result {
        Ok(new_image) => new_image,
        Err(_) =>  { assert!(false); return },
    };

    //
    // Need to split the arrays in to two x32 as Eq is not implements for [u8; 64]
    // and I'm too lazy to create it(yet)!
    //
    let hash = new_image.hash;
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
    assert_eq!(new_image.filename,"test.jpg");

    let dt_utc: DateTime<Utc> = Utc::now(); 

    println!("\nImage Struct: added_ts error");
    println!("last_mod_ts = {}",new_image.added_ts);
    println!("now = {}\n",dt_utc);
    assert!(dt_utc > new_image.added_ts);

    new_image.update_mod_time();

    println!("\nImage Struct: update mode time error");
    println!("last_mod_ts = {}",new_image.last_mod_ts);
    println!("now = {}\n",dt_utc);
    assert!(dt_utc < new_image.last_mod_ts);

    assert_eq!(new_image.dim, (3264, 2448));
}


fn test_default2_image() {
    let new_image_result = ImageData::load_default();

    let new_image : ImageData = match new_image_result {
        Ok(new_image) => new_image,
        Err(_) =>  { assert!(false); return },
    };

    //
    // Need to split the arrays in to two x32 as Eq is not implements for [u8; 64]
    // and I'm too lazy to create it(yet)!  
    let hash = new_image.hash;
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
    assert_eq!(new_image.filename,"test.jpg");
}

