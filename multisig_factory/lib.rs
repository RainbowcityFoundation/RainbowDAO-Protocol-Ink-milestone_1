#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod multisig_factory {
    use multisig::Multisig;
    use alloc::string::String;
    use ink_prelude::vec::Vec;
    use ink_prelude::collections::BTreeMap;
    use ink_storage::{collections::HashMap as StorageHashMap, };
    const CONTRACT_INIT_BALANCE: u128 = 1000 * 1_000_000_000_000;

    /// Generate a multi sign management contract
    /// multisign:HashMap of multisign index and addess
    /// index:The multisign index
    /// user_multisign:The user managed contracts
    #[ink(storage)]
    pub struct MultisigFactory {
        /// Stores a single `bool` value on the storage.
        multisign:StorageHashMap<u64,AccountId>,
        index:u64,
        user_multisign:StorageHashMap<AccountId,Vec<AccountId>>,
    }

    impl MultisigFactory {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                multisign:StorageHashMap::new(),
                index:0,
                user_multisign:StorageHashMap::new()
            }
        }
        /// Generate a multi sign management contract
        /// multisig_hash:the hash of multisig contract
        /// owners:the managers of multisig
        /// min_sign_count:Minimum number of signatures
        #[ink(message)]
        pub fn new_multisig(
            &mut self,
            multisig_hash:Hash,
            owners: Vec<AccountId>,
            min_sign_count: i32,
        ) -> AccountId {
            let version =  self.index;
            let salt = version.to_le_bytes();
            let instance_params = Multisig::new(owners.clone(),min_sign_count)
                .endowment(CONTRACT_INIT_BALANCE)
                .code_hash(multisig_hash)
                .salt_bytes(salt)
                .params();
            let init_result = ink_env::instantiate_contract(&instance_params);
            let contract_addr = init_result.expect("failed at instantiating the `multisig` contract");
            assert_eq!(self.index + 1 > self.index, true);
            self.multisign.insert(self.index, contract_addr);
            self.index += 1;
            for i in &owners {
                let user_mul = self.user_multisign.entry(i.clone()).or_insert(Vec::new());
                user_mul.push(contract_addr);
            }
            contract_addr
        }

        /// Get the contract managed by a user
        /// user: the address of user
        #[ink(message)]
        pub fn user_multisig(&self,user:AccountId) -> Vec<AccountId> {
            let list =  self.user_multisign.get(&user).unwrap().clone();
            list
        }
    }
}
