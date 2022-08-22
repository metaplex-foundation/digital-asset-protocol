use crate::api::DigitalAssetProtocolError;
use crate::blob::Asset;
use crate::generated::schema::{ModuleData, ModuleType};
use crate::module::ModuleProcessor;

pub struct RoyaltyModuleProcessor {}

pub static ROYALTY_MODULE_PROCESSOR: RoyaltyModuleProcessor = RoyaltyModuleProcessor {};

impl ModuleProcessor for RoyaltyModuleProcessor {
    fn cancel_sale<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn create(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        match asset.get_module(ModuleType::Royalty) {
            Some(ModuleData::RoyaltyData { .. }) => Ok(()),
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
        // TODO: Code that freezes royalties so they can't be changed when listed.
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
