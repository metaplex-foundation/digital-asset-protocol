use std::collections::BTreeMap;
use crate::api::DigitalAssetProtocolError;
use bebop::Record;
use crate::generated::schema::ModuleType;
use crate::generated::schema::ModuleData;

use crate::module::{ModuleId, SchemaId};

pub struct Asset<'raw> {
    pub layout: BTreeMap<ModuleId, Blob<'raw>>,
}

pub struct Blob<'raw> {
    pub schema: ModuleId,
    pub data: Vec<u8>,
    pub _runtime_data: Option<ModuleData<'raw>>,
}

impl<'raw> Blob<'raw> {
    pub fn new<T: 'static>(schema: ModuleId, data: Option<ModuleData<'raw>>) -> Self {
        Blob {
            schema,
            data: Vec::new(),
            _runtime_data: data,
        }
    }

    pub fn from_bytes(buf: &'raw [u8]) -> Result<Self, DigitalAssetProtocolError> {
        let m_type = ModuleType::try_from(buf[0])
            .map_err(|e| e.into())?;
        let schema =
            match m_type {
                ModuleType::Extension => {
                    let schema_id: SchemaId = buf[1..17].try_into()
                        .map_err(|_| {
                            DigitalAssetProtocolError::DeError("Invalid Schema ID".to_string())
                        })?;
                    Ok(ModuleId::Extension(schema_id))
                }
                m => Ok(ModuleId::Module(m))
            }?;
        let data = ModuleType::to_data(m_type, &buf[17..])?;
        Ok(Blob {
            schema,
            data: buf[17..].to_vec(),
            _runtime_data: data,
        })
    }

    // pub fn to_module(&self) -> Result<T, DigitalAssetProtocolError> {
    //     self._runtime_data.and_then(|a| {
    //         a.downcast_ref::<T>()
    //     })
    //         .ok_or(DigitalAssetProtocolError::ModuleError("No module Found".to_string()))
    // }
    //
    // pub fn to_data(&self) -> Result<T, DigitalAssetProtocolError> {
    //     Ok(())
    // }
}