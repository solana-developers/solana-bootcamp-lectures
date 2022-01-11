use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AccountTag {
    Uninitialized,
    Mint,
    TokenAccount,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Mint {
    pub tag: AccountTag,
    pub authority: Pubkey,
    pub supply: u64,
}

impl Mint {
    pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
        Ok(Self::try_from_slice(&ai.data.borrow())?)
    }

    fn validate(&self) -> ProgramResult {
        if self.tag != AccountTag::Mint {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
        let mint = Self::try_from_slice(&ai.data.borrow())?;
        mint.validate()?;
        Ok(mint)
    }

    pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
        Ok(self.serialize(&mut *ai.data.borrow_mut())?)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenAccount {
    pub tag: AccountTag,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

impl TokenAccount {
    pub fn load_unchecked(ai: &AccountInfo) -> Result<Self, ProgramError> {
        Ok(Self::try_from_slice(&ai.data.borrow())?)
    }

    fn validate(&self) -> ProgramResult {
        if self.tag != AccountTag::TokenAccount {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn load(ai: &AccountInfo) -> Result<Self, ProgramError> {
        let account = Self::try_from_slice(&ai.data.borrow())?;
        account.validate()?;
        Ok(account)
    }

    pub fn save(&self, ai: &AccountInfo) -> ProgramResult {
        Ok(self.serialize(&mut *ai.data.borrow_mut())?)
    }
}
