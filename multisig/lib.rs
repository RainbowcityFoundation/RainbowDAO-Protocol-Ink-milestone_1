#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
pub use self::multisig::{
    Multisig,
};
use ink_lang as ink;

#[ink::contract]
mod multisig {
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            Vec as StorageVec,
        },
        traits::{
            PackedLayout,
            SpreadLayout,
        },
    };

    /// Execution details
    /// id:the id of multisig
    /// status:the status of multisig
    /// to:Transfer token to an address
    /// amount:Number of transfers
    /// signature_count:Number of signatures
    /// signatures:Details of signature
    #[derive(scale::Encode, scale::Decode, Clone, SpreadLayout, PackedLayout)]
    #[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout)
    )]
    #[derive(Debug)]
    pub struct Transaction {
        id:u64,
        status: bool,
        to: AccountId,
        amount: u64,
        signature_count: i32,
        signatures: BTreeMap<AccountId, i32>,
    }

    /// Sign multiple transfer contracts
    /// owner : the creator of the contract
    /// transaction_idx : the index of transaction
    /// manager:Administrator who needs to sign
    /// transactions:Execution details
    /// min_sign_count : Minimum number of signatures
    #[ink(storage)]
    pub struct Multisig {
        owner: AccountId,
        transaction_idx: u64,
        manager: StorageHashMap<AccountId, i32>,
        transactions: StorageHashMap<u64, Transaction>,
        info: StorageHashMap<u64, AccountId>,
        min_sign_count: i32,
    }



    impl Multisig {
        #[ink(constructor)]
        pub fn new(owners: Vec<AccountId>,min_sign_count: i32,) -> Self {
            let mut map: StorageHashMap<AccountId, i32> = StorageHashMap::new();
            for addr in &owners{
                    map.insert(*addr,1);
                }
            Self {
                owner: Self::env().caller(),
                transaction_idx: 0,
                manager: map,
                transactions: StorageHashMap::new(),
                info: StorageHashMap::new(),
                min_sign_count,
            }
        }

        /// Create a multi sign transaction
        /// to:Transfer token to an address
        /// amount:The number of transfer
        #[ink(message)]
        pub fn creat_transfer(&mut self,to: AccountId ,amount: u64) -> bool {
            self.ensure_caller_is_manager();
            let from = self.env().caller();
            assert_eq!(self.env().balance() >= amount.into(), true);
            self.transactions.insert(self.transaction_idx,
                Transaction{
                    id:self.transaction_idx,
                    status: false,
                    to,
                    amount,
                    signature_count: 0,
                    signatures: BTreeMap::new(),
                }
            );
            self.transaction_idx += 1;
            true
        }
        /// Sign a transaction
        /// transaction_id:the id of transaction
        #[ink(message)]
        pub fn sign_transaction(&mut self, transaction_id: u64) -> bool {
            self.ensure_caller_is_manager();
            let from = self.env().caller();
            let mut t = self.transactions.get_mut(&transaction_id).unwrap();
            assert!(t.status == false, "out!");
            let if_sign = t.signatures.get(&from);
            assert!(if_sign == None, "out!");
            t.signatures.insert(from, 1);
            t.signature_count += 1;
            let addr = t.to;
            let num = t.amount;
            if t.signature_count >= self.min_sign_count {
                t.status = true;
                self.env().transfer(addr, num.into());
            }
            true
        }

        /// Get a transaction
        /// trans_id:the id of transaction
        #[ink(message)]
        pub fn get_transaction(&self,trans_id: u64) -> Transaction {
            self.transactions.get(&trans_id).unwrap().clone()
        }
        /// Add a multi sign on administrator
        /// addr:the address of manager
        #[ink(message)]
        pub fn add_manage(&mut self,addr: AccountId) -> bool {
            self.ensure_caller_is_owner();
            self.manager.insert(addr, 1);
            true
        }
        /// Remove a multi sign on administrator
        /// addr:the address of manager
        #[ink(message)]
        pub fn remove_manage(&mut self,addr: AccountId) -> bool {
            self.ensure_caller_is_owner();
            self.manager.insert(addr, 0);
            true
        }
        /// Get administrator list
        #[ink(message)]
        pub fn get_manage_list(&self) -> Vec<AccountId> {
            let mut manager_list = Vec::new();
            let mut iter = self.manager.keys();
            let mut role = iter.next();
            while role.is_some() {
                manager_list.push(role.unwrap().clone());
                role = iter.next();
            }
            manager_list
        }
        /// Get multi sign transaction list
        #[ink(message)]
        pub fn get_sign_list(&self) -> Vec<Transaction> {
            let mut sign_list = Vec::new();
            let mut iter = self.transactions.values();
            let mut sign = iter.next();
            while sign.is_some() {
                sign_list.push(sign.unwrap().clone());
                sign = iter.next();
            }
            sign_list
        }
        fn ensure_caller_is_owner(&self) -> bool{
            self.owner == self.env().caller()
        }

        fn ensure_caller_is_manager(&self) -> bool {
            let caller = self.env().caller();
            self.manager.get(&caller) == Some(&1) || self.owner == caller
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
            let mut account_vec = Vec::new();
            account_vec.push(accounts.alice);
            account_vec.push(accounts.bob);
            account_vec.push(accounts.eve);
            let mut multisig = Multisig::new(account_vec,2);
            //multisig.creat_transfer(accounts.bob,2);
            assert!(multisig.add_manage(accounts.alice) == true);
        }
    }
}
