use std::fmt::{self, Display};



struct Block {
    index: u64,
    previus_hash: String,
    timestamp: std::time::Instant,
    data: Vec<String>,
    hash: Option<String>
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
impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {}, {})", self.index, self.previus_hash, format!("{:?}", self.timestamp), format!("{:?}", self.data), format!("{:?}", self.hash) )
    }

}

fn calculate_hash(index: u64, previus_hash: String, timestamp: std::time::Instant, data: Vec<String>) -> String {
    use sha2::{Sha256,Digest};
    let mut hasher = Sha256::new();
    let before = index.to_string().parse::<String>().unwrap() + &previus_hash + &format!("{:?}",timestamp) + &format!("{:?}", data);
    hasher.update(before.as_bytes());
    format!("{:02x}",hasher.finalize())
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
    let sx: Block = Create_block::new(0, "None".to_string(), std::time::Instant::now(), vec!["ss".to_string()], None);
    println!("{}",sx);
    // for _ in 0..10000 {
        // println!("{}",calculate_hash(1,"sss".to_string(),std::time::Instant::now(),vec!["dd".to_string()]))
    // }
}
