use crate::api::DigitalAssetProtocolError;
use crate::blob::Asset;
use crate::module::{ModuleDataWrapper, ModuleProcessor};
use solana_program::account_info::AccountInfo;

pub struct SignatureModuleProcessor {}

pub static SIGNATURE_MODULE_PROCESSOR: SignatureModuleProcessor = SignatureModuleProcessor {};

impl ModuleProcessor for SignatureModuleProcessor {
    fn cancel_sale<'raw>(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
    fn create(&self, asset: &mut Asset) -> Result<(), DigitalAssetProtocolError> {
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
