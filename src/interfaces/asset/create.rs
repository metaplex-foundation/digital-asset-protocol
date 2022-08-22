use std::collections::HashMap;

use bebop::SliceWrapper;
use solana_program::msg;

use crate::api::{AccountWrapper, Constraints, DigitalAssetProtocolError};

use crate::blob::Asset;
use crate::generated::schema::{
    ActionData, Creator, DataItem, DataItemValue, ModuleData, ModuleType,
};
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
            creator_shares, // in percentage,
            authorities,
            royalty,
            uuid,
            ..
        } = &data
        {
            let accounts_size = accounts.accounts_length();
            let rng = 4..accounts_size;
            let mut accounts = accounts;
            let uuid = *required_field!(uuid)?;

            let seeds = [b"ASSET".as_ref(), uuid[0..8].as_ref()];
            let system = accounts.system_program(0)?;
            let mut asset_account = accounts.prepare_account(
                1,
                "asset",
                Constraints::pda(seeds.as_slice(), crate::id(), true, true),
            )?;
            let owner = accounts.prepare_account(2, "owner", Constraints::read_only())?;
            let payer = accounts.prepare_account(3, "payer", Constraints::payer())?;
            let mut creator_accounts = Vec::with_capacity(rng.len());
            let mut creator_data = Vec::with_capacity(rng.len());
            if rng.len() > 0 {
                let creator_shares = required_field!(creator_shares)?;
                if creator_shares.len() != rng.len() {
                    return Err(DigitalAssetProtocolError::ActionError(
                        "Creators and Creator Shares must match".to_string(),
                    ));
                }
                let mut i = 0;
                for c in rng {
                    let creator = accounts.prepare_account(
                        c as usize,
                        "creator",
                        Constraints::read_only_optional_signer(),
                    )?;
                    creator_data.push(Creator {
                        address: SliceWrapper::Raw(creator.info.key.as_ref()),
                        share: creator_shares[i],
                        verified: creator.info.is_signer.clone(),
                    });
                    creator_accounts.push(creator);
                    i += 1;
                }
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
                    ModuleType::Creators => {
                        new_asset.set_module(
                            ModuleType::Creators,
                            ModuleData::CreatorsData {
                                creators: creator_data.to_owned(),
                            },
                        );
                    }
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
            drop(creator_data);
            for p in processors {
                p.create(&mut new_asset)?;
            }
            asset_account.initialize_account(new_asset.size() as u64, &payer)?;
            let buffer = asset_account.mut_data();
            new_asset.save(buffer)?;
            return Ok(());
        }
        Err(DigitalAssetProtocolError::ActionError(
            "Invalid Action format, action must be CreateAssetV1".to_string(),
        ))
    }
}
