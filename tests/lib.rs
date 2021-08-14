#![allow(warnings)]

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::borsh::try_from_slice_unchecked;
use solana_program_test::*;
use solana_sdk::program_pack::Pack;
use solana_sdk::{
    account::Account,
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction, system_program,
    transaction::Transaction,
    transport::TransportError,
    msg
};
use example::{
    instruction,
    processor::process_instruction,
};
use std::mem;

// mod helpers;

/// Initialize an auction with a random resource, and generate bidders with tokens that can be used
/// for testing..

#[tokio::test]
async fn test_init() {
    // Create a program to attach accounts to.
    let program_id = Pubkey::new_unique();
    let mut program_test =
        ProgramTest::new("example", program_id, processor!(process_instruction));
    msg!("in setup_auction_1");
    // Start executing test.
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
}
