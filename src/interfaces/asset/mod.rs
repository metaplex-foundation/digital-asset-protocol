mod create;
// mod update;

use solana_program::pubkey::Pubkey;
// pub use update::*;
pub use create::*;

use crate::api::{DigitalAssetProtocolError, Message};
use crate::generated::schema::{ActionData, ModuleType};
use super::Interface;


static MODULE_LAYOUT: &[ModuleType] = &[
    ModuleType::Rights,
    ModuleType::Data,
    ModuleType::Ownership,
    ModuleType::Creators,
    ModuleType::Royalty,
    ModuleType::Governance,
    ModuleType::Extension,
];
pub static ASSET_INTERFACE: AssetInterface = AssetInterface {};

pub struct AssetInterface;


impl<'entry> Interface<'entry> for AssetInterface {
    fn handle_message(&self, message: &'entry mut Message<'entry>) -> Result<(), DigitalAssetProtocolError> {
        let context = &message.action.data;
        match context {
            ActionData::CreateAssetV1 { .. } => {
                CreateV1::run(message)
            }
            _ => Err(DigitalAssetProtocolError::InterfaceNoImpl)
        }
    }
}