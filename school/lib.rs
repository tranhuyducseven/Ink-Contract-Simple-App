#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::string::String;
use ink_prelude::vec::Vec;

#[ink::contract]
mod school {
    use super::*;
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::traits::{PackedLayout, SpreadLayout};
    pub type Id = u32;

    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub enum Role {
        Admin,
        Member,
    }
    #[derive(
        Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode, SpreadLayout, PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(::scale_info::TypeInfo, ::ink_storage::traits::StorageLayout)
    )]
    pub struct Student {
        pub name: String,
        pub age: u32,
        pub id: Id,
        pub role: Role,
        pub author: AccountId,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct School {
        list_students: ink_storage::Mapping<Id, Student>,
        list_ids: Vec<Id>,
    }

    impl School {
        #[ink(constructor)]
        pub fn new(name: String, age: u32) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                let admin = Student {
                    name,
                    age,
                    id: 0,
                    role: Role::Admin,
                    author: caller,
                };

                contract.list_students.insert(&(admin.id), &admin);
                contract.list_ids.push(0);
            })
        }
        #[ink(message)]
        pub fn get_student(&self, id: u32) -> Student {
            self.list_students.get(&id).unwrap()
        }
        #[ink(message)]
        pub fn add_student(&mut self, name: String, age: u32, id: Id) {
            let caller = Self::env().caller();
            let student = Student {
                name,
                age,
                id,
                role: Role::Member,
                author: caller,
            };
            self.list_students.insert(&id, &student);
            self.list_ids.push(id);
        }
        #[ink(message)]
        pub fn view_all(&self) -> Vec<Student> {
            let ids = self.list_ids.clone();
            let mut list_students: Vec<Student> = Vec::new();
            for id in ids.iter() {
                let student = self.get_student(*id);
                list_students.push(student);
            }
            list_students
        }

        #[ink(message)]
        pub fn remove_student(&mut self, id: Id) {
            self.list_students.remove(id);
            self.list_ids.retain(|&x| x != id)
        }
    }
}
// #[cfg(test)]
// mod tests {
//     use crate::school::Role;
//     use crate::school::School;
//     use crate::school::Student;
//     use ink_env::test;
//     /// Imports all the definitions from the outer scope so we can use them here.
//     use ink_env::AccountId;
//     use ink_env::Environment;
//     use ink_lang as ink;
//     fn set_caller(sender: AccountId) {
//         ink_env::test::set_caller::<Environment>(sender);
//     }
//     fn default_accounts() -> test::DefaultAccounts<Environment> {
//         ink_env::test::default_accounts::<Environment>()
//     }
//
//     //    /// We test if the default constructor does its job.
//     #[ink::test]
//     fn default_works() {
//         let school = School::new(String::from("Huy Duc"), 18, 1);
//     let accounts = default_accounts();
//        let author = accounts.alice;
//
//        let student = Student {
//            name: "Huy Duc".to_string(),
//            age: 18,
//            id: 1,
//            role: Role::Admin,
//            author,
//        };
//        assert_eq!(school.get(1), student);
//    }
//}
//
//     //#[ink::test]
//     //fn it_works() {
//     //    let mut school = School::new(false);
//     //    assert_eq!(school.get(), false);
//     //    bank.flip();
//     //    assert_eq!(school.get(), true);
//     //}
// }
