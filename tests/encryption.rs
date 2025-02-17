use weecrypt::core::{decrypt, encrypt};
use weecrypt::models::PlainHeader;

#[test]
fn test_full_flow() {
    //create test dirs and files
    let test_dir = std::env::temp_dir();
    let test_dir = test_dir.join("weecrypt_test/");
    std::fs::create_dir_all(&test_dir).unwrap();
    let test_file = test_dir.join("test_encryption_plain.txt");

    //add some content to the file
    std::fs::write(&test_file, "Hello, World!").unwrap();

    //encrypt the file
    let enc_path = test_dir.join("test_encryption_enc.wee");
    let key = [0; 32];
    let enc_file = encrypt(&test_file, &enc_path, &key).unwrap();

    //read bytes of the encrypted file
    let enc_bytes = std::fs::read(&enc_file).unwrap();
    //check plain header
    let plain_header = PlainHeader::from_bytes(enc_bytes[..PlainHeader::SIZE].try_into().unwrap());
    assert_eq!(plain_header.file_extension, [119, 101, 101]); // WEE
    assert_eq!(plain_header.version, 0);
    assert_ne!(plain_header.nonce, [0; 12]);

    //decrypt the file
    let dec_dir = test_dir.join("decrypted/");
    std::fs::create_dir_all(&dec_dir).unwrap();
    let dec_path = decrypt(&enc_file, &dec_dir, &key).unwrap();
    println!("decrypted path: {:?}", dec_path);

    //compare test_file with decrypted file
    let dec_file = dec_dir.join("test_encryption_plain.txt");
    let dec_content = std::fs::read_to_string(&dec_file).unwrap();
    let test_content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(dec_content, test_content);
}
