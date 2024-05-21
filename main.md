extern crate sha2;
use sha2::{Sha256, Digest};
use rand::Rng;
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};
mod hash;


// Block and Blockchain structures
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


// pub fn random_word(number: i32) -> String {
//     if number < 0 || number >= 5 {
//         panic!("Not possible");

//     }
    
//     let mut word: [&str; 5] = ["Fries","Time","Reality","Rich","Work"];
//     return word[number];
// }


// impl <StructName> = serves to use a defined struct
impl Blockchain {

   pub fn new() -> Blockchain {
    Blockchain {chain: Vec::new()}
   }

   pub fn add_block(&mut self, new_block: Block)  {
    self.chain.push(new_block);
   }

   // we want to change the hash and prev_hash to Sha256 
    pub fn new_block(block_number: i32,nonce: i32, timestamp: i32, hash: String,prev_hash: String) -> Block {
        
        let mut blocknum: i32 = 0; // block number
        
        block_number = blocknum + 1; //incrementing the number of blocks
        
        // New hashset to store nonce values
        let mut nonce_num = HashSet::new();
        
        // initialize a new rand thread, output random number 
        let mut rng = rand::thread_rng();
        let mut nonce_stored: i32 = rng.gen_range(999..99999);  

        // check if the Nounce already is stored in Hashset :
        // # true : New random nonce
        // # false : push to hashset
        if !nonce_num.contains(&nonce_stored) {
            let mut new_nonce = rand::thread_rng().gen_range(999..99999);
        } else {
            !nonce_num.insert(nonce_stored);
            nonce = nonce_stored;
        }

        // Initialize the SytemTime where we call UNIX EPOCH like the block.timestamp
        let start = SystemTime::now();         
        // TWO variables : since_the_epoch, timestamp
        // since_the_epoch is to retrive the unix epoch meaning duration since plus an expect in case of an error with the library
        // timestamp to convert in seconds i64
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Error while fetching the block timestamp");
        let timestamp = since_the_epoch.as_secs() as i32;


        prev_hash = ("TODO").to_string();
        
        //     match Blockchain.last() {
        //     Ok(value) => { 
        //         value.to_string()
        //     },
        //     Err(err) => {
        //         ("The vector is empty").to_string()
        //     },
        // };

        // =notice= : genhash to initialize sha256, hashing part of a block
        // takes an update with format! to formatted string 
        let mut genhash = Sha256::new();
        genhash.update(format!("{}{}{}{}", block_number, nonce, timestamp, prev_hash));
    
        // finally finalize to output a hash of 64 characters 
        // TO DO FINALIZE NOT WORKING HERE
        // hash = genhash;
        hash = ("TODO").to_string();

        // Pushing everything to new Block
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
    
    let mut zph_blockchain = Blockchain::new();

    for _ in 0..10 {
        // let new_block = zph_blockchain.add_block();
        // zph_blockchain.add_block(new_block);

    }

    for block in zph_blockchain.chain.iter() {
        println!("Block number: {}", block.block_number);
        println!("Nonce: {}", block.nonce);
        println!("Timestamp: {}", block.timestamp);
        println!("Hash: {}", block.hash);
        println!("Previous Hash: {}", block.prev_hash);
        println!("-------------------------");
    }
    
    // block_number: i32,nonce: i32, timestamp: i32, hash: String,prev_hash: String

}