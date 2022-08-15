use std::io::BufWriter;
use bebop::{Record, SliceWrapper, SubRecord};
use lazy_static::lazy_static;
use solana_program::account_info::AccountInfo;
use crate::api::DigitalAssetProtocolError;
use crate::blob::{Asset};
use crate::generated::schema::{ModuleData, ModuleType};
use crate::module::{ModuleDataWrapper, ModuleId, ModuleProcessor};


pub struct OwnershipModuleProcessor {}

pub static OWNERSHIP_MODULE_PROCESSOR: OwnershipModuleProcessor = OwnershipModuleProcessor {};

impl ModuleProcessor for OwnershipModuleProcessor {
    fn create(&self,
                    asset: &mut Asset
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

    fn update<'raw>(&self, asset: &mut Asset, new_data: ModuleData) -> Result<(), DigitalAssetProtocolError> {
        todo!()
    }
}

