extern crate bip_bencode;

use bip_bencode::{BencodeRef, BDecodeOpt, BRefAccess};
use std::fs::File;
use std::io::{Error, Read};

pub fn file_to_bytes(fname: &String) -> Result<Vec<u8>, Error> {
    let mut f = File::open(fname)?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn decode_bytes(bytes: &Vec<u8>) -> BencodeRef {
    BencodeRef::decode(bytes, BDecodeOpt::default()).unwrap()
}

pub fn get_announce(bencode: &BencodeRef) -> String {
    bencode.dict().unwrap().lookup("announce".as_bytes()).unwrap().str().unwrap().to_string()
}