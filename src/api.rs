use std::cell::{Ref, RefMut};
use std::collections::HashMap;
use std::ops::Deref;
use bebop::{DeserializeError, Record};
use solana_program::{decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError}, system_instruction};
use solana_program::account_info::AccountInfo;
use solana_program::hash::Hash;
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use thiserror::Error;
use crate::validation::{assert_key_equal, cmp_pubkeys};
use crate::generated::schema::{Action as IxAction, ActionData, Action};
use crate::interfaces::ContextAction;

// pub struct Action<'entry> {
//     pub standard: Interface,
//     pub program_id: Pubkey,
//     pub context: &'entry dyn ContextAction,
//     pub remaining_accounts: Vec<AccountInfo<'entry>>,
// }
//
// impl<'entry> Action<'entry> {
//     pub fn run(&mut self) -> Result<(), DigitalAssetProtocolError> {
//         self.context.run()
//     }
//
//
//
// }


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


pub struct Message<'entry> {
    accounts: &'entry [AccountInfo<'entry>],
    pub constrained_accounts: HashMap<&'static str, AccountInfoContext<'entry>>,
    data: RefMut<'entry, &'entry [u8]>,
    pub action: Action<'entry>,
}

impl<'entry> Message<'entry> {
    pub fn new(accounts: &'entry [AccountInfo<'entry>], data: RefMut<'entry, &'entry [u8]>) -> Result<Self, DigitalAssetProtocolError> {
        Ok(Message {
            data,
            accounts,
            constrained_accounts: HashMap::new(),
            action: Action::deserialize(&**data)?,
        })
    }

    pub fn system_program(&mut self, index: usize) -> Result<&'entry AccountInfoContext<'entry>, DigitalAssetProtocolError> {
        self.prepare_account(index, "system", Constraints::system_program()).map(|a| a.deref())
    }

    pub fn get_account(&'entry mut self, name: &str) -> Result<&'entry mut AccountInfoContext<'entry>, DigitalAssetProtocolError> {
        self.constrained_accounts.get_mut(name).ok_or(DigitalAssetProtocolError::InterfaceError(format!("Missing Account {}", name)))
    }

    pub fn prepare_account(&mut self, index: usize, name: &'static str, constraints: Constraints<'entry>) -> Result<&'entry mut AccountInfoContext<'entry>, DigitalAssetProtocolError> {
        let mut accx = AccountInfoContext {
            name,
            info: &self.accounts[index],
            seeds: constraints.seeds,
            bump: None,
            constraints,
        };
        accx.validate_constraint()?;
        self.constrained_accounts.insert(name, accx);
        Ok(&mut accx)
    }

    pub fn accounts_length(&self) -> usize {
        self.accounts.len()
    }
}


#[derive()]
pub struct Constraints<'entry> {
    pub(crate) seeds: Option<&'entry [&'entry [u8]]>,
    pub(crate) program_id: Option<Pubkey>,
    pub(crate) key_equals: Option<Pubkey>,
    pub(crate) writable: bool,
    pub(crate) signer: bool,
    pub(crate) program: bool,
    pub(crate) empty: bool,
    pub(crate) owned_by: Option<Pubkey>,
}

impl<'entry> Constraints<'entry> {
    pub fn pda(
        seeds: &'entry [&'entry [u8]],
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

pub struct AccountInfoContext<'entry> {
    pub name: &'static str,
    pub info: &'entry AccountInfo<'entry>,
    pub seeds: Option<&'entry [&'entry [u8]]>,
    pub bump: Option<u8>,
    pub constraints: Constraints<'entry>,
}

impl<'entry> AccountInfoContext<'entry> {
    pub fn mut_data(&mut self) -> &'entry mut RefMut<'entry, &'entry mut [u8]> {
        &mut self.info.data.borrow_mut()
    }

    pub fn initialize_account(&mut self,
                              initial_size: u64,
                              system: &'entry AccountInfoContext<'entry>,
                              payer: &'entry AccountInfoContext<'entry>,
    ) -> Result<(), DigitalAssetProtocolError> {
        let rent = Rent::get()?;
        let lamports = rent.minimum_balance(initial_size as usize);
        //validate address get bump
        invoke_signed(
            &system_instruction::create_account(payer.info.key, self.info.key, lamports, initial_size, &crate::id()),
            &[self.info.clone(), system.info.clone(), payer.info.clone()],
            &[self.seeds.unwrap()], // TODO get bump
        )?;
        Ok(())
    }
}

pub trait AccountConstraints {
    fn validate_constraint(&mut self) -> Result<(), DigitalAssetProtocolError>;
}

impl<'entry> AccountConstraints for AccountInfoContext<'entry> {
    fn validate_constraint(&mut self) -> Result<(), DigitalAssetProtocolError> {
        if self.constraints.program && !self.info.executable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} needs to be a program", self.info.key)));
        }
        if !self.constraints.program && self.info.executable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} can't be a program", self.info.key)));
        }

        if self.constraints.writable && !self.info.is_writable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} needs to be writable", self.info.key)));
        }
        if !self.constraints.writable && self.info.is_writable {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} can't be writable", self.info.key)));
        }

        // May need to change this to support optional signers
        if self.constraints.signer && !self.info.is_signer {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} needs to be a signer", self.info.key)));
        }
        if !self.constraints.signer && self.info.is_signer {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} can't be a signer", self.info.key)));
        }

        if let Some(ob) = self.constraints.owned_by {
            assert_key_equal(&ob, self.info.owner)?;
        }

        if self.constraints.empty && self.info.data_len() > 0 && self.info.lamports() > 0 {
            return Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} can't be a signer", self.info.key)));
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
            _ => Err(DigitalAssetProtocolError::InterfaceError(format!("Account with key {} has incorrect seeds", self.info.key)))
        }?;
        Ok(())
    }
}
