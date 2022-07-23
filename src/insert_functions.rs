use crate::*;

pub(crate) fn hash_course_type(course_type: &u8) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(course_type.to_string().as_bytes()));
    hash
}

pub(crate) fn hash_teacher_id(teacher_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(teacher_id.as_bytes()));
    hash
}

// pub(crate) fn get_type_of_course<String>(course: String) -> u8 {
//     let answer_nit = "&global_near_form::questions::AnswerNIT".to_string();
//     let answer_nbd = "&global_near_form::questions::AnswerNBD".to_string();
//     match type_of_course(&course) {
//         answer_nit => 0,
//         answer_nbd => 1,
//         _ => env::panic_str("Unknown course type"),
//     }
// }

// pub(crate) fn type_of_course<T>(_: &T) -> String {
//     let tipo = std::any::type_name::<T>().to_string();
//     env::log_str(&tipo);
//     tipo
// }

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
        let student = env::signer_account_id();
        let student_name = env::signer_account_id();

        let mut form_id = self.answer_nit.len().to_string();
        form_id.push_str(DELIMITER_NIT);
        form_id.push_str(&student_name.to_string());

        let form_id_cloned = form_id.clone();

        let ratings_str: Vec<&str> = answer_rating.split(" ").collect();
        let ratings_num: Vec<u8> = ratings_str
            .iter()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let answer: AnswerNIT = AnswerNIT {
            student,
            form_id: form_id_cloned,
            friday_professor: fr_p.clone(),
            monday_professor: mo_p.clone(),
            tuesday_professor: tu_p.clone(),
            wednesday_professor: we_p.clone(),
            thursday_professor: th_p.clone(),
            nit_main_professor: nit_p.clone(),
            answer_friday_class: ratings_num[0],
            answer_monday_class: ratings_num[1],
            answer_tuesday_class: ratings_num[2],
            answer_wednesday_class: ratings_num[3],
            answer_thursday_class: ratings_num[4],
            answer_nit_main_class: ratings_num[5],
            comment_friday_class: com_fr,
            comment_monday_class: com_mo,
            comment_tuesday_class: com_tu,
            comment_wednesday_class: com_we,
            comment_thursday_class: com_th,
            comment_main_class: com_nit,
        };

        let type_course: u8 = 0; // 0 = NIT
                                 //env::log_str(type_course.to_string().as_str());

        //assert_one_yocto();
        require!(
            !self.accounts_already_answered_nit.contains(&student_name),
            format!("You already answered the NIT form")
        );
        // assert!(
        //     !self.accounts_already_answered_nit.contains(&student_name),
        //     "You have already answered this form"
        // );
        self.accounts_already_answered_nit.insert(&student_name);

        self.answer_nit.insert(&form_id, &answer);
        self.storage_deposits
            .insert(&student_name, &STORAGE_PER_FORM);
        //Promise::new(env::current_account_id()).transfer(Balance::from(STORAGE_PER_FORM.clone()));

        let by_account_id = self.by_account_id.get(&student_name);
        if let Some(by_account_id) = by_account_id {
            let mut by_account_id = by_account_id;
            by_account_id.form_id_nit.push_str(&form_id);
            //by_account_id.insert(&form_id, &type_course);
            self.by_account_id.insert(&student_name, &by_account_id);
            //by_account_id
        } else {
            self.by_account_id.insert(
                &student_name,
                &StudentAccount {
                    balance: env::account_balance(),
                    locked_balance: env::account_locked_balance(),
                    nonce: env::block_height(),
                    public_key: env::signer_account_pk(),
                    used_gas: env::used_gas(),
                    form_id_nit: form_id.clone(),
                    form_id_nbd: "".to_string(),
                },
            );
            //bydfds
        };

        let mut by_course_type = self.by_course_type.get(&type_course).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByCourseTypeInner {
                    course_type_hash: hash_course_type(&type_course),
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

        let mut by_teacher_friday = self.by_teacher_id.get(&fr_p).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByTeacherIdInner {
                    teacher_id_hash: hash_teacher_id(&fr_p),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        by_teacher_friday.insert(&form_id);
        self.by_teacher_id.insert(&fr_p, &by_teacher_friday);

        let mut by_teacher_monday = self.by_teacher_id.get(&mo_p).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByTeacherIdInner {
                    teacher_id_hash: hash_teacher_id(&mo_p),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        by_teacher_monday.insert(&form_id);
        self.by_teacher_id.insert(&mo_p, &by_teacher_monday);

        let mut by_teacher_tuesday = self.by_teacher_id.get(&tu_p).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByTeacherIdInner {
                    teacher_id_hash: hash_teacher_id(&tu_p),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        by_teacher_tuesday.insert(&form_id);
        self.by_teacher_id.insert(&tu_p, &by_teacher_tuesday);

        let mut by_teacher_wednesday = self.by_teacher_id.get(&we_p).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByTeacherIdInner {
                    teacher_id_hash: hash_teacher_id(&we_p),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        by_teacher_wednesday.insert(&form_id);
        self.by_teacher_id.insert(&we_p, &by_teacher_wednesday);

        let mut by_teacher_thursday = self.by_teacher_id.get(&th_p).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByTeacherIdInner {
                    teacher_id_hash: hash_teacher_id(&th_p),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        by_teacher_thursday.insert(&form_id);
        self.by_teacher_id.insert(&th_p, &by_teacher_thursday);

        let mut by_teacher_nit = self.by_teacher_id.get(&nit_p).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::ByTeacherIdInner {
                    teacher_id_hash: hash_teacher_id(&nit_p),
                }
                .try_to_vec()
                .unwrap(),
            )
        });
        by_teacher_nit.insert(&form_id);
        self.by_teacher_id.insert(&nit_p, &by_teacher_nit);

        form_id
    }
}
