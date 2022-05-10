use std::any::Any;
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::modules::Module;

pub trait Blobbed {
    fn id(&self) -> &BlobId;
    fn dirty(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum BlobId {
    Module(Module),
    Extension([u8; 16]),
}

pub struct Blob<T: BorshDeserialize + BorshSerialize> {
    id: BlobId,
    /// Raw Data, for ser and der
    raw_data: Vec<u8>,
    /// Tracking changes to avoid needless serialization
    dirty: bool,
    /// Runtime type of data
    data: T,
}

impl<T: BorshDeserialize + BorshSerialize> Blob<T> {
    pub fn get_mut_data(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: 'static +  BorshDeserialize + BorshSerialize> Blobbed for Blob<T> {
    fn id(self: &Blob<T>) -> &BlobId {
        &self.id
    }

    fn dirty(&self) -> bool {
        self.dirty
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    
}

pub trait BlobBorsh<'a> {
    fn blob<T: 'static>(blob: &dyn Blobbed) -> Option<Blob<T>> where T: BorshDeserialize + BorshSerialize{
        blob.as_any().downcast_ref::<Blob<T>>().map(|a| *a)
    }

    fn serialize<W: Write>(self, writer: &mut W) -> std::io::Result<()>;
    fn deserialize<T: BorshDeserialize + BorshSerialize>(buf: &mut [u8], id: BlobId) -> std::io::Result<Blob<T>> {
        let d: T = BorshDeserialize::try_from_slice(buf)?;
        Ok(Blob {
            id,
            raw_data: buf.to_vec(), // DO I need to keep this around?
            dirty: false,
            data: d,
        })
    }
}

impl<T: BorshDeserialize + BorshSerialize> BlobBorsh<'_> for Blob<T> {
    fn serialize<W: Write>(self, writer: &mut W) -> std::io::Result<()> {
        BorshSerialize::serialize(&self.data, writer)
    }
}

