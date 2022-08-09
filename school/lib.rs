#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::string::String;

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
        Student,
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
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct School {
        list_students: ink_storage::Mapping<Id, Student>,
    }

    impl School {
        #[ink(constructor)]
        pub fn new(name: String, age: u32, id: Id) -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let _caller = Self::env().caller();
                let admin = Student {
                    name,
                    age,
                    id,
                    role: Role::Admin,
                };

                contract.list_students.insert(&id, &admin);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            // Even though we're not explicitly initializing the `Mapping`,
            // we still need to call this
            ink_lang::utils::initialize_contract(|_| {})
        }

        // Grab the number at the caller's AccountID, if it exists
        #[ink(message)]
        pub fn get(&self, id: u32) -> Student {
            self.list_students.get(&id).unwrap()
        }
    }
}
#[cfg(test)]
mod tests {
    /// Imports all the definitions from the outer scope so we can use them here.
    use crate::school::Role;
    use crate::school::School;
    use crate::school::Student;

    /// Imports `ink_lang` so we can use `#[ink::test]`.
    use ink_lang as ink;

    //    /// We test if the default constructor does its job.
    #[ink::test]
    fn default_works() {
        let school = School::new(String::from("Huy Duc"), 18, 1);
        let student = Student {
            name: "Huy Duc".to_string(),
            age: 18,
            id: 1,
            role: Role::Admin,
        };
        assert_eq!(school.get(1), student);
    }

    //#[ink::test]
    //fn it_works() {
    //    let mut school = School::new(false);
    //    assert_eq!(school.get(), false);
    //    bank.flip();
    //    assert_eq!(school.get(), true);
    //}
}
