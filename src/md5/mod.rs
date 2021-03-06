//! module md5 implements the MD5 hash algorithm as defined in RFC 1321.
//!
//! MD5 is cryptographically broken and should not be used for secure applications.

use md5::{Digest, Md5};

pub use super::Hash;

/// A MD5 is an instance of MD5.
//pub struct MD5(Md5);
pub type MD5 = Md5;

/// The blocksize of MD5 in bytes.
pub const BLOCK_SIZE: usize = 64;
/// The size of an MD5 checksum in bytes.
pub const SIZE: usize = 16;

impl Hash for MD5 {
    fn size() -> usize {
        Md5::output_size()
    }

    fn block_size() -> usize {
        BLOCK_SIZE
    }

    fn reset(&mut self) {
        Digest::reset(self);
    }

    fn sum(&mut self) -> Vec<u8> {
        self.clone().result().to_vec()
    }
}

/// new returns a new hash.Hash computing the MD5 checksum.
pub fn new() -> MD5 {
    Md5::new()
}

/// sum returns the MD5 checksum of the data.
pub fn sum(data: &[u8]) -> [u8; SIZE] {
    let d = Md5::digest(data);

    let mut out = [0u8; SIZE];
    (&mut out[..]).copy_from_slice(d.as_slice());

    out
}
