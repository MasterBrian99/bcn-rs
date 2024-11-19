use std::thread::sleep;
use serde::{Deserialize, Serialize};
use sha256::digest;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::util::mine;
use crate::util::mine::DIFFICULTY;

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct Block {
    index: u64,
    timestamp: u128,
    bpm: u64,
    hash: String,
    prev_hash: String,
    difficulty: u32,
    nonce:String
}

impl Block {
    pub fn new()->Block {
        let mut block:Block=Block{
            hash: String::new(),
            index: 0,
            bpm: 0,
            prev_hash: "".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            nonce:"".to_string(),
            difficulty:DIFFICULTY
        };
        block.hash=calculate_hash(&block);
       block
    }
    pub fn generate_block(&mut self, old_block: &Block, bpm: u64) -> Block {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let mut block = Block {
            index: &old_block.index + 1,
            timestamp: now,
            bpm,
            prev_hash: old_block.hash.to_string(),
            hash: "".to_string(),
            difficulty:DIFFICULTY,
            nonce:"".to_string()
        };
        for i in 0.. {
            block.nonce = format!("{:x}", i);
            let new_hash = calculate_hash(&block);
            if !mine::is_hash_valid(&new_hash, block.difficulty as usize) {
                println!("{} do more work!", new_hash);
                println!("{}", format!("{:x}", i));
                sleep(Duration::from_secs(1));
                continue;
            } else {
                println!("{} work done!", new_hash);
                block.hash = new_hash;
                break;
            }
        }
        // let hash = calculate_hash(&block);
        // block.hash = hash;
        block
    }
    // pub fn replace_chain(new_blocks:Vec<Block>,mut chain:web::Data<AppState>){
    //     let mut blockchain=chain.blockchain.lock().unwrap();
    //     if new_blocks.len() > blockchain.len(){
    //         chain=web::Data::new(AppState {
    //             blockchain:Mutex::new(new_blocks)
    //         });
    //     }
    //
    // }
}
pub fn calculate_hash( block:&Block) -> String {
    let mut record = "".to_string();
    record.push_str(&block.index.to_string());
    record.push_str(&block.timestamp.to_string());
    record.push_str(&block.bpm.to_string());
    record.push_str(&block.prev_hash);
    record.push_str(&block.nonce);
    record.push_str(&block.difficulty.to_string());
    let val = digest(record);
    val
}
pub fn block_valid(block:&Block,  old_block: &Block) -> bool {
    if old_block.index + 1 != block.index {
        return false;
    }
    if old_block.hash != block.prev_hash {
        return false;
    }
    let hash=calculate_hash(&block).to_string().clone();
    if hash != block.hash {
        return false;
    }

    true
}
