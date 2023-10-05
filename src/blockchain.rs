extern crate sha2;
use sha2::{Sha256, Digest};


pub struct Block {
    pub block_number: i32,
    pub nonce: i32, 
    pub timestamp: i32,
    pub hash: String,
    pub prev_hash: String,

}

pub struct Blockchain {
    pub chain: Vec<Block>,
}


pub fn random_word(number: i32) -> String {
    if (number < 0 || number >= 5){
        panic!("Not possible");

    }
    
    let mut word: [&str; 5] = ["Fries","Time","Reality","Rich","Work"];
    return word[number];
}

impl Block {
    pub fn newBlock(block_number: i32,
    nonce: i32, 
    timestamp: i32,
    hash: String,
    prev_hash: String,) -> Block {
        hash = Sha256::new();
        let mut key = String::new();

        Block {
            block_number,
            nonce,
            timestamp,
            hash,
            prev_hash
        }
}
}
fn main() {
    let ZPHBlockchain = Blockchain {chain: Vec::new()};

}