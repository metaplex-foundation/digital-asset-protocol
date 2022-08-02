use bebop::{DeserializeError, Record};
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use thiserror::Error;
use crate::interfaces::{asset};
use crate::generated::schema::{
    Action as IxAction,
    Interface,
    ActionData,
};
use crate::interfaces::ContextAction;

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

    fn match_context(action_data: ActionData) -> Result<(Box<dyn ContextAction>, usize), DigitalAssetProtocolError> {
        match action_data {
            ActionData::CreateAssetV1 {..} => {
                let d = asset::CreateV1::new(accounts, lifecycle)?;
                Ok((Box::new(d.0), d.1))
            }
            ActionData::UpdateAssetV1 {..} => {
                let d = asset::UpdateV1::new(accounts, lifecycle)?;
                Ok((Box::new(d.0), d.1))
            }
            _ => Err(DigitalAssetProtocolError::InterfaceNoImpl)
        }
    }

    pub fn from_instruction(program_id: &Pubkey,
                            accounts: &'info [AccountInfo<'info>],
                            instruction_data: &'info [u8]) -> Result<Action<'info>, DigitalAssetProtocolError> {
        let action = IxAction::deserialize(instruction_data)
            .map_err(|res| {
                DigitalAssetProtocolError::DeError(res.to_string())
            })?;

        return match action.standard {
            Interface::Nft => {
                let action_context = match_context(action.data)?;
                Ok(Action {
                    standard: action.standard,
                    program_id: program_id.clone(),
                    context: Box::new(action_data.0),
                    remaining_accounts: accounts[action_context.1..].to_vec(),
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
