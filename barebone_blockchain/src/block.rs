use std::fmt::{ 
	Debug, 
};

use super::*;

type BlockHash = Vec<u8>;

#[derive(Debug)]
pub struct Block {
	pub index: u32,
	pub timestamp: u128,
	pub hash: BlockHash,
	pub prev_block_hash: BlockHash,
	pub nounce: u64,
	// pub transactions: Vec<Transaction>,
	pub difficulty: u128,
}

impl Block {
	pub fn new(
		index: u32, 
		timestamp: u128, 
		prev_block_hash: BlockHash, 
		nounce: u64, 
		// transactions: Vec<Transaction>, 
		difficulty: u128
	) -> Self {
		Block { 
			index, 
			timestamp, 
			hash: vec![0; 32],
			prev_block_hash,
			nounce, 
			// transactions,
			difficulty, 
		}
	}
}

impl Hashing for Block {
	fn bytes(&self) -> Vec<u8> {
		let mut bytes = vec![];

		bytes.extend(&u32_bytes(&self.index));
		bytes.extend(&u128_bytes(&self.timestamp));
		bytes.extend(&self.prev_block_hash);
		bytes.extend(&u64_bytes(&self.nounce));
		bytes.extend(&u128_bytes(&self.difficulty));

		bytes
	}

}


