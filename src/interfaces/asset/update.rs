use std::cell::RefMut;
use std::collections::{BTreeMap};
use solana_program::account_info::AccountInfo;
use crate::api::{DigitalAssetProtocolError};
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use crate::blob::Asset;
use crate::generated::schema::{ActionData};


pub struct UpdateV1<'info> {
    pub id: &'info AccountInfo<'info>,
    pub owner: &'info AccountInfo<'info>,
    pub payer: &'info AccountInfo<'info>,
    pub payload: String,
}

impl<'info> UpdateV1<'info> {
    pub fn new(accounts: &'info [AccountInfo<'info>], action: ActionData) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::UpdateAssetV1 {
            msg
        } = action {
            let program = &accounts[0];
            let system = &accounts[1];
            let rent = &accounts[2];
            let id = &accounts[3];
            let owner = &accounts[4];
            let payer = &accounts[5];
            return Ok((UpdateV1 {
                id,
                owner,
                payer,
                payload: msg.unwrap().parse().unwrap(),
            }, 6)
            );
        }
        Err(DigitalAssetProtocolError::ActionError("Invalid Action format, action must be UpdateAssetV1".to_string()))
    }
}

impl<'info> ContextAction for UpdateV1<'info> {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Update
    }

    fn run(self) -> Result<(), DigitalAssetProtocolError> {
        let mut data = self.id.try_borrow_mut_data().map_err(|_| {
            DigitalAssetProtocolError::ActionError("Issue with Borrowing Data".to_string())
        })?;
        let mut asset = Asset::load_mut(&mut *data)?;
        asset.
        asset.save(data)?;
        Ok(())
    }
}