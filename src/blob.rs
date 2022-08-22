use std::cell::RefMut;
use std::collections::HashMap;

use crate::api::DigitalAssetProtocolError;
use bebop::{Record, SubRecord};
use solana_program::msg;

use solana_program::program_memory::sol_memset;

use crate::generated::schema::{BlobContainer, ModuleData, ModuleType};

pub struct Asset<'info> {
    pub dirty: bool,
    raw: Option<BlobContainer<'info>>,
}

impl<'info> Asset<'info> {
    pub fn new() -> Asset<'info> {
        Asset {
            raw: None,
            dirty: true,
        }
    }

    pub fn set_module(&mut self, id: ModuleType, data: ModuleData<'info>) {
        if !self.dirty {
            self.dirty = true;
        }
        if self.raw.is_none() {
            self.raw = Some(BlobContainer {
                blobs: HashMap::new(),
            })
        }
        self.raw
            .as_mut()
            .and_then(|f| f.blobs.insert(id as u8, data));
    }

    pub fn get_module(&mut self, id: ModuleType) -> Option<&mut ModuleData<'info>> {
        self.raw.as_mut().and_then(|f| f.blobs.get_mut(&(id as u8)))
    }

    pub fn size(&mut self) -> usize {
        self.raw
            .as_mut()
            .and_then(|f| Some(f.serialized_size()))
            .unwrap_or(0)
    }

    pub fn save(mut self, destination: RefMut<&mut [u8]>) -> Result<(), DigitalAssetProtocolError> {
        let len = destination.len();
        let mut destination = destination;
        let mut dest: &mut [u8] = &mut *destination;
        sol_memset(dest, 0, len);
        self.raw
            .unwrap()
            .serialize(&mut dest)
            .map_err(|e| DigitalAssetProtocolError::DeError(e.to_string()))?;
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
