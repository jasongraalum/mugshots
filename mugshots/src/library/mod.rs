// Copyright 2018 Jason Graalum
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
//
// Process a image file to add it to mugshots
// Build the metadata
//

use std;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Read};
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use std::ffi::OsString;
//use image;
//use image::{GenericImage, ImageError};

//use sha2::{Digest, Sha512};
//use chrono::{DateTime, Utc};

use std::collections::BTreeMap;
use photo::ImageData;

const LIB_EXTENSION: &str = "mgst";

pub struct ImageLibrary {
    pub name: String,
    directory: OsString,
    image_count: u32,
    image_tree: std::collections::BTreeMap<u8, ImageData>,
}

impl ImageLibrary {
    pub fn create(name: String, directory_name: String) -> io::Result<ImageLibrary> {
        //
        // Check that name is a valid file name in directory
        // If not, either directory doesn't exist, isn't writable or there
        // already is a file by the same name, return IO Error
        //

        // Does directory exist, if not can I create it?
        let directory_path = Path::new(&directory_name);

        //
        // If directory name is a file, error out
        //
        if (&directory_path).is_file() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Target directory invalid"),
            ));
        }

        if (&directory_path).exists() == false {
            println!(
                "The target library directory does not exist: {}",
                directory_name
            );
            println!(
                "Would you like to create: {:?} (y/n)?",
                &directory_path.to_str()
            );

            let yn_input_option: Option<char> = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as char);

            let yn_input = yn_input_option.unwrap();

            if yn_input.to_ascii_lowercase() != 'y' {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Target directory not created"),
                ));
            }

            match fs::create_dir_all(&directory_path) {
                Ok(_) => println!(
                    "Library directory created: {:?}",
                    &directory_path.canonicalize().unwrap()
                ),
                Err(err) => return Err(err),
            }
        }

        let library_filename = name + "." + LIB_EXTENSION;
        let library_path_os: OsString = OsString::from(directory_path.to_str().unwrap().clone());
        let library_path_str: String = String::from(directory_path.to_str().unwrap().clone());

        //let library_dir: &Path = Path::new(directory_path);

        // Write Library Meta Data to library directory
        // Check for existing metadata file
        let mut meta_file = PathBuf::from(&library_path_os);
        meta_file.push(&library_filename);

        if meta_file.exists() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("MugShots library already exists"),
            ));
        }

        println!("meta_file = {:?}", meta_file);

        let mut meta_f = File::create(meta_file)?;
        meta_f.write_fmt(format_args!("name : {}\n", &library_filename))?;
        meta_f.write_fmt(format_args!("directory : {}\n", library_path_str))?;
        meta_f.write_fmt(format_args!("image_count : 0\n"))?;

        Ok(ImageLibrary {
            name: library_filename,
            directory: library_path_os,
            image_count: 0,
            image_tree: BTreeMap::new(),
        })
    }

    pub fn add_image(filename: String) {
        ImageData::load_file(&filename);
    }
}


