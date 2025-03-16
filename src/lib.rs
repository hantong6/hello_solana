mod error;
mod instructions;
mod processor;
mod state;

use solana_program::account_info::AccountInfo;
use solana_program::{entrypoint, msg};
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use crate::processor::Processor;

entrypoint!(process_instructon);

fn process_instructon(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    Processor::process(program_id, _accounts, _instruction_data)
}