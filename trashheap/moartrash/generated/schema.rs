//
// This code was generated by a tool.
//
//
//   bebopc version:
//       2.4.2
//
//
//   bebopc source:
//       https://github.com/RainwayApp/bebop
//
//
// Changes to this file may cause incorrect behavior and will be lost if
// the code is regenerated.
//

#![allow(warnings)]

use bebop::FixedSized as _;
use core::convert::TryInto as _;
use std::io::Write as _;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Standard {
    Unknown = 0,
    NFTv1 = 1,
    Nft = 2,
    NFTPrintable = 3,
    NFTGroup = 4,
    FungibleAsset = 5,
}

impl ::core::convert::TryFrom<u32> for Standard {
    type Error = ::bebop::DeserializeError;

    fn try_from(value: u32) -> ::bebop::DeResult<Self> {
        match value {
            0 => Ok(Standard::Unknown),
            1 => Ok(Standard::NFTv1),
            2 => Ok(Standard::Nft),
            3 => Ok(Standard::NFTPrintable),
            4 => Ok(Standard::NFTGroup),
            5 => Ok(Standard::FungibleAsset),
            d => Err(::bebop::DeserializeError::InvalidEnumDiscriminator(
                d.into(),
            )),
        }
    }
}

impl ::core::convert::From<Standard> for u32 {
    fn from(value: Standard) -> Self {
        match value {
            Standard::Unknown => 0,
            Standard::NFTv1 => 1,
            Standard::Nft => 2,
            Standard::NFTPrintable => 3,
            Standard::NFTGroup => 4,
            Standard::FungibleAsset => 5,
        }
    }
}

impl ::bebop::SubRecord<'_> for Standard {
    const MIN_SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
    const EXACT_SERIALIZED_SIZE: Option<usize> = Some(::std::mem::size_of::<u32>());

    #[inline]
    fn serialized_size(&self) -> usize {
        ::std::mem::size_of::<u32>()
    }

    #[inline]
    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        u32::from(*self)._serialize_chained(dest)
    }

    #[inline]
    fn _deserialize_chained(raw: &[u8]) -> ::bebop::DeResult<(usize, Self)> {
        let (n, v) = u32::_deserialize_chained(raw)?;
        Ok((n, v.try_into()?))
    }
}

impl ::bebop::FixedSized for Standard {
    const SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
}

#[derive(Clone, Debug, PartialEq)]
pub enum Action<'raw> {
    /// An unknown type which is likely defined in a newer version of the schema.
    Unknown,

    /// Discriminator 1
    Create {
        standard: Standard,
        layout: ::std::collections::HashMap<u32, Module<'raw>>,
    },

    /// Discriminator 2
    Transfer {},

    /// Discriminator 3
    Destroy {},

    /// Discriminator 4
    Update {
        layout: ::std::collections::HashMap<u32, Module<'raw>>,
    },

    /// Discriminator 5
    Extend {},
}

impl<'raw> ::bebop::SubRecord<'raw> for Action<'raw> {
    const MIN_SERIALIZED_SIZE: usize = ::bebop::LEN_SIZE + 1;

    fn serialized_size(&self) -> usize {
        ::bebop::LEN_SIZE
            + 1
            + match self {
                Action::Unknown => 0,
                Self::Create {
                    standard: ref _standard,
                    layout: ref _layout,
                } => _standard.serialized_size() + _layout.serialized_size(),
                Self::Transfer {} => 0,
                Self::Destroy {} => 0,
                Self::Update {
                    layout: ref _layout,
                } => _layout.serialized_size(),
                Self::Extend {} => 0,
            }
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        let size = self.serialized_size();
        ::bebop::write_len(dest, size - ::bebop::LEN_SIZE - 1)?;
        match self {
            Action::Unknown => {
                return Err(::bebop::SerializeError::CannotSerializeUnknownUnion);
            }
            Self::Create {
                standard: ref _standard,
                layout: ref _layout,
            } => {
                1u8._serialize_chained(dest)?;
                _standard._serialize_chained(dest)?;
                _layout._serialize_chained(dest)?;
            }
            Self::Transfer {} => {
                2u8._serialize_chained(dest)?;
            }
            Self::Destroy {} => {
                3u8._serialize_chained(dest)?;
            }
            Self::Update {
                layout: ref _layout,
            } => {
                4u8._serialize_chained(dest)?;
                _layout._serialize_chained(dest)?;
            }
            Self::Extend {} => {
                5u8._serialize_chained(dest)?;
            }
        }
        Ok(size)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let len = ::bebop::read_len(&raw)? + ::bebop::LEN_SIZE + 1;
        let mut i = ::bebop::LEN_SIZE + 1;
        let de = match raw[::bebop::LEN_SIZE] {
            1 => {
                let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;
                let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;

                Action::Create {
                    standard: v0,
                    layout: v1,
                }
            }
            2 => Action::Transfer {},
            3 => Action::Destroy {},
            4 => {
                let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;

                Action::Update { layout: v0 }
            }
            5 => Action::Extend {},
            _ => {
                i = len;
                Action::Unknown
            }
        };
        if !cfg!(feature = "unchecked") && i != len {
            debug_assert!(i > len);
            Err(::bebop::DeserializeError::CorruptFrame)
        } else {
            Ok((i, de))
        }
    }
}

impl<'raw> ::bebop::Record<'raw> for Action<'raw> {}

#[derive(Clone, Debug, PartialEq)]
pub enum Module<'raw> {
    /// An unknown type which is likely defined in a newer version of the schema.
    Unknown,

    /// Discriminator 1
    Ownership {
        model: OwnershipModel,
        owner: ::bebop::SliceWrapper<'raw, u8>,
    },

    /// Discriminator 2
    Royalty {
        royalty_percent: u8,
        model: RoyaltyModel,
        target: ::std::vec::Vec<RoyaltyTarget<'raw>>,
        locked: bool,
    },

    /// Discriminator 3
    Creators {
        creator_list: ::std::vec::Vec<Creator<'raw>>,
    },
}

impl<'raw> ::bebop::SubRecord<'raw> for Module<'raw> {
    const MIN_SERIALIZED_SIZE: usize = ::bebop::LEN_SIZE + 1;

    fn serialized_size(&self) -> usize {
        ::bebop::LEN_SIZE
            + 1
            + match self {
                Module::Unknown => 0,
                Self::Ownership {
                    model: ref _model,
                    owner: ref _owner,
                } => _model.serialized_size() + _owner.serialized_size(),
                Self::Royalty {
                    royalty_percent: ref _royalty_percent,
                    model: ref _model,
                    target: ref _target,
                    locked: ref _locked,
                } => {
                    _royalty_percent.serialized_size()
                        + _model.serialized_size()
                        + _target.serialized_size()
                        + _locked.serialized_size()
                }
                Self::Creators {
                    creator_list: ref _creator_list,
                } => _creator_list.serialized_size(),
            }
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        let size = self.serialized_size();
        ::bebop::write_len(dest, size - ::bebop::LEN_SIZE - 1)?;
        match self {
            Module::Unknown => {
                return Err(::bebop::SerializeError::CannotSerializeUnknownUnion);
            }
            Self::Ownership {
                model: ref _model,
                owner: ref _owner,
            } => {
                1u8._serialize_chained(dest)?;
                _model._serialize_chained(dest)?;
                _owner._serialize_chained(dest)?;
            }
            Self::Royalty {
                royalty_percent: ref _royalty_percent,
                model: ref _model,
                target: ref _target,
                locked: ref _locked,
            } => {
                2u8._serialize_chained(dest)?;
                _royalty_percent._serialize_chained(dest)?;
                _model._serialize_chained(dest)?;
                _target._serialize_chained(dest)?;
                _locked._serialize_chained(dest)?;
            }
            Self::Creators {
                creator_list: ref _creator_list,
            } => {
                3u8._serialize_chained(dest)?;
                _creator_list._serialize_chained(dest)?;
            }
        }
        Ok(size)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let len = ::bebop::read_len(&raw)? + ::bebop::LEN_SIZE + 1;
        let mut i = ::bebop::LEN_SIZE + 1;
        let de = match raw[::bebop::LEN_SIZE] {
            1 => {
                let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;
                let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;

                Module::Ownership {
                    model: v0,
                    owner: v1,
                }
            }
            2 => {
                let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;
                let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;
                let (read, v2) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;
                let (read, v3) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;

                Module::Royalty {
                    royalty_percent: v0,
                    model: v1,
                    target: v2,
                    locked: v3,
                }
            }
            3 => {
                let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                i += read;

                Module::Creators { creator_list: v0 }
            }
            _ => {
                i = len;
                Module::Unknown
            }
        };
        if !cfg!(feature = "unchecked") && i != len {
            debug_assert!(i > len);
            Err(::bebop::DeserializeError::CorruptFrame)
        } else {
            Ok((i, de))
        }
    }
}

impl<'raw> ::bebop::Record<'raw> for Module<'raw> {}

pub const MAX_MODULES: i32 = 10;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ModuleType {
    Invalid = 0,
    Ownership = 1,
    Royalty = 2,
    Creators = 3,
}

impl ::core::convert::TryFrom<u32> for ModuleType {
    type Error = ::bebop::DeserializeError;

    fn try_from(value: u32) -> ::bebop::DeResult<Self> {
        match value {
            0 => Ok(ModuleType::Invalid),
            1 => Ok(ModuleType::Ownership),
            2 => Ok(ModuleType::Royalty),
            3 => Ok(ModuleType::Creators),
            d => Err(::bebop::DeserializeError::InvalidEnumDiscriminator(
                d.into(),
            )),
        }
    }
}

impl ::core::convert::From<ModuleType> for u32 {
    fn from(value: ModuleType) -> Self {
        match value {
            ModuleType::Invalid => 0,
            ModuleType::Ownership => 1,
            ModuleType::Royalty => 2,
            ModuleType::Creators => 3,
        }
    }
}

impl ::bebop::SubRecord<'_> for ModuleType {
    const MIN_SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
    const EXACT_SERIALIZED_SIZE: Option<usize> = Some(::std::mem::size_of::<u32>());

    #[inline]
    fn serialized_size(&self) -> usize {
        ::std::mem::size_of::<u32>()
    }

    #[inline]
    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        u32::from(*self)._serialize_chained(dest)
    }

    #[inline]
    fn _deserialize_chained(raw: &[u8]) -> ::bebop::DeResult<(usize, Self)> {
        let (n, v) = u32::_deserialize_chained(raw)?;
        Ok((n, v.try_into()?))
    }
}

impl ::bebop::FixedSized for ModuleType {
    const SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OwnershipModel {
    Invalid = 0,
    Single = 1,
    Token = 2,
}

impl ::core::convert::TryFrom<u32> for OwnershipModel {
    type Error = ::bebop::DeserializeError;

    fn try_from(value: u32) -> ::bebop::DeResult<Self> {
        match value {
            0 => Ok(OwnershipModel::Invalid),
            1 => Ok(OwnershipModel::Single),
            2 => Ok(OwnershipModel::Token),
            d => Err(::bebop::DeserializeError::InvalidEnumDiscriminator(
                d.into(),
            )),
        }
    }
}

impl ::core::convert::From<OwnershipModel> for u32 {
    fn from(value: OwnershipModel) -> Self {
        match value {
            OwnershipModel::Invalid => 0,
            OwnershipModel::Single => 1,
            OwnershipModel::Token => 2,
        }
    }
}

impl ::bebop::SubRecord<'_> for OwnershipModel {
    const MIN_SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
    const EXACT_SERIALIZED_SIZE: Option<usize> = Some(::std::mem::size_of::<u32>());

    #[inline]
    fn serialized_size(&self) -> usize {
        ::std::mem::size_of::<u32>()
    }

    #[inline]
    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        u32::from(*self)._serialize_chained(dest)
    }

    #[inline]
    fn _deserialize_chained(raw: &[u8]) -> ::bebop::DeResult<(usize, Self)> {
        let (n, v) = u32::_deserialize_chained(raw)?;
        Ok((n, v.try_into()?))
    }
}

impl ::bebop::FixedSized for OwnershipModel {
    const SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RoyaltyModel {
    Invalid = 0,
    Address = 1,
    Fanout = 2,
    Creators = 3,
}

impl ::core::convert::TryFrom<u32> for RoyaltyModel {
    type Error = ::bebop::DeserializeError;

    fn try_from(value: u32) -> ::bebop::DeResult<Self> {
        match value {
            0 => Ok(RoyaltyModel::Invalid),
            1 => Ok(RoyaltyModel::Address),
            2 => Ok(RoyaltyModel::Fanout),
            3 => Ok(RoyaltyModel::Creators),
            d => Err(::bebop::DeserializeError::InvalidEnumDiscriminator(
                d.into(),
            )),
        }
    }
}

impl ::core::convert::From<RoyaltyModel> for u32 {
    fn from(value: RoyaltyModel) -> Self {
        match value {
            RoyaltyModel::Invalid => 0,
            RoyaltyModel::Address => 1,
            RoyaltyModel::Fanout => 2,
            RoyaltyModel::Creators => 3,
        }
    }
}

impl ::bebop::SubRecord<'_> for RoyaltyModel {
    const MIN_SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
    const EXACT_SERIALIZED_SIZE: Option<usize> = Some(::std::mem::size_of::<u32>());

    #[inline]
    fn serialized_size(&self) -> usize {
        ::std::mem::size_of::<u32>()
    }

    #[inline]
    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        u32::from(*self)._serialize_chained(dest)
    }

    #[inline]
    fn _deserialize_chained(raw: &[u8]) -> ::bebop::DeResult<(usize, Self)> {
        let (n, v) = u32::_deserialize_chained(raw)?;
        Ok((n, v.try_into()?))
    }
}

impl ::bebop::FixedSized for RoyaltyModel {
    const SERIALIZED_SIZE: usize = ::std::mem::size_of::<u32>();
}

#[derive(Clone, Debug, PartialEq)]
pub struct RoyaltyTarget<'raw> {
    pub address: ::bebop::SliceWrapper<'raw, u8>,
    pub share: u8,
}

impl<'raw> ::bebop::SubRecord<'raw> for RoyaltyTarget<'raw> {
    const MIN_SERIALIZED_SIZE: usize =
        <::bebop::SliceWrapper<'raw, u8>>::MIN_SERIALIZED_SIZE + <u8>::MIN_SERIALIZED_SIZE;

    #[inline]
    fn serialized_size(&self) -> usize {
        self.address.serialized_size() + self.share.serialized_size()
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        Ok(self.address._serialize_chained(dest)? + self.share._serialize_chained(dest)?)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
            let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
            return Err(::bebop::DeserializeError::MoreDataExpected(missing));
        }

        let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;

        Ok((
            i,
            Self {
                address: v0,
                share: v1,
            },
        ))
    }
}

impl<'raw> ::bebop::Record<'raw> for RoyaltyTarget<'raw> {}

#[derive(Clone, Debug, PartialEq)]
pub struct Creator<'raw> {
    pub address: ::bebop::SliceWrapper<'raw, u8>,
    pub share: u8,
    pub verified: bool,
}

impl<'raw> ::bebop::SubRecord<'raw> for Creator<'raw> {
    const MIN_SERIALIZED_SIZE: usize = <::bebop::SliceWrapper<'raw, u8>>::MIN_SERIALIZED_SIZE
        + <u8>::MIN_SERIALIZED_SIZE
        + <bool>::MIN_SERIALIZED_SIZE;

    #[inline]
    fn serialized_size(&self) -> usize {
        self.address.serialized_size()
            + self.share.serialized_size()
            + self.verified.serialized_size()
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        Ok(self.address._serialize_chained(dest)?
            + self.share._serialize_chained(dest)?
            + self.verified._serialize_chained(dest)?)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
            let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
            return Err(::bebop::DeserializeError::MoreDataExpected(missing));
        }

        let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;
        let (read, v2) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;

        Ok((
            i,
            Self {
                address: v0,
                share: v1,
                verified: v2,
            },
        ))
    }
}

impl<'raw> ::bebop::Record<'raw> for Creator<'raw> {}

#[derive(Clone, Debug, PartialEq)]
pub struct Asset<'raw> {
    pub layout: ::std::collections::HashMap<u32, Module<'raw>>,
}

impl<'raw> ::bebop::SubRecord<'raw> for Asset<'raw> {
    const MIN_SERIALIZED_SIZE: usize =
        <::std::collections::HashMap<u32, Module<'raw>>>::MIN_SERIALIZED_SIZE;

    #[inline]
    fn serialized_size(&self) -> usize {
        self.layout.serialized_size()
    }

    fn _serialize_chained<W: ::std::io::Write>(&self, dest: &mut W) -> ::bebop::SeResult<usize> {
        Ok(self.layout._serialize_chained(dest)?)
    }

    fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
        let mut i = 0;
        if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
            let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
            return Err(::bebop::DeserializeError::MoreDataExpected(missing));
        }

        let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
        i += read;

        Ok((i, Self { layout: v0 }))
    }
}

impl<'raw> ::bebop::Record<'raw> for Asset<'raw> {}

#[cfg(feature = "bebop-owned-all")]
pub mod owned {
    #![allow(warnings)]

    use bebop::FixedSized as _;
    use core::convert::TryInto as _;
    use std::io::Write as _;

    pub use super::Standard;

    #[derive(Clone, Debug, PartialEq)]
    pub enum Action {
        /// An unknown type which is likely defined in a newer version of the schema.
        Unknown,

        /// Discriminator 1
        Create {
            standard: Standard,
            layout: ::std::collections::HashMap<u32, Module>,
        },

        /// Discriminator 2
        Transfer {},

        /// Discriminator 3
        Destroy {},

        /// Discriminator 4
        Update {
            layout: ::std::collections::HashMap<u32, Module>,
        },

        /// Discriminator 5
        Extend {},
    }

    impl<'raw> ::core::convert::From<super::Action<'raw>> for Action {
        fn from(value: super::Action) -> Self {
            match value {
                super::Action::Unknown => Self::Unknown,
                super::Action::Create {
                    standard: _standard,
                    layout: _layout,
                } => Self::Create {
                    standard: _standard,
                    layout: _layout
                        .into_iter()
                        .map(|(key, value)| (key, value.into()))
                        .collect(),
                },
                super::Action::Transfer {} => Self::Transfer {},
                super::Action::Destroy {} => Self::Destroy {},
                super::Action::Update { layout: _layout } => Self::Update {
                    layout: _layout
                        .into_iter()
                        .map(|(key, value)| (key, value.into()))
                        .collect(),
                },
                super::Action::Extend {} => Self::Extend {},
            }
        }
    }
    impl<'raw> ::bebop::SubRecord<'raw> for Action {
        const MIN_SERIALIZED_SIZE: usize = ::bebop::LEN_SIZE + 1;

        fn serialized_size(&self) -> usize {
            ::bebop::LEN_SIZE
                + 1
                + match self {
                    Action::Unknown => 0,
                    Self::Create {
                        standard: ref _standard,
                        layout: ref _layout,
                    } => _standard.serialized_size() + _layout.serialized_size(),
                    Self::Transfer {} => 0,
                    Self::Destroy {} => 0,
                    Self::Update {
                        layout: ref _layout,
                    } => _layout.serialized_size(),
                    Self::Extend {} => 0,
                }
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            let size = self.serialized_size();
            ::bebop::write_len(dest, size - ::bebop::LEN_SIZE - 1)?;
            match self {
                Action::Unknown => {
                    return Err(::bebop::SerializeError::CannotSerializeUnknownUnion);
                }
                Self::Create {
                    standard: ref _standard,
                    layout: ref _layout,
                } => {
                    1u8._serialize_chained(dest)?;
                    _standard._serialize_chained(dest)?;
                    _layout._serialize_chained(dest)?;
                }
                Self::Transfer {} => {
                    2u8._serialize_chained(dest)?;
                }
                Self::Destroy {} => {
                    3u8._serialize_chained(dest)?;
                }
                Self::Update {
                    layout: ref _layout,
                } => {
                    4u8._serialize_chained(dest)?;
                    _layout._serialize_chained(dest)?;
                }
                Self::Extend {} => {
                    5u8._serialize_chained(dest)?;
                }
            }
            Ok(size)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let len = ::bebop::read_len(&raw)? + ::bebop::LEN_SIZE + 1;
            let mut i = ::bebop::LEN_SIZE + 1;
            let de = match raw[::bebop::LEN_SIZE] {
                1 => {
                    let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;
                    let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;

                    Action::Create {
                        standard: v0,
                        layout: v1,
                    }
                }
                2 => Action::Transfer {},
                3 => Action::Destroy {},
                4 => {
                    let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;

                    Action::Update { layout: v0 }
                }
                5 => Action::Extend {},
                _ => {
                    i = len;
                    Action::Unknown
                }
            };
            if !cfg!(feature = "unchecked") && i != len {
                debug_assert!(i > len);
                Err(::bebop::DeserializeError::CorruptFrame)
            } else {
                Ok((i, de))
            }
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Action {}

    #[derive(Clone, Debug, PartialEq)]
    pub enum Module {
        /// An unknown type which is likely defined in a newer version of the schema.
        Unknown,

        /// Discriminator 1
        Ownership {
            model: OwnershipModel,
            owner: ::std::vec::Vec<u8>,
        },

        /// Discriminator 2
        Royalty {
            royalty_percent: u8,
            model: RoyaltyModel,
            target: ::std::vec::Vec<RoyaltyTarget>,
            locked: bool,
        },

        /// Discriminator 3
        Creators {
            creator_list: ::std::vec::Vec<Creator>,
        },
    }

    impl<'raw> ::core::convert::From<super::Module<'raw>> for Module {
        fn from(value: super::Module) -> Self {
            match value {
                super::Module::Unknown => Self::Unknown,
                super::Module::Ownership {
                    model: _model,
                    owner: _owner,
                } => Self::Ownership {
                    model: _model,
                    owner: _owner.iter().map(|value| value).collect(),
                },
                super::Module::Royalty {
                    royalty_percent: _royalty_percent,
                    model: _model,
                    target: _target,
                    locked: _locked,
                } => Self::Royalty {
                    royalty_percent: _royalty_percent,
                    model: _model,
                    target: _target.into_iter().map(|value| value.into()).collect(),
                    locked: _locked,
                },
                super::Module::Creators {
                    creator_list: _creator_list,
                } => Self::Creators {
                    creator_list: _creator_list
                        .into_iter()
                        .map(|value| value.into())
                        .collect(),
                },
            }
        }
    }
    impl<'raw> ::bebop::SubRecord<'raw> for Module {
        const MIN_SERIALIZED_SIZE: usize = ::bebop::LEN_SIZE + 1;

        fn serialized_size(&self) -> usize {
            ::bebop::LEN_SIZE
                + 1
                + match self {
                    Module::Unknown => 0,
                    Self::Ownership {
                        model: ref _model,
                        owner: ref _owner,
                    } => _model.serialized_size() + _owner.serialized_size(),
                    Self::Royalty {
                        royalty_percent: ref _royalty_percent,
                        model: ref _model,
                        target: ref _target,
                        locked: ref _locked,
                    } => {
                        _royalty_percent.serialized_size()
                            + _model.serialized_size()
                            + _target.serialized_size()
                            + _locked.serialized_size()
                    }
                    Self::Creators {
                        creator_list: ref _creator_list,
                    } => _creator_list.serialized_size(),
                }
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            let size = self.serialized_size();
            ::bebop::write_len(dest, size - ::bebop::LEN_SIZE - 1)?;
            match self {
                Module::Unknown => {
                    return Err(::bebop::SerializeError::CannotSerializeUnknownUnion);
                }
                Self::Ownership {
                    model: ref _model,
                    owner: ref _owner,
                } => {
                    1u8._serialize_chained(dest)?;
                    _model._serialize_chained(dest)?;
                    _owner._serialize_chained(dest)?;
                }
                Self::Royalty {
                    royalty_percent: ref _royalty_percent,
                    model: ref _model,
                    target: ref _target,
                    locked: ref _locked,
                } => {
                    2u8._serialize_chained(dest)?;
                    _royalty_percent._serialize_chained(dest)?;
                    _model._serialize_chained(dest)?;
                    _target._serialize_chained(dest)?;
                    _locked._serialize_chained(dest)?;
                }
                Self::Creators {
                    creator_list: ref _creator_list,
                } => {
                    3u8._serialize_chained(dest)?;
                    _creator_list._serialize_chained(dest)?;
                }
            }
            Ok(size)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let len = ::bebop::read_len(&raw)? + ::bebop::LEN_SIZE + 1;
            let mut i = ::bebop::LEN_SIZE + 1;
            let de = match raw[::bebop::LEN_SIZE] {
                1 => {
                    let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;
                    let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;

                    Module::Ownership {
                        model: v0,
                        owner: v1,
                    }
                }
                2 => {
                    let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;
                    let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;
                    let (read, v2) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;
                    let (read, v3) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;

                    Module::Royalty {
                        royalty_percent: v0,
                        model: v1,
                        target: v2,
                        locked: v3,
                    }
                }
                3 => {
                    let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
                    i += read;

                    Module::Creators { creator_list: v0 }
                }
                _ => {
                    i = len;
                    Module::Unknown
                }
            };
            if !cfg!(feature = "unchecked") && i != len {
                debug_assert!(i > len);
                Err(::bebop::DeserializeError::CorruptFrame)
            } else {
                Ok((i, de))
            }
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Module {}

    pub use super::MAX_MODULES;

    pub use super::ModuleType;

    pub use super::OwnershipModel;

    pub use super::RoyaltyModel;

    #[derive(Clone, Debug, PartialEq)]
    pub struct RoyaltyTarget {
        pub address: ::std::vec::Vec<u8>,
        pub share: u8,
    }

    impl<'raw> ::core::convert::From<super::RoyaltyTarget<'raw>> for RoyaltyTarget {
        fn from(value: super::RoyaltyTarget) -> Self {
            Self {
                address: value.address.iter().map(|value| value).collect(),
                share: value.share,
            }
        }
    }

    impl<'raw> ::bebop::SubRecord<'raw> for RoyaltyTarget {
        const MIN_SERIALIZED_SIZE: usize =
            <::std::vec::Vec<u8>>::MIN_SERIALIZED_SIZE + <u8>::MIN_SERIALIZED_SIZE;

        #[inline]
        fn serialized_size(&self) -> usize {
            self.address.serialized_size() + self.share.serialized_size()
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            Ok(self.address._serialize_chained(dest)? + self.share._serialize_chained(dest)?)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let mut i = 0;
            if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
                let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
                return Err(::bebop::DeserializeError::MoreDataExpected(missing));
            }

            let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;

            Ok((
                i,
                Self {
                    address: v0,
                    share: v1,
                },
            ))
        }
    }

    impl<'raw> ::bebop::Record<'raw> for RoyaltyTarget {}

    #[derive(Clone, Debug, PartialEq)]
    pub struct Creator {
        pub address: ::std::vec::Vec<u8>,
        pub share: u8,
        pub verified: bool,
    }

    impl<'raw> ::core::convert::From<super::Creator<'raw>> for Creator {
        fn from(value: super::Creator) -> Self {
            Self {
                address: value.address.iter().map(|value| value).collect(),
                share: value.share,
                verified: value.verified,
            }
        }
    }

    impl<'raw> ::bebop::SubRecord<'raw> for Creator {
        const MIN_SERIALIZED_SIZE: usize = <::std::vec::Vec<u8>>::MIN_SERIALIZED_SIZE
            + <u8>::MIN_SERIALIZED_SIZE
            + <bool>::MIN_SERIALIZED_SIZE;

        #[inline]
        fn serialized_size(&self) -> usize {
            self.address.serialized_size()
                + self.share.serialized_size()
                + self.verified.serialized_size()
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            Ok(self.address._serialize_chained(dest)?
                + self.share._serialize_chained(dest)?
                + self.verified._serialize_chained(dest)?)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let mut i = 0;
            if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
                let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
                return Err(::bebop::DeserializeError::MoreDataExpected(missing));
            }

            let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v1) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;
            let (read, v2) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;

            Ok((
                i,
                Self {
                    address: v0,
                    share: v1,
                    verified: v2,
                },
            ))
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Creator {}

    #[derive(Clone, Debug, PartialEq)]
    pub struct Asset {
        pub layout: ::std::collections::HashMap<u32, Module>,
    }

    impl<'raw> ::core::convert::From<super::Asset<'raw>> for Asset {
        fn from(value: super::Asset) -> Self {
            Self {
                layout: value
                    .layout
                    .into_iter()
                    .map(|(key, value)| (key, value.into()))
                    .collect(),
            }
        }
    }

    impl<'raw> ::bebop::SubRecord<'raw> for Asset {
        const MIN_SERIALIZED_SIZE: usize =
            <::std::collections::HashMap<u32, Module>>::MIN_SERIALIZED_SIZE;

        #[inline]
        fn serialized_size(&self) -> usize {
            self.layout.serialized_size()
        }

        fn _serialize_chained<W: ::std::io::Write>(
            &self,
            dest: &mut W,
        ) -> ::bebop::SeResult<usize> {
            Ok(self.layout._serialize_chained(dest)?)
        }

        fn _deserialize_chained(raw: &'raw [u8]) -> ::bebop::DeResult<(usize, Self)> {
            let mut i = 0;
            if raw.len() - i < Self::MIN_SERIALIZED_SIZE {
                let missing = Self::MIN_SERIALIZED_SIZE - (raw.len() - i);
                return Err(::bebop::DeserializeError::MoreDataExpected(missing));
            }

            let (read, v0) = ::bebop::SubRecord::_deserialize_chained(&raw[i..])?;
            i += read;

            Ok((i, Self { layout: v0 }))
        }
    }

    impl<'raw> ::bebop::Record<'raw> for Asset {}
}
