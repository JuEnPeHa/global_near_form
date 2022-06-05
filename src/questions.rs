use crate::*;

pub(crate) fn assert_signer_is_owner(owner_id: AccountId) {
    let signer = env::signer_account_id();
    if signer != owner_id {
        env::panic_str("Only the owner can call this function");
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StudentAccount {
    pub balance: Balance,
    pub locked_balance: Balance,
    pub nonce: u64,
    pub public_key: PublicKey,
    pub used_gas: Gas,
    pub form_id_nit: FormId,
    pub form_id_nbd: FormId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AnswerNIT {
    //pub nonce: U64,
    pub student: AccountId,
    pub form_id: FormId,

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
    fn questions_nit(&self) -> QuestionNIT;

    fn set_friday_professor(&mut self, professor: AccountId);
    fn delete_friday_professor(&mut self, professor: AccountId);
    fn set_monday_professor(&mut self, professor: AccountId);
    fn delete_monday_professor(&mut self, professor: AccountId);
    fn set_tuesday_professor(&mut self, professor: AccountId);
    fn delete_tuesday_professor(&mut self, professor: AccountId);
    fn set_wednesday_professor(&mut self, professor: AccountId);
    fn delete_wednesday_professor(&mut self, professor: AccountId);
    fn set_thursday_professor(&mut self, professor: AccountId);
    fn delete_thursday_professor(&mut self, professor: AccountId);
    fn set_nit_main_professor(&mut self, professor: AccountId);
    fn delete_nit_main_professor(&mut self, professor: AccountId);

    fn get_question_friday_class(&self) -> String;
    fn change_question_friday_class(&mut self, question: String);
    fn get_question_monday_class(&self) -> String;
    fn change_question_monday_class(&mut self, question: String);
    fn get_question_tuesday_class(&self) -> String;
    fn change_question_tuesday_class(&mut self, question: String);
    fn get_question_wednesday_class(&self) -> String;
    fn change_question_wednesday_class(&mut self, question: String);
    fn get_question_thursday_class(&self) -> String;
    fn change_question_thursday_class(&mut self, question: String);
    fn get_question_nit_main_class(&self) -> String;
    fn change_question_nit_main_class(&mut self, question: String);
    fn get_friday_professor(&self) -> Vec<AccountId>;
    fn get_monday_professor(&self) -> Vec<AccountId>;
    fn get_tuesday_professor(&self) -> Vec<AccountId>;
    fn get_wednesday_professor(&self) -> Vec<AccountId>;
    fn get_thursday_professor(&self) -> Vec<AccountId>;
    fn get_nit_main_professor(&self) -> Vec<AccountId>;
    fn get_comment_friday_class(&self) -> String;
    fn change_comment_friday_class(&mut self, comment: String);
    fn get_comment_monday_class(&self) -> String;
    fn change_comment_monday_class(&mut self, comment: String);
    fn get_comment_tuesday_class(&self) -> String;
    fn change_comment_tuesday_class(&mut self, comment: String);
    fn get_comment_wednesday_class(&self) -> String;
    fn change_comment_wednesday_class(&mut self, comment: String);
    fn get_comment_thursday_class(&self) -> String;
    fn change_comment_thursday_class(&mut self, comment: String);
    fn get_comment_main_class(&self) -> String;
    fn change_comment_main_class(&mut self, comment: String);
}

#[near_bindgen]
impl QuestionNITExt for Contract {
    fn questions_nit(&self) -> QuestionNIT {
        self.question_nit.get().unwrap()
    }

    fn get_friday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().friday_professor
    }
    fn set_friday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.friday_professor.retain(|x| x != &professor);
        question_nit.friday_professor.push(professor);
        self.question_nit.set(&question_nit);
    }
    fn delete_friday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.friday_professor.retain(|x| x != &professor);
        self.question_nit.set(&question_nit);
    }

    fn get_monday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().monday_professor
    }
    fn set_monday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.monday_professor.retain(|x| x != &professor);
        question_nit.monday_professor.push(professor);
        self.question_nit.set(&question_nit);
    }
    fn delete_monday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.monday_professor.retain(|x| x != &professor);
        self.question_nit.set(&question_nit);
    }

    fn get_tuesday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().tuesday_professor
    }
    fn set_tuesday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.tuesday_professor.retain(|x| x != &professor);
        question_nit.tuesday_professor.push(professor);
        self.question_nit.set(&question_nit);
    }
    fn delete_tuesday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.tuesday_professor.retain(|x| x != &professor);
        self.question_nit.set(&question_nit);
    }

    fn get_wednesday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().wednesday_professor
    }
    fn set_wednesday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.wednesday_professor.retain(|x| x != &professor);
        question_nit.wednesday_professor.push(professor);
        self.question_nit.set(&question_nit);
    }
    fn delete_wednesday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.wednesday_professor.retain(|x| x != &professor);
        self.question_nit.set(&question_nit);
    }

    fn get_thursday_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().thursday_professor
    }
    fn set_thursday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.thursday_professor.retain(|x| x != &professor);
        question_nit.thursday_professor.push(professor);
        self.question_nit.set(&question_nit);
    }
    fn delete_thursday_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.thursday_professor.retain(|x| x != &professor);
        self.question_nit.set(&question_nit);
    }

    fn get_nit_main_professor(&self) -> Vec<AccountId> {
        self.question_nit.get().unwrap().nit_main_professor
    }
    fn set_nit_main_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.nit_main_professor.retain(|x| x != &professor);
        question_nit.nit_main_professor.push(professor);
        self.question_nit.set(&question_nit);
    }
    fn delete_nit_main_professor(&mut self, professor: AccountId) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.nit_main_professor.retain(|x| x != &professor);
        self.question_nit.set(&question_nit);
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

    fn change_question_friday_class(&mut self, question_friday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.question_friday_class = question_friday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_question_monday_class(&mut self, question_monday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.question_monday_class = question_monday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_question_tuesday_class(&mut self, question_tuesday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.question_tuesday_class = question_tuesday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_question_wednesday_class(&mut self, question_wednesday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.question_wednesday_class = question_wednesday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_question_thursday_class(&mut self, question_thursday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.question_thursday_class = question_thursday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_question_nit_main_class(&mut self, question_nit_main_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.question_nit_main_class = question_nit_main_class;
        self.question_nit.set(&question_nit);
    }

    fn change_comment_friday_class(&mut self, comment_friday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.comment_friday_class = comment_friday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_comment_monday_class(&mut self, comment_monday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.comment_monday_class = comment_monday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_comment_tuesday_class(&mut self, comment_tuesday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.comment_tuesday_class = comment_tuesday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_comment_wednesday_class(&mut self, comment_wednesday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.comment_wednesday_class = comment_wednesday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_comment_thursday_class(&mut self, comment_thursday_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.comment_thursday_class = comment_thursday_class;
        self.question_nit.set(&question_nit);
    }
    fn change_comment_main_class(&mut self, comment_main_class: String) {
        assert_signer_is_owner(self.owner_id.clone());
        let mut question_nit = self.questions_nit();
        question_nit.comment_main_class = comment_main_class;
        self.question_nit.set(&question_nit);
    }
}
