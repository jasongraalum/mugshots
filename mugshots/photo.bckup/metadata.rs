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
extern crate chrono;
use chrono::prelude::*;

//
// Crypto Functions to create hash of images
//
//

extern crate uuid;
use uuid::Uuid;

extern crate image;
use image::GenericImage;

use std::path::Path;
use std::path::PathBuf;

use std::ffi::OsStr;
use std::ffi::OsString;

pub struct PhotoMetaData {
    hash: uuid::Uuid,
    img_type: image::ImageFormat,
    thumbhash: uuid::Uuid,
//    timestamp: chrono::NaiveDateTime,
//    last_mod: chrono::NaiveDateTime,
//    xdim: i32,
//    ydim: i32,
    filepath: OsString,
}

impl PhotoMetaData {
    pub fn new() -> PhotoMetaData
    {
        PhotoMetaData {
            hash: Uuid::nil(),
            thumbhash: Uuid::nil(),
            img_type: image::ImageFormat::PNG,
            filepath : OsString::from("temp")
        }
    }

    ///
    /// Return the image type
    /// 
    pub fn get_type(&self)->image::ImageFormat
    {
        self.img_type
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
