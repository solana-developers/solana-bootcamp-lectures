use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum RuntimeLimitationInstruction {
    Stack, // 4KB
    Runtime { max_iter: usize }, // 200000 compute units
    // Do you accumulate more stack per CPI
    Cpi { size: u64 }, // Depth of 4. 
    // Max allocation of create_account = 10KB
    ZeroCopy,
    Tx { data: Vec<u8> }, // 1232 bytes 
}
