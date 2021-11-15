use super::transaction::*;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Block {
	pub index: u8,
	pub transactions: Vec<Transaction>,
	pub timestamp: String,
	pub hash: String,
	pub previous_hash: String,
	pub proof: i32,
}