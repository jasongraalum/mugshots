//
// Process a image file to add it to mugshots
// Build the metadata
//

use std;
use std::io;
//use std::io::prelude::*;
use std::io::{Read, Error, ErrorKind, SeekFrom};
use std::fs;
use std::path::Path;


use std::ffi::OsString;
//use image;
//use image::{GenericImage, ImageError};

//use sha2::{Digest, Sha512};
//use chrono::{DateTime, Utc};

use std::collections::BTreeMap;
use photo::ImageData;

const LIB_EXTENSION : &str = "mgst";

pub struct ImageLibrary {
    name: String,
    directory: OsString,
    image_count: u32,
    image_tree: std::collections::BTreeMap< u8, ImageData>,
}

impl ImageLibrary {
    pub fn create(name: String, directory_name : String) -> io::Result<ImageLibrary>
    {
        //
        // Check that name is a valid file name in directory
        // If not, either directory doesn't exist, isn't writable or there
        // already is a file by the same name, return IO Error
        //

        // Does directory exist, if not can I create it?
        let mut directory_path = Path::new(&directory_name);

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

            println!("The target library directory does not exist: {}", directory_name);
            println!("Would you like to create: {:?} (y/n)?", &directory_path.to_str());

            let yn_input_option: Option<char> = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as char);

            let mut yn_input = yn_input_option.unwrap();

            if yn_input.to_ascii_lowercase() != 'y' {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Target directory not created"),
                ));
            }

            match fs::create_dir_all(&directory_path) {
                Ok(_) => println!("Library directory created: {:?}", &directory_path.canonicalize().unwrap()),
                Err(err) => return Err(err)
            }
        }

        let library_filename = name + "."  + LIB_EXTENSION;
        let full_file_path: OsString  = OsString::from(directory_path.to_str().unwrap().clone());

        let library_dir: &Path = Path::new(directory_path);

        // Write Library Meta Data to library directory
        // Check for existing metadata file



        Ok(ImageLibrary { name: library_filename,  directory: full_file_path, image_count: 0, image_tree: BTreeMap::new() })
    }
}

#[test]
fn test_new_imagelibrary()
{
    let new_library : ImageLibrary = ImageLibrary::create("testLibrary".to_string(),"./test/testLibraries".to_string()).unwrap();
    let name = "testLibrary.".to_string() + LIB_EXTENSION;
    assert_eq!(new_library.name,name);
}
