use std::collections::HashSet;
use serde_json;
use sha2::{Sha256, Digest};
use super::block::*;
use super::transaction::Transaction;
use chrono::prelude::{DateTime, Utc};
use std::time::SystemTime;

pub struct Blockchain {
	// chain, pending, peers
	pub pending: Vec<Transaction>,
	pub chain: Vec<Block>,
	pub nodes: HashSet<String>
}

pub trait Hash {
	fn hash(&self, block: &Block) -> String;
}

impl Hash for Blockchain {
	fn hash(&self, block: &Block) -> String {
		let json = serde_json::to_string(&block.transactions).unwrap();
		let hash_string = format!("{}{}{}{}{}", &block.index, json, &block.timestamp, &block.previous_hash, &block.proof);

		let mut hasher = Sha256::new();
		hasher.update(hash_string.as_bytes());

		return format!("{:x}", hasher.finalize());
	}
}

pub trait Transactions {
	fn new_transaction(&mut self, sender: String, recipient: String, data: String);
}

impl Transactions for Blockchain {
	fn new_transaction(&mut self, sender: String, recipient: String, data: String) {
		let transaction = Transaction {
			sender:  sender,
			recipient: recipient,
			amount: data
		};

		self.pending.push(transaction);
	}
}

pub trait Chain {
	fn new_block(&mut self, previous_hash: String, proof: i32) -> usize;
	fn get_block(&self, i: usize) -> &Block;
	fn last_block(&self) -> &Block;
	fn pow(&self, last_block: &Block) -> i32;
	fn valid_proof(&self, last_proof: i32, proof: i32, last_hash: &String) -> bool;
	
}

impl Chain for Blockchain {
	fn new_block(&mut self, previous_hash: String, proof: i32) -> usize {
		let pending = self.pending.to_vec();

		let now = &SystemTime::now();
		let dt: DateTime<Utc> = now.clone().into();

		let mut block = Block {
			index: (self.chain.len() as u8) + 1,
			timestamp: format!("{}", dt.format("%+")),
			hash: String::from(""),
			previous_hash: previous_hash,
			proof: proof,
			transactions: pending
		};

		block.hash = self.hash(&block);

		self.pending = vec![];
		self.chain.push(block);

		self.chain.len() - 1
	}

	fn get_block(&self, i: usize) -> &Block {
		&self.chain[i]
	}

	fn last_block(&self) -> &Block {
		self.get_block(self.chain.len() - 1)
	}

	fn pow(&self, last_block: &Block) -> i32 {
		let last_proof = last_block.proof;
		let last_hash = self.hash(&last_block);

		let mut proof: i32 = 0;
		while self.valid_proof(last_proof, proof, &last_hash) == false {
			proof = proof + 1;
		}

		proof
	}

	fn valid_proof(&self, last_proof: i32, proof: i32, last_hash: &String) -> bool {
		let guess = format!("{}{}{}", last_proof, proof, last_hash); // proof as nonce
		let mut hasher = Sha256::new();
		hasher.update(guess.as_bytes());
		let hash = format!("{:x}", hasher.finalize());
		return &hash[..2] == "00";
	}
}

pub fn build_blockchain(genesis: Block) -> Blockchain {
	let blockchain = Blockchain {
		pending: vec![],
		chain: vec![genesis],
		nodes: HashSet::new()
	};

	blockchain
}