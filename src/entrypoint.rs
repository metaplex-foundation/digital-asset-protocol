#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]

use crate::api::DigitalAssetProtocolError;
use crate::generated::schema::{ModuleType,
                               OwnershipModel,
                               RoyaltyModel,
                               Action,
                               Asset,
                               Module,
                               Standard,
};
use bebop::prelude::*;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::collections::HashMap;
use thiserror::Error;
use core::pin::Pin;


pub trait ActionContext {
    fn validate(&self) -> Result<(), DigitalAssetProtocolError>;
    fn act(
        &self,
        modules: HashMap<ModuleType, Module>) -> Result<Asset, DigitalAssetProtocolError>;
}

pub struct CreateNFTContext<'a> {
    address: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    remaining_accounts: &'a [AccountInfo<'a>],
}

impl ActionContext for CreateNFTContext<'_> {
    fn validate(&self) -> Result<(), DigitalAssetProtocolError> {
        if !self.payer.is_signer {
            return Err(DigitalAssetProtocolError::ActionError("Payer must sign".to_string()));
        }
        Ok(())
    }
    fn act(
        &self,
        modules: HashMap<ModuleType, Module>,
    ) -> Result<Asset, DigitalAssetProtocolError> {
        let mut asset = Asset {
            layout: HashMap::new(),
        };
        for (ind, m) in modules.iter_mut() {
            create(*ind, self, &mut asset, m);
        }
        Ok(asset)
    }
}

fn create<'a>(
    id: ModuleType,
    ctx: &'a CreateNFTContext<'a>,
    asset: &'a mut Asset,
    module: &'a mut Module,
) -> Result<(), DigitalAssetProtocolError> {
    match id {
        ModuleType::Ownership => create_ownership(ctx, asset, module),
        ModuleType::Royalty => create_royalty(ctx, asset, module),
        ModuleType::Creators => create_creators(ctx, asset, module),
        _ => Err(DigitalAssetProtocolError::ActionError(
            "Not Implemented".to_string(),
        )),
    }
}

fn create_ownership<'a>(
    ctx: &'a CreateNFTContext<'a>,
    asset: &'a mut Asset,
    module: &'a mut Module,
) -> Result<(), DigitalAssetProtocolError> {
    // validation of create ownership specific stuff
    let Module::Ownership { model, owner } = module;
    if *model == OwnershipModel::Token && ctx.owner.owner != &spl_token::id() {
        return Err(DigitalAssetProtocolError::ModuleError(
            "Token Owner must be a Mint".to_string(),
        ));
    }
    asset.layout.entry(ModuleType::Ownership as u32).and_modify(move |m| {
        m = module
    });
    Ok(())
}

fn create_royalty<'a>(
    ctx: &'a CreateNFTContext<'a>,
    asset: &'a mut Asset,
    module: &'a mut Module,
) -> Result<(), DigitalAssetProtocolError> {
    // validation of create ownership specific stuff
    let Module::Royalty { model, target, .. } = module;
    let creators_model = model == &RoyaltyModel::Creators;
    let creator_module = asset.layout.get(&ModuleType::Creators);
    if creators_model && creator_module.is_none() {
        return Err(DigitalAssetProtocolError::ModuleError(
            "Creators Must be set".to_string(),
        ));
    }
    if creators_model && !target.is_empty() {
        *target = Vec::default()
    }
    asset.layout.entry(ModuleType::Royalty as u32).and_modify( move |m| {
        m = module
    });
    Ok(())
}

fn create_creators<'a>(
    ctx: &'a CreateNFTContext<'a>,
    asset: &'a mut Asset,
    module: &'a mut Module,
) -> Result<(), DigitalAssetProtocolError> {
    let Module::Creators { creator_list } = module;
    if creator_list.is_empty() {
        return Err(DigitalAssetProtocolError::ModuleError(
            "1 or more creators must be present".to_string(),
        ));
    }
    asset.layout.entry(ModuleType::Creators as u32).and_modify(move |m| {
        m = module
    });
    Ok(())
}

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    let action = Action::deserialize(instruction_data)
        .map_err(|e| DigitalAssetProtocolError::DeError(e.to_string()))?;

    let ctx = match action {
        Action::Unknown => Err(DigitalAssetProtocolError::ActionError(
            "No Action Found".to_string(),
        )),
        Action::Create { layout, standard } => {
            let address = accounts[0].clone();
            let owner = accounts[1].clone();
            let payer = accounts[2].clone();
            let authority = accounts[3].clone();

            Ok(CreateNFTContext {
                address,
                owner,
                payer,
                authority,
                remaining_accounts: &accounts[4..],
            })
        }
    };
    ctx?.validate()?;
    Ok(())
}
