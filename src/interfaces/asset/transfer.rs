use crate::api::DigitalAssetProtocolError;
use crate::blob::Asset;
use crate::generated::schema::owned::{
    ActionData, Authority, Creator, DataItemValue, ModuleData, ModuleType, OwnershipModel,
    RoyaltyModel, RoyaltyTarget,
};
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use crate::module::ModuleDataWrapper;
use crate::required_field;
use crate::validation::validate_creator_shares;
use solana_program::account_info::AccountInfo;
use std::collections::BTreeMap;

pub struct TransferV1<'info> {
    pub id: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub creators: Vec<Creator>,
    pub ownership_model: OwnershipModel,
    pub authorities: Vec<Authority>,
    pub royalty_model: RoyaltyModel,
    pub royalty_target: Option<RoyaltyTarget>,
}

impl<'info> TransferV1<'info> {
    pub fn new(
        accounts: &[AccountInfo<'info>],
        action: ActionData,
    ) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::TransferAssetV1 {
            royalty_model,
            royalty_target,
            ownership_model,
            creator_shares, // in percentage,
            authorities,
            ..
        } = action
        {
            // Need program id System program,
            let program = accounts[0].clone();
            let system = accounts[1].clone();
            let rent = accounts[2].clone();
            let id = accounts[3].clone();
            let owner = accounts[4].clone();
            let payer = accounts[5].clone();
            let payer_authority = payer.clone();
            let shares: Vec<u8> = required_field!(creator_shares)?;
            let creators = &accounts[6..accounts.len()];
            let remaining_accounts_index = 6 + creators.len();
            validate_creator_shares(creators, &shares)?;
            let creator_list = creators
                .iter()
                .enumerate()
                .map(|(i, ai)| {
                    let verified = ai.is_signer;
                    Creator {
                        address: ai.key.to_bytes().to_vec(),
                        share: shares[i],
                        verified,
                    }
                })
                .collect();
            let ownership_model = required_field!(ownership_model)?;
            let royalty_model = required_field!(royalty_model)?;
            let royalty_target = royalty_target;

            return Ok((
                TransferV1 {
                    id,
                    owner,
                    payer,
                    creators: creator_list,
                    ownership_model,
                    authorities: authorities.unwrap_or_else(|| {
                        vec![Authority {
                            scopes: vec!["*".to_string()],
                            address: payer_authority.key.to_bytes().to_vec(),
                        }]
                    }),
                    royalty_model,
                    royalty_target,
                },
                remaining_accounts_index,
            ));
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be TransferAssetV1".to_string(),
        ))
    }
}

impl<'info> ContextAction for TransferV1<'info> {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Transfer
    }

    fn run(&self) -> Result<(), DigitalAssetProtocolError> {
        let mut data = self.id.try_borrow_mut_data().map_err(|_| {
            DigitalAssetProtocolError::ActionError("Issue with Borrowing Data".to_string())
        })?;
        let modules = vec![
            ModuleType::Data,
            ModuleType::Ownership,
            ModuleType::Creators,
            ModuleType::Royalty,
            ModuleType::Governance,
            ModuleType::Rights,
            ModuleType::Extension,
        ];
        let asset = Asset::load(&self.id.data.borrow()).unwrap();
        let owner_key = self.owner.key.to_bytes();
        for m in modules {
            let processor = ModuleType::to_processor(m);
            let data: Option<&mut ModuleDataWrapper> = asset.get_module(m);
            processor.transfer(&mut asset)?;
        }
        //Save asset
        asset.save(data)?;
        Ok(())
    }
}
