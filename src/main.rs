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
    // let assembler = asm::Assembler::new();
    // let code = assembler
    //     .asm(
    //         r#"
    //     PUSH17 0x67600035600757FE5B60005260086018F3
    //     PUSH1 0
    //     MSTORE
    //     PUSH1 0x11
    //     PUSH1 0xF
    //     PUSH1 0
    //     CREATE
    //     PUSH1 0
    //     PUSH1 0
    //     PUSH1 0
    //     PUSH1 0
    //     PUSH1 0
    //     DUP6
    //     PUSH2 0xFFFF
    //     CALL
    //     STOP
    // "#,
    //     )
    //     .unwrap();
    // println!("{}", hex::encode(code.clone()));

    let code = hex::decode("608060405234801561000f575f80fd5b506004361061003f575f3560e01c80633fb5c1cb146100435780638381f58a1461005f578063f2c9ecd81461007d575b5f80fd5b61005d60048036038101906100589190610102565b61009b565b005b6100676100b1565b604051610074919061013c565b60405180910390f35b6100856100b6565b604051610092919061013c565b60405180910390f35b6103e8816100a99190610182565b5f8190555050565b5f5481565b5f6103e85f546100c69190610182565b905090565b5f80fd5b5f819050919050565b6100e1816100cf565b81146100eb575f80fd5b50565b5f813590506100fc816100d8565b92915050565b5f60208284031215610117576101166100cb565b5b5f610124848285016100ee565b91505092915050565b610136816100cf565b82525050565b5f60208201905061014f5f83018461012d565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61018c826100cf565b9150610197836100cf565b92508282019050808211156101af576101ae610155565b5b9291505056fea264697066735822122069e2c9c9514348b945579e77ad83ba17fabb3e7941e621d9e563c526c1ec099f64736f6c63430008150033").unwrap();
    let args = vec![];


    let mut state = InMemoryStateDB::new();
    let caller = Address::ZERO;
    state.create_object(caller);
    let contract_address = state.create_contract(caller, code);
    let blk_ctx = BlockContext::new();
    let mut vm = Interpreter::new(Box::new(state), &blk_ctx);
    vm.run(caller, caller, contract_address, args, U256::ZERO)
        .unwrap();
}
