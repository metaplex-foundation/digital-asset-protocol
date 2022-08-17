pub mod asset;


use crate::api::{DigitalAssetProtocolError, AccountWrapper};
use crate::generated::schema::{ActionData, InterfaceType};
use crate::interfaces::asset::ASSET_INTERFACE;
use crate::lifecycle::Lifecycle;


pub trait ContextAction {
    fn lifecycle(&self) -> &Lifecycle;
    fn run(self) -> Result<(), DigitalAssetProtocolError>;
}

pub trait Interface {
    fn process_action<'entry>(&self, accounts: AccountWrapper<'entry>, data: ActionData<'entry>) -> Result<(), DigitalAssetProtocolError>;
}


pub fn get_interface<'entry>(interface: InterfaceType) -> Result<&'entry dyn Interface, DigitalAssetProtocolError> {
    match interface {
        InterfaceType::Nft => {
            Ok(
                &ASSET_INTERFACE
            )
        }
        _ => Err(DigitalAssetProtocolError::InterfaceError("Interface Not Supported".to_string()))
    }
}
