use crate::api::DigitalAssetProtocolError;
use crate::blob::Asset;
use crate::generated::schema::owned::{
    ActionData, Authority, Creator, ModuleType, OwnershipModel, RoyaltyModel, RoyaltyTarget,
};
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use crate::module::ModuleDataWrapper;
use crate::required_field;
use crate::validation::validate_creator_shares;
use solana_program::account_info::AccountInfo;

pub struct SellV1<'info> {
    pub id: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub new_owner: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub creators: Vec<Creator>,
    pub ownership_model: OwnershipModel,
    pub authorities: Vec<Authority>,
    pub royalty_model: RoyaltyModel,
    pub royalty_target: Option<RoyaltyTarget>,
}

impl<'info> SellV1<'info> {
    pub fn new(
        accounts: &[AccountInfo<'info>],
        action: ActionData,
    ) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::SellAssetV1 {
            royalty_model,
            royalty_target,
            ownership_model,
            creator_shares, // in percentage,
            authorities,
            ..
        } = action
        {
            // Need program id System program,
            let _program = accounts[0].clone();
            let _system = accounts[1].clone();
            let _rent = accounts[2].clone();
            let id = accounts[3].clone();
            let owner = accounts[4].clone();
            let new_owner = accounts[5].clone();
            let payer = accounts[6].clone();
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
                SellV1 {
                    id,
                    owner,
                    new_owner,
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
            "Invalid Action format, action must be SellAssetV1".to_string(),
        ))
    }
}

impl<'info> ContextAction for SellV1<'info> {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Sell
    }

    fn run(&self) -> Result<(), DigitalAssetProtocolError> {
        let raw_data = self.id.try_borrow_mut_data().map_err(|_| {
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
        let mut asset = Asset::load(&self.id.data.borrow()).unwrap();
        let _owner_key = self.owner.key.to_bytes();
        let _new_owner_key = self.new_owner.key.to_bytes();
        for m in modules {
            let processor = ModuleType::to_processor(m);
            let _module_data: Option<&mut ModuleDataWrapper> = asset.get_module(m);
            processor.sell(&mut asset)?;
        }
        //Save asset
        asset.save(raw_data)?;
        Ok(())
    }
}
