use std::collections::BTreeSet;
use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use crate::modules::Module;
use crate::state::asset::Asset;
use crate::state::nft::NFTStandard;

#[derive(Clone, BorshDeserialize, BorshSerialize, FromPrimitive)]
pub enum Standard {
    Unknown,
    NFTv1,
    NFT,
    NFTPrintable,
    NFTGroup,
    FungibleAsset,
}

impl Standard {
    pub fn get_instance(&self) -> Option<Box<dyn Standardized>> {
        match self {
            Standard::NFT => {
                Some(Box::new(NFTStandard::new()))
            }
            _ => {
                None
            }
        }
    }
}

pub trait Standardized {
    fn standard(&self) -> Standard;
    fn modules(&self) -> &BTreeSet<Module>;
    fn valid_asset(&self, asset: &Asset) -> bool {
        let req_modules = self.modules();
        let asset_modules = asset.modules();
        if req_modules.len() > asset_modules.len() {
            return false;
        }
        for mods in req_modules {
            if asset_modules.get(mods).is_none() {
                return false;
            }
        }
        return true;
    }
}
