use crate::*;

pub trait ModifierFunctions {
    fn delete_answer_nit(&mut self, form_id: FormId);
}

#[near_bindgen]
impl ModifierFunctions for Contract {
    fn delete_answer_nit(&mut self, form_id: FormId) {
        let student_account: AccountId = form_id.split(DELIMITER_NIT).collect()[1];
        self.answer_nit.remove(&form_id);
    }
}