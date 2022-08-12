use crate::api::DigitalAssetProtocolError;
use crate::blob::Asset;
use crate::generated::schema::owned::{ModuleData, ModuleType};
use crate::module::{ModuleDataWrapper, ModuleId, ModuleProcessor};
use bebop::{Record, SliceWrapper, SubRecord};
use lazy_static::lazy_static;
use solana_program::account_info::AccountInfo;
use std::io::BufWriter;

pub struct OwnershipModuleProcessor {}

pub static OWNERSHIP_MODULE_PROCESSOR: OwnershipModuleProcessor = OwnershipModuleProcessor {};

impl ModuleProcessor for OwnershipModuleProcessor {
    fn cancel_sale<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn create<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        match asset.get_module(ModuleType::Ownership) {
            Some(ModuleDataWrapper::Structured(ModuleData::OwnershipData { .. })) => Ok(()),
            _ => Err(DigitalAssetProtocolError::ModuleError(
                "Incorrect Data Type for Module".to_string(),
            )),
        }?;
        Ok(())
    }
    fn delegate<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn delete<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn freeze<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn list_for_sale<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn transfer<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn update<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}
