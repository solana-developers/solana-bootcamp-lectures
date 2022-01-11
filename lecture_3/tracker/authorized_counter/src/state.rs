use solana_program::{pubkey::Pubkey};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
