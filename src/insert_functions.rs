use crate::*;

pub (crate) fn hash_course_type(course_type: u8) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(course_type.to_string().as_bytes()));
    hash
}

pub (crate) fn get_type_of_course<T>(course: &T) -> u8 {
    match type_of_course(&course) {
        AnswerNIT => 0,
        //AnswerNBD => 1,
        _ => env::panic_str("Unknown course type"),
    }
}

pub (crate) fn type_of_course<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

pub trait AnsweringNIT {
    fn answer_nit(
        &mut self, 
        answer_rating: String, 
        fr_p: AccountId, 
        mo_p: AccountId, 
        tu_p: AccountId,
        we_p: AccountId,
        th_p: AccountId,
        nit_p: AccountId,
        com_fr: String,
        com_mo: String,
        com_tu: String,
        com_we: String,
        com_th: String,
        com_nit: String,
    ) -> FormId;
}

#[near_bindgen]
impl AnsweringNIT for Contract {
    #[payable]
    fn answer_nit(
        &mut self, 
        answer_rating: String, 
        fr_p: AccountId, 
        mo_p: AccountId, 
        tu_p: AccountId,
        we_p: AccountId,
        th_p: AccountId,
        nit_p: AccountId,
        com_fr: String,
        com_mo: String,
        com_tu: String,
        com_we: String,
        com_th: String,
        com_nit: String,
    ) -> FormId {
        let ratings_str: Vec<&str> = answer_rating.split(" ").collect();
        let ratings_num: Vec<u8> = ratings_str.iter().map(|x| x.parse::<u8>().unwrap()).collect();

        let answer: AnswerNIT = AnswerNIT {
            friday_professor: fr_p,
            monday_professor: mo_p,
            tuesday_professor: tu_p,
            wednesday_professor: we_p,
            thursday_professor: th_p,
            nit_main_professor: nit_p,
            answer_friday_class: ratings_num[0] + 1,
            answer_monday_class: ratings_num[1] + 1,
            answer_tuesday_class: ratings_num[2] + 1,
            answer_wednesday_class: ratings_num[3] + 1,
            answer_thursday_class: ratings_num[4] + 1,
            answer_nit_main_class: ratings_num[5] + 1,
            comment_friday_class: com_fr,
            comment_monday_class: com_mo,
            comment_tuesday_class: com_tu,
            comment_wednesday_class: com_we,
            comment_thursday_class: com_th,
            comment_main_class: com_nit,
        };

        let type_course: u8 = get_type_of_course(&answer);
        env::log_str(type_course.to_string().as_str());

        let student_name = env::signer_account_id();
        assert_one_yocto();
        assert!(!self.accounts_already_answered_nit.contains(&student_name), "You have already answered this form");
        self.accounts_already_answered_nit.insert(&student_name);

        let mut form_id = self.answer_nit.len().to_string();
        form_id.push_str(DELIMITER_NIT);
        form_id.push_str(&student_name.to_string());

        self.answer_nit.insert(&form_id, &answer);
        self.storage_deposits.insert(&student_name, &STORAGE_PER_FORM);
        Promise::new(env::current_account_id()).transfer(Balance::from(STORAGE_PER_FORM.clone()));

        self.by_account_id.insert(&student_name, &StudentAccount {
            balance: env::account_balance(),
            locked_balance: env::account_locked_balance(),
            nonce: env::block_height(),
            public_key: env::signer_account_pk(),
            used_gas: env::used_gas(),
        });

        let mut by_course_type = self.by_course_type.get(&type_course).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByCourseTypeInner {
                    course_type_hash: hash_course_type(type_course),
                }
                .try_to_vec()
                .unwrap(),
            )
            // let mut set = UnorderedSet::new();
            // set.insert(&form_id);
            // self.by_course_type.insert(&CourseType::NIT, &set);
            // self.by_course_type.get(&CourseType::NIT).unwrap()
        });
        by_course_type.insert(&form_id);
        self.by_course_type.insert(&type_course, &by_course_type);

        //self.by_course_type.insert(&CourseType::NIT, &form_id);

        //let form_id = "".to_string();
        form_id
    }
}