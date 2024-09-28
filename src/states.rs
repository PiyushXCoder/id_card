use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub(crate) struct Card {
    pub(crate) name: String,
    pub(crate) bio: String,
    pub(crate) bump: u8,
}
