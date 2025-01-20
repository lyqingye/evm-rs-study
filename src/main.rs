#![allow(unused)]

use alloy_primitives::{Address, U256};
use context::BlockContext;
use state::{InMemoryStateDB, StateDB};
use vm::Interpreter;
mod context;
mod error;
mod i256;
mod instructions;
mod mem;
mod opcode;
mod opcode_table;
mod stack;
mod state;
mod vm;

pub struct OpcodeInst {
    pub opcode: u8,
    pub name: &'static str,
    pub description: &'static str,
}

fn main() {
    let hex_string = "6080604052600436106025575f3560e01c80639134cbd7146029578063cf3155d4146031575b5f80fd5b602f6039565b005b60376046565b005b63ad3a8b9e5f526004601cfd5b6040517fad3a8b9e00000000000000000000000000000000000000000000000000000000815260040160405180910390fdfea2646970667358221220f5afa90c41e2fb89cbb1bca5dfc7198bcd8e62f14ff00f7a2563c0d4158c623164736f6c63430008150033";
    let code = hex::decode(hex_string).unwrap();

    let mut state = InMemoryStateDB::new();
    let caller = Address::ZERO;
    state.create_account(caller);
    let contract_address = state.create_contract(caller, code);
    let mut vm = Interpreter::new(Box::new(state), BlockContext::new());
    vm.run(caller, caller, contract_address, vec![], U256::ZERO)
        .unwrap();
}
