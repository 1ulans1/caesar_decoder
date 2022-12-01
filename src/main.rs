extern crate core;

mod decoder;

use std::io;
use crate::decoder::Decode;

fn main() {
    println!("Enter a ciphertext: ");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("failed to readline");

    let mut decode: Vec<Decode> = decoder::decode_message(message);
    let decode: Vec<Decode> = decode.into_iter().filter(|a| a.word_matches != 0).collect();
    decode.iter().for_each(|d| println!("{}", d.deciphered));
}
