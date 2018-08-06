///
/// Copyright © 2018 Jason Graalum
///
//
// Process a image file to add it to mugshots
// Build the metadata
//

pub mod image_hash;

use self::image_hash::ImageHash;

use std::io;
//use std::env;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, SeekFrom};
use std::fs::File;
use std::path::Path;

use image;
use image::{GenericImage};


use chrono::{DateTime, Utc};

pub struct ImageData {
    pub filename: String,
    pub hash: ImageHash,
    pub thumb_hash: ImageHash,
    pub image_format: ::image::ImageFormat,
    pub added_ts: DateTime<Utc>,
    pub last_mod_ts: DateTime<Utc>,
    pub dim: (u32, u32),
    pub tags: Box<Vec<String>>,
}

impl ImageData {
    pub fn get_last_mod_ts(&self) -> &DateTime<Utc> {
        &self.last_mod_ts
    }
    pub fn get_added_ts(&self) -> &DateTime<Utc> {
        &self.added_ts
    }

    pub fn get_dim(&self) -> &(u32,u32)
    {
        &self.dim
    }

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
            Err(_) => {
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
            tags: Box::new(Vec::new()),
        };

        let image_result = image::open(Path::new(&filename));
        let image = match image_result {
            Err(_err) => {
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

    pub fn update_mod_time(&mut self)
    {
        self.last_mod_ts = Utc::now();
    }

    pub fn add_tag(&mut self, new_tag: String) { self.tags.push(new_tag); }
}

