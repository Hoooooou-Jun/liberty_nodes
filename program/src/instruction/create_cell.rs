use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  account_info::{AccountInfo, next_account_info},
  entrypoint::ProgramResult,
  msg,
  program_error::ProgramError,
  pubkey::Pubkey,
};
use crate::state::CellAccount;

pub fn create_cell(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();
  let cell_account = next_account_info(accounts_iter)?;

  if cell_account.owner != program_id {
      msg!("Error: Account does not have the correct program owner");
      return Err(ProgramError::IncorrectProgramId);
  }

  let input_data = CellAccount::try_from_slice(instruction_data)
      .map_err(|_| ProgramError::InvalidInstructionData)?;

  let mut cell_data = CellAccount::try_from_slice(&cell_account.data.borrow())?;
  cell_data.authority = input_data.authority;
  cell_data.vote = input_data.vote;
  cell_data.url = input_data.url;
  cell_data.content = input_data.content;
  cell_data.serialize(&mut *cell_account.data.borrow_mut())?;

  msg!("Data updated for authority: {}", cell_data.authority);
  Ok(())
}
