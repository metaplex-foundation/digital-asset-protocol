use crate::api::DigitalAssetProtocolError;
use crate::blob::Asset;
use crate::generated::schema::owned::{ModuleData, ModuleType};
use crate::module::{ModuleDataWrapper, ModuleId, ModuleProcessor};
use bebop::{Record, SliceWrapper, SubRecord};
use lazy_static::lazy_static;
use solana_program::account_info::AccountInfo;
use std::io::BufWriter;

pub struct RoyaltyModuleProcessor {}

pub static ROYALTY_MODULE_PROCESSOR: RoyaltyModuleProcessor = RoyaltyModuleProcessor {};

impl ModuleProcessor for RoyaltyModuleProcessor {
    fn cancel_sale<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn create<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        match asset.get_module(ModuleType::Royalty) {
            Some(ModuleDataWrapper::Structured(ModuleData::RoyaltyData { .. })) => Ok(()),
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
    fn sell<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}
