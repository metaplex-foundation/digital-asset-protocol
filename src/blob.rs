use std::cell::{Ref, RefMut};
use std::collections::{BTreeMap, HashMap};
use crate::api::DigitalAssetProtocolError;
use bebop::{Record, SeResult, SliceWrapper, SubRecord};
use solana_program::account_info::AccountInfo;
use solana_program::program_memory::sol_memset;
use solana_program::pubkey::Pubkey;
use crate::generated::schema::{Blob, BlobContainer, DataItem, ModuleType, DataItemValue, ModuleData};

use crate::module::{ModuleDataWrapper, SchemaId};
use crate::required_field;

pub struct Asset<'info> {
    pub dirty: bool,
    raw: Option<BlobContainer<'info>>,
}


impl<'info> Asset {
    pub fn new() -> Asset<'info> {
        Asset {
            raw: None,
            dirty: true,
        }
    }

    pub fn set_module(&mut self, id: ModuleType, data: ModuleData) {
        if !self.dirty {
            self.dirty = true;
        }
        self.raw.and_then(|mut f| f.blobs.insert(id as u8, data));
    }

    pub fn get_module(&mut self, id: ModuleType) -> Option<&mut ModuleData> {
        self.raw.and_then(|mut f| f.blobs.get_mut(&(id as u8)))
    }

    pub fn size(&mut self) -> usize {
        self.raw.and_then(|mut f| Some(f.serialized_size())).unwrap_or(0)
    }

    pub fn save(mut self, destination: RefMut<&mut [u8]>) -> Result<(), DigitalAssetProtocolError> {
        let len = destination.len();
        let mut dest = destination;
        sol_memset(*dest, 0, len);
        self.raw.unwrap().serialize(&mut *dest)
            .map_err(|e| {
                DigitalAssetProtocolError::DeError(e.to_string())
            })?;
        self.raw = None;
        Ok(())
    }

    pub fn load_mut(source: &'info mut [u8]) -> Result<Asset<'info>, DigitalAssetProtocolError> {
        let container = BlobContainer::deserialize(source)?;

        Ok(Asset {
            raw: Some(container),
            dirty: false,
        })
    }

    pub fn load(source: &'info [u8]) -> Result<Asset<'info>, DigitalAssetProtocolError> {
        let container = BlobContainer::deserialize(source)?;

        Ok(Asset {
            raw: Some(container),
            dirty: false,
        })
    }
}
