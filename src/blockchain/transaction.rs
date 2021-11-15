use serde::{Serialize};
use std::clone::Clone;

#[derive(Debug, Serialize, Clone)]
pub struct Transaction {
	pub sender: String,
	pub recipient: String,
	pub amount: String
}