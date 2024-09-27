pub(crate) mod instructions;
pub(crate) mod states;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
        program_error::ProgramError, pubkey::Pubkey,
    };

    entrypoint!(process_instruction);

    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let command = Card::try_from_slice(_instruction_data);

        // let account_iter = &mut _accounts.iter();
        // let creater = next_account_info(account_iter)?;

        #[allow(unused)]
        match command {
            Err(_) => return Err(ProgramError::InvalidArgument),
            Ok(Card::Create { id, name }) => {
                todo!("Create account with card info");
            }
            Ok(Card::Update { id, name }) => {
                todo!("Update account with new card info")
            }
            Ok(Card::Delete) => {}
            _ => {
                todo!("Delete accoun of given card")
            }
        }

        Ok(())
    }

    #[derive(BorshSerialize, BorshDeserialize, Debug)]
    pub(crate) enum Card {
        Create { id: u64, name: String },
        Update { id: u64, name: String },
        Delete,
    }
}
