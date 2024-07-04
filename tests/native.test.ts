u// Manually initialize variables that are automatically defined in Playground
const PROGRAM_ID = new web3.PublicKey("HGWnztHyhBVRAghFkH2iFWgRE3XpUJa8uxAbrdcAC1yt");
const connection = new web3.Connection("https://api.devnet.solana.com", "confirmed");
const wallet = { keypair: web3.Keypair.generate() };

se solana_program::{
	account_info::AccountInfo,
	entrypoint,
	entrypoint::ProgramResult,
	pubkey::Pubkey,
};

mod state;
mod instruction {
	pub mod create_cell;
}

use crate::instruction::create_cell::create_cell;

entrypoint!(process_instruction);

pub fn process_instruction(
	program_id: &Pubkey,
	accounts: &[AccountInfo],
	instruction_data: &[u8],
) -> ProgramResult {
	create_cell(program_id, accounts, instruction_data)
}
