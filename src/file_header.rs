use std::mem::size_of;

use crate::modules::Version;

const FILE_EXTENSION: &'static str = "wee";

///The plain header is the unencrypted part of the file which contains the `wee` file extensions,
/// the version of weecrypt used to encrypt the file with, and the nonce used to encrypt the file with.
#[derive(Debug)]
#[repr(C)]
pub struct PlainHeader {
    ///3 Bytes showing the `wee` file extension in utf-8.
    pub file_extension: [u8; 3],
    ///The version of `weecrypt` used to encrypt the file with.
    pub version: Version,
    ///The nonce used to encrypt the file with.
    pub nonce: [u8; 12],
}

impl PlainHeader {
    pub const SIZE: usize = size_of::<Self>();

    pub fn new(nonce: [u8; 12]) -> Self {
        let version: Version = env!("CARGO_PKG_VERSION").parse().expect("Invalid version");
        Self {
            file_extension: <[u8; 3]>::try_from(FILE_EXTENSION.as_bytes()).unwrap(),
            version,
            nonce,
        }
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8; 18] {
        //using unsafe to make this free.
        unsafe { &*(self as *const PlainHeader as *const [u8; 18]) }
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        let mut file_extension = [0; 3];
        let mut version = [0; 3];
        let mut nonce = [0; 12];

        file_extension.copy_from_slice(&bytes[..3]);
        version.copy_from_slice(&bytes[3..6]);
        nonce.copy_from_slice(&bytes[6..18]);

        Self {
            file_extension,
            version: version.into(),
            nonce,
        }
    }
}

#[derive(Debug)]
pub struct HiddenHeader {
    pub original_name: String,
    pub original_name_length: u8,
}

impl HiddenHeader {
    pub fn new(original_name: String) -> Self {
        Self {
            original_name_length: original_name.len() as u8,
            original_name,
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(self.original_name_length);
        bytes.append(&mut self.original_name.as_bytes().to_vec());
        bytes
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            original_name: String::from_utf8(bytes[1..].to_vec()).unwrap(),
            original_name_length: bytes[0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_header_as_bytes() {
        let header = PlainHeader::new([3; 12]);
        let bytes = header.as_bytes();

        println!("{:?}", bytes);

        assert_eq!(bytes.len(), 18);
        //check that first 3 bytes are the file extension
        assert_eq!(bytes[..3], [119, 101, 101]);
        //check that the next 3 bytes are the version
        let version: Version = env!("CARGO_PKG_VERSION").parse().unwrap();
        assert_eq!(bytes[3], version.major);
        assert_eq!(bytes[4], version.minor);
        assert_eq!(bytes[5], version.patch);
        //check that the last 12 bytes are the nonce and all equal to 0
        assert_eq!(bytes[6..], [3; 12]);
    }
}
