use std::any::Any;
use std::io;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use crate::api::DigitalAssetProtocolError;
use crate::modules::ownership::OwnershipModule;
use crate::state::{Action, Lifecycle};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub enum Module {
    Ownership,
    Data,
    Governance,
    Creators,
    Royalty,
    Rights,
    Supply,
    Grouped,
    Provenance,
    Signature,
    Usage,
    Extension
}

impl Module {
    pub fn get_instance(self) -> Option<Box<dyn Modularized>> {
        match self {
            Module::Ownership => Some(Box::new(OwnershipModule::new())),
            _ => None

        }
    }

}
pub trait Modularized {
    fn module_type(&self) -> Module;
    fn events(&self) -> Vec<Lifecycle>;
    // fn modify<'a>(&self, asset: &'a mut Asset) -> Result<(), DigitalAssetProtocolError>;
}