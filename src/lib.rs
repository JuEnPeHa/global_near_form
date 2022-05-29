use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ promise_result_as_success,
    assert_one_yocto, env, ext_contract, near_bindgen, AccountId, Balance,
    Gas, PanicOnDefault, Promise, CryptoHash, BorshStorageKey, require,
};

pub use crate::questions::*;
mod questions;

const STORAGE_PER_FORM: u128 = 100 * env::STORAGE_PRICE_PER_BYTE;
const DELIMITER_NIT: &str = ":NIT:";
const DELIMITER_NBD: &str = ":NBD:";

pub type FormId = String;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    AnswerNIT,
    AnswerNBD,
    StorageDeposits,
    QuestionNIT,
    QuestionNBD,
    ByAccountId,
    ByAccountIdInner { account_id_hash: CryptoHash },
    ByCourseType,
    ByCourseTypeInner { course_type_hash: CryptoHash },
    ByTeacherId,
    ByTeacherIdInner { teacher_id_hash: CryptoHash },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub question_nit: LazyOption<QuestionNIT>,
    pub by_account_id: LookupMap<AccountId, StudentAccount>,
    pub accounts_already_answered_nit: UnorderedSet<AccountId>,
    pub storage_deposits: UnorderedMap<AccountId, Balance>,

    pub answer_nit: UnorderedMap<FormId, AnswerNIT>,
    pub answer_nbd: UnorderedMap<FormId, /*AnswerNBD*/ AnswerNIT>,

    pub by_course_type: LookupMap<CourseType, UnorderedSet<FormId>>,

    pub by_teacher_id: LookupMap<AccountId, UnorderedSet<FormId>>,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CourseType {
    NIT,
    NBD,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta() -> Self {
        let owner_id = env::signer_account_id();
        Self::new(
            owner_id,
            QuestionNIT {
                question_friday_class: String::from("¿Cómo calificarías al profesor del taller de Introducción a NEAR?"),
                comment_friday_class: String::from("Cualquier comentario o sugerencia para el profesor es bien recibido"),
                friday_professor: vec![AccountId::new_unchecked("fritzwagner.near".to_string())] ,
                question_monday_class: String::from("¿Cómo calificarías al profesor del taller de Introducción a las dApps?"),
                comment_monday_class: String::from("Cualquier comentario o sugerencia para el profesor es bien recibido"),
                monday_professor: vec![AccountId::new_unchecked("ramgor.near".to_string())] ,
                question_tuesday_class: String::from("¿Cómo calificarías al profesor del taller de Introducción a los NFTs?"),
                comment_tuesday_class: String::from("Cualquier comentario o sugerencia para el profesor es bien recibido"),
                tuesday_professor: vec![AccountId::new_unchecked("alecaseg.near".to_string())] ,
                question_wednesday_class: String::from("¿Cómo calificarías al profesor del taller de Introducción al Ecommerce?"),
                comment_wednesday_class: String::from("Cualquier comentario o sugerencia para el profesor es bien recibido"),
                wednesday_professor: vec![AccountId::new_unchecked("luisaponte99.near".to_string())] ,
                question_thursday_class: String::from("¿Cómo calificarías al profesor del taller de Introducción al Desarrollo?"),
                comment_thursday_class: String::from("Cualquier comentario o sugerencia para el profesor es bien recibido"),
                thursday_professor: vec![AccountId::new_unchecked("jeph.near".to_string())] ,
                question_nit_main_class: String::from("¿Cómo calificarías al profesor de la semana del NIT?"),
                comment_main_class: String::from("Cualquier comentario o sugerencia para el profesor es bien recibido"),
                nit_main_professor: vec![AccountId::new_unchecked("maruja.near".to_string())] ,
            }
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, question_nit: QuestionNIT) -> Self {
        require!(!env::state_exists(), "Contract already initialized");
        let this = Self {
            owner_id,
            question_nit: LazyOption::new(
                StorageKey::QuestionNIT.try_to_vec().unwrap(),
                Some(&question_nit),
            ),
            by_account_id: LookupMap::new(StorageKey::ByAccountId.try_to_vec().unwrap()),
            accounts_already_answered_nit: UnorderedSet::new(StorageKey::AnswerNIT.try_to_vec().unwrap()),
            storage_deposits: UnorderedMap::new(StorageKey::StorageDeposits.try_to_vec().unwrap()),

            answer_nit: UnorderedMap::new(StorageKey::AnswerNIT.try_to_vec().unwrap()),
            answer_nbd: UnorderedMap::new(StorageKey::AnswerNBD.try_to_vec().unwrap()),

            by_course_type: LookupMap::new(StorageKey::ByCourseType.try_to_vec().unwrap()),

            by_teacher_id: LookupMap::new(StorageKey::ByTeacherId.try_to_vec().unwrap()),
        };
        this
    }
}