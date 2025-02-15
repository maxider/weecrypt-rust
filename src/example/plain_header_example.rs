use std::fs::File;
use std::io::{Read, Write};
use crate::file_header;

fn write_file() -> std::io::Result<()> {
    let mut file = File::create_new("test.wee")?;
    let header = file_header::PlainHeader::new([0; 12]);
    file.write_all(header.as_bytes())?;
    Ok(())
}

fn read_plain_header() -> std::io::Result<file_header::PlainHeader> {
    let mut file = File::open("test.wee")?;
    let mut header = [0; file_header::PlainHeader::SIZE];
    file.read_exact(&mut header)?;
    Ok(file_header::PlainHeader::from_bytes(&header))
}

pub fn run_file_example() -> std::io::Result<()> {
    let _ = std::fs::remove_file("test.wee");
    write_file()?;
    let header = read_plain_header()?;
    println!("{:?}", header);
    Ok(())
}