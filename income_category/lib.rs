#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::income_category::{
    IncomeCategory
};
use ink_lang as ink;

#[ink::contract]
mod income_category {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{
            PackedLayout,
            SpreadLayout,
        }
    };
    /// Charging information
    /// is_used:it is open
    /// fee:the fee of this category
    /// token:Which token is used for charging
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]

    #[derive(Debug)]
    pub struct IncomeInfo {
       pub is_used:bool,
       pub fee: u128,
       pub token: AccountId
    }
    /// All charging types of rainbow agreement are set here
    /// owner:The manager of this contract
    /// category:HashMap of category name and detail
    #[ink(storage)]
    pub struct IncomeCategory {
        owner:AccountId,
        category:StorageHashMap<String, IncomeInfo>,
    }

    impl IncomeCategory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(owner:AccountId) -> Self {
            Self {
                owner,
                category: StorageHashMap::new()
            }
        }
        ///Add a category
        /// name:category's name
        /// income:category's info
        #[ink(message)]
        #[ink(selector = 0xDEADBEEF)]
        pub fn save_category(&mut self,name:String,income:IncomeInfo) -> bool {
            self.only_owner(Self::env().caller());
            self.category.insert(name,income);
            true
        }

        ///Get a category by name
        /// name:category's name
        #[ink(message)]
        pub fn get_category(&mut self,name:String) -> IncomeInfo {
           self.category.get(&name).unwrap().clone()
        }

        /// Change contract administrator
        /// new_owner:the address of new owner
        #[ink(message)]
        pub fn transfer_owner(&mut self,new_owner:AccountId) -> bool {
            self.only_owner(Self::env().caller());
            self.owner = new_owner;
            true
        }


        /// Show all category
        #[ink(message)]
        pub fn list_category(&self) -> BTreeMap<String,IncomeInfo> {
            // let mut route_vec = Vec::new();
            let mut iter = self.category.values();
            let mut key_iter = self.category.keys();
            let mut category = iter.next();
            let mut name = key_iter.next();
            let mut hash_map = BTreeMap::new();
            while category.is_some() {
                hash_map.insert(name.unwrap().clone(),category.unwrap().clone());
                // route_vec.push(route.unwrap().clone());
                category = iter.next();
                name = key_iter.next();
            }
            hash_map
        }


        fn only_owner(&self,sender:AccountId) {
            assert_eq!(self.owner, sender);
        }
    }
    #[cfg(test)]
    mod tests {


        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;


        #[ink::test]
        fn init_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut income_category = IncomeCategory::new(accounts.alice);
            income_category.save_category(String::from("test"),IncomeInfo{is_used:false,fee:1,token:AccountId::from([0x01; 32])});
            let income:IncomeInfo =income_category.get_category(String::from("test"));
            assert!(income.fee == 1);
        }
    }
}
