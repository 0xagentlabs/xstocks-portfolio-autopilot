#![no_std]

use pinocchio::{
    account::AccountView, cpi::Signer, entrypoint, error::ProgramError, Address, ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

entrypoint!(process_instruction);
pinocchio::nostd_panic_handler!();

const PREFIX: &[u8] = b"portfolio";
pub const STATE_LEN: usize = 160;

#[repr(u32)]
pub enum AutopilotError {
    BadInstruction = 100,
    BadAccounts,
    Unauthorized,
    BadPda,
    AlreadyInitialized,
    InvalidWeights,
    InvalidRiskLimits,
    InvalidStatus,
    DuplicateExecution,
    Overflow,
}

fn err(e: AutopilotError) -> ProgramError {
    ProgramError::Custom(e as u32)
}
fn u16_at(d: &[u8], o: usize) -> Result<u16, ProgramError> {
    Ok(u16::from_le_bytes(
        d.get(o..o + 2)
            .ok_or(err(AutopilotError::BadInstruction))?
            .try_into()
            .unwrap(),
    ))
}
fn u64_at(d: &[u8], o: usize) -> Result<u64, ProgramError> {
    Ok(u64::from_le_bytes(
        d.get(o..o + 8)
            .ok_or(err(AutopilotError::BadInstruction))?
            .try_into()
            .unwrap(),
    ))
}
fn require_authority(a: &AccountView, state: &[u8]) -> ProgramResult {
    if !a.is_signer() || state.get(8..40) != Some(a.address().as_ref()) {
        return Err(err(AutopilotError::Unauthorized));
    }
    Ok(())
}
fn require_state<'a>(
    program_id: &Address,
    state: &'a AccountView,
) -> Result<pinocchio::account::Ref<'a, [u8]>, ProgramError> {
    if state.owner() != program_id || state.data_len() != STATE_LEN {
        return Err(err(AutopilotError::BadAccounts));
    }
    state.try_borrow()
}

pub fn process_instruction(
    program_id: &Address,
    accounts: &mut [AccountView],
    data: &[u8],
) -> ProgramResult {
    let tag = *data.first().ok_or(err(AutopilotError::BadInstruction))?;
    match tag {
        0 => initialize(program_id, accounts, data),
        1 => update_strategy(program_id, accounts, data),
        2 => set_status(program_id, accounts, data),
        3 => record_execution(program_id, accounts, data),
        _ => Err(err(AutopilotError::BadInstruction)),
    }
}

// [0 tag, 1 bump, 2 asset_count, 3.. name(24), 27.. weights(8*u16),
// 43.. core ratios(8*u16), 59 rebalance_bps u16, 61 slippage_bps u16,
// 63 max_daily_trades u16, 65 max_turnover_bps u16]
fn initialize(program_id: &Address, a: &mut [AccountView], ix: &[u8]) -> ProgramResult {
    if a.len() != 3 || ix.len() != 67 {
        return Err(err(AutopilotError::BadAccounts));
    }
    let (authority, rest) = a.split_first_mut().unwrap();
    let (state, _system) = rest.split_first_mut().unwrap();
    if !authority.is_signer() || !authority.is_writable() || !state.is_writable() {
        return Err(err(AutopilotError::Unauthorized));
    }
    let bump = ix[1];
    let bump_seed = [bump];
    let expected = Address::derive_address(
        &[PREFIX, authority.address().as_ref()],
        Some(bump),
        program_id,
    );
    if state.address() != &expected {
        return Err(err(AutopilotError::BadPda));
    }
    if state.data_len() != 0 || state.lamports() != 0 {
        return Err(err(AutopilotError::AlreadyInitialized));
    }
    validate_config(ix)?;
    let seeds = [
        pinocchio::cpi::Seed::from(PREFIX),
        pinocchio::cpi::Seed::from(authority.address().as_ref()),
        pinocchio::cpi::Seed::from(&bump_seed),
    ];
    CreateAccount::with_minimum_balance(authority, state, STATE_LEN as u64, program_id, None)?
        .invoke_signed(&[Signer::from(&seeds)])?;
    let mut out = state.try_borrow_mut()?;
    out.fill(0);
    out[0..8].copy_from_slice(b"XSTOCKS1");
    out[8..40].copy_from_slice(authority.address().as_ref());
    out[40] = bump;
    out[41] = 0; // DRAFT
    out[42] = ix[2];
    out[43..67].copy_from_slice(&ix[3..27]);
    out[67..83].copy_from_slice(&ix[27..43]);
    out[83..99].copy_from_slice(&ix[43..59]);
    out[99..107].copy_from_slice(&ix[59..67]);
    Ok(())
}

fn validate_config(ix: &[u8]) -> ProgramResult {
    let count = ix[2] as usize;
    if count == 0 || count > 8 {
        return Err(err(AutopilotError::InvalidWeights));
    }
    let mut sum = 0u32;
    for i in 0..count {
        let w = u16_at(ix, 27 + i * 2)?;
        let core = u16_at(ix, 43 + i * 2)?;
        if core > 10_000 {
            return Err(err(AutopilotError::InvalidWeights));
        }
        sum = sum
            .checked_add(w as u32)
            .ok_or(err(AutopilotError::Overflow))?;
    }
    if sum != 10_000 {
        return Err(err(AutopilotError::InvalidWeights));
    }
    if u16_at(ix, 59)? > 5_000
        || u16_at(ix, 61)? > 1_000
        || u16_at(ix, 63)? == 0
        || u16_at(ix, 65)? > 10_000
    {
        return Err(err(AutopilotError::InvalidRiskLimits));
    }
    Ok(())
}

// same config bytes as initialize after the tag; increments version
fn update_strategy(program_id: &Address, a: &mut [AccountView], ix: &[u8]) -> ProgramResult {
    if a.len() != 2 || ix.len() != 67 {
        return Err(err(AutopilotError::BadAccounts));
    }
    validate_config(ix)?;
    let snapshot = require_state(program_id, &a[1])?;
    require_authority(&a[0], &snapshot)?;
    drop(snapshot);
    if !a[1].is_writable() {
        return Err(err(AutopilotError::BadAccounts));
    }
    let mut out = a[1].try_borrow_mut()?;
    out[42] = ix[2];
    out[43..67].copy_from_slice(&ix[3..27]);
    out[67..83].copy_from_slice(&ix[27..43]);
    out[83..99].copy_from_slice(&ix[43..59]);
    out[99..107].copy_from_slice(&ix[59..67]);
    let version = u64::from_le_bytes(out[107..115].try_into().unwrap())
        .checked_add(1)
        .ok_or(err(AutopilotError::Overflow))?;
    out[107..115].copy_from_slice(&version.to_le_bytes());
    Ok(())
}

// statuses: 0 draft, 1 running, 2 paused, 3 risk_paused, 4 error, 5 stopped
fn set_status(program_id: &Address, a: &mut [AccountView], ix: &[u8]) -> ProgramResult {
    if a.len() != 2 || ix.len() != 2 || ix[1] > 5 {
        return Err(err(AutopilotError::InvalidStatus));
    }
    let snapshot = require_state(program_id, &a[1])?;
    require_authority(&a[0], &snapshot)?;
    drop(snapshot);
    if !a[1].is_writable() {
        return Err(err(AutopilotError::BadAccounts));
    }
    a[1].try_borrow_mut()?[41] = ix[1];
    Ok(())
}

// [tag, execution_id u64, input u64, output u64, fee u64, side u8, asset_index u8]
fn record_execution(program_id: &Address, a: &mut [AccountView], ix: &[u8]) -> ProgramResult {
    if a.len() != 2 || ix.len() != 35 {
        return Err(err(AutopilotError::BadInstruction));
    }
    let snapshot = require_state(program_id, &a[1])?;
    require_authority(&a[0], &snapshot)?;
    if snapshot[41] != 1 {
        return Err(err(AutopilotError::InvalidStatus));
    }
    if ix[34] >= snapshot[42] || ix[33] > 1 {
        return Err(err(AutopilotError::BadInstruction));
    }
    let id = u64_at(ix, 1)?;
    if u64::from_le_bytes(snapshot[115..123].try_into().unwrap()) >= id {
        return Err(err(AutopilotError::DuplicateExecution));
    }
    drop(snapshot);
    let mut out = a[1].try_borrow_mut()?;
    out[115..123].copy_from_slice(&id.to_le_bytes());
    let trades = u64::from_le_bytes(out[123..131].try_into().unwrap())
        .checked_add(1)
        .ok_or(err(AutopilotError::Overflow))?;
    let turnover = u64::from_le_bytes(out[131..139].try_into().unwrap())
        .checked_add(u64_at(ix, 9)?)
        .ok_or(err(AutopilotError::Overflow))?;
    let fees = u64::from_le_bytes(out[139..147].try_into().unwrap())
        .checked_add(u64_at(ix, 25)?)
        .ok_or(err(AutopilotError::Overflow))?;
    out[123..131].copy_from_slice(&trades.to_le_bytes());
    out[131..139].copy_from_slice(&turnover.to_le_bytes());
    out[139..147].copy_from_slice(&fees.to_le_bytes());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_bad_weight_sum() {
        let mut x = [0u8; 67];
        x[2] = 1;
        x[27..29].copy_from_slice(&9999u16.to_le_bytes());
        x[43..45].copy_from_slice(&8000u16.to_le_bytes());
        x[59..61].copy_from_slice(&500u16.to_le_bytes());
        x[61..63].copy_from_slice(&50u16.to_le_bytes());
        x[63..65].copy_from_slice(&10u16.to_le_bytes());
        x[65..67].copy_from_slice(&2000u16.to_le_bytes());
        assert!(validate_config(&x).is_err());
    }
    #[test]
    fn accepts_valid_config() {
        let mut x = [0u8; 67];
        x[2] = 2;
        x[27..29].copy_from_slice(&6000u16.to_le_bytes());
        x[29..31].copy_from_slice(&4000u16.to_le_bytes());
        x[43..45].copy_from_slice(&8000u16.to_le_bytes());
        x[45..47].copy_from_slice(&9000u16.to_le_bytes());
        x[59..61].copy_from_slice(&500u16.to_le_bytes());
        x[61..63].copy_from_slice(&50u16.to_le_bytes());
        x[63..65].copy_from_slice(&10u16.to_le_bytes());
        x[65..67].copy_from_slice(&2000u16.to_le_bytes());
        assert!(validate_config(&x).is_ok());
    }
}
