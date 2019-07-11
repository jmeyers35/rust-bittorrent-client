extern crate clap;
mod parse;
use clap::{App, Arg};
fn main() {
    let matches = App::new("Bittorrent Client")
        .version("1.0")
        .author("Jacob Meyers")
        .arg(Arg::with_name("FILE")
        .help("Torrent file")
        .required(true))
        .get_matches();

    let fname = String::from(matches.value_of("FILE").unwrap());
    
    let file_read_result = parse::file_to_bytes(&fname).unwrap();

    let bencode_bytes = parse::decode_bytes(&file_read_result);

    println!("{}", parse::get_announce(&bencode_bytes));


}
