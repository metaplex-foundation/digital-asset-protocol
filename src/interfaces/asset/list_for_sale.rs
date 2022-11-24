use crate::api::DigitalAssetProtocolError;
use crate::generated::schema::owned::ActionData;
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use solana_program::account_info::AccountInfo;

pub struct ListForSaleV1 {}

impl<'info> ListForSaleV1 {
    pub fn new(
        accounts: &[AccountInfo<'info>],
        action: ActionData,
    ) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::ListForSaleAssetV1 { msg } = action {
            return Ok((ListForSaleV1 {}, 0));
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be ListForSaleAssetV1".to_string(),
        ))
    }
}

impl ContextAction for ListForSaleV1 {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::ListForSale
    }

    fn run(&self) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}
