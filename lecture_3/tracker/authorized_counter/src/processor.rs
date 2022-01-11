use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::CounterInstruction;
use crate::state::Counter;

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
        let instruction = CounterInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        match instruction {
            CounterInstruction::Increment => {
                msg!("Instruction: Increment");
                // Decode AccountInfo's
                let accounts_iter = &mut accounts.iter();
                let counter_ai = next_account_info(accounts_iter)?;
                let authority = next_account_info(accounts_iter)?;
                assert_with_msg(
                    authority.is_signer,
                    ProgramError::MissingRequiredSignature,
                    "Authority must sign",
                )?;
                // Deserialize account data
                let mut counter = Counter::try_from_slice(&counter_ai.data.borrow())?;
                if counter.count == 0 {
                    // Set the authority if this is the first time the counter has been used. 
                    counter.authority = *authority.key;
                }
                assert_with_msg(
                    counter.authority == *authority.key,
                    ProgramError::MissingRequiredSignature,
                    "Attempted to increment with an invalid authority",
                )?;
                // Update account data
                counter.count += 1;
                msg!("Global count: {}", counter.count);
                // Serialize account
                counter.serialize(&mut *counter_ai.data.borrow_mut())?;
            }
        }
        Ok(())
    }
}
