pub mod asset;
use crate::api::DigitalAssetProtocolError;
use crate::lifecycle::Lifecycle;


pub trait ContextAction {
    fn lifecycle(&self) -> &Lifecycle;
    fn run(&mut self) -> Result<(), DigitalAssetProtocolError>;
}