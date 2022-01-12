use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use bytemuck::{from_bytes, from_bytes_mut, Zeroable, Pod};

use std::{
    cell::{Ref, RefMut},
    mem,
};

#[derive(BorshSerialize, BorshDeserialize, Default, Debug, Copy, Clone, Zeroable, Pod)]
#[repr(C)]
pub struct Offer {
    pub offer_mint: Pubkey, // NFT
    pub buyer_mint: Pubkey, // USDC, SOL, BTC
    pub offer_amount: u64, // 1
    pub buyer_amount: u64, // price the user is asking
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy)]
pub struct MarketplaceBorsh {
    pub user: Pubkey,
    pub offers: [Offer; 256]
}

impl Default for MarketplaceBorsh {
    fn default() -> Self {
        Self {
            user: Pubkey::default(),
            offers: [Offer::default(); 256]
        }    
    }
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct Marketplace {
    pub user: Pubkey,
    pub offers: [Offer; 256]
}

impl ZeroCopy for Marketplace {}

pub trait ZeroCopy: Pod {
    fn load<'a>(account: &'a AccountInfo) -> Result<Ref<'a, Self>, ProgramError> {
        let size = mem::size_of::<Self>();
        Ok(Ref::map(account.try_borrow_data()?, |data| {
            from_bytes(&data[..size])
        }))
    }

    fn load_mut<'a>(account: &'a AccountInfo) -> Result<RefMut<'a, Self>, ProgramError> {
        let size = mem::size_of::<Self>();
        Ok(RefMut::map(account.try_borrow_mut_data()?, |data| {
            from_bytes_mut(&mut data[..size])
        }))
    }
}