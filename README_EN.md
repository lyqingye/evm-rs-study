# EVM Disassembler and Interpreter

[中文](README.md) | [English](README_EN.md)

### Introduction

This project is an EVM (Ethereum Virtual Machine) implementation written in Rust, designed for learning and understanding the inner workings of the Ethereum Virtual Machine. It includes a disassembler and interpreter that can execute EVM bytecode.

### Features

- EVM bytecode interpreter
- Assembly code support
- In-memory state management
- Basic block context simulation
- Contract creation and execution

### Example

Here's a simple example of how to use the interpreter:

```rust
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
```

In this example, we demonstrate how to:

1. Create an assembler instance
2. Compile EVM assembly code
3. Initialize state database
4. Create contract
5. Set up block context
6. Run EVM interpreter to execute code

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

