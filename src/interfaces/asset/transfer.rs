use crate::api::DigitalAssetProtocolError;
use crate::generated::schema::owned::ActionData;
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use solana_program::account_info::AccountInfo;

pub struct TransferV1 {}

impl<'info> TransferV1 {
    pub fn new(
        accounts: &[AccountInfo<'info>],
        action: ActionData,
    ) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::TransferAssetV1 { msg } = action {
            return Ok((TransferV1 {}, 0));
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be TransferAssetV1".to_string(),
        ))
    }
}

impl ContextAction for TransferV1 {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Transfer
    }

    fn run(&self) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}
