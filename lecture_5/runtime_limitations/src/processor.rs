use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    log::sol_log_compute_units,
    program_error::ProgramError,
    program::invoke,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::instruction::RuntimeLimitationInstruction;
use crate::state::{MarketplaceBorsh, Marketplace, ZeroCopy};

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
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        sol_log_compute_units();
        let instruction = RuntimeLimitationInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        let accounts_iter = &mut accounts.iter();
        match instruction {
            RuntimeLimitationInstruction::Stack => {
                let marketplace_ai = next_account_info(accounts_iter)?;
                let user = next_account_info(accounts_iter)?;
                // let mut marketplace: Box<MarketplaceBorsh> = Box::<MarketplaceBorsh>::new(MarketplaceBorsh::try_from_slice(&marketplace_ai.data.borrow())?);
                // marketplace.user = *user.key;
                // marketplace.serialize(&mut *marketplace_ai.data.borrow_mut())?;
            }
            RuntimeLimitationInstruction::ZeroCopy => {
                let marketplace_ai = next_account_info(accounts_iter)?;
                let user = next_account_info(accounts_iter)?;
                let mut marketplace = Marketplace::load_mut(marketplace_ai)?;
                marketplace.user = *user.key;
                msg!("Assigned marketplace for user {}", *user.key);
            }
            RuntimeLimitationInstruction::Runtime { max_iter }=> {
                for i in 0..max_iter {
                    if i % 1000 == 0 {
                        sol_log_compute_units();
                    }
                }
            }
            RuntimeLimitationInstruction::Cpi { size } => {
                let from = next_account_info(accounts_iter)?;
                let to = next_account_info(accounts_iter)?;
                let system_program = next_account_info(accounts_iter)?;
                invoke(
                    &solana_program::system_instruction::create_account(
                        from.key,
                        to.key,
                        Rent::get()?.minimum_balance(size as usize),
                        size,
                        program_id,
                    ),
                    &[
                        from.clone(),
                        to.clone(),
                        system_program.clone(),
                    ]
                )?;
            }
            RuntimeLimitationInstruction::Tx { data }=> {
                let buffer = &mut next_account_info(accounts_iter)?.try_borrow_mut_data()?;
                buffer.copy_from_slice(data.as_slice());
                msg!("Echo {}", std::str::from_utf8(&buffer).unwrap());
                sol_log_compute_units();
            }
        }
        Ok(())
    }
}
