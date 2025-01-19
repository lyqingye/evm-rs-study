use crate::i256::{i256_cmp, i256_div, i256_mod};
use crate::state::{Account, State};
use crate::{mem::Memory, stack::Stack};
use alloy_primitives::{Address, FixedBytes, U256};
use core::panic;
use std::cmp::{min, Ordering};

pub struct Context {
    pub stack: Stack,
    pub memory: Memory,
    pub pc: usize,
    pub caller: Address,
    pub origin: Address,
    pub contract: Address,
    pub code: Vec<u8>,
    pub call_data: Vec<u8>,
    pub return_data: Vec<u8>,
    pub value: U256,
}

impl Context {
    pub fn new() -> Self {
        Context {
            stack: Stack::new(),
            memory: Memory::new(),
            pc: 0,
            caller: Address::ZERO,
            origin: Address::ZERO,
            contract: Address::ZERO,
            code: Vec::new(),
            call_data: Vec::new(),
            return_data: Vec::new(),
            value: U256::ZERO,
        }
    }
}

pub struct BlockContext {
    pub chain_id: U256,
    pub block_number: U256,
    pub block_timestamp: U256,
    pub block_coinbase: U256,
    pub block_difficulty: U256,
    pub block_gas_limit: U256,
    pub block_base_fee: U256,
    pub gas_price: U256,
    pub base_fee: U256,
}

impl BlockContext {
    pub fn new() -> Self {
        BlockContext {
            block_number: U256::ZERO,
            block_timestamp: U256::ZERO,
            block_coinbase: U256::ZERO,
            block_difficulty: U256::ZERO,
            block_gas_limit: U256::ZERO,
            block_base_fee: U256::ZERO,
            gas_price: U256::ZERO,
            base_fee: U256::ZERO,
            chain_id: U256::ZERO,
        }
    }
}
