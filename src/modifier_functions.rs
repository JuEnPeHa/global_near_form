use crate::*;

pub trait ModifierFunctions {
    fn delete_answer_nit(&mut self, form_id: FormId);
}

#[near_bindgen]
impl ModifierFunctions for Contract {
    fn delete_answer_nit(&mut self, form_id: FormId) {
        let student_account: AccountId = AccountId::new_unchecked(form_id.split(DELIMITER_NIT).collect::<Vec<&str>>()[1].to_string());
        let form: Option<AnswerNIT> = self.answer_nit.get(&form_id);
        if let Some(form) = form {
                self.answer_nit.remove(&form.form_id);
                self.accounts_already_answered_nit.remove(&form.student);

                let mut by_teacher_friday = self.by_teacher_id.get(&form.friday_professor).unwrap();
                by_teacher_friday.remove(&form.form_id);
                self.by_teacher_id.insert(&form.friday_professor, &by_teacher_friday);

                let mut by_teacher_monday = self.by_teacher_id.get(&form.monday_professor).unwrap();
                by_teacher_monday.remove(&form.form_id);
                self.by_teacher_id.insert(&form.monday_professor, &by_teacher_monday);

                let mut by_teacher_tuesday = self.by_teacher_id.get(&form.tuesday_professor).unwrap();
                by_teacher_tuesday.remove(&form.form_id);
                self.by_teacher_id.insert(&form.tuesday_professor, &by_teacher_tuesday);

                let mut by_teacher_wednesday = self.by_teacher_id.get(&form.wednesday_professor).unwrap();
                by_teacher_wednesday.remove(&form.form_id);
                self.by_teacher_id.insert(&form.wednesday_professor, &by_teacher_wednesday);

                let mut by_teacher_thursday = self.by_teacher_id.get(&form.thursday_professor).unwrap();
                by_teacher_thursday.remove(&form.form_id);
                self.by_teacher_id.insert(&form.thursday_professor, &by_teacher_thursday);

                let mut by_teacher_nit = self.by_teacher_id.get(&form.nit_main_professor).unwrap();
                by_teacher_nit.remove(&form.form_id);
                self.by_teacher_id.insert(&form.nit_main_professor, &by_teacher_nit);


        } else {
            env::panic_str("Form not found");
        }
        let type_course: u8 = 0; // 0 = NIT
        //self.answer_nit.remove(&form_id);

        self.storage_deposits.remove(&student_account);
        self.by_account_id.remove(&student_account);

        let mut by_course_type = self.by_course_type.get(&type_course).unwrap();
        by_course_type.remove(&form_id);
        self.by_course_type.insert(&type_course, &by_course_type);



    }
}