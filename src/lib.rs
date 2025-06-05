use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};
#[derive(BorshDeserialize, BorshSerialize)]
enum Instruction {
    Increment(u32),
    Decrement(u32),
}
#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    let instruction = Instruction::try_from_slice(instruction_data)?;
    match instruction {
        Instruction::Increment(value) => {
            msg!("Incrementing counter by {}", value);
            counter_data.count += value;
        }
        Instruction::Decrement(value) => {
            msg!("Decrementing counter by {}", value);
            counter_data.count -= value;
        }
    }
    counter_data.serialize(&mut &mut acc.data.borrow_mut()[..])?;
    msg!("Counter updated: {}", counter_data.count);
    Ok(())
}
