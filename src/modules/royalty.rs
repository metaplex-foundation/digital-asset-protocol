use crate::api::DigitalAssetProtocolError;
use crate::blob::{Asset, Blob};
use crate::generated::schema::ModuleType;
use crate::module::{ModuleDataWrapper, ModuleId, ModuleProcessor};
use bebop::{Record, SliceWrapper, SubRecord};
use lazy_static::lazy_static;
use solana_program::account_info::AccountInfo;
use std::io::BufWriter;

pub struct RoyaltyModuleProcessor {}

pub static ROYALTY_MODULE_PROCESSOR: RoyaltyModuleProcessor = RoyaltyModuleProcessor {};

impl ModuleProcessor for RoyaltyModuleProcessor {
    fn create<'raw>(
        &self,
        asset: &mut Asset<'raw>,
        module_data: Option<ModuleDataWrapper<'raw>>,
    ) -> Result<(), DigitalAssetProtocolError> {
        let royalty_data = match module_data {
            Some(ModuleDataWrapper::Structured(d)) => Ok(d),
            _ => Err(DigitalAssetProtocolError::ModuleError(
                "Incorrect Data Type for Module".to_string(),
            )),
        }?;

        let mut raw_data = Vec::with_capacity(royalty_data.serialized_size());
        royalty_data
            .serialize(&mut raw_data)
            .map_err(|e| DigitalAssetProtocolError::ModuleError(e.to_string()))?;
        let blob = Blob {
            schema: ModuleId::Module(ModuleType::Royalty),
            data: raw_data,
            _runtime_data: Some(royalty_data.to_owned()),
        };
        asset
            .layout
            .insert(ModuleId::Module(ModuleType::Royalty), blob);
        Ok(())
    }
}
