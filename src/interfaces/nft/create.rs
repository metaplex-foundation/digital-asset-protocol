use std::collections::BTreeMap;
use bebop::SliceWrapper;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::api::{DigitalAssetProtocolError, MutableAccount, MutSignerAccount, ReadOnlyAccount};
use crate::interfaces::ContextAction;
use crate::lifecycle::Lifecycle;
use crate::module::{DataItem, ModuleDataWrapper};
use crate::blob::Asset;
use crate::generated::schema::{Creator, Authority, ModuleData, ModuleType, NFTCreate, OwnershipModel, RoyaltyModel, RoyaltyTarget};

pub struct CreateV1<'info> {
    //Programs
    pub system: AccountInfo<'info>,
    pub compressor: AccountInfo<'info>,
    // Sysvars
    pub clock: Clock,
    pub rent: Rent,
    // Accounts
    pub id: MutableAccount<'info>,
    pub owner: ReadOnlyAccount<'info>,
    pub payer: MutSignerAccount<'info>,
    // Data
    pub creators: Option<Vec<Creator<'info>>>,
    pub ownership_model: OwnershipModel,
    pub authorities: Vec<Authority<'info>>,
    pub royalty_model: RoyaltyModel,
    pub royalty_target: Option<RoyaltyTarget<'info>>,
    pub royalty_bp: u16,
    pub compress: bool,
    pub uri: String,
}

impl<'info> CreateV1<'info> {
    pub fn new(accounts: &[AccountInfo<'info>], data: NFTCreate) -> Result<(Self, usize), DigitalAssetProtocolError> {
        let system = accounts[1].clone();
        let compressor = accounts[2].clone(); //chew it
        let payer = accounts[3].clone();
        let payer_key = payer.key;
        let creators = accounts.slice(4);
        let uri = data.uri.ok_or(
            || Err(DigitalAssetProtocolError::InterfaceError("Uri must be present".to_string())
            ))?.to_string();
        let royalty_config = data.royalty.ok_or(
            || Err(DigitalAssetProtocolError::InterfaceError("Royalty Config Required".to_string())
            ))?;
        Ok(
            (
                CreateV1 {
                    system,
                    compressor,
                    clock: Clock::get()?,
                    rent: Rent::get()?,
                    id: MutableAccount::new(2, accounts[2].clone())?,
                    owner: ReadOnlyAccount::new(3, accounts[3].clone())?,
                    payer: MutSignerAccount::new(4, payer)?,
                    creators: data.creators,
                    ownership_model: data.ownership_model.unwrap_or(OwnershipModel::Single),
                    authorities: data.authorities.unwrap_or(
                        vec![
                            Authority {
                                scopes: vec!["*"],
                                address: SliceWrapper::Raw(payer_key.as_ref()),
                            }
                        ]
                    ),
                    royalty_model: royalty_config.royalty_model.unwrap_or(RoyaltyModel::Creators),
                    royalty_bp: royalty_config.royalty_bp.unwrap_or(0),
                    royalty_target: royalty_config.royalty_target,
                    compress: data.compress.unwrap_or(false),
                    uri,
                },
                3
            )
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
                        model: OwnershipModel::Single,
                        owner: SliceWrapper::from_raw(&owner_key),
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