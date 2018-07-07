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



