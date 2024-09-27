use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub(crate) struct Card {
    name: String,
    bio: String,
}
