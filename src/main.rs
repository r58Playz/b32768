use base32768::{decode, encode};
use std::env::args;
use std::io::{stdin, Read};

fn main() {
    match args()
        .nth(1)
        .expect("need either encode or decode")
        .as_str()
    {
        "encode" => {
            let mut input = Vec::new();
            stdin()
                .read_to_end(&mut input)
                .expect("failed to read stdin");
            print!("{}", encode(&input).expect("failed to encode"));
        }
        "decode" => {
            let mut input = String::new();
            stdin()
                .read_to_string(&mut input)
                .expect("failed to read stdin");
            let mut decoded: Vec<u8> = Vec::new();
            decode(&input, &mut decoded).expect("failed to decode");
            unsafe {
                print!(
                    "{}",
                    std::str::from_utf8_unchecked(&decoded)
                )
            };
        }
        "-h" | "--help" | "help" => {
            println!("usage: b32768 encode|decode");
            println!("       pipe the input into stdin.");
        }
        _ => {
            println!("invalid args")
        }
    }
}
