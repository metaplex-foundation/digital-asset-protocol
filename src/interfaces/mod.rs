pub mod asset;

use crate::api::{DigitalAssetProtocolError, Message};
use crate::generated::schema::InterfaceType;
use crate::interfaces::asset::ASSET_INTERFACE;
use crate::lifecycle::Lifecycle;


pub trait ContextAction {
    fn lifecycle(&self) -> &Lifecycle;
    fn run(self) -> Result<(), DigitalAssetProtocolError>;
}

pub trait Interface<'entry> {
    fn handle_message(&self, message: &'entry mut Message<'entry>) -> Result<(), DigitalAssetProtocolError>;
}


pub fn get_interface<'entry>(message: &'entry Message<'entry>) -> Result<&dyn Interface, DigitalAssetProtocolError> {
    match message.action.interface {
        InterfaceType::Nft => {
            Ok(
                &ASSET_INTERFACE
            )
        }
        _ => Err(DigitalAssetProtocolError::InterfaceError("Interface Not Supported".to_string()))
    }
}
