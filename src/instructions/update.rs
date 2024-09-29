use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
};
use std::cell::RefCell;

use crate::states::Card;

pub(crate) fn update_card_account(
    accounts: &[AccountInfo],
    name: Option<String>,
    bio: Option<String>,
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let card_account_pda = next_account_info(account_iter)?;

    let mut card = Card::try_from_slice(*card_account_pda.data.borrow()).unwrap();

    if let Some(v) = name {
        card.name = v;
    }

    if let Some(v) = bio {
        card.bio = v;
    }

    msg!("{:?}", card);

    if card_account_pda.data_len() < (borsh::to_vec(&card)?).len() {
        todo!("Increase data size if small");
    }

    card.serialize(&mut (&mut RefCell::borrow_mut(&card_account_pda.data)[..]))?;
    Ok(())
}
