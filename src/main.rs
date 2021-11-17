#[macro_use]
extern crate derive_new;

use std::fmt::{self, Debug, Display};

use serde::{Deserialize, Serialize};

const PROOF: &str = "0";

#[derive(Serialize, Deserialize, Debug, Clone, new)]
struct Block {
    index: u64,
    previus_hash: String,
    timestamp: String,
    data: Vec<Transaction>,
    hash: String,
    proof: Option<u128>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Wallet {
    pub_key: String,
    priv_key: String,
}
impl Wallet {
    fn get(&self) -> String {
        self.pub_key.clone()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockChain {
    blocks: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Block {
    fn test_block(&self) -> String {
        calculate_hash_proof(
            self.index.clone(),
            self.previus_hash.clone(),
            self.timestamp.clone(),
            self.data.clone(),
            PROOF.clone(),
        )
        .0
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    reciver: String,
    amount: u64,
    hash: Option<String>,
}
impl Transaction {
    fn new(sender: Wallet, reciver: Wallet, amount: u64) -> Transaction {
        let sender = sender.pub_key.clone();
        let reciver = reciver.pub_key.clone();
        let x = Transaction {
            sender,
            reciver,
            amount,
            hash: None,
        };
        calculate_hash_transaction(x)
    }
}
fn calculate_hash_transaction(transaction: Transaction) -> Transaction {
    use sha3::{Digest, Sha3_512};
    let mut hasher = Sha3_512::new();
    hasher.update(transaction.sender.clone());
    hasher.update(transaction.reciver.clone());
    hasher.update(transaction.amount.to_string().as_bytes());
    let hash = format!("{:20x}", hasher.finalize());
    let x = Transaction {
        sender: transaction.sender,
        reciver: transaction.reciver,
        amount: transaction.amount,
        hash: Some(hash),
    };
    x
}
impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-{:?}-{}", self.sender, self.reciver, self.amount)
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
            chrono::offset::Utc::now()
                .timestamp_millis()
                .to_string()
                .to_string(),
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
            timestamp: chrono::offset::Utc::now().timestamp_millis().to_string(),
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
        let blocksss = self.blocks.clone();
        for x in 1..self.blocks.len() {
            // println!("{} || {}", self.blocks[x].test_block(),self.blocks[x + 1].previus_hash);

            let test = blocksss[x].test_block();
            let prev = blocksss[x - 1].previus_hash.clone();
            if test != prev {
                // println!("||||||||||||||| {:?} |||||||||||||||||||| {:?} ||||||||||||||||||", self.blocks[x].test_block(), self.blocks[x + 1].previus_hash);

                return false;
            }
        }
        return true;
    }
    // funcion for mining pending transactions
    fn mine(&mut self, proof: &str) {
        let mut data = self.get_pendding_transactions();
        if data.len() > 0 {
            self.add_block(data, proof);
            self.clear_pendding_transactions();
        }
    }
    // how much money the user has using wallet
    fn get_balance(&self, wallet: Wallet) -> u128 {
        let mut balance: u128 = 0;
        for x in self.blocks.clone() {
            for y in x.data.clone() {
                if y.sender == wallet.pub_key {
                    let amount = y.amount as u128;
                    balance += amount;
                }
                if y.reciver == wallet.pub_key {
                    let amount = y.amount as u128;
                    balance += amount;
                }
            }
        }
        println!("{}", balance);
        balance
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
    use sha3::{Digest, Sha3_512};
    let proof = proof.to_owned();
    let mut hasher = Sha3_512::new();
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
fn main() {
    let olek = generate_wallet();
    let anna = generate_wallet();
    let mut blockchin: BlockChain = Blockchain::new();
    let s: Transaction = Transaction::new(olek.clone(), anna, 22);
    let time = chrono::offset::Utc::now()
        .timestamp_millis()
        .to_string()
        .to_string();
    let calc = calculate_hash_proof(0, "".to_string(), time.clone(), vec![s.clone()], PROOF);
    let start: Block = Block::new(
        0,
        "".to_string(),
        time,
        vec![s.clone()],
        calc.0,
        Some(calc.1),
    );
    blockchin.add_block_thirst(start);

    // end of starrt code

    // let mut transactions = vec![];

    // for x in 0..=33 {
    //     let a: Transaction = Transaction::new(x.to_string(), (x + 10).to_string(), x + 100);
    //     transactions.push(a);
    // }

    // for x in transactions {
    //     blockchin.add_transaction(x);
    // }
    // let mut transaction = vec![];
    // for _ in 0..10 {
    //     std::thread::sleep(std::time::Duration::from_millis(1000));

    //     let one = generate_wallet();
    //     // sleep for 1 second
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     let two = generate_wallet();
    //     let s: Transaction = Transaction::new(one, two, 100);
    //     blockchin.add_transaction(s.clone());
    //     transaction.push(s);
    // }

    blockchin.mine(PROOF);
    // create_pending(&mut blockchin, PROOF);
    let json = serde_json::to_string_pretty(&blockchin).unwrap();
    println!("{}", json);
    std::fs::write("json.json", json).expect("Unable to write file");

    if blockchin.is_good() {
        println!("XD")
    }

    let nic = generate_wallet();
    // check user balance
    println!(" is {}", blockchin.get_balance(olek));

    // blockchin

    // let contents =
    //     std::fs::read_to_string("json.json").expect("Something went wrong reading the file");
    // let bc: BlockChain = serde_json::from_str(&contents).unwrap();
    // if bc.is_good() {
    //     panic!("oh no");
    // }

    // use rsa::{PaddingScheme, PublicKey, RsaPrivateKey};
    // let mut rng = rand::rngs::OsRng;
    // let padding = PaddingScheme::new_pkcs1v15_encrypt();
    // let privet_key =
    //     rsa::RsaPrivateKey::new(&mut rng, 333).expect("Oh nie nie da sie privata stworzyc");
    // let public = rsa::RsaPublicKey::from(&privet_key);
    // let enc_data = public
    //     .encrypt(&mut rng, padding, b"s")
    //     .expect("can't encrypt data");
    // println!("{:?}", enc_data);
    // let padding = PaddingScheme::new_pkcs1v15_encrypt();
    // let decode = privet_key.decrypt(padding, &enc_data).unwrap();
    // println!("{}", String::from_utf8_lossy(&decode));
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
fn generate_wallet() -> Wallet {
    let key = openssl::rsa::Rsa::generate(1024).expect("Failed to generate key"); //2048
    let priv_key = key.private_key_to_pem().unwrap();
    let pub_key = key.public_key_to_pem().unwrap();
    let priv_key = hex::encode(priv_key);
    let pub_key = hex::encode(pub_key);
    Wallet { pub_key, priv_key }
}
