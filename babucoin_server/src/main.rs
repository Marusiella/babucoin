#[macro_use]
extern crate derive_new;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{self, Debug, Display};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    previus_hash: String,
    timestamp: String,
    data: Vec<Transaction>,
    hash: String,
    proof: Option<u128>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockChain {
    blocks: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug, Clone, new)]
struct Transaction {
    sender: String,
    reciver: String,
    amount: u64,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.sender, self.reciver, self.amount)
    }
}

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
    fn get_pendding_transactions(&self) -> Vec<Transaction> {
        self.pending_transactions.clone()
    }
    fn clear_pendding_transactions(&mut self) {
        self.pending_transactions.clear();
    }
    fn is_good(&self) -> bool {
        for x in 0..self.blocks.len() - 1 {
            if self.blocks[x].hash != self.blocks[x + 1].previus_hash {
                return false;
            }
        }
        return true;
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
            println!(
                "Mined! : {} difficulty: {}",
                format!("{:02x}", hasher.clone().finalize()),
                x
            );
            i = x;
            break;
        } else {
            hasher.update(x.to_string().as_bytes());
        }
    }
    (format!("{:02x}", hasher.finalize()), i)
}



#[get("/getblockchain")]
async fn hello() -> impl Responder {
    let contents = std::fs::read_to_string("blockchain.json")
        .expect("Something went wrong reading the file");
    let bc: BlockChain = serde_json::from_str(&contents).unwrap();
    if bc.is_good() {
        println!("Is valid")
    } else {
        panic!("Can't valid blockchain.json");
    }
    HttpResponse::Ok().body(serde_json::to_string(&bc).unwrap())
}
#[actix_web::main]

async fn main() -> std::io::Result<()>  {
    
    let contents = std::fs::read_to_string("blockchain.json")
        .expect("Something went wrong reading the file");
    let bc: BlockChain = serde_json::from_str(&contents).unwrap();
    if !bc.is_good() {
        panic!("Can't valid blockchain.json");
    }
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn create_pending(blockchin: &mut BlockChain, proof: &str) {
    let mut tran: Vec<Transaction> = Vec::new();
    for x in blockchin.clone().get_pendding_transactions() {
        tran.push(x.clone());
        if tran.len() == 5 {
            blockchin.add_block(tran.clone(), proof);
            tran.clear();
        } else if blockchin.clone().get_pendding_transactions().len() < 5 {
            blockchin.add_block(tran.clone(), proof);
            tran.clear();
        }
    }
    blockchin.clear_pendding_transactions();
}
