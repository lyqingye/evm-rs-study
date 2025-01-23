use crate::opcode::*;
use crate::{
    context::{BlockContext, Context},
    error::EVMError,
    instructions::*,
    state::StateDB,
};
use anyhow::Result;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub type InstFn = fn(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError>;

macro_rules! inst {
    ($opcode:expr, $name:expr, $desc:expr, $func:expr) => {
        ($opcode, $name, $desc, $func as InstFn)
    };
}

pub static OPCODE_TABLE: Lazy<HashMap<u8, (&str, &str, InstFn)>> =
    Lazy::new(|| {
        let mut opcode_table = HashMap::new();
        let instructions = [
            inst!(STOP, "STOP", "Halts execution", stop),
            inst!(ADD, "ADD", "Adds two numbers", add),
            inst!(MUL, "MUL", "Multiplies two numbers", mul),
            inst!(SUB, "SUB", "Subtracts two numbers", sub),
            inst!(DIV, "DIV", "Divides two numbers", div),
            inst!(SDIV, "SDIV", "Divides two numbers", sign_div),
            inst!(MOD, "MOD", "Modulo two numbers", modulo),
            inst!(SMOD, "SMOD", "Modulo two numbers", sign_modulo),
            inst!(
                ADDMOD,
                "ADDMOD",
                "Add modulo two numbers",
                add_mod
            ),
            inst!(
                MULMOD,
                "MULMOD",
                "Multiply modulo two numbers",
                mul_mod
            ),
            inst!(EXP, "EXP", "Exponentiation", exp),
            inst!(
                SIGNEXTEND,
                "SIGNEXTEND",
                "Sign extend",
                sign_extend
            ),
            inst!(LT, "LT", "Less than", lt),
            inst!(GT, "GT", "Greater than", gt),
            inst!(SLT, "SLT", "Signed less than", slt),
            inst!(SGT, "SGT", "Signed greater than", sgt),
            inst!(EQ, "EQ", "Equal", eq),
            inst!(ISZERO, "ISZERO", "Is zero", is_zero),
            inst!(AND, "AND", "Bitwise AND", and),
            inst!(OR, "OR", "Bitwise OR", or),
            inst!(XOR, "XOR", "Bitwise XOR", xor),
            inst!(NOT, "NOT", "Bitwise NOT", not),
            inst!(BYTE, "BYTE", "Get byte", byte),
            inst!(SHL, "SHL", "Shift left", shl),
            inst!(SHR, "SHR", "Shift right", shr),
            inst!(SAR, "SAR", "Shift right arithmetic", sar),
            inst!(KECCAK256, "KECCAK256", "Keccak256", keccak256),
            inst!(ADDRESS, "ADDRESS", "Address", address),
            inst!(BALANCE, "BALANCE", "Balance", balance),
            inst!(ORIGIN, "ORIGIN", "Origin", origin),
            inst!(CALLER, "CALLER", "Caller", caller),
            inst!(CALLVALUE, "CALLVALUE", "Call value", call_value),
            inst!(
                CALLDATALOAD,
                "CALLDATALOAD",
                "Call data load",
                call_data_load
            ),
            inst!(
                CALLDATASIZE,
                "CALLDATASIZE",
                "Call data size",
                call_data_size
            ),
            inst!(
                CALLDATACOPY,
                "CALLDATACOPY",
                "Call data copy",
                call_data_copy
            ),
            inst!(CODESIZE, "CODESIZE", "Code size", code_size),
            inst!(CODECOPY, "CODECOPY", "Code copy", code_copy),
            inst!(
                EXTCODESIZE,
                "EXTCODESIZE",
                "Ext code size",
                ext_code_size
            ),
            inst!(
                EXTCODECOPY,
                "EXTCODECOPY",
                "Ext code copy",
                ext_code_copy
            ),
            inst!(
                RETURNDATASIZE,
                "RETURNDATASIZE",
                "Return data size",
                return_data_size
            ),
            inst!(
                RETURNDATACOPY,
                "RETURNDATACOPY",
                "Return data copy",
                return_data_copy
            ),
            inst!(
                EXTCODEHASH,
                "EXTCODEHASH",
                "Ext code hash",
                ext_code_hash
            ),
            inst!(BLOCKHASH, "BLOCKHASH", "Block hash", block_hash),
            inst!(COINBASE, "COINBASE", "Coinbase", coinbase),
            inst!(TIMESTAMP, "TIMESTAMP", "Timestamp", timestamp),
            inst!(NUMBER, "NUMBER", "Number", block_number),
            inst!(CHAINID, "CHAINID", "Chain ID", chain_id),
            inst!(DIFFICULTY, "DIFFICULTY", "Difficulty", difficulty),
            inst!(GAS, "GAS", "Gas", gas),
            inst!(GASLIMIT, "GASLIMIT", "Gas limit", gas_limit),
            inst!(GASPRICE, "GASPRICE", "Gas price", gas_price),
            inst!(
                SELFBALANCE,
                "SELFBALANCE",
                "Self balance",
                self_balance
            ),
            inst!(BASEFEE, "BASEFEE", "Base fee", base_fee),
            inst!(BLOBHASH, "BLOBHASH", "Blob hash", blob_hash),
            inst!(
                BLOBHASHFEE,
                "BLOBHASHFEE",
                "Blob hash fee",
                blob_hash_fee
            ),
            inst!(POP, "POP", "Pop", pop),
            inst!(MLOAD, "MLOAD", "Memory load", mload),
            inst!(MSTORE, "MSTORE", "Memory store", mstore),
            inst!(MSTORE8, "MSTORE8", "Memory store 8", mstore8),
            inst!(SLOAD, "SLOAD", "Storage load", sload),
            inst!(SSTORE, "SSTORE", "Storage store", sstore),
            inst!(JUMP, "JUMP", "Jump", jump),
            inst!(JUMPI, "JUMPI", "Jump if", jumpi),
            inst!(PC, "PC", "Program counter", pc),
            inst!(MSIZE, "MSIZE", "Memory size", msize),
            inst!(
                JUMPDEST,
                "JUMPDEST",
                "Jump destination",
                jump_dest
            ),
            inst!(TLOAD, "TLOAD", "Tload", tload),
            inst!(TSTORE, "TSTORE", "Tstore", tstore),
            inst!(MCOPY, "MCOPY", "Memory copy", mcopy),
            inst!(PUSH0, "PUSH0", "Push 0", push0),
            inst!(PUSH1, "PUSH1", "Push 1", push::<1>),
            inst!(PUSH2, "PUSH2", "Push 2", push::<2>),
            inst!(PUSH3, "PUSH3", "Push 3", push::<3>),
            inst!(PUSH4, "PUSH4", "Push 4", push::<4>),
            inst!(PUSH5, "PUSH5", "Push 5", push::<5>),
            inst!(PUSH6, "PUSH6", "Push 6", push::<6>),
            inst!(PUSH7, "PUSH7", "Push 7", push::<7>),
            inst!(PUSH8, "PUSH8", "Push 8", push::<8>),
            inst!(PUSH9, "PUSH9", "Push 9", push::<9>),
            inst!(PUSH10, "PUSH10", "Push 10", push::<10>),
            inst!(PUSH11, "PUSH11", "Push 11", push::<11>),
            inst!(PUSH12, "PUSH12", "Push 12", push::<12>),
            inst!(PUSH13, "PUSH13", "Push 13", push::<13>),
            inst!(PUSH14, "PUSH14", "Push 14", push::<14>),
            inst!(PUSH15, "PUSH15", "Push 15", push::<15>),
            inst!(PUSH16, "PUSH16", "Push 16", push::<16>),
            inst!(PUSH17, "PUSH17", "Push 17", push::<17>),
            inst!(PUSH18, "PUSH18", "Push 18", push::<18>),
            inst!(PUSH19, "PUSH19", "Push 19", push::<19>),
            inst!(PUSH20, "PUSH20", "Push 20", push::<20>),
            inst!(PUSH21, "PUSH21", "Push 21", push::<21>),
            inst!(PUSH22, "PUSH22", "Push 22", push::<22>),
            inst!(PUSH23, "PUSH23", "Push 23", push::<23>),
            inst!(PUSH24, "PUSH24", "Push 24", push::<24>),
            inst!(PUSH25, "PUSH25", "Push 25", push::<25>),
            inst!(PUSH26, "PUSH26", "Push 26", push::<26>),
            inst!(PUSH27, "PUSH27", "Push 27", push::<27>),
            inst!(PUSH28, "PUSH28", "Push 28", push::<28>),
            inst!(PUSH29, "PUSH29", "Push 29", push::<29>),
            inst!(PUSH30, "PUSH30", "Push 30", push::<30>),
            inst!(PUSH31, "PUSH31", "Push 31", push::<31>),
            inst!(PUSH32, "PUSH32", "Push 32", push::<32>),
            inst!(DUP1, "DUP1", "Duplicate top stack item", dup::<1>),
            inst!(DUP2, "DUP2", "Duplicate top stack item", dup::<2>),
            inst!(DUP3, "DUP3", "Duplicate top stack item", dup::<3>),
            inst!(DUP4, "DUP4", "Duplicate top stack item", dup::<4>),
            inst!(DUP5, "DUP5", "Duplicate top stack item", dup::<5>),
            inst!(DUP6, "DUP6", "Duplicate top stack item", dup::<6>),
            inst!(DUP7, "DUP7", "Duplicate top stack item", dup::<7>),
            inst!(DUP8, "DUP8", "Duplicate top stack item", dup::<8>),
            inst!(DUP9, "DUP9", "Duplicate top stack item", dup::<9>),
            inst!(
                DUP10,
                "DUP10",
                "Duplicate top stack item",
                dup::<10>
            ),
            inst!(
                DUP11,
                "DUP11",
                "Duplicate top stack item",
                dup::<11>
            ),
            inst!(
                DUP12,
                "DUP12",
                "Duplicate top stack item",
                dup::<12>
            ),
            inst!(
                DUP13,
                "DUP13",
                "Duplicate top stack item",
                dup::<13>
            ),
            inst!(
                DUP14,
                "DUP14",
                "Duplicate top stack item",
                dup::<14>
            ),
            inst!(
                DUP15,
                "DUP15",
                "Duplicate top stack item",
                dup::<15>
            ),
            inst!(
                DUP16,
                "DUP16",
                "Duplicate top stack item",
                dup::<16>
            ),
            inst!(SWAP1, "SWAP1", "Swap top stack item", swap::<1>),
            inst!(SWAP2, "SWAP2", "Swap top stack item", swap::<2>),
            inst!(SWAP3, "SWAP3", "Swap top stack item", swap::<3>),
            inst!(SWAP4, "SWAP4", "Swap top stack item", swap::<4>),
            inst!(SWAP5, "SWAP5", "Swap top stack item", swap::<5>),
            inst!(SWAP6, "SWAP6", "Swap top stack item", swap::<6>),
            inst!(SWAP7, "SWAP7", "Swap top stack item", swap::<7>),
            inst!(SWAP8, "SWAP8", "Swap top stack item", swap::<8>),
            inst!(SWAP9, "SWAP9", "Swap top stack item", swap::<9>),
            inst!(
                SWAP10,
                "SWAP10",
                "Swap top stack item",
                swap::<10>
            ),
            inst!(
                SWAP11,
                "SWAP11",
                "Swap top stack item",
                swap::<11>
            ),
            inst!(
                SWAP12,
                "SWAP12",
                "Swap top stack item",
                swap::<12>
            ),
            inst!(
                SWAP13,
                "SWAP13",
                "Swap top stack item",
                swap::<13>
            ),
            inst!(
                SWAP14,
                "SWAP14",
                "Swap top stack item",
                swap::<14>
            ),
            inst!(
                SWAP15,
                "SWAP15",
                "Swap top stack item",
                swap::<15>
            ),
            inst!(
                SWAP16,
                "SWAP16",
                "Swap top stack item",
                swap::<16>
            ),
            inst!(LOG0, "LOG0", "Log", log::<0>),
            inst!(LOG1, "LOG1", "Log", log::<1>),
            inst!(LOG2, "LOG2", "Log", log::<2>),
            inst!(LOG3, "LOG3", "Log", log::<3>),
            inst!(LOG4, "LOG4", "Log", log::<4>),
            inst!(CREATE, "CREATE", "Create contract", nop),
            inst!(CALL, "CALL", "Call", nop),
            inst!(CALLCODE, "CALLCODE", "Call code", nop),
            inst!(RETURN, "RETURN", "Return", ret),
            inst!(DELEGATECALL, "DELEGATECALL", "Delegate call", nop),
            inst!(CREATE2, "CREATE2", "Create contract 2", nop),
            inst!(STATICCALL, "STATICCALL", "Static call", nop),
            inst!(REVERT, "REVERT", "Revert", revert),
            inst!(INVALID, "INVALID", "Invalid", invalid),
            inst!(SELFDESTRUCT, "SELFDESTRUCT", "Self destruct", nop),
        ];

        for &(opcode, name, description, function) in &instructions {
            opcode_table
                .insert(opcode, (name, description, function));
        }
        opcode_table
    });
