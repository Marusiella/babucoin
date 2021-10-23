#[macro_use]
extern crate derive_new;

use std::{fmt::{self, Debug, Display}};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::time::Instant;

#[derive(Serialize, Deserialize,Debug, Clone)]
struct Block {
    index: u64,
    previus_hash: String,
    timestamp: DateTime<Utc>,
    data: Vec<Transaction>,
    hash: String,
    proof: Option<u128>,
}
#[derive(Serialize, Deserialize,Debug, Clone)]
struct BlockChain {
    blocks: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize,Debug, Clone, new)]
struct Transaction {
    sender: String,
    reciver: String,
    amount: u64,
}
// impl Debug for Transaction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}-{}-{}", self.sender, self.reciver, self.amount)
//     }
// }
impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.sender, self.reciver, self.amount)
    }
}
// trait NewTransaction {
//     fn new(sender: String, reciver: String, amount: u64) -> Transaction;
// }
// impl NewTransaction for Transaction {
//     fn new(sender: String, reciver: String, amount: u64) -> Transaction {
//         Transaction {
//             sender,
//             reciver,
//             amount
//         }
//     }
// }
trait Create_block {
    fn new(
        index: u64,
        previus_hash: String,
        timestamp: DateTime<Utc>,
        data: Vec<Transaction>,
        hash: String,
        proof: u128,
    ) -> Self;
}
impl Create_block for Block {
    fn new(
        index: u64,
        previus_hash: String,
        timestamp: DateTime<Utc>,
        data: Vec<Transaction>,
        hash: String,
        proof: u128,
    ) -> Block {
        Block {
            index,
            previus_hash,
            timestamp,
            data,
            hash,
            proof: Some(proof),
        }
    }
}
trait blockchain {
    fn new() -> Self;
}
impl blockchain for BlockChain {
    fn new() -> BlockChain {
        BlockChain {
            blocks: Vec::new(),
            pending_transactions: Vec::new(),
        }
    }
}

impl BlockChain {
    fn add_block_thirst(&mut self, block: Block) {
        self.blocks.push(block);
    }
    fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }
    fn add_block(&mut self, data: Vec<Transaction>, proof: &str) {
        let (calculate_hash, proof) = calculate_hash_proof(
            self.blocks
                .last()
                .expect("Can't get previous block index")
                .index
                + 1,
            self.blocks
                .last()
                .expect("Can't get previous block hash")
                .hash
                .clone(),
                chrono::offset::Utc::now(),
            data.clone(),
            proof,
        );

        self.add_block_thirst(Block {
            index: self
                .blocks
                .last()
                .expect("Can't get previous block index")
                .index
                + 1,
            previus_hash: self
                .blocks
                .last()
                .expect("Can't get previous block hash")
                .hash
                .clone(),
            timestamp: chrono::offset::Utc::now(),
            data,
            hash: calculate_hash,
            proof: Some(proof),
        })
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} {:?} {:?} {:?}",
            self.index, self.previus_hash, self.timestamp, self.data, self.hash
        )
    }
}

// fn calculate_hash(index: u64, previus_hash: String, timestamp: std::time::Instant, data: Vec<String>) -> String {
//     use sha2::{Sha256,Digest};
//     let mut hasher = Sha256::new();
//     let before = index.to_string().parse::<String>().unwrap() + &previus_hash + &format!("{:?}",timestamp) + &format!("{:?}", data);
//     hasher.update(before.as_bytes());
//     format!("{:02x}",hasher.finalize())
// }

fn calculate_hash_proof(
    index: u64,
    previus_hash: String,
    timestamp: DateTime<Utc>,
    data: Vec<Transaction>,
    proof: &str,
) -> (String, u128) {
    use sha2::{Digest, Sha512};
    let proof = proof.to_owned();
    let mut hasher = Sha512::new();
    let before = index.to_string().parse::<String>().unwrap()
        + &previus_hash
        + &format!("{:?}", timestamp)
        + &format!("{:?}", data);
    hasher.update(before.as_bytes());
    let steps: u128 = std::u128::MAX;
    let mut i = 0;
    for x in 0..steps {
        // let mut s = hasher.clone();
        if format!("{:02x}", hasher.clone().finalize())[..proof.len()] == proof {
            // println!("{} difficulty: {}",format!("{:02x}",hasher.clone().finalize()), x);
            i = x;
            break;
        } else {
            // println!("{:02x}", hasher.clone().finalize());
            hasher.update(x.to_string().as_bytes());
        }
        // if x%1000000 == 0 { println!("{}",x)}
    }
    (format!("{:02x}", hasher.finalize()), i)
}
fn main() {
    // println!("{}",calculate_hash(1,"sss".to_string(),std::time::Instant::now(),vec!["dd".to_string()]))
    // let first = Block {
    //     index: 0,
    //     previus_hash: "None".to_string(),
    //     timestamp: std::time::Instant::now(),
    //     data: vec!["sss send 2 to yyy".to_string()],
    //     hash: None
    // };
    // let sx: Block = Create_block::new(0, "None".to_string(), std::time::Instant::now(), vec!["ss".to_string()], None);
    // println!("{}",sx);
    // for _ in 0..10000 {
    // println!("{}",calculate_hash(1,"sss".to_string(),std::time::Instant::now(),vec!["dd".to_string()]))
    // // }
    // use sha2::{Sha512,Digest};
    // let mut hasher = Sha512::new();
    // let before = "qwerty";
    // hasher.update(before.as_bytes());

    // println!("{}",format!("{:02x}",hasher.finalize()));
    let proof = "0k";
    let mut blockchin: BlockChain = blockchain::new();
    let s: Transaction = Transaction::new("Olek".to_string(), "Anna".to_string(), 100);
    let time = chrono::offset::Utc::now();
    let calc = calculate_hash_proof(1, "".to_string(), time, vec![s.clone()], proof);
    let start: Block = Create_block::new(1, "".to_string(), time, vec![s.clone()], calc.0, calc.1);
    blockchin.add_block_thirst(start);
    let s: Transaction = Transaction::new("Olek".to_string(), "Anna".to_string(), 20);
    blockchin.add_block(vec![s],proof);
    println!("{:?}", blockchin);

    // let (x,y) = calculate_hash_proof(1,"sss".to_string(),std::time::Instant::now(),vec!["dd".to_string()], "bab0".to_string());
    // println!("Mined {} Diff: {}",x,y);
}
