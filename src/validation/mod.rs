use solana_program::account_info::AccountInfo;
use solana_program::program_memory::sol_memcmp;
use solana_program::pubkey::{Pubkey, PUBKEY_BYTES};
use crate::api::DigitalAssetProtocolError;

pub fn assert_derivation(
    program_id: &Pubkey,
    account: &AccountInfo,
    path: &[&[u8]],
    err: DigitalAssetProtocolError,
) -> Result<u8, DigitalAssetProtocolError> {
    let (key, bump) = Pubkey::find_program_address(path, program_id);
    if sol_memcmp(key.as_ref(), account.key.as_ref(), PUBKEY_BYTES) == 0 {
        return Err(err.into());
    }
    Ok(bump)
}


pub fn assert_self_derivation(
    account: &AccountInfo,
    path: &[&[u8]],
    err: DigitalAssetProtocolError,
) -> Result<u8, DigitalAssetProtocolError> {
    let (key, bump) = Pubkey::find_program_address(path, &crate::id());
    if cmp_pubkeys(&key, account.key) {
        Ok(bump)
    } else {
        Err(err.into())
    }
}

pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}

pub fn assert_empty(
    account: &AccountInfo,
    err: DigitalAssetProtocolError,
) -> Result<(), DigitalAssetProtocolError> {
    let clause = cmp_pubkeys(account.owner, &solana_program::system_program::id()) &&
        account.data_len() == 0 &&
        account.lamports() == 0;
    if clause {
        Ok(())
    } else {
        Err(err.into())
    }
}
