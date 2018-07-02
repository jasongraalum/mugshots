#[allow(dead_code)]
// 
// Process a image file to add it to mugshots
// Build the metadata
//

extern crate sha2;
extern crate generic_array;
extern crate chrono;
extern crate uuid;
extern crate image;

use self::metadata::PhotoMetaData;
//
// chrono NaiveDateTime is used to hold the photo timestamp
// and lastModified date/time
//
use chrono::prelude::*;

//
// Crypto Functions to create hash of images
//
use sha2::{Sha256, Digest};
use std::fs;

use uuid::Uuid;

use image::GenericImage;

use std::path::Path;
use std::path::PathBuf;
use std::ffi::OsStr;
use std::ffi::OsString;

use std::result;
use std::io;

fn open_image(filepath: PathBuf) -> io::Result<>
{
    // Does file exist and is it readable?
    let img = image::open(&filepath);

    match img {
        Err(err) => {
            eprintln!("{:?}", err);
            return Err("Image Error");
        },
        Ok(i) => {
            let hash =  gen_image_hash(filepath);
            match hash {
                Ok(h) => {
                    println!("{:?}", hash);
                    return Ok(PhotoMetaData::new());
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err("Image Error");
                }
            }
        }
    }

fn add_image(filepath: PathBuf, relocate: bool, root_dir: PathBuf) -> result::Result<PhotoMetaData,>
{
    // Does file exist and is it readable?
    let img = image::open(&filepath);

    match img {
        Err(err) => {
            eprintln!("{:?}", err);
            return Err("Image Error");
        },
        Ok(i) => {
            let hash =  gen_image_hash(filepath);
            match hash {
                Ok(h) => {
                    println!("{:?}", hash);
                    return Ok(PhotoMetaData::new());
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err("Image Error");
                }
            }
        }
    }
    //
    // Is the file an image file - as defined by it's extension?
    //
    // Get size and timestamps
    // Create thumbnail
    //
    // TBD - run classification/identification
    // 
    // return metadata structure
}


// Borrow a reference to the PhotoMetaData
fn create_thumbnail(metadata: &PhotoMetaData)
{
    // Create thumbnail and store in mirror hierarchy.
    // Update metadata with new thumbnail details
}

fn gen_image_hash(image_filepath: PathBuf) -> io::Result<uuid::Uuid>
{
    let mut file_option = fs::File::open(&image_filepath);
    match file_option {
        Ok(file) => {
            let hash = Sha256::digest_reader(&mut file).unwrap();
            println!("{:?}", hash);
            return Ok(uuid::Uuid::nil());
            
        }
        Err(err) => return Err(err)
    }
}

mod metadata;

