use std::collections::HashMap;

use bebop::SliceWrapper;

use crate::api::{AccountWrapper, Constraints, DigitalAssetProtocolError};

use crate::blob::Asset;
use crate::generated::schema::{ActionData, DataItem, DataItemValue, ModuleData, ModuleType};
use crate::interfaces::asset::MODULE_LAYOUT;
use crate::required_field;

pub struct SellV1 {}

impl SellV1 {
    pub fn run<'entry>(
        accounts: AccountWrapper<'entry>,
        data: ActionData<'entry>,
    ) -> Result<(), DigitalAssetProtocolError> {
        if let ActionData::SellAssetV1 {
            ownership_model,
            royalty_model,
            royalty_target,
            creator_shares,
            authorities,
            ..
        } = &data
        {
            return Ok(());
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be SellAssetV1".to_string(),
        ))
    }
}
