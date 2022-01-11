use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum CounterInstruction {
    Increment,
}

pub fn increment(
    program_id: Pubkey,
    counter: Pubkey,
    authority: Pubkey,
    instruction: CounterInstruction,
) -> Result<Instruction, ProgramError> {
    // TODO Fix instruction
    Ok(Instruction {
        accounts: vec![
            AccountMeta::new(counter, false),
            AccountMeta::new_readonly(authority, true),
        ],
        data: instruction.try_to_vec()?,
        program_id,
    })
}
