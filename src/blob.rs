use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;
use crate::api::DigitalAssetProtocolError;
use bebop::{Record, SliceWrapper, SubRecord};
use solana_program::program_memory::sol_memset;
use solana_program::pubkey::Pubkey;
use crate::generated::schema::{Blob, BlobContainer, DataItem, ModuleType};
use crate::generated::schema::ModuleData;

use crate::module::{ModuleDataWrapper, SchemaId};
use crate::required_field;

pub struct Asset<'raw> {
    pub dirty: bool,
    pub layout: BTreeMap<ModuleType, ModuleDataWrapper<'raw>>,
}

impl<'raw> Asset<'raw> {
    pub fn new() -> Asset<'raw> {
        Asset {
            layout: BTreeMap::new(),
            dirty: true
        }
    }

    pub fn save(mut self, destination: RefMut<&mut [u8]>) -> Result<(), DigitalAssetProtocolError> {
        // Clear the data, this is the naive approach, we can optimize this with specific/tracked module offsets
        let mut dest = *destination;
        let mut offset = 0;
        sol_memset(*destination, 0, destination.len());
        let mut blobs = Vec::with_capacity(self.layout.len());
        for (id, data) in self.layout {
            let blob = match data {
                ModuleDataWrapper::Structured(md) => {
                    Blob {
                        module_id: Some(id as u8),
                        structured_module: Some(md),
                        data_module: None,
                    }
                }
                ModuleDataWrapper::Unstructured(unstructred_data) => {
                    let mut data_module = Vec::with_capacity(unstructred_data.len());
                    for (key, val) in unstructred_data {
                        data_module.push(DataItem {
                            key: &*key,
                            value: val,
                        })
                    }

                    Blob {
                        module_id: Some(id as u8),
                        structured_module: None,
                        data_module: Some(data_module),
                    }
                }
            };
            blobs.push(blob);
        }
        let container = BlobContainer {
            blobs
        };
        container.serialize(&mut dest)
            .map_err(|e| {
                DigitalAssetProtocolError::DeError(e.to_string())
            })?;
        Ok(())
    }


    pub fn load_mut(source: RefMut<&mut [u8]>) -> Result<Asset<'raw>, DigitalAssetProtocolError> {
        load(source)
    }

    pub fn load(source: Ref<&[u8]>) -> Result<Asset<'raw>, DigitalAssetProtocolError> {
        let mut layout = BTreeMap::new();
        for blob  in container.blobs {
            let module_id = blob.module_id;
            required_field!(module_id)?;
            let module_id = ModuleType::try_from(module_id.unwrap())?;
            match (blob.data_module, blob.structured_module) {
                (Some(data_items), None) => {
                    let mut bespoke_data = BTreeMap::new();
                    for di in data_items {
                        bespoke_data.insert(di.key.to_string(), di.value);
                    }
                    layout.insert (module_id, ModuleDataWrapper::Unstructured(bespoke_data));
                }
                (None, Some(module_data)) => {
                    layout.insert(module_id, ModuleDataWrapper::Structured(module_data));
                }
                _ => {
                    return Err(DigitalAssetProtocolError::DeError("Invalid Blob".to_string()));
                }
            }
        }

        Ok(Asset {
            dirty: false,
            layout
        })
    }
}
