use std::cell::{Ref, RefMut};
use bebop::{DeserializeError, Record};
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use crate::validation::cmp_pubkeys;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use thiserror::Error;
use crate::interfaces::create::Create;
use crate::generated::schema::{
    Action as IxAction,
    Interface,
    ActionData,
};
use crate::interfaces::ContextAction;
use crate::module::AccountMap;

pub fn derive(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, program_id)
}

pub struct Constraints<'info> {
    seeds: Option<&'info[&'info[u8]]>,
    program_id: Option<Pubkey>,
    key_equals: Option<Pubkey>,
    writable: bool,
    signer: bool,
    program: bool,
    empty: bool,
}

impl Constraints {

}


pub struct AccountInfoContext<'info> {
    pub info: AccountInfo<'info>,
    mut_data_ref: Option<RefMut<'info, &'info mut [u8]>>,
    data_ref: Option<Ref<'info, &'info mut [u8]>>,
    pub seeds: Option<&'info [&'info [u8]]>,
    pub bump: Option<u8>,
    pub constraints: Constraints<'info>
}

// pub type SeedsConstraintFn = dyn FnOnce(&[&[u8]], &Pubkey) -> Constraints;
//
// pub static PROGRAM_DATA_ACCOUNT: SeedsConstraintFn = |seeds, program_id| {
//     Constraints {
//         derived_key_equals: Some(derive(seeds, program_id)),
//         key_equals: None,
//         writable: false,
//         signer: false,
//         program: false,
//         empty: false,
//     }
// };
//
// pub static WRITABLE_PROGRAM_DATA_ACCOUNT: SeedsConstraintFn = |seeds, program_id| {
//     Constraints {
//         derived_key_equals: Some(derive(seeds, program_id)),
//         key_equals: None,
//         writable: true,
//         signer: false,
//         program: false,
//         empty: false,
//     }
// };
//
//
// pub static SIGNING_PROGRAM_DATA_ACCOUNT: SeedsConstraintFn = |seeds, program_id| {
//     Constraints {
//         derived_key_equals: Some(derive(seeds, program_id)),
//         key_equals: None,
//         writable: true,
//         signer: true,
//         program: false,
//         empty: false,
//     }
// };
//
// pub static SYSTEM_PROGRAM: Constraints = Constraints {
//     derived_key_equals: None,
//     key_equals: Some(|| {
//         solana_program::system_program::id()
//     }),
//     writable: false,
//     signer: false,
//     program: true,
//     empty: false,
// };

pub trait AccountConstraints {
    fn validate_constraint(mut self) -> Result<(), DigitalAssetProtocolError>;
}

impl<'info> ConstrainedAccount for AccountInfoContext{
    fn validate_constraint(mut self) -> Result<(), DigitalAssetProtocolError> {
        if let Some(kef) = self.constraints.key_equals {
            cmp_pubkeys(&kef, self.info.key)?;
        }
        match (self.constraints.seeds, self.constraints.program_id) {
            (Some(seeds), Some(prg)) => {
                let (pubkey, bump) = derive(seeds, &prg);
                cmp_pubkeys(&pubkey, self.info.key)?;
                self.bump = Some(bump);
            }
            _ => DigitalAssetProtocolError::InterfaceError(format!("Account with key {} has incorrect seeds", self.info.key))
        }?;

        Ok(())
    }
}


pub struct Action<'info> {
    pub standard: Interface,
    pub program_id: Pubkey,
    pub context: Box<dyn ContextAction + 'info>,
    pub remaining_accounts: Vec<AccountInfo<'info>>,
}

impl<'info> Action<'info> {
    pub fn run(&mut self) -> Result<(), DigitalAssetProtocolError> {
        self.context.run()
    }

    pub fn from_instruction(program_id: &Pubkey,
                            accounts: &[AccountInfo<'info>],
                            instruction_data: &[u8]) -> Result<Action<'info>, DigitalAssetProtocolError> {
        let action = IxAction::deserialize(instruction_data)
            .map_err(|res| {
                DigitalAssetProtocolError::DeError(res.to_string())
            })?;

        return match action.standard {
            Interface::IdentityAsset => {
                let lifecycle = action.data;
                let (context, pointer) = match lifecycle {
                    ActionData::CreateIdentity { uri } => {
                        Ok(Create::new(accounts, uri.to_string()))
                    }
                    _ => Err(DigitalAssetProtocolError::InterfaceNoImpl)
                }?;

                Ok(Action {
                    standard: action.standard,
                    program_id: program_id.clone(),
                    context: Box::new(context),
                    remaining_accounts: accounts[pointer..].to_vec(),
                })
            }
            _ => Err(DigitalAssetProtocolError::InterfaceNoImpl)
        };
    }
}


#[derive(Error, Debug)]
pub enum DigitalAssetProtocolError {
    #[error("Error in Module: {0}")]
    ModuleError(String),

    #[error("Error in EntryPoint: {0}")]
    EntryPointError(String),

    #[error("Error in Action Parsing: {0}")]
    ActionError(String),

    #[error("Deserialization failed: {0}")]
    DeError(String),

    #[error("Interface has no implementation")]
    InterfaceNoImpl,

    #[error("Interface Error: {0}")]
    InterfaceError(String),
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

impl Into<DigitalAssetProtocolError> for DeserializeError {
    fn into(self) -> DigitalAssetProtocolError {
        DigitalAssetProtocolError::DeError(self.to_string())
    }
}

impl<T> DecodeError<T> for DigitalAssetProtocolError {
    fn type_of() -> &'static str {
        "Dasset Error"
    }
}
