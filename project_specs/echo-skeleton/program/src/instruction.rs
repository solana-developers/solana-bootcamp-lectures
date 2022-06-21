use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum EchoInstruction {
    /// The contents of the data vector that is provided to the instruction will be copied into the echo_buffer account.
    ///
    /// If the `echo_buffer` account length ( N ) is smaller than the length of data, the instruction will copy the
    /// first N bytes of data into `echo_buffer`.
    ///
    /// If `echo_buffer` has any non-zero data, the instruction will fail.
    ///
    /// Accounts:
    /// | index | writable | signer | description                                  |
    /// |-------|----------|--------|----------------------------------------------|
    /// | 0     | ✅       | ❌     | echo_buffer: Destination account of the data  |
    Echo { data: Vec<u8> },
    /// This instruction will allocate `buffer_size` bytes to the `authorized_buffer` account and assign it the Echo Program.
    ///
    /// The first 9 bytes of authorized_buffer will be set with the following data:
    ///     byte 0: bump_seed
    ///     bytes 1-8: buffer_seed
    ///
    /// Accounts:
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | authorized_buffer: PDA of Echo Program that only `authority` can write to |
    /// | 1     | ❌       | ✅     | authority: Pubkey with sole write access to `authorized_buffer`           |
    /// | 2     | ❌       | ❌     | system_program: Used to allocate the buffer                               |
    InitializeAuthorizedEcho {
        buffer_seed: u64,
        buffer_size: usize,
    },
    /// The contents of the data vector that is provided to the instruction will be copied into the `authorized_buffer` account
    /// starting from index 9 (will NOT override the bump_seed and buffer_seed).
    ///
    /// If the remaining `authorized_buffer` account length ( N ) is smaller than the length of `data`, copy the first N bytes
    /// of data into `authorized_buffer`.
    ///
    /// Initially, if `authorized_buffer` has any non-zero data past index 9, you should should zero out all of the data outside
    /// of the first 9 bytes.
    ///
    /// If any account besides the `authority` attempts to write to the `authorized_buffer`, the instruction will fail.
    ///
    /// Accounts:
    /// | index | writable | signer | description                                                              |
    /// |-------|----------|--------|--------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | authorized_buffer: PDA of Echo Program that only `authority` can write to |
    /// | 1     | ❌       | ✅     | authority: Pubkey with sole write access to `authorized_buffer`           |
    AuthorizedEcho { data: Vec<u8> },
    /// This instruction will allocate `buffer_size` bytes to the `vending_machine_buffer` account and assign it the Echo Program.
    ///
    /// The first 9 bytes of `vending_machine_buffer` will be set with the following data:
    ///     byte 0: bump_seed
    ///     bytes 1-8: price
    ///
    /// Accounts:
    /// | index | writable | signer | description                                                                                         |
    /// |-------|----------|--------|-----------------------------------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | vending_machine_buffer: PDA of the Echo Program that only holders of a particular token can write to |
    /// | 1     | ❌       | ❌     | vending_machine_mint: Pubkey with sole write access to `authorized_buffer`                           |
    /// | 2     | ❌       | ✅     | payer: Pubkey that allocates the `vending_machine_buffer`                                            |
    /// | 3     | ❌       | ❌     | system_program: Used to allocate the buffer                                                          |
    InitializeVendingMachineEcho {
        // Number of tokens required change the buffer
        price: u64,
        buffer_size: usize,
    },
    /// The contents of the data vector that is provided to the instruction should be copied into the account starting from
    /// index 9 (you do NOT want to override the bump_seed and price).
    ///
    /// If the remaining account length ( N ) is smaller than the length of data, copy the first N bytes of data into
    /// `vending_machine_buffer`.
    ///
    /// Initially, if `vending_machine_buffer` has any non-zero data past index 9, you should should zero out all of the
    /// data outside of the first 9 bytes.
    ///
    /// Before any data is copied over, the user must burn a `price` amount of tokens from the `user_token_account`.
    /// This will require a cross program invocation to the Token Program. If this instruction succeed (verifies that the
    /// user in fact has sufficient tokens), then the copy can occur.
    ///
    /// This instruction should fail in the case that the mint of the `vending_machine_buffer` does not match the mint
    /// used to seed the PDA.  You can verify this by comparing the output of `Pubkey::create_program_address` with the correct
    /// seeds to the value of `vending_machine_buffer.key`.
    ///
    /// Accounts:
    /// | index | writable | signer | description                                                                                         |
    /// |-------|----------|--------|-----------------------------------------------------------------------------------------------------|
    /// | 0     | ✅       | ❌     | vending_machine_buffer: PDA of the Echo Program that only holders of a particular token can write to |
    /// | 1     | ❌       | ✅     | user: This is authority of the token account that is using the vending machine                       |
    /// | 2     | ✅       | ❌     | user_token_account: This is the token account that will pay for the use of the vending machine       |
    /// | 3     | ❌       | ❌     | vending_machine_mint: This is the token mint that is accepted by the `vending_machine_buffer`        |
    /// | 3     | ❌       | ❌     | token_program: Used to burn the vending machine tokens                                               |
    VendingMachineEcho { data: Vec<u8> },
}
