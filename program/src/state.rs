use solana_program::pubkey::Pubkey;
use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CellAccount {
	pub authority: Pubkey,
	pub vote: i8,
	pub url: String,
	pub content: String,
}
