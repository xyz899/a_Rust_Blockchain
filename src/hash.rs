extern crate sha2;
use sha2::{digest::generic_array::GenericArray, Digest, Sha256};
use std::{string::String, io::stdin};

fn hash() {
    let mut hash =  Sha256::new();

    println!("Message to hash : ");
    let mut message = String::new();
    stdin().read_line(&mut message).expect("Could not read data");

    hash.update(message);

    let result = hash.finalize();
    // println!("Hashed : {:x}", result);
    result;

}
