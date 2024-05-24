use std::fs::File;
use std::io::{Read, Write};

use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Key};

use crate::file_header::{HiddenHeader, PlainHeader};

mod file_header;
mod example;

fn main() {
    // encrypte_file().unwrap();
    decrypt_file().unwrap();

    // let plaintext = b"Hello, world!";
    // let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).unwrap();
    // let decrypted = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
    // //decrypted to string with utf8¿¿
    // let decrypted = String::from_utf8(decrypted.to_vec()).unwrap();
    // println!("Encrypted: {:?}", ciphertext);
    // println!("Decrypted: {:?}", decrypted);
}

fn encrypte_file() -> std::io::Result<()> {
    let key = get_key();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let mut file = File::open("hello.txt")?;
    let mut plain_buffer = Vec::new();
    let _ = file.read_to_end(&mut plain_buffer)?;

    let plain = PlainHeader::new(nonce.into());
    let hidden = HiddenHeader::new("hello.txt".to_string());

    let mut file = File::create("test_123_nicht_der_alte_name.wee")?;

    let mut bytes: Vec<u8> = Vec::new();
    bytes.append(&mut plain.as_bytes().to_vec());

    let mut to_cipher = Vec::new();
    to_cipher.append(&mut hidden.as_bytes());
    to_cipher.append(&mut plain_buffer);
    let mut ciphertext = cipher.encrypt(&nonce, to_cipher.as_ref()).unwrap();
    bytes.append(&mut ciphertext);

    file.write_all(bytes.as_ref())?;
    Ok(())
}

fn decrypt_file() -> std::io::Result<()> {
    let key = get_key();
    let cipher = Aes256Gcm::new(&key);
    let mut file = File::open("test_123_nicht_der_alte_name.wee")?;
    let mut read_buffer = Vec::new();
    let _ = file.read_to_end(&mut read_buffer)?;

    let plain = PlainHeader::from_bytes(read_buffer[..PlainHeader::SIZE].try_into().unwrap());
    let rest_enc = &read_buffer[PlainHeader::SIZE..];

    let nonce = plain.nonce;
    
    return Ok(());
    
    let decrypted = cipher.decrypt(&nonce.into(), rest_enc).unwrap();
    let size = decrypted[0];
    let name = &decrypted[1..=size as usize];
    let name = String::from_utf8(name.to_owned()).unwrap();

    let mut file = File::create(name)?;
    file.write_all(&decrypted[(size as usize + 1)..])?;

    Ok(())
}

fn get_key() -> Key<Aes256Gcm> {
    let mut key: Vec<u8> = b"my_secret_key".to_vec();
    while key.len() < 32 {
        key.push(0);
    };
    Key::<Aes256Gcm>::from_slice(&key[..]).clone()
}