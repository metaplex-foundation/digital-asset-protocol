#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]


use std::collections::BTreeMap;
use std::time::Duration;
use borsh::{BorshDeserialize, BorshSerialize};
use thiserror::Error;
use solana_program::{
    entrypoint,
    entrypoint::{
        ProgramResult
    },
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    account_info::{
        AccountInfo
    },
};
use crate::api::DigitalAssetProtocolError;
use crate::state::{Lifecycle, Standard};

pub struct ProgramContext<'a> {
    accounts: Vec<AccountInfo<'a>>,
    cpi: bool,
}

impl ProgramContext {
    pub fn new(program: &Pubkey, accounts: &[AccountInfo]) -> Self {
        ProgramContext {
            accounts: accounts.to_vec(),
            cpi: program != crate::id(),
        }
    }
}

pub trait Payload {}

impl Payload for Action::Create {
    fn validate(&self, pc: ProgramContext) -> Result<(), DigitalAssetProtocolError> {
        match &self {}
    }
}

impl Payload for Action::Delete {
    fn validate(&self, pc: ProgramContext) -> Result<(), DigitalAssetProtocolError> {
        match &self {}
    }
}

#[derive(FromPrimitive)]
pub enum Action<'a> {
    Transfer(Standard, TransferAction<'a>),
    Update(Standard, UpdateAction<'a>),
    Create(Standard, CreateAction<'a>),
    Delete(Standard, DeleteAction<'a>),
    // SupplyChange(Standard, Vec<u8>),
    Freeze(Standard, FreezeAction<'a>),
    // Burn(Standard, Vec<u8>),
}


#[derive(BorshSerialize, BorshDeserialize)]
pub struct CreateAction<'a> {
    asset: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    data: Data
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UpdateAction<'a> {
    asset: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    data: Data
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FreezeActionParams {
    duration: Option<u64>,
    delegate: Option<Pubkey>
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FreezeAction<'a> {
    asset: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    data: FreezeActionParams
}


#[derive(BorshSerialize, BorshDeserialize)]
pub struct TransferAction<'a> {
    asset: AccountInfo<'a>,
    current_owner: AccountInfo<'a>,
    new_owner: AccountInfo<'a>,
    payer: AccountInfo<'a>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct DeleteAction<'a> {
    asset: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    payer: AccountInfo<'a>,
}


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub enum Module {
    Ownership,
    // Governance,
    // Creators,
    // Royalty,
    // Rights,
    // Supply,
    // Grouped,
    // Provenance,
    // Signature,
    // Usage,
    Extension,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ModuleData {
    Ownership(OwnershipData),
    // Governance(GovernanceData),
    // Creators(CreatorsData),
    // Royalty(RoyaltyData),
    //Rights(),
    // Supply,
    // Grouped,
    // Provenance,
    // Signature,
    // Usage,
    // Extension(BTreeMap<String, Vec<u8>>),
}


#[derive(BorshSerialize, BorshDeserialize)]
enum OwnershipModel {
    Token,
    Single,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OwnershipModuleData {
    ownership_model: OwnershipModel,
    owner: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Data {
    items: Vec<ModuleData>
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Asset {
    pub address: Pubkey,
    pub data: Data,
}

pub fn parse_action(instruction_data: &[u8]) -> Result<Action, DigitalAssetProtocolError> {
    BorshDeserialize::try_from_slice::<Action>(instruction_data).map_err(|e| {
        DigitalAssetProtocolError::NotSupportedYet
    })
}


pub fn run_actions<'a>(a: ActionType, accounts: &'a [AccountInfo<'a>]) -> Result<ActionType, DigitalAssetProtocolError> {
    match a {
        (Lifecycle::Create, Standard::NFT) => {

        }
        _ => Err(DigitalAssetProtocolError::NotSupportedYet)
    }
}

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let action = parse_action(instruction_data)?;



    Ok(())

    // let action = instruction_to_context(program_id: &'a Pubkey,
    //                                     accounts: &'a [AccountInfo<'a>],
    //                                     instruction_data: &[u8])?;
    // /// Check Action Preconditions
    // action.valid()?;
    // /// Gather Needed context data
    // let standard = action.standard();
    // let event = action.lifecycle();
    // /// Inform
    // msg!("Performing {} on a {} asset.", event, standard);
    // /// Select Modules
    // let module_standard = standard.get_instance();
    //
    // if module_standard.is_none() {
    //     /// TODO Enforce payload header with modules
    //     return Err(
    //         ProcessorError::NotImplemented("Unknown Spec Not implemented".to_string()).into()
    //     );
    // }
    // let concrete_standard = standard.get_instance().unwrap();
    // let asset = action.asset()?;
    // for module in concrete_standard.modules() {}

    // match action_header {
    //     ActionHeader(Standard::NFT, 1) => {
    //         match action.lifecycle {
    //             Lifecycle::Create => {
    //                 let standard = NFTStandard::new();
    //                 let asset_data = action.get_data();
    //                 let asset = Asset::try_from_slice(asset_data)?;
    //                 if standard.valid_asset(asset) {
    //                     for m in standard.modules() {
    //                         let module = asset.deserialize_module(m)?;
    //                         module.modify(action, asset)?
    //                     }
    //                     let writer = BufWriter::
    //
    //                     asset.serialize(writer);
    //                 } else {
    //                     Err(io::Error::new(ErrorKind::InvalidData, format!("Modules are not correct for {} standard", standard.standard)))
    //                 }
    //             }
    //         }
    //     }
    //     _ => Err(Error::new(ErrorKind::InvalidData, "Unsupported Header"))
    // }
}


