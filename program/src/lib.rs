use solana_program::{
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
