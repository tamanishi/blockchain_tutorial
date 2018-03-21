#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate time;
extern crate crypto;

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use time::*;

struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>
}

#[derive(Clone, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String
}

#[derive(Clone, Serialize, Deserialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u64
}

impl Blockchain {
    pub fn new () -> Blockchain {
        Blockchain {
            chain: vec!{},
            current_transactions: vec!{},
        }
    }

    pub fn new_block (&mut self, proof: u64) -> Block {
        let block = match self.chain.last() {
            Some(last) => {
                Block {
                    index: self.chain.len() as u64,
                    timestamp: now().to_timespec().sec,
                    transactions: self.current_transactions.clone(),
                    proof: proof,
                    previous_hash:  Blockchain::hash(&last),
                }
            },
            None => {
                Block {
                    index: 0,
                    timestamp: now().to_timespec().sec,
                    transactions: self.current_transactions.clone(),
                    proof: proof,
                    previous_hash: "".to_owned(),
                }
            }
        };

        self.current_transactions.clear();
        // push clone
        self.chain.push(block.clone());
        // return original
        return block;
    }

    pub fn new_transaction (&mut self, sender: &str, recipient: &str, amount: u64) -> u64 {
        let transaction = Transaction {
            sender: sender.to_owned(),
            recipient: recipient.to_owned(),
            amount: amount,
        };

        self.current_transactions.push(transaction);
        match self.last_block() {
            Some(block) => block.index + 1,
            None => 1,
        }
    }

    pub fn last_block (&self) -> Option<&Block> {
        match self.chain.len() {
            0 => None,
            n => Some(&self.chain[n - 1]),
        }
    }

    fn hash (block: &Block) -> String {
        let input = serde_json::to_string(block);
        let mut sha = Sha256::new();
        sha.input_str(&input.unwrap());
        sha.result_str()
    }

    pub fn proof_of_work (&self, block: &Block) -> u64 {
        let mut proof = 0;
        while !Blockchain::is_valid_proof(block, proof) {
            proof += 1;
        }
        proof
    }

    fn is_valid_proof (block: &Block, proof: u64) -> bool {
        let mut test_block = block.clone();
        test_block.proof = proof;
        let hash = Blockchain::hash(&test_block);
        match &hash[0..4] {
            "0000" => {
                println!("found proof");
                println!("proof: {}", proof);
                println!("hash: {}", hash);
                true
            },
            _ => false
        }
    }
}

fn new_transaction (blockchain: &mut Blockchain, transaction: &Transaction) -> u64 {
    let index = blockchain.new_transaction(&transaction.sender, &transaction.recipient, transaction.amount);
    println!("a new transaction was added to block {}", index);
    index
}

fn mine (blockchain: &mut Blockchain) {
    let proof = {
        // avoid error[E0502] cannot bororw `*blockchain` as mutable because it is also borrowed as imutable
        // https://qiita.com/knknkn1162/items/1d190880efffe3578d92
        let last_block = {
            match blockchain.last_block() {
                None => panic!("empty blockchain"),
                Some(block) => block
            }
        };
        let proof = blockchain.proof_of_work(&last_block);
        proof
    };

    blockchain.new_transaction("0", "my address", 1);
    let block = blockchain.new_block(proof);

    println!("mined a new block");
    println!("index: {}", block.index);
    println!("last transaction: {}", serde_json::to_string(&block.transactions[block.transactions.len() - 1]).unwrap());
    println!("proof: {:x}", block.proof);
    println!("previous hash: {}", block.previous_hash);
}

fn main() {
    // create blockchain
    let mut blockchain = Blockchain::new();
    // add genesis block
    blockchain.new_block(0);
    // add a transaction
    let mut transaction = Transaction {sender: "my address".to_string(), recipient: "your address".to_string(), amount: 100};
    new_transaction(&mut blockchain, &transaction);
    transaction.amount = 200;
    new_transaction(&mut blockchain, &transaction);
    transaction.amount = 300;
    new_transaction(&mut blockchain, &transaction);
    // mine
    mine(&mut blockchain);

    transaction.amount = 400;
    new_transaction(&mut blockchain, &transaction);
    transaction.amount = 500;
    new_transaction(&mut blockchain, &transaction);
    // mine
    mine(&mut blockchain);
}


