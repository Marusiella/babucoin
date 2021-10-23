use std::fmt::{self, Display};


struct Block {
    index: u64,
    previus_hash: String,
    timestamp: std::time::Instant,
    data: Vec<String>,
    hash: Option<String>
}
struct BlockChain {
    blocks: Vec<Block>,
    pending_transactions: Vec<String>
}
trait Create_block {
    fn new (index: u64,
        previus_hash: String,
        timestamp: std::time::Instant,
        data: Vec<String>,
        hash: Option<String>) -> Self;
}
impl Create_block for Block {
    fn new(index: u64, previus_hash: String, timestamp: std::time::Instant, data: Vec<String>, hash: Option<String>) -> Block {
        Block {
            index,
            previus_hash,
            timestamp,
            data,
            hash
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
            pending_transactions: Vec::new()
        }
    }
}

impl BlockChain {
    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
    fn add_transaction(&mut self, transaction: String) {
        self.pending_transactions.push(transaction);
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{} {:?} {:?} {:?} {:?}", self.index, self.previus_hash, self.timestamp, self.data, self.hash)
    }
}

// fn calculate_hash(index: u64, previus_hash: String, timestamp: std::time::Instant, data: Vec<String>) -> String {
//     use sha2::{Sha256,Digest};
//     let mut hasher = Sha256::new();
//     let before = index.to_string().parse::<String>().unwrap() + &previus_hash + &format!("{:?}",timestamp) + &format!("{:?}", data);
//     hasher.update(before.as_bytes());
//     format!("{:02x}",hasher.finalize())
// }

fn calculate_hash_proof(index: u64, previus_hash: String, timestamp: std::time::Instant, data: Vec<String>, proof: String) -> (String, u128) {
    use sha2::{Sha512,Digest};
    let mut hasher = Sha512::new();
    let before = index.to_string().parse::<String>().unwrap() + &previus_hash + &format!("{:?}",timestamp) + &format!("{:?}", data);
    hasher.update(before.as_bytes());
    let steps: u128 = std::u128::MAX;
    let mut i = 0;
    for x in 0..steps {
        // let mut s = hasher.clone();
        if format!("{:02x}",hasher.clone().finalize())[..proof.len()] == proof {
            // println!("{} difficulty: {}",format!("{:02x}",hasher.clone().finalize()), x);
            i = x;
            break;
        } else {
            // println!("{:02x}", hasher.clone().finalize());
            hasher.update(x.to_string().as_bytes());
        }
        // if x%1000000 == 0 { println!("{}",x)}
    }
    (format!("{:02x}",hasher.finalize()),i)
}
fn main() {
    // println!("{}",calculate_hash(1,"sss".to_string(),std::time::Instant::now(),vec!["dd".to_string()]))
    let first = Block {
        index: 0,
        previus_hash: "None".to_string(),
        timestamp: std::time::Instant::now(),
        data: vec!["sss send 2 to yyy".to_string()],
        hash: None
    };
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


    let mut blockchin: BlockChain = blockchain::new();
    let block: Block = Create_block::new(1,"sss".to_string(),std::time::Instant::now(),vec!["dd".to_string()], None);
    blockchin.add_block(block);


    let (x,y) = calculate_hash_proof(1,"sss".to_string(),std::time::Instant::now(),vec!["dd".to_string()], "bab0".to_string());
    println!("Mined {} Diff: {}",x,y);
    

}
