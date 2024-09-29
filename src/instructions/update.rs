use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use std::cell::RefCell;

use crate::states::Card;

pub(crate) fn update_card_account(
    accounts: &[AccountInfo],
    name: Option<String>,
    bio: Option<String>,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?;
    let card_account_pda = next_account_info(account_iter)?;

    let mut card = Card::try_from_slice(*card_account_pda.data.borrow()).unwrap();

    if let Some(v) = name {
        card.name = v;
    }

    if let Some(v) = bio {
        card.bio = v;
    }

    let card_account_span = (borsh::to_vec(&card)?).len();
    let lamports_required = (Rent::get()?).minimum_balance(card_account_span);

    if lamports_required > card_account_pda.lamports() {
        invoke(
            &system_instruction::transfer(
                user.key,
                card_account_pda.key,
                lamports_required - card_account_pda.lamports(),
            ),
            &[user.clone(), card_account_pda.clone()],
        )?;
    }

    card_account_pda.realloc((borsh::to_vec(&card)?).len(), false)?;

    card.serialize(&mut (&mut RefCell::borrow_mut(&card_account_pda.data)[..]))?;
    Ok(())
}
