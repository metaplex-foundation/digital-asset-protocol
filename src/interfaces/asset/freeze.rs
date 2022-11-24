use crate::api::DigitalAssetProtocolError;
use crate::generated::schema::owned::ActionData;
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use solana_program::account_info::AccountInfo;

pub struct FreezeV1 {}

impl<'info> FreezeV1 {
    pub fn new(
        accounts: &[AccountInfo<'info>],
        action: ActionData,
    ) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::FreezeAssetV1 { msg } = action {
            return Ok((FreezeV1 {}, 0));
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be FreezeAssetV1".to_string(),
        ))
    }
}

impl ContextAction for FreezeV1 {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Freeze
    }

    fn run(&self) -> Result<(), DigitalAssetProtocolError> {
        Ok(())
    }
}
