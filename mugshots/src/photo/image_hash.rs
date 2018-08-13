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
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Result;



#[derive(Debug)]
pub struct ImageHash {
    value : [u8; 32],
}

impl ImageHash {
    pub fn nil() -> ImageHash
    {
        ImageHash { value : [0; 32]}
    }

    pub fn load(slice : &[u8]) -> ImageHash {
        ImageHash { value : array_ref!(slice, 0, 32).clone()}

    }

    pub fn new(mut image_f: File) -> Result<ImageHash>
    {
        let hash_result = Sha256::digest_reader(&mut image_f);
        match hash_result {
            Err(err) => Err(err),
            Ok(hash) => {
                Ok(ImageHash { value : array_ref!(hash.as_slice(),0, 32).clone() })
            }
        }
    }

}

impl PartialEq for ImageHash {
    fn eq(&self, other: &ImageHash) -> bool {
        self.value.iter().zip(other.value.iter()).all(|(a,b)| a == b)
    }
}



