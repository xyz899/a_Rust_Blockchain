extern crate sha2;
use sha2::{Sha256, Digest};
use std::{string::String, io::stdin};

fn main() {
    let mut hash =  Sha256::new();

    println!("Message to hash : ");
    let mut message = String::new();
    stdin().read_line(&mut message).expect("Could not read data");

    hash.update(message);

    let result = hash.finalize();
    println!("Hashed : {:x}", result);

}
