/**
* Stub of a "compound"-style contract
*/
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::near_bindgen;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc<'_> = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Compound {}

impl Default for Compound {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
#[allow(unused_variables)]
impl Compound {
    pub fn ft_on_transfer(&self, sender_id: ValidAccountId, amount: U128, msg: String) -> U128 {
        U128::from(0)
    }
}
