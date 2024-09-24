use borsh::{BorshDeserialize, BorshSerialize};

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
    };

    use crate::Instruction;

    // declare and export the program's entrypoint
    entrypoint!(process_instruction);

    // program entrypoint's implementation
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let data = Instruction::try_from_slice(_instruction_data);

        msg!("{:?}", data);

        msg!(&format!("program_id: {}", _program_id));
        msg!(&format!("accounts: {:?}", _accounts));
        msg!(&format!(
            "instruction_data: {:?}",
            String::from_utf8_lossy(_instruction_data)
        ));
        let account_iter = &mut _accounts.iter();
        let account = next_account_info(account_iter)?;

        if account.owner == _program_id {
            msg!("Yes it is right account!");
        } else {
            msg!("Nope!");
        }

        // gracefully exit the program
        Ok(())
    }

    #[derive(BorshSerialize, BorshDeserialize)]
    struct GreetData {
        greeted_count: u32,
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum Instruction {
    CreateCard { id: u64, name: String, bio: String },
    UpdateCard { id: u64, name: String, bio: String },
    DeleteCard { id: u64 },
}
