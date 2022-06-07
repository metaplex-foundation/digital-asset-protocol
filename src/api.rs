use bebop::{DeserializeError, Record};
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use thiserror::Error;
use crate::interfaces::create::CreateIdentity;
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
                    ActionData::CreateIdentity { uri } =>{
                        Ok(CreateIdentity::new(accounts, uri.to_string()))
                    }
                        ,
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

impl Into<DigitalAssetProtocolError> for DeserializeError  {
    fn into(self) -> DigitalAssetProtocolError {
        DigitalAssetProtocolError::DeError(self.to_string())
    }
}

impl<T> DecodeError<T> for DigitalAssetProtocolError {
    fn type_of() -> &'static str {
        "Dasset Error"
    }
}
