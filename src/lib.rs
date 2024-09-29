pub(crate) mod instructions;
pub(crate) mod states;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
        pubkey::Pubkey,
    };

    use crate::instructions;

    solana_program::entrypoint!(process_instruction);

    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let command = Card::try_from_slice(_instruction_data);

        #[allow(unused)]
        match command {
            Err(_) => return Err(ProgramError::InvalidArgument),
            Ok(Card::Create { name, bio, bump }) => {
                instructions::create_card_account(_program_id, _accounts, &name, &bio, bump);
            }
            Ok(Card::Update { name, bio }) => {
                instructions::update_card_account(_accounts, name, bio);
            }
            Ok(Card::Delete) => {
                instructions::delete_card_account(_accounts);
            }
            _ => {
                msg!("Bad request")
            }
        }

        Ok(())
    }

    #[derive(BorshSerialize, BorshDeserialize, Debug)]
    enum Card {
        Create {
            name: String,
            bio: String,
            bump: u8,
        },
        Update {
            name: Option<String>,
            bio: Option<String>,
        },
        Delete,
    }
}
