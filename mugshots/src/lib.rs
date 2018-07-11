#[macro_use]
extern crate arrayref;
///
/// Copyright © 2018 Jason Graalum
///
//
// mugshots Library
//
// A mugshots instances is made up of a database containing metadata
// for each photo and a repository containing the actual photo files.
//
// The respective models define the structures and APIs into each
//

//
// Sub module for all functions related to photo manipulations
// including creation and modification of metadata to be stored
// in the database
//
extern crate chrono;
extern crate image;
extern crate sha2;
extern crate uuid;
//
// Crypto Functions to create hash of images
//
//

pub mod photo;
pub mod library;

//
// Submodule for all functions related to database interface
//
//mod database;

//
// Submodule for all fuctions related to managing the repository of
// actual photo files
//
//mod repository;

//
// Submodule for all functions related to user interface to the
// mugshots instance
//
//mod interface;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
