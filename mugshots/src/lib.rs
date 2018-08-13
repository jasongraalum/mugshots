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
#[macro_use]
extern crate arrayref;
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
extern crate html5ever;
extern crate reqwest;
extern crate url;


//
// Crypto Functions to create hash of images
//
//

pub mod photo;
pub mod library;
pub mod web_map;

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

