use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_number_answers(&self) -> U64 {
        U64(
            self.accounts_already_answered_nit.len() as u64, /*+ self.accounts_already_answered_nbd.len() as u64*/
        )
    }

    pub fn get_accounts_already_answered(&self) -> Vec<AccountId> {
        self.accounts_already_answered_nit.iter().collect()
    }

    pub fn get_number_answers_nit(&self) -> U64 {
        U64(self.accounts_already_answered_nit.len() as u64)
    }
    /*
    pub fn get_number_answers_nbd(&self) -> U64 {
        U64(self.accounts_already_answered_nbd.len() as u64)
    }*/

    pub fn get_answer_by_form_id_nit(&self, form_id: FormId) -> Option<AnswerNIT> {
        self.answer_nit.get(&form_id)
    }

    pub fn get_answers_nit(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<AnswerNIT> {
        let by_course_type = self.by_course_type.get(&0u8);
        let answers = if let Some(by_course_type) = by_course_type {
            by_course_type
        } else {
            UnorderedSet::new(b"a")
        };
        let keys = answers.as_vector();
        let start = u128::from(from_index.unwrap_or(U128(0)));
        keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(100) as usize)
            .map(|key| self.answer_nit.get(&key).unwrap())
            .collect::<Vec<AnswerNIT>>()
    }

    pub fn get_answers_by_teacher_id(
        &self,
        teacher_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<AnswerNIT> {
        let by_teacher_id = self.by_teacher_id.get(&teacher_id);
        let answers = if let Some(by_teacher_id) = by_teacher_id {
            by_teacher_id
        } else {
            UnorderedSet::new(b"b")
        };
        let keys = answers.as_vector();
        let start = u128::from(from_index.unwrap_or(U128(0)));
        keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(100) as usize)
            .map(|key| self.answer_nit.get(&key).unwrap())
            .collect::<Vec<AnswerNIT>>()
    }

    pub fn get_student_account(&self, student_account: AccountId) -> Option<StudentAccount> {
        self.by_account_id.get(&student_account)
    }

    pub fn get_number_teacher_clases(&self, teacher_id: AccountId) -> U64 {
        let by_teacher_id = self.by_teacher_id.get(&teacher_id);
        let answers = if let Some(by_teacher_id) = by_teacher_id {
            by_teacher_id
        } else {
            UnorderedSet::new(b"c")
        };
        U64(answers.len() as u64)
    }

    // pub fn get_answer_by_form_id_nbd(
    //     &self,
    //     form_id: FormId
    // ) -> Option<AnswerNBD> {
    //     self.answer_nbd.get(&form_id)
    // }
}
