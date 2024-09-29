use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
};

pub(crate) fn delete_card_account(accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?;
    let card_account_pda = next_account_info(account_iter)?;

    let lamports = card_account_pda.lamports();
    **user.try_borrow_mut_lamports()? += lamports;
    **card_account_pda.try_borrow_mut_lamports()? -= lamports;

    Ok(())
}
