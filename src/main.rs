#![allow(unused)]

use alloy_primitives::{Address, U256};
use context::BlockContext;
use state::{InMemoryStateDB, StateDB};
use vm::Interpreter;
mod asm;
mod context;
mod error;
mod i256;
mod instructions;
mod mem;
mod opcode;
mod opcode_table;
mod stack;
mod state;
mod u256;
mod vm;

fn main() {
    let assembler = asm::Assembler::new();
    let code = assembler
        .asm(
            r#"
        PUSH17 0x67600035600757FE5B60005260086018F3
        PUSH1 0
        MSTORE
        PUSH1 0x11
        PUSH1 0xF
        PUSH1 0
        CREATE
        PUSH1 0
        PUSH1 0
        PUSH1 0
        PUSH1 0
        PUSH1 0
        DUP6
        PUSH2 0xFFFF
        CALL
        STOP
    "#,
        )
        .unwrap();
    println!("{}", hex::encode(code.clone()));

    let mut state = InMemoryStateDB::new();
    let caller = Address::ZERO;
    state.create_object(caller);
    let contract_address = state.create_contract(caller, code);
    let blk_ctx = BlockContext::new();
    let mut vm = Interpreter::new(Box::new(state), &blk_ctx);
    vm.run(caller, caller, contract_address, vec![], U256::ZERO)
        .unwrap();
}
