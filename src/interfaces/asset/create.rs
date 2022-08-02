use std::collections::{BTreeMap};
use std::fmt::format;
use bebop::SliceWrapper;
use solana_program::account_info::AccountInfo;
use crate::api::{DigitalAssetProtocolError};
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use crate::module::{DataItem, ModuleDataWrapper};
use crate::blob::Asset;
use crate::generated::schema::{Authority, ModuleData, ModuleType, OwnershipModel, RoyaltyModel, RoyaltyTarget, JsonDataSchema, Creator, ActionData, BlobContainer, DataItem};
use crate::required_field;
use crate::validation::validate_creator_shares;

pub struct CreateV1<'info> {
    pub id: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub creators: Vec<Creator<'info>>,
    pub ownership_model: OwnershipModel,
    pub authorities: Vec<Authority<'info>>,
    pub royalty_model: RoyaltyModel,
    pub royalty_target: Option<RoyaltyTarget<'info>>,
    pub off_chain_schema: JsonDataSchema,
    pub uri: String,
}

impl<'info> CreateV1<'info> {
    pub fn new(accounts: &[AccountInfo<'info>], action: ActionData<'info>) -> Result<(Self, usize), DigitalAssetProtocolError> {
        if let ActionData::CreateAssetV1 {
            uri,
            data_schema,
            royalty_model,
            royalty_target,
            ownership_model,
            creator_shares, // in percentage,
            authorities,
            ..
        } = action {
            // Need program id System program,
            let program = accounts[0].clone();
            let system = accounts[1].clone();
            let rent = accounts[2].clone();
            let id = accounts[3].clone();
            let owner = accounts[4].clone();
            let payer = accounts[5].clone();
            let payer_authority = payer.clone();
            let shares = required_field!(creator_shares)?;
            let creators = &accounts[6..accounts.len()];
            let remaining_accounts_index = 6 + creators.len();
            validate_creator_shares(creators, &shares)?;
            let creator_list = creators.iter().enumerate().map(|(i, ai)|{
                let verified = ai.is_signer;
                Creator{
                    address: SliceWrapper::Raw(ai.key.as_ref()),
                    share: shares[i],
                    verified
                }
            }).collect();
            let uri = required_field!(uri)?.to_string();
            let ownership_model = required_field!(ownership_model)?;
            let royalty_model = required_field!(royalty_model)?;
            let royalty_target = royalty_target;

            return Ok((
                CreateV1 {
                    id,
                    owner,
                    payer,
                    creators: creator_list,
                    ownership_model,
                    authorities: authorities.unwrap_or(vec![Authority{
                        scopes: vec![
                            "*"
                        ],
                        address: SliceWrapper::Raw(payer_authority.key.as_ref())
                    }]),
                    royalty_model,
                    royalty_target,
                    off_chain_schema: data_schema.unwrap_or(JsonDataSchema::Core),
                    uri: uri.to_string(),
                },
                remaining_accounts_index
            ));
        }
        Err(DigitalAssetProtocolError::ActionError("Invalid Action format, action must be CreateAssetV1".to_string()))
    }
}

impl ContextAction for CreateV1<'_> {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Create
    }

    fn run(&mut self) -> Result<(), DigitalAssetProtocolError> {
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
        let mut new_asset = Asset::new();
        let owner_key = self.owner.key.to_bytes();
        for m in modules {
            let processor = ModuleType::to_processor(m);
            let data: Option<ModuleDataWrapper> = match m {
                ModuleType::Ownership => {
                    Some(ModuleDataWrapper::Structured(ModuleData::OwnershipData {
                        model: OwnershipModel::Single,
                        owner: SliceWrapper::from_raw(&owner_key),
                    }))
                }
                ModuleType::Data => {
                    let mut data = BTreeMap::new();
                    data.insert("schema".to_string(), DataItem::Int(self.off_chain_schema as u32));
                    data.insert("uri".to_string(), DataItem::String(self.uri.clone()));
                    Some(
                        ModuleDataWrapper::Unstructured(data)
                    )
                }
                _ => {
                    None
                }
            };
            processor.create(&mut new_asset, data)?;
        }
        //Save asset
        new_asset.save(data)?;
        Ok(())
    }
}


