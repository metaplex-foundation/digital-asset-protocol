mod cancel_sale;
mod create;
mod delegate;
mod delete;
mod freeze;
mod list_for_sale;
mod sell;
mod transfer;
mod update;

pub use cancel_sale::*;
pub use create::*;
pub use delegate::*;
pub use delete::*;
pub use freeze::*;
pub use list_for_sale::*;
pub use sell::*;
pub use transfer::*;
pub use update::*;

use super::Interface;
use crate::api::{AccountWrapper, DigitalAssetProtocolError};
use crate::generated::schema::{ActionData, ModuleType};

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
    fn process_action<'entry>(
        &self,
        accounts: AccountWrapper<'entry>,
        data: ActionData<'entry>,
    ) -> Result<(), DigitalAssetProtocolError> {
        let context = data;
        match context {
            ActionData::CancelSaleAssetV1 { .. } => CancelSaleV1::run(accounts, context),
            ActionData::CreateAssetV1 { .. } => CreateV1::run(accounts, context),
            ActionData::DelegateAssetV1 { .. } => DelegateV1::run(accounts, context),
            ActionData::DeleteAssetV1 { .. } => DeleteV1::run(accounts, context),
            ActionData::FreezeAssetV1 { .. } => FreezeV1::run(accounts, context),
            ActionData::ListForSaleAssetV1 { .. } => ListForSaleV1::run(accounts, context),
            ActionData::SellAssetV1 { .. } => SellV1::run(accounts, context),
            ActionData::TransferAssetV1 { .. } => TransferV1::run(accounts, context),
            ActionData::UpdateAssetV1 { .. } => UpdateV1::run(accounts, context),
            _ => Err(DigitalAssetProtocolError::InterfaceNoImpl),
        }
    }
}
