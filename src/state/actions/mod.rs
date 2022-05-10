use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::{ProgramError};
use crate::validation::{assert_empty, assert_self_derivation};
use thiserror::Error;
use crate::state::asset::{Asset, AssetError};
use crate::state::Standard;
use std::any::{Any, TypeId};
use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use crate::api::DigitalAssetProtocolError;

const ASSET_PREFIX: &str = "💾dasset";

pub fn asset_seeds(uuid: &[u8]) -> Vec<&[u8]> {
    vec![ASSET_PREFIX.as_bytes(), uuid]
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum Lifecycle {
    Transfer,
    ListForSale,
    CancelListing,
    Update,
    Sale,
    Create,
    Delete,
    SupplyChange,
    Freeze,
    Burn,
}

impl Display for Lifecycle {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}


#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Instruction Error: Invalid {0}")]
    InstructionError(String),
    #[error("Invalid Lifecycle: {0} does not exist")]
    InvalidLifeCycle(String),
    #[error("Invalid Payer")]
    InvalidPayer,
    #[error("Invalid Account. Account {0}:{1} Must be Empty")]
    AccountMustBeEmpty(String, Pubkey),
    #[error("Bump Doesnt Match: Account {0}:{1}")]
    BumpDoesntMatch(String, Pubkey),
    #[error("Derived Pubkey Doesnt Match: Account {0}:{1} ")]
    PubkeyDoesntMatch(String, Pubkey),
}

pub fn instruction_to_context<'a>(program_id: &'a Pubkey,
                                  accounts: &'a [AccountInfo<'a>],
                                  instruction_data: &'a [u8]) -> Result<Box<dyn Action<'a> + 'a>, DigitalAssetProtocolError> {
    let life = BorshDeserialize::try_from_slice(instruction_data);
    match life {
        Err(e) => {
            Err(action_error(ActionError::InvalidLifeCycle("Invalid".to_string())))
        }
        Ok(Lifecycle::Create) => {
            Ok(Box::new(CreateContext::from_instruction(
                program_id.clone(),
                accounts,
                &instruction_data[1..],
            )))
        }
        Ok(e) => {
            let re_str = e.to_string();
            Err(action_error(ActionError::InvalidLifeCycle(re_str)))
        }
    }
}


pub trait Action<'a> {
    fn lifecycle(&self) -> Lifecycle;
    fn valid(&self) -> Result<(), DigitalAssetProtocolError>;
    fn program_id(&self) -> &Pubkey;
    fn standard(&self) -> Standard;
    fn is_cpi(&self) -> bool {
        self.program_id() != &crate::id()
    }
    fn module_data(&self) -> &'a [u8];
    fn asset(&self) -> Result<Asset, AssetError> {
        Asset::try_deserialize(&mut self.module_data())
    }
}

pub trait ActionRun {
    fn run_action<F>(&self, func: F) -> Result<Option<Lifecycle>, DigitalAssetProtocolError>
        where F: FnOnce(&Self) -> Result<Option<Lifecycle>, DigitalAssetProtocolError>;
}

pub struct CreateContext<'a> {
    program_id: Pubkey,
    pub asset_account: AccountInfo<'a>,
    pub payer: AccountInfo<'a>,
    pub standard: Standard,
    //1
    pub uuid: &'a [u8],
    // 16
    pub asset_account_bump: u8,
    pub modules: &'a [u8],
    //tail
    pub owner: AccountInfo<'a>,
}

pub fn action_error(err: ActionError) -> DigitalAssetProtocolError {
    DigitalAssetProtocolError::ActionError(Box::new(err))
}

impl ActionRun for CreateContext<'_> {
    fn run_action<F>(&self, func: F) -> Result<Option<Lifecycle>, DigitalAssetProtocolError>
        where F: FnOnce(&Self) -> Result<Option<Lifecycle>, DigitalAssetProtocolError> {
        func(self as &CreateContext)
    }
}

impl<'a> Action<'a> for CreateContext<'a> {
    fn lifecycle(&self) -> Lifecycle {
        Lifecycle::Create
    }
    // TODO -> Account labels
    fn valid(&self) -> Result<(), DigitalAssetProtocolError> {
        if !self.payer.is_signer {
            return Err(action_error(ActionError::InvalidPayer));
        }
        assert_empty(
            &self.asset_account,
            action_error(
                ActionError::AccountMustBeEmpty("Asset Account".to_string(),
                                                *self.asset_account.key)),
        )?;
        let bump = assert_self_derivation(
            &self.asset_account,
            &asset_seeds(self.uuid),
            action_error(ActionError::PubkeyDoesntMatch(
                "Asset Account".to_string(),
                *self.asset_account.key)),
        )?;
        if bump != self.asset_account_bump {
            return Err(action_error(ActionError::BumpDoesntMatch(
                "Asset Account".to_string(),
                *self.asset_account.key)));
        }
        Ok(())
    }
    fn program_id(&self) -> &Pubkey {
        &self.program_id
    }

    fn standard(&self) -> Standard {
        self.standard.to_owned()
    }

    fn module_data(&self) -> &'a [u8] {
        self.modules
    }
}

impl<'a> CreateContext<'a> {
    fn from_instruction(program_id: Pubkey,
                        accounts: &[AccountInfo<'a>],
                        instruction_data: &'a [u8]) -> Self {
        let standard: Option<Standard> = BorshDeserialize::try_from_slice(&[instruction_data[0]]).ok();
        CreateContext {
            program_id: program_id.clone(),
            asset_account: accounts[0].to_owned(),
            payer: accounts[1].to_owned(),
            standard: standard.or_else(|| Some(Standard::Unknown)).unwrap(),
            uuid: &instruction_data[1..16],
            asset_account_bump: instruction_data[17],
            modules: &instruction_data[18..],
            owner: accounts[2].to_owned(),
        }
    }
}