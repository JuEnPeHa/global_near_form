use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ promise_result_as_success,
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance,
    Gas, PanicOnDefault, Promise, CryptoHash, BorshStorageKey,
};

trait AccountIdExt {
    fn is_zero(&self) -> bool;
}

impl AccountIdExt for AccountId {
    fn is_zero(&self) -> bool {
        self.is_zero()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
    pub balance: Balance,
    pub nonce: U64,
    pub creation_height: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
    pub nonce: U64,
    pub answer_friday_class: u8,
    pub friday_professor: AccountId,
    pub answer_monday_class: u8,
    pub monday_professor: AccountId,
    pub answer_tuesday_class: u8,
    pub tuesday_professor: AccountId,
    pub answer_wednesday_class: u8,
    pub wednesday_professor: AccountId,
    pub answer_thursday_class: u8,
    pub thursday_professor: AccountId,
    

}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub by_account_id: LookupMap<AccountId, Account>,
    pub accounts_already_answered_form: UnorderedSet<AccountId>,

}

#[near_bindgen]
impl Contract {

}