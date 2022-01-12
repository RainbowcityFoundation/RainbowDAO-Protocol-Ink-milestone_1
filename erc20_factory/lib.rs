#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod erc20_factory {
    use erc20::Erc20;
    use income_category::IncomeCategory;
    use route_manage::RouteManage;
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::{collections::HashMap as StorageHashMap, };
    const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

    /// Erc20Factory contract of rainbow protocol
    /// #Fields
    /// route_addr:The address  of route contract
    /// length:Erc20 index
    /// token_list:HashMap of Erc20 index and address
    #[ink(storage)]
    pub struct Erc20Factory {
        route_addr:AccountId,
        length:u128,
        token_list:StorageHashMap<u128,AccountId>
    }

    impl Erc20Factory {
        #[ink(constructor)]
        pub fn new(route_addr:AccountId) -> Self {
            Self {
                route_addr,
                length:0,
                token_list:StorageHashMap::new()
            }
        }
       /// Generate a new erc20 token
       /// #Fields
       /// erc20_code_hash:The hash  of erc20 contract
       /// initial_supply:Total supply
       /// name:the name of token
       /// symbol:the symbol of token
       /// decimals:the decimals of token
       /// owner:the manager of token
        #[ink(message)]
        pub fn new_erc20(
            &mut self,
            erc20_code_hash:Hash,
            initial_supply: Balance,
            name:String,
            symbol:String,
            decimals:u8,
            owner:AccountId
        ) -> AccountId {
            let version =  self.length;
            let salt = version.to_le_bytes();
            let instance_params = Erc20::new(initial_supply,name,symbol,decimals,owner)
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(erc20_code_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `Erc20` contract");
            let income_category_addr =  self.get_contract_addr(String::from("income_category"));
            if income_category_addr != AccountId::default()  {
                self.send_income_fee(income_category_addr);
            }
            self.token_list.insert(self.length,contract_addr);
            self.length+=1;
            contract_addr
        }
        /// Get the number of erc20
        #[ink(message)]
        pub fn get_length(&self) -> u128 {
            self.length
        }
        /// Get the erc20 by index
        #[ink(message)]
        pub fn get_token_by_index(&self,index:u128) -> AccountId {
            self.token_list.get(&index).copied().unwrap_or(AccountId::default())
        }
        /// Show all tokens
        #[ink(message)]
        pub fn list_token(&self) -> Vec<AccountId> {
            let mut token_vec = Vec::new();
            let mut iter = self.token_list.values();
            let mut token = iter.next();
            while token.is_some() {
                token_vec.push(token.unwrap().clone());
                token = iter.next();
            }
            token_vec
        }
        fn send_income_fee(&mut self,income_category_addr:AccountId) -> bool {
            let mut income_instance: IncomeCategory = ink_env::call::FromAccountId::from_account_id(income_category_addr);
            let category =  income_instance.get_category(String::from("erc20"));
            if category.is_used {
                self.get_fee_from_user(category.token,category.fee,income_category_addr);
            }
            true
        }

        fn get_fee_from_user(&mut self,token_account:AccountId,fee:u128,to_account:AccountId) -> bool {
            let mut erc20_instance: Erc20 = ink_env::call::FromAccountId::from_account_id(token_account);
            erc20_instance.transfer_from(Self::env().caller(),to_account,fee);
            true
        }
        /// Get the address of a contract
        #[ink(message)]
        pub fn get_contract_addr(&self,target_name:String) ->AccountId {
            let route_instance: RouteManage = ink_env::call::FromAccountId::from_account_id(self.route_addr);
            return route_instance.query_route_by_name(target_name);
        }
    }
}
