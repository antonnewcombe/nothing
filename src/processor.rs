//use crate::errors::AuctionError;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, borsh::try_from_slice_unchecked, clock::UnixTimestamp,
    entrypoint::ProgramResult, hash::Hash, msg, program_error::ProgramError, pubkey::Pubkey,
};
use std::{cmp, mem};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    msg!("process instruction");
    Ok(())
}

