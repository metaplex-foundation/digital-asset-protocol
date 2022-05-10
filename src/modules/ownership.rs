
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use crate::modules::{Modularized, Module};
use thiserror::Error;
use crate::state::Lifecycle;

#[derive(Error, Debug)]
pub enum OwnershipModuleError {
    #[error("Instruction Owner: Owner Address must be an Identity, System Program owner, Or Mint")]
    CreationInvalidOwner,
    #[error("Token ownership must be combined to a single address with all shares first")]
    TransferNotAllowedForMintOwnership,
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

pub struct OwnershipModule {}

impl OwnershipModule {
    pub fn new() -> Self {
        OwnershipModule{}
    }
}

impl Modularized for OwnershipModule {
    fn module_type(&self) -> Module {
        Module::Ownership
    }

    fn events(&self) -> Vec<Lifecycle> {
        vec![Lifecycle::Create, Lifecycle::Transfer]
    }

    // fn modify<'a>(&self, action: &dyn Action, asset: &'a mut Asset) -> Result<(), DigitalAssetProtocolError> {
    //     match action.lifecycle() {
    //         Lifecycle::Create => {
    //             let own: &mut Blob<OwnershipModuleData> = asset.get_module_data::<OwnershipModuleData>(self.module_type()).unwrap();
    //             let own_mut = own.get_mut_data();
    //             let cc: &CreateContext = action.as_any().downcast_ref::<CreateContext>().expect("CC");
    //             let system = cc.owner.owner != &solana_program::system_program::id();
    //             let identity = cc.owner.owner != &crate::id();
    //             let token = cc.owner.owner != &spl_token::id();
    //             if !system && !identity && !token {
    //                 return Err(ModuleError(OwnershipModuleError::CreationInvalidOwner.into()));
    //             }
    //
    //             own_mut.ownership_model = if token {
    //                 OwnershipModel::Token
    //             } else {
    //                 OwnershipModel::Token
    //             };
    //             own_mut.owner = *cc.owner.key;
    //             Ok(())
    //         }
    //         Lifecycle::Update => {
    //             // Update Ownership
    //
    //
    //             Ok(())
    //         }
    //
    //         _ => Ok(())
    //     }
    // }
}
