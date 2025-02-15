use std::mem::size_of;

use crate::modules::Version;

const FILE_EXTENSION: &'static str = "wee";

///The plain header is the unencrypted part of the file which contains the `wee` file extensions,
/// the version of weecrypt used to encrypt the file with, and the nonce used to encrypt the file with.
#[derive(Debug)]
pub struct PlainHeader {
    ///3 Bytes showing the `wee` file extension in utf-8.
    file_extension: [u8; 3],
    ///The version of `weecrypt` used to encrypt the file with.
    version: Version,
    ///The nonce used to encrypt the file with.
    pub(crate) nonce: [u8; 12],
}

impl PlainHeader {
    pub const SIZE: usize = size_of::<Self>();

    pub fn new(nonce: [u8; 12]) -> Self {
        Self {
            file_extension: <[u8; 3]>::try_from(FILE_EXTENSION.as_bytes()).unwrap(),
            version: Version {
                major: 0,
                minor: 0,
                patch: 1,
            },
            nonce,
        }
    }

    pub fn as_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0; Self::SIZE];
        bytes[..3].copy_from_slice(&self.file_extension);
        bytes[3] = self.version.major;
        bytes[4] = self.version.minor;
        bytes[5] = self.version.patch;
        bytes[6..].copy_from_slice(&self.nonce);
        bytes
    }

    pub fn from_bytes(bytes: [u8; Self::SIZE]) -> Self {
        Self {
            file_extension: <[u8; 3]>::try_from(&bytes[..3]).unwrap(),
            version: Version {
                major: bytes[3],
                minor: bytes[4],
                patch: bytes[5],
            },
            nonce: <[u8; 12]>::try_from(&bytes[6..]).unwrap(),
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
