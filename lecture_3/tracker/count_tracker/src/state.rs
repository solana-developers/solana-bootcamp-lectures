use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Tracker {
    pub bump: u8, // bump seed of tracker
    pub auth_bump: u8, // bump seed of the auth
    pub counter: Pubkey,
    pub count: u64, // tracked value
}
