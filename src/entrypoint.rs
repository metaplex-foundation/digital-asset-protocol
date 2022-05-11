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

pub struct CreateNFTContext<'a> {
    address: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    payer: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    remaining_accounts: Vec<AccountInfo<'a>>,
}

impl<'a> CreateNFTContext<'_> {
    fn validate(&self) -> Result<(), DigitalAssetProtocolError> {
        if !self.payer.is_signer {
            return Err(DigitalAssetProtocolError::ActionError("Payer must sign".to_string()));
        }
        Ok(())
    }
    fn act(
        &self,
        mut asset: &'a mut Asset<'a>,
    ) -> Result<&'a Asset<'a>, DigitalAssetProtocolError> {
        let mut asset_new = asset;

        // let id = ModuleType::try_from(*ind).map_err(|e| {
        //     DigitalAssetProtocolError::ActionError(e.to_string())
        // })?;
        let id = ModuleType::Royalty;
        asset_new = match id {
            // ModuleType::Ownership => {
            //     create_ownership(self, asset_new)
            // }
            ModuleType::Royalty => {
                create_royalty(self, asset_new)
            }
            // ModuleType::Creators => {
            //     create_creators(self, asset_new)
            // }
            _ => Err(DigitalAssetProtocolError::ActionError(
                "Not Implemented".to_string(),
            )),
        }?;

        Ok(asset_new)
    }
}

pub fn get_thing<T>(mtype: &ModuleType, map: &HashMap<ModuleType, Module>) -> Option<T> {
    map.get(&(mtype as u32)).map(|m| {
        m
    })
}

//
// fn create_ownership<'a>(
//     ctx: &CreateNFTContext,
//     asset: &'a mut Asset<'a>,
// ) -> Result<&'a mut Asset<'a>, DigitalAssetProtocolError> {
//     // validation of create ownership specific stuff
//     let Some(Module::Ownership { model, owner }) =
//         asset.layout.get(&(ModuleType::Ownership as u32));
//     if *model == OwnershipModel::Token && ctx.owner.owner != &spl_token::id() {
//         return Err(DigitalAssetProtocolError::ModuleError(
//             "Token Owner must be a Mint".to_string(),
//         ));
//     }
//     Ok(asset)
// }
//
fn create_royalty<'a>(
    ctx: &CreateNFTContext,
    asset: &'a mut Asset<'a>,
) -> Result<&'a mut Asset<'a>, DigitalAssetProtocolError> {
    // validation of create ownership specific stuff
    let module_data = asset.layout.get(&(ModuleType::Royalty as u32));
    if let Some(Module::Royalty {
                    model,
                    target,
                    royalty_percent,
                    locked
                }) = module_data {
        let creators_model = *model == RoyaltyModel::Creators;
        let creator_module = asset.layout.get(&(ModuleType::Creators as u32));
        if creators_model && creator_module.is_none() {
            return Err(DigitalAssetProtocolError::ModuleError(
                "Creators Must be set".to_string(),
            ));
        }
        let target = if creators_model && !target.is_empty() {
            Vec::default()
        } else {
            target.clone()
        };
        asset.layout.insert(ModuleType::Royalty as u32, Module::Royalty {
            model: *model,
            target,
            royalty_percent: 100,
            locked: false,
        });
    } else {
        return Err(DigitalAssetProtocolError::ModuleError(
            "1 or more creators must be present".to_string(),
        ));
    }
    Ok(asset)
}

// fn create_creators<'a>(
//     ctx: &CreateNFTContext,
//     asset: &'a mut Asset<'a>,
// ) -> Result<&'a mut Asset<'a>, DigitalAssetProtocolError> {
//     let module_data = asset.layout.get(&(ModuleType::Creators as u32));
//     if module_data.is_none() {
//         return Err(DigitalAssetProtocolError::ModuleError(
//             "1 or more creators must be present".to_string(),
//         ));
//     }
//     let Module::Creators { creator_list } = module_data.unwrap();
//
//     if creator_list.is_empty() {
//         return Err(DigitalAssetProtocolError::ModuleError(
//             "1 or more creators must be present".to_string(),
//         ));
//     }
//     Ok(asset)
// }

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let action = Action::deserialize(instruction_data)
        .map_err(|e| DigitalAssetProtocolError::DeError(e.to_string()))?;

    let res = match action {
        Action::Create { layout, standard } => {
            let address = accounts[0].clone();
            let owner = accounts[1].clone();
            let payer = accounts[2].clone();
            let authority = accounts[3].clone();

            let ctx = CreateNFTContext {
                address,
                owner,
                payer,
                authority,
                remaining_accounts: accounts[4..].to_vec(),
            };
            let mut asset = Asset {
                layout,
            };
            ctx.validate()?;
            let asset = ctx.act(&mut asset)?;
            asset.serialize(&mut *accounts[0].try_borrow_mut_data()?)
                .map_err(|e| {
                    DigitalAssetProtocolError::DeError(
                        e.to_string()
                    )
                })
        }
        _ => Err(DigitalAssetProtocolError::ActionError(
            "No Action Found".to_string(),
        ))
    };


    Ok(())
}


