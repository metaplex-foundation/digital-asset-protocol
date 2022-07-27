use std::collections::BTreeMap;
use bebop::SliceWrapper;
use solana_program::account_info::AccountInfo;
use crate::api::{DigitalAssetProtocolError};
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use crate::module::{DataItem, ModuleDataWrapper};
use crate::blob::Asset;
use crate::generated::schema::{Authority, ModuleData, ModuleType, OwnershipModel, RoyaltyModel, RoyaltyTarget};
use crate::generated::schema::owned::Creator;

pub struct CreateV1<'info> {
    pub id: AccountInfo<'info>,
    pub owner: AccountInfo<'info>,
    pub payer: AccountInfo<'info>,
    pub creators: Vec<Creator>,
    pub ownership_model: OwnershipModel,
    pub authorities: Vec<Authority<'info>>,
    pub royalty_model: RoyaltyModel,
    pub royalty_target: Option<RoyaltyTarget<'info>>,
    pub uri: String,
}

impl<'info> CreateV1<'info> {
    pub fn new(accounts: &[AccountInfo<'info>], uri: String) -> (Self, usize) {
        // Need program id System program,
        let program = accounts[0].clone();
        let system = accounts[1].clone();
        let rent = accounts[2].clone();
        l
        let creators =accounts[4]
        (
            CreateV1 {
                id: accounts[0].clone(),
                owner: accounts[1].clone(),
                payer: accounts[2].clone(),
                creators: vec![


                ],
                ownership_model: OwnershipModel::Invalid,
                authorities: vec![],
                royalty_model: RoyaltyModel::Invalid,
                royalty_target: None,
                uri,
            },
            3
        )
    }
}

impl ContextAction for CreateV1<'_> {
    fn lifecycle(&self) -> &Lifecycle {
        &Lifecycle::Create
    }

    fn run(&mut self) -> Result<(), DigitalAssetProtocolError> {
        let data = self.id.try_borrow_mut_data().map_err(|_| {
            DigitalAssetProtocolError::ActionError("Issue with Borrowing Data".to_string())
        })?;

        let modules = vec![
            ModuleType::Data,
            ModuleType::Ownership,
        ];
        let mut new_asset = Asset {
            layout: BTreeMap::new()
        };
        let owner_key = self.owner.key.to_bytes();
        for m in modules {
            let processor = ModuleType::to_processor(m);

            let data: Option<ModuleDataWrapper> = match m {
                ModuleType::Ownership => {
                    Some(ModuleDataWrapper::Structured(ModuleData::OwnershipData {
                        model: OwnershipModel::Single,git add
                        owner: SliceWrapper::from_raw(&owner_key)
                    }))
                }
                ModuleType::Data => {
                    let mut data = BTreeMap::new();
                    data.insert("uri".to_string(), DataItem::String(self.uri.clone()));
                    Some(ModuleDataWrapper::Unstructured(data))
                }
                _ => {
                    None
                }
            };
            processor.create(&mut new_asset, data)?;
        }
        Ok(())
    }
}