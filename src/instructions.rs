use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TokenInstruction {
    CreateToken {decimals: u8},
    Mint {amount: u64}
}