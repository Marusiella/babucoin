#[macro_use]
extern crate derive_new;


use std::{fmt::{self, Debug, Display}};

use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize,Debug, Clone)]
struct Block {
    index: u64,
    previus_hash: String,
    timestamp: String,
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
trait Createblock {
    fn new(
        index: u64,
        previus_hash: String,
        timestamp: String,
        data: Vec<Transaction>,
        hash: String,
        proof: u128,
    ) -> Self;
}
impl Createblock for Block {
    fn new(
        index: u64,
        previus_hash: String,
        timestamp: String,
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
trait Blockchain {
    fn new() -> Self;
}
impl Blockchain for BlockChain {
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
                chrono::offset::Utc::now().to_string(),
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
            timestamp: chrono::offset::Utc::now().to_string(),
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
    timestamp: String,
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

        if format!("{:02x}", hasher.clone().finalize())[..proof.len()] == proof {
            println!("Mined! : {} difficulty: {}",format!("{:02x}",hasher.clone().finalize()), x);
            i = x;
            break;
        } else {

            hasher.update(x.to_string().as_bytes());
        }

    }
    (format!("{:02x}", hasher.finalize()), i)
}
fn main() {

    let proof = "0ac8";
    let mut blockchin: BlockChain = Blockchain::new();
    let s: Transaction = Transaction::new("Olek".to_string(), "Anna".to_string(), 100);
    let time = chrono::offset::Utc::now().to_string();
    let calc = calculate_hash_proof(1, "".to_string(), time.clone(), vec![s.clone()], proof);
    let start: Block = Createblock::new(1, "".to_string(), time, vec![s.clone()], calc.0, calc.1);
    blockchin.add_block_thirst(start);
    let s: Transaction = Transaction::new("Olek".to_string(), "Anna".to_string(), 20);
    blockchin.add_block(vec![s],proof);
    let json = serde_json::to_string_pretty(&blockchin).unwrap();
    println!("{}", json);
}
