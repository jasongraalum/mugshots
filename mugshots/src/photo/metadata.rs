#[allow(dead_code)]
//
// The photo_metadata will match the database schema. It would be
// nice to have a single source defining both, but that is for
// another day.
//
//

//
// chrono NaiveDateTime is used to hold the photo timestamp
// and lastModified date/time
//
//

use chrono::prelude::*;

use image::*;
//use uuid::*;

use image::GenericImage;

use std::path::Path;
use std::path::PathBuf;

use std::ffi::OsStr;
use std::ffi::OsString;


pub struct PhotoMetaData {
    hash: [u8; 64],
    thumbhash: [u8; 64],
    img_type: ::image::ImageFormat,
//    timestamp: chrono::NaiveDateTime,
//    last_mod: chrono::NaiveDateTime,
//    xdim: i32,
//    ydim: i32,
    filepath: OsString,
}


implra> PhotoMetaData {
    pub fn new() -> PhotoMetaData<'a>
    {
        PhotoMetaData {
            hash: self.gen_image_hash(),
            thumbhash: [u8; 64],
            img_type: ::image::ImageFormat::PNG,
            filepath : OsString::from("temp")
        }
    }

    pub fn new(filename &str) -> PhotoMetaData<'a>
    {
        PhotoMetaData {
            hash: [u8; 64],
            thumbhash: [u8; 64],
            img_type: ::image::ImageFormat::PNG,
            filepath : OsString::from("temp")
        }
    }

    ///
    /// Return the image type
    /// 
    pub fn get_type(&self) -> ::image::ImageFormat
    {
        self.img_type
    }

    ///
    /// Set the image type
    /// 
    pub fn set_type(&mut self, img_type: ::image::ImageFormat)
    {
        self.img_type = img_type;
    }

    ///
    /// Set the photo hash value 
    /// 
    pub fn set_hash(&mut self, hash: [u8; 64])
    {
        self.hash = hash;
    }

    ///
    /// Return the photo hash value 
    /// 
    pub fn get_hash(&mut self) -> &[u8]
    {
        &self.hash
    }

    ///
    /// Set the path of the photo file
    ///
    pub fn set_path(&mut self,pathname: PathBuf)
    {
        self.filepath = pathname.canonicalize().unwrap().into_os_string();
    }

    pub fn get_path(&self) -> &OsString
    {
        &self.filepath 
    }

}


#[test]
fn test_setget_path()
{
    let path_str = "/tmp/".to_string();

    let p = PathBuf::from(&path_str);
    let os_str = p.into_os_string();

    let p = PathBuf::from(&path_str);

    let mut img_metadata =  PhotoMetaData::new();
    img_metadata.set_path(p);

    let p = PathBuf::from(&path_str).canonicalize().unwrap();
    assert_eq!(img_metadata.get_path(),&p.into_os_string());
}
