#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;
pub use self::route_manage::{
    RouteManage,
};

#[ink::contract]
mod route_manage {

    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };

    /// Manage the route of the rainbow protocol
    /// owner : the manager of contract
    /// index : the index of all route
    /// route_map:HashMap of route name and address
    #[ink(storage)]
    pub struct RouteManage {
        owner:AccountId,
        index:u64,
        route_map:StorageHashMap<String,AccountId>,
    }

    impl RouteManage {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner:Self::env().caller(),
                index: 0,
                route_map : StorageHashMap::new(),
            }
        }
        fn only_core(&self,sender:AccountId) {
            assert_eq!(self.owner, sender);
        }

        /// Add a route
        /// name : the name of route
        /// value : the address of route
        /// # Panics
        /// Only core contracts can be called
        #[ink(message)]
        pub fn add_route(&mut self, name: String,value:AccountId) -> bool {
            self.only_core(Self::env().caller());
            assert_eq!(self.index + 1 > self.index, true);
            self.route_map.insert(name,value);
            self.index += 1;
            true
        }
        /// Show all route
        #[ink(message)]
        pub fn list_route(&self) -> BTreeMap<String,AccountId> {
            // let mut route_vec = Vec::new();
            let mut iter = self.route_map.values();
            let mut key_iter = self.route_map.keys();
            let mut route = iter.next();
            let mut name = key_iter.next();
            let mut hash_map = BTreeMap::new();
            while route.is_some() {
                hash_map.insert(name.unwrap().clone(),route.unwrap().clone());
                // route_vec.push(route.unwrap().clone());
                route = iter.next();
                name = key_iter.next();
            }
            hash_map
        }
        /// Get a route by name
        /// name:the name of route
        #[ink(message)]
        pub fn query_route_by_name(&self, name: String) -> AccountId {
            self.route_map.get(&name).copied().unwrap_or(AccountId::default())
        }
        /// Change routing address
        /// name : the name of route
        /// value : the address of route
        /// # Panics
        /// Only core contracts can be called
        #[ink(message)]
        pub fn change_route(&mut self,name:String,value:AccountId) -> bool {
            self.only_core(Self::env().caller());
            self.route_map[&name] = value;
            true
        }
    }


    #[cfg(test)]
    mod tests {


        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;


        #[ink::test]
        fn add_route_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut route_manage = RouteManage::new();
            route_manage.add_route(String::from("test"),accounts.alice);
            assert!(route_manage.query_route_by_name(String::from("test")) == accounts.alice);

        }
        #[ink::test]
        fn change_route_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");
            let mut route_manage = RouteManage::new();
            route_manage.add_route(String::from("test"),accounts.alice);
            route_manage.change_route(String::from("test"),accounts.bob);
            assert!(route_manage.query_route_by_name(String::from("test")) == accounts.bob);
        }
    }
}
