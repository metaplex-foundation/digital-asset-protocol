use crate::api::DigitalAssetProtocolError;
use crate::generated::schema::owned::ActionData;
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use solana_program::account_info::AccountInfo;

pub struct CancelSaleV1 {}

impl<'info> CancelSaleV1 {
    pub fn new(
        accounts: &[AccountInfo<'info>],
        action: ActionData,
    ) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::CancelSaleAssetV1 { msg } = action {
            return Ok((CancelSaleV1 {}, 0));
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be CancelSaleAssetV1".to_string(),
        ))
    }
}

impl ContextAction for CancelSaleV1 {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::CancelSale
    }

    fn run(&self) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}
