use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ promise_result_as_success,
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance,
    Gas, PanicOnDefault, Promise, CryptoHash, BorshStorageKey, require,
};

trait AccountIdExt {
    fn is_zero(&self) -> bool;
}

impl AccountIdExt for AccountId {
    fn is_zero(&self) -> bool {
        self.as_bytes().iter().all(|b| *b == 0)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StudentAccount {
    pub balance: Balance,
    pub nonce: U64,
    pub creation_height: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AnswerNIT {
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

    pub answer_nit_main_class: u8,
    pub nit_main_professor: AccountId,

}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionNIT {
    pub nonce: String,
    pub question_friday_class: String,
    pub friday_professor: Vec<AccountId>,
    pub question_monday_class: String,
    pub monday_professor: Vec<AccountId>,
    pub question_tuesday_class: String,
    pub tuesday_professor: Vec<AccountId>,
    pub question_wednesday_class: String,
    pub wednesday_professor: Vec<AccountId>,
    pub question_thursday_class: String,
    pub thursday_professor: Vec<AccountId>,

    pub question_nit_main_class: String,
    pub nit_main_professor: Vec<AccountId>,

}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub by_account_id: LookupMap<AccountId, StudentAccount>,
    pub accounts_already_answered_nit: UnorderedSet<AccountId>,
    pub storage_deposits: UnorderedMap<AccountId, Balance>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta() -> Self {
        let owner_id = env::signer_account_id();
        Self::new(
            owner_id,
        )
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        require!(!env::state_exists(), "Contract already initialized");
        let this = Self {
            owner_id,

        };
        this
    }
}