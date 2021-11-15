#![allow(unused_variables)]

mod blockchain;
use blockchain::{Hash, Transactions, Chain};

use uuid::Uuid;

fn mine(chain: &mut blockchain::Blockchain, node: &String) -> usize {
	// create coinbase transaction
	chain.new_transaction(String::from("0"), node.to_string(), String::from("1"));

	let last_block = chain.last_block();
	let proof = chain.pow(&last_block);

	let previous = chain.hash(last_block);

	chain.new_block(previous, proof)
}

fn main() {
	let node = Uuid::new_v4().to_simple().to_string();

	let genesis = blockchain::Block {
		index: 1,
		timestamp: String::from("test"),
		hash: String::from("1"),
		previous_hash: String::from("1"),
		proof: 100,
		transactions: vec![]
	};

	let mut chain = blockchain::build_blockchain(genesis);

	chain.new_transaction("a sender like spike".to_string(), "a reciepent like frosty".to_string(), "amount moneyz".to_string());

	let block_index = mine(&mut chain, &node);
	//let block = chain.get_block(block_index);
	//let transactions = serde_json::to_string_pretty(&block.transactions).unwrap();
	//println!("\nNew Block Forged\n- Proof: {}\n- Previous Hash: {}\n- Transactions: {}", block.proof, block.previous_hash, transactions);

	mine(&mut chain, &node);

	println!("\n:::\n{}", serde_json::to_string_pretty(&chain.chain).unwrap());
}