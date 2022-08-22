use solana_program::msg;
use crate::api::DigitalAssetProtocolError;
use crate::blob::{Asset};
use crate::generated::schema::{ModuleData, ModuleType};
use crate::module::{ModuleProcessor};


pub struct OwnershipModuleProcessor {}

pub static OWNERSHIP_MODULE_PROCESSOR: OwnershipModuleProcessor = OwnershipModuleProcessor {};

impl ModuleProcessor for OwnershipModuleProcessor {
    fn create(&self,
              asset: &mut Asset,
    )
              -> Result<(), DigitalAssetProtocolError> {
        match asset.get_module(ModuleType::Ownership) {
            Some(ModuleData::OwnershipData { .. }) => Ok(()),
            _ => {
                Err(DigitalAssetProtocolError::ModuleError("Incorrect Data Type for Module".to_string()))
            }
        }?;
        Ok(())
    }
}

