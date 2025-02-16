use crate::models::{HiddenHeader, PlainHeader};
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit};
use anyhow::Context;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn encrypt<'a>(
    path_to_encrypt: &Path,
    new_file_path: &'a Path,
    key: Key<Aes256Gcm>,
) -> anyhow::Result<&'a Path> {
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let filename = path_to_encrypt
        .file_stem()
        .context("Could not get file stem")?;
    let mut file = File::open(path_to_encrypt).context("Could not open file")?;

    let mut plain_buffer = Vec::new();
    let _ = file.read_to_end(&mut plain_buffer)?;

    let plain = PlainHeader::new(nonce.into());
    let hidden = HiddenHeader::new(filename.into());

    let mut file = File::create(new_file_path.clone())?;

    let mut bytes: Vec<u8> = Vec::new();
    bytes.append(&mut plain.as_bytes().to_vec());

    let mut to_cipher = Vec::new();
    to_cipher.append(&mut hidden.as_bytes());
    to_cipher.append(&mut plain_buffer);
    let mut ciphertext = cipher.encrypt(&nonce, to_cipher.as_ref()).unwrap();
    bytes.append(&mut ciphertext);

    file.write_all(bytes.as_ref())?;

    Ok(new_file_path)
}
