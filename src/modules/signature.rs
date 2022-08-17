
use crate::api::DigitalAssetProtocolError;
use crate::blob::Asset;
use crate::module::{ModuleProcessor};

pub struct SignatureModuleProcessor {}

pub static SIGNATURE_MODULE_PROCESSOR: SignatureModuleProcessor = SignatureModuleProcessor {};

impl ModuleProcessor for SignatureModuleProcessor {
    fn create(&self,
                    _asset: &mut Asset
    )
                    -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}