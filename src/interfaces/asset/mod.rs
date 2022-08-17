mod create;
// mod update;



// pub use update::*;
pub use create::*;

use crate::api::{DigitalAssetProtocolError, AccountWrapper};
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


impl Interface for AssetInterface {
    fn process_action<'entry>(&self, accounts: AccountWrapper<'entry>, data: ActionData<'entry>) -> Result<(), DigitalAssetProtocolError> {
        let context = data;
        match context {
            ActionData::CreateAssetV1 { .. } => {
                CreateV1::run(accounts, context)
            }
            _ => Err(DigitalAssetProtocolError::InterfaceNoImpl)
        }
    }
}