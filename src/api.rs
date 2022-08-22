use std::cell::RefMut;

use bebop::DeserializeError;
use solana_program::account_info::AccountInfo;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
    system_instruction,
};

use crate::validation::assert_key_equal;
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DigitalAssetProtocolError {
    #[error("Error in Module: {0}")]
    ModuleError(String),

    #[error("Error in Interface: {0}")]
    InterfaceError(String),

    #[error("Error in EntryPoint: {0}")]
    EntryPointError(String),

    #[error("Error in Action Parsing: {0}")]
    ActionError(String),

    #[error("Deserialization failed: {0}")]
    DeError(String),

    #[error("Interface has no implementation")]
    InterfaceNoImpl,
}

impl PrintProgramError for DigitalAssetProtocolError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<DigitalAssetProtocolError> for ProgramError {
    fn from(e: DigitalAssetProtocolError) -> Self {
        msg!(&e.to_string());
        ProgramError::Custom(0)
    }
}

impl From<ProgramError> for DigitalAssetProtocolError {
    fn from(e: ProgramError) -> Self {
        msg!(&e.to_string());
        DigitalAssetProtocolError::InterfaceError(e.to_string())
    }
}

impl<T> DecodeError<T> for DigitalAssetProtocolError {
    fn type_of() -> &'static str {
        "Dasset Error"
    }
}

impl From<DeserializeError> for DigitalAssetProtocolError {
    fn from(e: DeserializeError) -> Self {
        DigitalAssetProtocolError::DeError(e.to_string())
    }
}

pub fn derive(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, program_id)
}

pub struct AccountWrapper<'entry> {
    accounts: &'entry [AccountInfo<'entry>],
}

impl<'entry> AccountWrapper<'entry> {
    pub fn new(accounts: &'entry [AccountInfo<'entry>]) -> Result<Self, DigitalAssetProtocolError> {
        Ok(AccountWrapper { accounts })
    }

    pub fn system_program<'action>(
        &mut self,
        index: usize,
    ) -> Result<AccountInfoContext<'entry, 'action>, DigitalAssetProtocolError> {
        self.prepare_account(index, "system", Constraints::system_program())
    }

    pub fn prepare_account<'action>(
        &mut self,
        index: usize,
        name: &'static str,
        constraints: Constraints<'action>,
    ) -> Result<AccountInfoContext<'entry, 'action>, DigitalAssetProtocolError> {
        let mut accx = AccountInfoContext {
            name,
            info: &self.accounts[index],
            bump: None,
            constraints,
        };
        msg!("{} {}", index, accx.info.key);
        accx.validate_constraint()?;
        Ok(accx)
    }

    pub fn accounts_length(&self) -> usize {
        self.accounts.len()
    }
}

#[derive()]
pub struct Constraints<'action> {
    pub(crate) seeds: Option<&'action [&'action [u8]]>,
    pub(crate) program_id: Option<Pubkey>,
    pub(crate) key_equals: Option<Pubkey>,
    pub(crate) writable: bool,
    pub(crate) signer: bool,
    pub(crate) program: bool,
    pub(crate) empty: bool,
    pub(crate) owned_by: Option<Pubkey>,
}

impl<'action> Constraints<'action> {
    pub fn pda(
        seeds: &'action [&'action [u8]],
        program_id: Pubkey,
        write: bool,
        empty: bool,
    ) -> Self {
        Constraints {
            seeds: Some(seeds),
            program_id: Some(program_id),
            key_equals: None,
            writable: write,
            signer: false,
            program: false,
            empty,
            owned_by: None,
        }
    }

    pub fn system_program() -> Self {
        Constraints {
            seeds: None,
            program_id: None,
            key_equals: Some(solana_program::system_program::id()),
            writable: false,
            signer: false,
            program: true,
            empty: false,
            owned_by: None,
        }
    }

    pub fn read_only() -> Self {
        Constraints {
            seeds: None,
            program_id: None,
            key_equals: None,
            writable: false,
            signer: false,
            program: false,
            empty: false,
            owned_by: None,
        }
    }

    pub fn payer() -> Self {
        Constraints {
            seeds: None,
            program_id: None,
            key_equals: None,
            writable: true,
            signer: true,
            program: false,
            empty: false,
            owned_by: None,
        }
    }
}

pub struct AccountInfoContext<'entry, 'action> {
    pub name: &'static str,
    pub info: &'entry AccountInfo<'entry>,
    pub bump: Option<u8>,
    pub constraints: Constraints<'action>,
}

impl<'entry, 'action> AccountInfoContext<'entry, 'action> {
    pub fn mut_data(&mut self) -> RefMut<'entry, &'entry mut [u8]> {
        self.info.data.borrow_mut()
    }

    pub fn initialize_account(
        &mut self,
        initial_size: u64,
        system: &AccountInfoContext<'entry, 'action>,
        payer: &AccountInfoContext<'entry, 'action>,
    ) -> Result<(), DigitalAssetProtocolError> {
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(initial_size as usize);
        //validate address get bump
        invoke_signed(
            &system_instruction::create_account(
                payer.info.key,
                self.info.key,
                lamports,
                initial_size,
                &crate::id(),
            ),
            &[self.info.clone(), system.info.clone(), payer.info.clone()],
            &[
                self.constraints.seeds.unwrap(),
                &[self.bump.unwrap().to_le_bytes().as_ref()],
            ], // TODO get bump
        )?;
        Ok(())
    }
}

pub trait AccountConstraints {
    fn validate_constraint(&mut self) -> Result<(), DigitalAssetProtocolError>;
}

impl<'entry, 'action> AccountConstraints for AccountInfoContext<'entry, 'action> {
    fn validate_constraint(&mut self) -> Result<(), DigitalAssetProtocolError> {
        if self.constraints.program && !self.info.executable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!(
                "Account with key {} needs to be a program",
                self.info.key
            )));
        }
        if !self.constraints.program && self.info.executable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!(
                "Account with key {} can't be a program",
                self.info.key
            )));
        }

        if self.constraints.writable && !self.info.is_writable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!(
                "Account with key {} needs to be writable",
                self.info.key
            )));
        }
        if !self.constraints.writable && self.info.is_writable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!(
                "Account with key {} can't be writable",
                self.info.key
            )));
        }

        // May need to change this to support optional signers
        if self.constraints.signer && !self.info.is_signer {
            return Err(DigitalAssetProtocolError::InterfaceError(format!(
                "Account with key {} needs to be a signer",
                self.info.key
            )));
        }
        if !self.constraints.signer && self.info.is_signer {
            return Err(DigitalAssetProtocolError::InterfaceError(format!(
                "Account with key {} can't be a signer",
                self.info.key
            )));
        }

        if let Some(ob) = self.constraints.owned_by {
            assert_key_equal(&ob, self.info.owner)?;
        }

        if self.constraints.empty && self.info.data_len() > 0 && self.info.lamports() > 0 {
            return Err(DigitalAssetProtocolError::InterfaceError(format!(
                "Account with key {} can't be a signer",
                self.info.key
            )));
        }

        if let Some(kef) = self.constraints.key_equals {
            assert_key_equal(&kef, self.info.key)?;
        }
        match (self.constraints.seeds, self.constraints.program_id) {
            (Some(seeds), Some(prg)) => {
                let (pubkey, bump) = derive(seeds, &prg);
                assert_key_equal(&pubkey, self.info.key)?;
                self.bump = Some(bump);
                Ok(())
            }
            (None, None) => Ok(()),
            _ => Err(DigitalAssetProtocolError::InterfaceError(format!(
                "Account with key {} has incorrect seeds",
                self.info.key
            ))),
        }?;
        Ok(())
    }
}
