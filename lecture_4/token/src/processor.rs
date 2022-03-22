use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instructions::TokenInstruction;
use crate::state::{Mint, AccountTag, TokenAccount};

pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
    if !statement {
        msg!(msg);
        Err(err)
    } else {
        Ok(())
    }
}

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TokenInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        let accounts_iter = &mut accounts.iter();
        match instruction {
            TokenInstruction::InitializeMint => {
                let mint_ai = next_account_info(accounts_iter)?;
                let mint_authority = next_account_info(accounts_iter)?;
                let mut mint = Mint::load_unchecked(mint_ai)?;
                assert_with_msg(
                    mint_authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Mint Authority must sign",
                )?;
                // TODO
                mint.tag = AccountTag::Mint;
                mint.authority = *mint_authority.key;
                mint.supply = 0;
                mint.save(mint_ai)?
            }
            TokenInstruction::InitializeTokenAccount => {
                let token_account_ai = next_account_info(accounts_iter)?;
                let mint_ai = next_account_info(accounts_iter)?;
                let mint = Mint::load(mint_ai)?;
                let owner = next_account_info(accounts_iter)?;
                let mut token_account = TokenAccount::load_unchecked(token_account_ai)?;
                // TODO
                token_account.tag = AccountTag::TokenAccount;
                token_account.owner = *owner.key;
                token_account.mint = *mint_ai.key;
                token_account.amount = 0;
                token_account.save(token_account_ai)?
            }
            TokenInstruction::Mint { amount } => {
                msg!("Instruction: Mint");
                let token_account_ai = next_account_info(accounts_iter)?;
                let mint_ai = next_account_info(accounts_iter)?;
                let mint_authority = next_account_info(accounts_iter)?;
                let mut token_account = TokenAccount::load(token_account_ai)?;
                let mut mint = Mint::load(mint_ai)?;
                assert_with_msg(
                    mint_authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Mint Authority must sign",
                )?;
                assert_with_msg(
                    mint.authority == *mint_authority.key,
                    ProgramError::MissingRequiredSignature,
                    "Mint Authority mismatch",
                )?;
                // TODO
                // unsafe 
                // amount = u64::max_value();
                mint.supply += amount;
                token_account.amount += amount;

                token_account.save(token_account_ai)?;
                mint.save(mint_ai)?;
            }
            TokenInstruction::Burn { amount } => {
                msg!("Instruction: Burn");
                let token_account_ai = next_account_info(accounts_iter)?;
                let mint_ai = next_account_info(accounts_iter)?;
                let owner = next_account_info(accounts_iter)?;
                let mut token_account = TokenAccount::load(token_account_ai)?;
                let mut mint = Mint::load(mint_ai)?;
                assert_with_msg(
                    owner.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Token owner must sign",
                )?;
                assert_with_msg(
                    token_account.owner == *owner.key,
                    ProgramError::MissingRequiredSignature,
                    "Token owner mismatch",
                )?;
                assert_with_msg(
                    token_account.amount >= amount, 
                    ProgramError::InvalidAccountData,
                    "Attempting to burn more than account balance",
                )?;
                // TODO
                mint.supply -= amount;
                token_account.amount -= amount;
                token_account.save(token_account_ai)?;
                mint.save(mint_ai)?;
            }
            TokenInstruction::Transfer { amount } => {
                msg!("Instruction: Transfer");
                let src_token_account_ai = next_account_info(accounts_iter)?;
                let dst_token_account_ai = next_account_info(accounts_iter)?;
                let owner = next_account_info(accounts_iter)?;
                let mut src_token_account = TokenAccount::load(src_token_account_ai)?;
                let mut dst_token_account = TokenAccount::load(dst_token_account_ai)?;
                assert_with_msg(
                    owner.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Token owner must sign",
                )?;
                assert_with_msg(
                    src_token_account.owner == *owner.key,
                    ProgramError::MissingRequiredSignature,
                    "Token owner mismatch",
                )?;
                assert_with_msg(
                    src_token_account.amount >= amount, 
                    ProgramError::InvalidAccountData,
                    "Attempting to transfer more than account balance",
                )?;
                assert_with_msg(
                    src_token_account.mint == dst_token_account.mint, 
                    ProgramError::InvalidAccountData,
                    "Token account mints do not match",
                )?;
                // TODO
                src_token_account.amount -= amount;
                dst_token_account.amount += amount;
                src_token_account.save(src_token_account_ai)?;
                dst_token_account.save(dst_token_account_ai)?;
            }
        }
        Ok(())
    }
}
