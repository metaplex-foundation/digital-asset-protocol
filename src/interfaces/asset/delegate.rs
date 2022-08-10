use crate::api::DigitalAssetProtocolError;
use crate::generated::schema::owned::ActionData;
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use solana_program::account_info::AccountInfo;

pub struct DelegateV1 {}

impl<'info> DelegateV1 {
    pub fn new(
        accounts: &[AccountInfo<'info>],
        action: ActionData,
    ) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::DelegateAssetV1 { msg } = action {
            return Ok((DelegateV1 {}, 0));
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be DelegateAssetV1".to_string(),
        ))
    }
}

impl ContextAction for DelegateV1 {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Delegate
    }

    fn run(&self) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}
