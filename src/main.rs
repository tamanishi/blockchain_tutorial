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

    pub fn new_block (mut self, proof: u64) -> Block {
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

    pub fn new_transacion (mut self, sender: &str, recipient: &str, amount: u64) -> u64 {
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

    fn proof_of_work (self, block: &Block) -> u64 {
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
            "0000" => true,
            _ => false,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
