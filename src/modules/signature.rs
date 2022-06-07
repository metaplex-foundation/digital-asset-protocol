use solana_program::account_info::AccountInfo;
use crate::api::DigitalAssetProtocolError;
use crate::blob::Asset;
use crate::module::{ModuleProcessor, ModuleDataWrapper};

pub struct SignatureModuleProcessor {}

pub static SIGNATURE_MODULE_PROCESSOR: SignatureModuleProcessor = SignatureModuleProcessor {};

impl ModuleProcessor for SignatureModuleProcessor {
    fn create<'raw>(&self,
                    asset: &mut Asset<'raw>,
                    module_data: Option<ModuleDataWrapper<'raw>>,
    )
                    -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}