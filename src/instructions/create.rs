use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use std::cell::RefCell;

use crate::states::Card;

pub(crate) fn create_card_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: &str,
    bio: &str,
    bump: u8,
) -> ProgramResult {
    // msg!("Number of accounts: {}", accounts.len());
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?;
    // msg!("Got user account {}", user.key);
    let card_account_pda = next_account_info(account_iter)?;
    // msg!("Got card account {}", card_account_pda.key);

    let card = Card {
        name: name.to_string(),
        bio: bio.to_string(),
        bump,
    };
    // msg!("Card ready!");
    // msg!("Bump: {}", bump);

    let card_account_span = (borsh::to_vec(&card)?).len();
    let lamports_required = (Rent::get()?).minimum_balance(card_account_span);
    // msg!("Rent: {}", lamports_required);

    // msg!("Invoking!");
    invoke_signed(
        &system_instruction::create_account(
            user.key, // User is payer
            card_account_pda.key,
            lamports_required,
            card_account_span as u64,
            program_id,
        ),
        &[user.clone(), card_account_pda.clone()],
        &[&[user.key.as_ref(), &[bump]]],
    )?;

    card.serialize(&mut (&mut RefCell::borrow_mut(&card_account_pda.data)[..]))?;
    Ok(())
}
