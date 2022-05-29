use crate::*;

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
    //pub nonce: U64,
    pub answer_friday_class: u8,
    pub friday_professor: AccountId,
    pub comment_friday_class: String,

    pub answer_monday_class: u8,
    pub monday_professor: AccountId,
    pub comment_monday_class: String,

    pub answer_tuesday_class: u8,
    pub tuesday_professor: AccountId,
    pub comment_tuesday_class: String,

    pub answer_wednesday_class: u8,
    pub wednesday_professor: AccountId,
    pub comment_wednesday_class: String,

    pub answer_thursday_class: u8,
    pub thursday_professor: AccountId,
    pub comment_thursday_class: String,

    pub answer_nit_main_class: u8,
    pub nit_main_professor: AccountId,
    pub comment_main_class: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct QuestionNIT {
    //pub nonce: String,
    pub question_friday_class: String,
    pub friday_professor: Vec<AccountId>,
    pub comment_friday_class: String,
    pub question_monday_class: String,
    pub monday_professor: Vec<AccountId>,
    pub comment_monday_class: String,
    pub question_tuesday_class: String,
    pub tuesday_professor: Vec<AccountId>,
    pub comment_tuesday_class: String,
    pub question_wednesday_class: String,
    pub wednesday_professor: Vec<AccountId>,
    pub comment_wednesday_class: String,
    pub question_thursday_class: String,
    pub thursday_professor: Vec<AccountId>,
    pub comment_thursday_class: String,
    pub question_nit_main_class: String,
    pub nit_main_professor: Vec<AccountId>,
    pub comment_main_class: String,
}

pub trait QuestionNITExt {
    fn questionsNIT(&self) -> QuestionNIT;

    fn get_question_friday_class(&self) -> String;
    fn get_question_monday_class(&self) -> String;
    fn get_question_tuesday_class(&self) -> String;
    fn get_question_wednesday_class(&self) -> String;
    fn get_question_thursday_class(&self) -> String;
    fn get_question_nit_main_class(&self) -> String;
    fn get_friday_professor(&self) -> Vec<AccountId>;
    fn get_monday_professor(&self) -> Vec<AccountId>;
    fn get_tuesday_professor(&self) -> Vec<AccountId>;
    fn get_wednesday_professor(&self) -> Vec<AccountId>;
    fn get_thursday_professor(&self) -> Vec<AccountId>;
    fn get_nit_main_professor(&self) -> Vec<AccountId>;
    fn get_comment_friday_class(&self) -> String;
    fn get_comment_monday_class(&self) -> String;
    fn get_comment_tuesday_class(&self) -> String;
    fn get_comment_wednesday_class(&self) -> String;
    fn get_comment_thursday_class(&self) -> String;
    fn get_comment_main_class(&self) -> String;
}

#[near_bindgen]
impl QuestionNITExt for Contract {

    fn questionsNIT(&self) -> QuestionNIT {
        self.question_nit.get().unwrap()
    }

    fn get_question_friday_class(&self) -> String {
        self.question_nit.get().unwrap().question_friday_class
    }
    fn get_question_monday_class(&self) -> String {
        self.question_nit.get().unwrap().question_monday_class
    }
    fn get_question_tuesday_class(&self) -> String {
        self.question_nit.get().unwrap().question_tuesday_class
    }
    fn get_question_wednesday_class(&self) -> String {
        self.question_nit.get().unwrap().question_wednesday_class
    }
    fn get_question_thursday_class(&self) -> String {
        self.question_nit.get().unwrap().question_thursday_class
    }
    fn get_question_nit_main_class(&self) -> String {
        self.question_nit.get().unwrap().question_nit_main_class
    }
    fn get_friday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().friday_professor
    }
    fn get_monday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().monday_professor
    }
    fn get_tuesday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().tuesday_professor
    }
    fn get_wednesday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().wednesday_professor
    }
    fn get_thursday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().thursday_professor
    }
    fn get_nit_main_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().nit_main_professor
    }
    fn get_comment_friday_class(&self) -> String {
        self.question_nit.get().unwrap().comment_friday_class
    }
    fn get_comment_monday_class(&self) -> String {
        self.question_nit.get().unwrap().comment_monday_class
    }
    fn get_comment_tuesday_class(&self) -> String {
        self.question_nit.get().unwrap().comment_tuesday_class
    }
    fn get_comment_wednesday_class(&self) -> String {
        self.question_nit.get().unwrap().comment_wednesday_class
    }
    fn get_comment_thursday_class(&self) -> String {
        self.question_nit.get().unwrap().comment_thursday_class
    }
    fn get_comment_main_class(&self) -> String {
        self.question_nit.get().unwrap().comment_main_class
    }
}