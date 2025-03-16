use borsh::BorshDeserialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::sysvar::Sysvar;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::{initialize_mint, mint_to};
use spl_token::state::Mint;
use crate::instructions::TokenInstruction;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TokenInstruction::try_from_slice(instruction_data)?;
        msg!("Instruction: {:?}", instruction);
        match instruction {
            TokenInstruction::CreateToken { decimals } => Self::create_token(accounts, decimals),
            TokenInstruction::Mint { amount } => Self::mint_token(accounts, amount)
        }
    }

    fn create_token(accounts: &[AccountInfo], decimals: u8) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let mint = next_account_info(account_info_iter)?;
        let mint_authority = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;
        let system = next_account_info(account_info_iter)?;
        let token = next_account_info(account_info_iter)?;
        msg!("create account: {}", mint.key);
        let create_account_ins = create_account(
            payer.key,
            mint.key,
            Rent::get()?.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token.key
        );
        let create_account_acc = [
            mint.clone(),
            payer.clone(),
            system.clone(),
            token.clone()
        ];
        invoke(
            &create_account_ins,
            &create_account_acc
        )?;
        msg!("init token: {}", mint.key);
        let mint_init_ins = initialize_mint(
            token.key,
            mint.key,
            mint_authority.key,
            None,
            decimals
        )?;
        let mint_init_acc = [
            mint.clone(),
            rent.clone(),
            token.clone(),
            mint_authority.clone()
        ];
        invoke_signed(
            &mint_init_ins,
            &mint_init_acc,
            &[]
        )?;
        msg!("create token {} success", mint.key);
        Ok(())
    }

    fn mint_token(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let mint = next_account_info(account_info_iter)?;
        let target = next_account_info(account_info_iter)?;
        let rent = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;
        let system = next_account_info(account_info_iter)?;
        let token = next_account_info(account_info_iter)?;
        let ata = next_account_info(account_info_iter)?;
        msg!("target account: {}", target.key);
        if target.lamports() == 0 {
            let create_ata_ins = create_associated_token_account(
                payer.key,
                payer.key,
                mint.key,
                token.key
            );
            let create_ata_acc = [
                payer.clone(),
                target.clone(),
                mint.clone(),
                system.clone(),
                token.clone(),
                rent.clone(),
                ata.clone()
            ];
            invoke(
                &create_ata_ins,
                &create_ata_acc
            )?;
            msg!("create target account success")
        }
        let mint_target_ins = mint_to(
            token.key,
            mint.key,
            target.key,
            payer.key,
            &[payer.key],
            amount
        )?;
        let mint_target_acc = [
            mint.clone(),
            payer.clone(),
            target.clone(),
            token.clone()
        ];
        invoke(
            &mint_target_ins,
            &mint_target_acc
        )?;
        msg!("mint {} to target {} success", amount, target.key);
        Ok(())
    }

}