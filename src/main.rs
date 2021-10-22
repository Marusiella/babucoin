use sha2::digest::generic_array::{GenericArray, typenum::{UInt, UTerm, bit::{B0, B1}}};

struct Block {
    index: u64,
    previus_hash: String,
    timestamp: std::time::Instant,
    data: Vec<String>,
    hash: String
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
    for _ in 0..100 {
        println!("{}",calculate_hash(1,"sss".to_string(),std::time::Instant::now(),vec!["dd".to_string()]))
    }
}
