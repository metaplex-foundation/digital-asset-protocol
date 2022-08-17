#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]

use std::cell::{RefCell, RefMut};
use crate::api::{DigitalAssetProtocolError, Message};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use crate::interfaces::{
    get_interface,
    Interface
};
use thiserror::Error;
use crate::generated::schema::Action;

entrypoint!(process_instruction);
fn process_instruction<'entry>(
    program_id: &Pubkey,
    accounts: &'entry [AccountInfo<'entry>],
    instruction_data: &'entry [u8],
) -> ProgramResult {
    // Pin to this 'entry lifetime
    let mut ix_data = RefCell::new(instruction_data);
    let accounts = accounts;
    // What im trying to do is pin the life times here, hence the descriptive liftime name, so that as the data flows through I can limit copy and always refer back to the to the entry point lifetime
    // Create a structure that wraps them to avoid copy
    let msg = Message::new(
        accounts,
        ix_data.borrow_mut()
    )?;
    let iface = get_interface(&msg)?;
    // Instead of showing any concrete types here, we let the interface selected by the client handle the message, it will be in charge of all validation instead of outside the interface.
    iface.handle_message(&mut msg)?;
    Ok(())
}


