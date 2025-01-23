use alloy_primitives::{keccak256, Address, U256};
use anyhow::Result;
use std::collections::HashMap;

use crate::error::EVMError;

pub trait StateDB {
    // account
    fn create_object(&mut self, address: Address);
    fn create_contract(
        &mut self,
        caller: Address,
        code: Vec<u8>,
    ) -> Address;
    fn set_code(&mut self, cotnract: Address, code: Vec<u8>);

    // balance
    fn transfer(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<(), EVMError>;
    fn sub_balance(
        &mut self,
        address: Address,
        value: U256,
    ) -> Result<U256, EVMError>;
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
    fn set_state(
        &mut self,
        address: Address,
        slot: U256,
        value: U256,
    );

    fn get_transition_state(
        &self,
        address: Address,
        slot: U256,
    ) -> U256;
    fn set_transition_state(
        &mut self,
        address: Address,
        slot: U256,
        value: U256,
    );

    // state transaction
    fn prepare(&mut self);
    fn commit(&mut self);

    // log
    fn add_log(
        &mut self,
        address: Address,
        topics: Vec<U256>,
        data: Vec<u8>,
    );
}

pub struct InMemoryStateDB {
    objects: HashMap<Address, StateObject>,
    storage: HashMap<(Address, U256), U256>,

    dirty_storage: HashMap<(Address, U256), U256>,
    dirty_objects: HashMap<Address, StateObject>,
    transition_storage: HashMap<(Address, U256), U256>,
    logs: Vec<(Address, Vec<U256>, Vec<u8>)>,
}

impl InMemoryStateDB {
    pub fn new() -> Self {
        InMemoryStateDB {
            objects: HashMap::new(),
            storage: HashMap::new(),
            dirty_storage: HashMap::new(),
            dirty_objects: HashMap::new(),
            transition_storage: HashMap::new(),
            logs: Vec::new(),
        }
    }
}

impl InMemoryStateDB {
    fn get_object(&self, address: &Address) -> Option<StateObject> {
        match self.dirty_objects.get(address) {
            Some(account) => Some(account.clone()),
            None => self.objects.get(address).cloned(),
        }
    }

    fn get_object_mut(
        &mut self,
        address: &Address,
    ) -> Option<&mut StateObject> {
        match self.dirty_objects.get_mut(address) {
            Some(account) => Some(account),
            None => self.objects.get_mut(address),
        }
    }

    fn get_object_mut_or_create(
        &mut self,
        address: &Address,
    ) -> &mut StateObject {
        match self.get_object(address) {
            Some(account) => self.get_object_mut(address).unwrap(),
            None => {
                let account =
                    StateObject::new_with_address(address.clone());
                self.set_account(address.clone(), account);
                self.get_object_mut(address).unwrap()
            }
        }
    }

    fn set_account(
        &mut self,
        address: Address,
        account: StateObject,
    ) {
        self.dirty_objects.insert(address, account);
    }
}

impl StateDB for InMemoryStateDB {
    fn create_object(&mut self, address: Address) {
        self.set_account(
            address,
            StateObject::new_with_address(address),
        );
    }

    fn create_contract(
        &mut self,
        caller: Address,
        code: Vec<u8>,
    ) -> Address {
        let account = self.get_object(&caller).unwrap();
        let nonce = account.nonce;
        let contract_address = caller.create(nonce);

        let mut contract = StateObject::new();
        contract.address = contract_address;
        contract.code = code;

        self.set_account(contract_address, contract);
        contract_address
    }

    fn set_code(&mut self, cotnract: Address, code: Vec<u8>) {
        match self.get_object_mut(&cotnract) {
            Some(account) => {
                account.code_hash = keccak256(&code).into();
                account.code = code;
            }
            None => {
                self.set_account(
                    cotnract,
                    StateObject::new_with_code(cotnract, code),
                );
            }
        }
    }

    fn transfer(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<(), EVMError> {
        self.sub_balance(from, value)?;
        self.add_balance(to, value);
        Ok(())
    }

    fn sub_balance(
        &mut self,
        address: Address,
        value: U256,
    ) -> Result<U256, EVMError> {
        match self.get_object_mut(&address) {
            Some(account) => {
                let balance = account.balance;
                if balance < value {
                    return Err(EVMError::InsufficientBalance);
                }
                account.balance -= value;
                Ok(balance)
            }
            None => {
                if value.is_zero() {
                    Ok(U256::ZERO)
                } else {
                    Err(EVMError::InsufficientBalance)
                }
            }
        }
    }

    fn add_balance(&mut self, address: Address, value: U256) -> U256 {
        let account = self.get_object_mut_or_create(&address);
        let balance = account.balance;
        account.balance += value;
        balance
    }

    fn get_balance(&self, address: Address) -> U256 {
        match self.get_object(&address) {
            Some(account) => account.balance,
            None => U256::ZERO,
        }
    }

    fn get_nonce(&self, address: Address) -> u64 {
        match self.get_object(&address) {
            Some(account) => account.nonce,
            None => 0,
        }
    }

    fn set_nonce(&mut self, address: Address, nonce: u64) {
        self.get_object_mut_or_create(&address).nonce = nonce;
    }

    fn get_code(&self, address: Address) -> Vec<u8> {
        match self.get_object(&address) {
            Some(account) => account.code.clone(),
            None => Vec::new(),
        }
    }

    fn get_code_hash(&self, address: Address) -> U256 {
        match self.get_object(&address) {
            Some(account) => account.code_hash,
            None => U256::ZERO,
        }
    }

    fn get_code_size(&self, address: Address) -> usize {
        match self.get_object(&address) {
            Some(account) => account.code.len(),
            None => 0,
        }
    }

    fn exists(&self, address: Address) -> bool {
        self.get_object(&address).is_some()
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

    fn set_state(
        &mut self,
        address: Address,
        slot: U256,
        value: U256,
    ) {
        self.dirty_storage.insert((address, slot), value);
    }

    fn prepare(&mut self) {
        self.dirty_storage.clear();
        self.dirty_objects.clear();
        self.transition_storage.clear();
    }

    fn commit(&mut self) {
        for (slot, value) in self.dirty_storage.iter() {
            self.storage.insert(slot.clone(), value.clone());
        }
        self.dirty_storage.clear();
        for (address, account) in self.dirty_objects.iter() {
            self.objects
                .insert(address.clone(), account.clone());
        }
        self.dirty_objects.clear();
        self.transition_storage.clear();
    }

    fn add_log(
        &mut self,
        address: Address,
        topics: Vec<U256>,
        data: Vec<u8>,
    ) {
        self.logs.push((address, topics, data));
    }

    fn get_transition_state(
        &self,
        address: Address,
        slot: U256,
    ) -> U256 {
        match self.transition_storage.get(&(address, slot)) {
            Some(value) => value.clone(),
            None => U256::ZERO,
        }
    }

    fn set_transition_state(
        &mut self,
        address: Address,
        slot: U256,
        value: U256,
    ) {
        self.transition_storage
            .insert((address, slot), value);
    }
}

#[derive(Clone)]
pub struct StateObject {
    pub balance: U256,
    pub nonce: u64,
    pub code: Vec<u8>,
    pub code_hash: U256,
    pub address: Address,
}

impl StateObject {
    pub fn new() -> Self {
        StateObject {
            balance: U256::ZERO,
            nonce: 0,
            code: Vec::new(),
            code_hash: U256::ZERO,
            address: Address::ZERO,
        }
    }

    pub fn new_with_address(address: Address) -> Self {
        StateObject {
            balance: U256::ZERO,
            nonce: 0,
            code: Vec::new(),
            code_hash: U256::ZERO,
            address,
        }
    }

    pub fn new_with_code(address: Address, code: Vec<u8>) -> Self {
        let code_hash = keccak256(&code).into();
        StateObject {
            balance: U256::ZERO,
            nonce: 0,
            code,
            code_hash,
            address,
        }
    }
}
