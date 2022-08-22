use std::collections::HashMap;

use bebop::SliceWrapper;

use crate::api::{AccountWrapper, Constraints, DigitalAssetProtocolError};

use crate::blob::Asset;
use crate::generated::schema::{ActionData, DataItem, DataItemValue, ModuleData, ModuleType};
use crate::interfaces::asset::MODULE_LAYOUT;
use crate::required_field;

pub struct CreateV1 {}

impl CreateV1 {
    pub fn run<'entry>(
        accounts: AccountWrapper<'entry>,
        data: ActionData<'entry>,
    ) -> Result<(), DigitalAssetProtocolError> {
        if let ActionData::CreateAssetV1 {
            uri,
            data_schema,
            royalty_model,
            royalty_target,
            ownership_model,
            creator_shares: _, // in percentage,
            authorities: _,
            royalty,
            uuid,
            ..
        } = &data
        {
            let accounts_size = accounts.accounts_length();
            let rng = 4..accounts_size;
            let mut accounts = accounts;
            let uuid = *required_field!(uuid)?;
            let seeds = ["ASSET".as_bytes(), uuid[0..8].as_ref()];
            let system = accounts.system_program(0)?;
            let mut asset_account = accounts.prepare_account(
                1,
                "asset",
                Constraints::pda(seeds.as_slice(), crate::id(), true, true),
            )?;
            let owner = accounts.prepare_account(2, "owner", Constraints::read_only())?;
            let payer = accounts.prepare_account(3, "payer", Constraints::payer())?;
            let mut creator_accounts = Vec::with_capacity(rng.len());
            for c in rng {
                creator_accounts.push(accounts.prepare_account(
                    c as usize,
                    "creator",
                    Constraints::read_only(),
                )?);
            }

            let uri = required_field!(uri)?;
            let ownership_model = required_field!(ownership_model)?;
            let royalty_model = required_field!(royalty_model)?;
            // Required field doesnt work here for some lifetime and borrow reason
            let royalty_target =
                royalty_target
                    .as_ref()
                    .ok_or(DigitalAssetProtocolError::DeError(
                        "Royalty Target ".parse().unwrap(),
                    ))?;
            let off_chain_schema = required_field!(data_schema)?;

            // TODO -> make fluent interface for this or macros

            let mut new_asset = Asset::new();
            let mut processors = Vec::with_capacity(MODULE_LAYOUT.len());
            for mt in MODULE_LAYOUT.iter() {
                processors.push(ModuleType::to_processor(mt));
                match mt {
                    ModuleType::Rights => {
                        //TODO set rules based on the interface
                    }
                    ModuleType::Ownership => new_asset.set_module(
                        ModuleType::Ownership,
                        ModuleData::OwnershipData {
                            model: ownership_model,
                            owner: SliceWrapper::Raw(owner.info.key.as_ref()),
                        },
                    ),
                    ModuleType::Royalty => {
                        new_asset.set_module(
                            ModuleType::Royalty,
                            ModuleData::RoyaltyData {
                                model: royalty_model,
                                target: royalty_target.to_owned(),
                                royalty: royalty.unwrap_or(0),
                                locked: false,
                            },
                        );
                    }
                    ModuleType::Data => {
                        let mut data: HashMap<u8, DataItem> = HashMap::with_capacity(2);
                        data.insert(
                            0,
                            DataItem {
                                key: "uri",
                                value: DataItemValue::String {
                                    value: Some(<&str>::clone(&uri)),
                                },
                            },
                        );
                        data.insert(
                            1,
                            DataItem {
                                key: "schema",
                                value: DataItemValue::Int {
                                    value: Some(off_chain_schema as i32),
                                },
                            },
                        );
                        new_asset.set_module(ModuleType::Data, ModuleData::Data { layout: data });
                    }
                    _ => {}
                }
            }

            for p in processors {
                p.create(&mut new_asset)?;
            }

            asset_account.initialize_account(new_asset.size() as u64, &system, &payer)?;
            let buffer = asset_account.mut_data();
            new_asset.save(buffer)?;
            return Ok(());
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be CreateAssetV1".to_string(),
        ))
    }
}

// pub struct CreateV1<'info> {
//     pub system: &'info AccountInfo<'info>,
//     pub id: &'info AccountInfo<'info>,
//     pub owner: &'info AccountInfo<'info>,
//     pub payer: &'info AccountInfo<'info>,
//     pub uuid: &'info [u8],
//     pub creators: Vec<Creator<'info>>,
//     pub ownership_model: OwnershipModel,
//     pub authorities: Vec<Authority<'info>>,
//     pub royalty_model: RoyaltyModel,
//     pub royalty: u16,
//     pub royalty_target: Vec<RoyaltyTarget<'info>>,
//     pub off_chain_schema: JsonDataSchema,
//     pub uri: &'info str,
// }
//
// impl<'info> CreateV1<'info> {
//     pub fn new(accounts: &'info [AccountInfo<'info>], action: ActionData<'info>) -> Result<(Self, usize), DigitalAssetProtocolError> {
//         if let ActionData::CreateAssetV1 {
//             uri,
//             data_schema,
//             royalty_model,
//             royalty_target,
//             ownership_model,
//             creator_shares, // in percentage,
//             authorities,
//             royalty,
//             uuid,
//             ..
//         } = action {
//             // Need program id System program,
//             let system = &accounts[0];
//             let id = &accounts[1];
//             let owner = &accounts[2];
//             let payer = &accounts[3];
//             let payer_authority = &payer;
//             let shares: SliceWrapper<u8> = required_field!(creator_shares)?;
//             if accounts.len() < 5 {
//                 return Err(DigitalAssetProtocolError::InterfaceError("Creators Must be Present".to_string()));
//             }
//             let creators = &accounts[4..accounts.len()];
//             let remaining_accounts_index = 4 + creators.len();
//             validate_creator_shares(creators, &shares)?;
//             let creator_list: Vec<Creator> = creators.iter().enumerate().map(|(i, ai)| {
//                 let verified = ai.is_signer;
//                 Creator {
//                     address: SliceWrapper::Raw(ai.key.as_ref()),
//                     share: shares[i],
//                     verified,
//                 }
//             }).collect();
//             let uri = required_field!(uri)?;
//             let ownership_model = required_field!(ownership_model)?;
//             let royalty_model = required_field!(royalty_model)?;
//             let royalty_target = required_field!(royalty_target)?;
//             let uuid = &*required_field!(uuid)?;
//
//             return Ok((
//                 CreateV1 {
//                     system,
//                     id,
//                     owner,
//                     payer,
//                     creators: creator_list,
//                     ownership_model,
//                     royalty: royalty.unwrap_or(0),
//                     authorities: authorities.unwrap_or(vec![Authority {
//                         scopes: vec![
//                             "*"
//                         ],
//                         address: SliceWrapper::Raw(payer_authority.key.as_ref()),
//                     }]),
//                     royalty_model,
//                     royalty_target: royalty_target,
//                     off_chain_schema: data_schema.unwrap_or(JsonDataSchema::Core),
//                     uri,
//                     uuid: uuid.as_slice(),
//                 },
//                 remaining_accounts_index
//             ));
//         }
//         Err(DigitalAssetProtocolError::ActionError("Invalid Action format, action must be CreateAssetV1".to_string()))
//     }
// }
//
// impl<'info> ContextAction for CreateV1<'info> {
//     fn lifecycle(&self) -> &Lifecycle {
//         &Lifecycle::Create
//     }
//
//     fn run(self) -> Result<(), DigitalAssetProtocolError> {
//         let modules = vec![
//             ModuleType::Data,
//             ModuleType::Ownership,
//             ModuleType::Creators,
//             ModuleType::Royalty,
//             ModuleType::Governance,
//             ModuleType::Rights,
//             ModuleType::Extension,
//         ];
//         let mut new_asset = Asset::new();
//         let owner_key = self.owner.key.to_bytes();
//         for m in modules {
//             match m {
//                 ModuleType::Ownership => {
//                     new_asset.set_module(ModuleType::Ownership, ModuleData::OwnershipData {
//                         model: self.ownership_model,
//                         owner: SliceWrapper::Raw(owner_key.as_ref()),
//                     })
//                 }
//                 ModuleType::Royalty => {
//                     new_asset.set_module(ModuleType::Royalty, ModuleData::RoyaltyData {
//                         model: self.royalty_model,
//                         target: self.royalty_target.to_owned(),
//                         royalty: self.royalty,
//                         locked: false,
//                     });
//                 }
//                 ModuleType::Data => {
//                     let mut data: HashMap<u8, DataItem> = HashMap::with_capacity(2);
//                     data.insert(0, DataItem { key: "uri", value: DataItemValue::String { value: Some(self.uri.clone()) } });
//                     data.insert(1, DataItem { key: "schema", value: DataItemValue::Int { value: Some(self.off_chain_schema as i32) } });
//                     new_asset.set_module(ModuleType::Data, ModuleData::Data {
//                         layout: data
//                     });
//                 }
//                 _ => {}
//             };
//         }
//         for m in modules {
//             let processor = ModuleType::to_processor(m);
//             processor.create(&mut new_asset)?;
//         }
//         //Save asset
//         let rent = Rent::get()?;
//         let size = new_asset.size();
//         let lamports = rent.minimum_balance(size);
//         //validate address get bump
//         let seeds = [
//             "DAS-ASSET".as_bytes(),
//             &[Interface::Nft as u8],
//             self.uuid
//         ];
//         invoke_signed(
//             &system_instruction::create_account(self.payer.key, self.id.key, lamports, size as u64, &crate::id()),
//             &[self.id.clone(), self.system.clone(), self.payer.clone()],
//             &[seeds.as_slice()],
//         )?;
//         let mut data = self.id.try_borrow_mut_data().map_err(|_| {
//             DigitalAssetProtocolError::ActionError("Issue with Borrowing Data".to_string())
//         })?;
//         new_asset.save(data)?;
//         Ok(())
//     }
// }
