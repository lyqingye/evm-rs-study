use alloy_primitives::{Address, U256};
use anyhow::Result;
use std::collections::HashMap;

use crate::error::EVMError;

pub trait StateDB {
    // account
    fn create_account(&mut self, address: Address);
    fn create_contract(&mut self, caller: Address, code: Vec<u8>) -> Address;
    fn transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), EVMError>;
    // balance
    fn sub_balance(&mut self, address: Address, value: U256) -> U256;
    fn add_balance(&mut self, address: Address, value: U256) -> U256;
    fn get_balance(&self, address: Address) -> U256;

    // nonce
    fn get_nonce(&self, address: Address) -> u64;
    fn set_nonce(&mut self, address: Address, nonce: u64);

    // code
    fn get_code(&self, address: Address) -> Vec<u8>;
    fn get_code_hash(&self, address: Address) -> U256;
    fn get_code_size(&self, address: Address) -> usize;
    fn exists(&self, address: Address) -> bool;

    // storage
    fn get_state(&self, address: Address, slot: U256) -> U256;
    fn set_state(&mut self, address: Address, slot: U256, value: U256);

    // state transaction
    fn prepare(&mut self);
    fn commit(&mut self);
}

pub struct InMemoryStateDB {
    accounts: HashMap<Address, Account>,
    storage: HashMap<(Address, U256), U256>,
    dirty_storage: HashMap<(Address, U256), U256>,
}

impl InMemoryStateDB {
    pub fn new() -> Self {
        InMemoryStateDB {
            accounts: HashMap::new(),
            storage: HashMap::new(),
            dirty_storage: HashMap::new(),
        }
    }
}

impl StateDB for InMemoryStateDB {
    fn create_account(&mut self, address: Address) {
        self.accounts
            .insert(address, Account::new_with_address(address));
    }

    fn create_contract(&mut self, caller: Address, code: Vec<u8>) -> Address {
        let account = self.accounts.get(&caller).unwrap();
        let nonce = account.nonce;
        let contract_address = caller.create(nonce);

        let mut contract = Account::new();
        contract.address = contract_address;
        contract.code = code;
        self.accounts.insert(contract_address, contract);
        contract_address
    }

    fn transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), EVMError> {
        match self.accounts.get_mut(&from) {
            Some(from_account) => {
                if from_account.balance < value {
                    return Err(EVMError::InsufficientBalance);
                }
                from_account.balance -= value;
            }
            None => return Err(EVMError::InsufficientBalance),
        }

        let to_account = self
            .accounts
            .entry(to)
            .or_insert(Account::new_with_address(to));
        to_account.balance += value;
        Ok(())
    }

    fn sub_balance(&mut self, address: Address, value: U256) -> U256 {
        match self.accounts.get_mut(&address) {
            Some(account) => {
                let balance = account.balance;
                account.balance -= value;
                balance
            }
            None => U256::ZERO,
        }
    }

    fn add_balance(&mut self, address: Address, value: U256) -> U256 {
        match self.accounts.get_mut(&address) {
            Some(account) => {
                let balance = account.balance;
                account.balance += value;
                balance
            }
            None => U256::ZERO,
        }
    }

    fn get_balance(&self, address: Address) -> U256 {
        match self.accounts.get(&address) {
            Some(account) => account.balance,
            None => U256::ZERO,
        }
    }

    fn get_nonce(&self, address: Address) -> u64 {
        match self.accounts.get(&address) {
            Some(account) => account.nonce,
            None => 0,
        }
    }

    fn set_nonce(&mut self, address: Address, nonce: u64) {
        let account = self.accounts.get_mut(&address).unwrap();
        account.nonce = nonce;
    }

    fn get_code(&self, address: Address) -> Vec<u8> {
        match self.accounts.get(&address) {
            Some(account) => account.code.clone(),
            None => Vec::new(),
        }
    }

    fn get_code_hash(&self, address: Address) -> U256 {
        match self.accounts.get(&address) {
            Some(account) => account.code_hash,
            None => U256::ZERO,
        }
    }

    fn get_code_size(&self, address: Address) -> usize {
        match self.accounts.get(&address) {
            Some(account) => account.code.len(),
            None => 0,
        }
    }

    fn exists(&self, address: Address) -> bool {
        self.accounts.contains_key(&address)
    }

    fn get_state(&self, address: Address, slot: U256) -> U256 {
        match self.dirty_storage.get(&(address, slot)) {
            Some(value) => value.clone(),
            None => match self.storage.get(&(address, slot)) {
                Some(value) => value.clone(),
                None => U256::ZERO,
            },
        }
    }

    fn set_state(&mut self, address: Address, slot: U256, value: U256) {
        self.dirty_storage.insert((address, slot), value);
    }

    fn prepare(&mut self) {
        self.dirty_storage.clear()
    }

    fn commit(&mut self) {
        for (slot, value) in self.dirty_storage.iter() {
            self.storage.insert(slot.clone(), value.clone());
        }
        self.dirty_storage.clear();
    }
}

#[derive(Clone)]
pub struct Account {
    pub balance: U256,
    pub nonce: u64,
    pub code: Vec<u8>,
    pub code_hash: U256,
    pub address: Address,
}

impl Account {
    pub fn new() -> Self {
        Account {
            balance: U256::ZERO,
            nonce: 0,
            code: Vec::new(),
            code_hash: U256::ZERO,
            address: Address::ZERO,
        }
    }

    pub fn new_with_address(address: Address) -> Self {
        Account {
            balance: U256::ZERO,
            nonce: 0,
            code: Vec::new(),
            code_hash: U256::ZERO,
            address,
        }
    }
}
