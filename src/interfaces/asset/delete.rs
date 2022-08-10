use crate::api::DigitalAssetProtocolError;
use crate::generated::schema::owned::ActionData;
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use solana_program::account_info::AccountInfo;

pub struct DeleteV1 {}

impl<'info> DeleteV1 {
    pub fn new(
        accounts: &[AccountInfo<'info>],
        action: ActionData,
    ) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::DeleteAssetV1 { msg } = action {
            return Ok((DeleteV1 {}, 0));
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be DeleteAssetV1".to_string(),
        ))
    }
}

impl ContextAction for DeleteV1 {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Delete
    }

    fn run(&self) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}
