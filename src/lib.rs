use near_sdk::collections::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshSerialize, BorshSerialize, PanicOnDefault)]
pub struct Marketplace{
    products: UnorderedMap<String, String>
}
