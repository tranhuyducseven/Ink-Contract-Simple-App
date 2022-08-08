#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod school {
    use ink_storage::traits::SpreadAllocate;
    enum Role {
        Admin,
        Student,
    }
    pub struct Student {
        name: String,
        age: i32,
        role: Role,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct School {
        list_students: ink_storage::Mapping<AccountId, Student>,
    }

    impl School {
        #[ink(constructor)]
        pub fn new(name: String, age: i32) -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                let admin = Student {
                    name,
                    age,
                    role: Role::Admin,
                };

                contract.list_students.insert(&caller, &admin);
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
        pub fn get(&self) -> Student {
            let caller = Self::env().caller();
            self.list_students.get(&caller)
        }
    }
}

// mod tests {
//     /// Imports all the definitions from the outer scope so we can use them here.
//     use super::*;
//
//     /// Imports `ink_lang` so we can use `#[ink::test]`.
//     use ink_lang as ink;
//
//     /// We test if the default constructor does its job.
//     #[ink::test]
//     fn default_works() {
//         let school = School::default();
//         assert_eq!(bank.get(), false);
//     }
//
//     /// We test a simple use case of our contract.
//     #[ink::test]
//     fn it_works() {
//         let mut school = School::new(false);
//         assert_eq!(school.get(), false);
//         bank.flip();
//         assert_eq!(school.get(), true);
//     }
// }
