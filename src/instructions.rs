use std::cmp::{min, Ordering};

use crate::{
    context::{BlockContext, Context},
    error::EVMError,
    i256::{i256_cmp, i256_div, i256_mod},
    opcode::JUMPDEST,
    opcode_table::InstFn,
    state::StateDB,
};
use alloy_primitives::{Address, FixedBytes, U256};
use anyhow::Result;

pub fn stop(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    return Err(EVMError::Stop);
}

pub fn add(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a.wrapping_add(b));
    Ok(())
}

pub fn mul(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a.wrapping_mul(b));
    Ok(())
}

pub fn sub(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a.wrapping_sub(b));
    Ok(())
}

pub fn div(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a.wrapping_div(b));
    Ok(())
}

pub fn sign_div(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a: U256 = ctx.stack.pop();
    let b: U256 = ctx.stack.pop();
    ctx.stack.push(i256_div(a, b));
    Ok(())
}

pub fn modulo(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a.wrapping_rem(b));
    Ok(())
}

pub fn sign_modulo(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(i256_mod(a, b));
    Ok(())
}

pub fn add_mod(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    let c = ctx.stack.pop();
    ctx.stack.push(a.wrapping_add(b).wrapping_rem(c));
    Ok(())
}

pub fn mul_mod(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    let c = ctx.stack.pop();
    ctx.stack.push(a.wrapping_mul(b).wrapping_rem(c));
    Ok(())
}

pub fn exp(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a.pow(b));
    Ok(())
}

pub fn sign_extend(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let k = ctx.stack.pop();
    let x = ctx.stack.pop();

    if k < U256::from(31) {
        let ext = k.as_limbs()[0];
        let bit_index = (8 * ext + 7) as usize;
        let bit = x.bit(bit_index);
        let mask = (U256::from(1) << bit_index) - U256::from(1);
        let v = if bit { x | !mask } else { x & mask };
        ctx.stack.push(v);
    } else {
        ctx.stack.push(x);
    }
    Ok(())
}

pub fn lt(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    if a < b {
        ctx.stack.push(U256::from(1));
    } else {
        ctx.stack.push(U256::from(0));
    }
    Ok(())
}

pub fn gt(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    if a > b {
        ctx.stack.push(U256::from(1));
    } else {
        ctx.stack.push(U256::from(0));
    }
    Ok(())
}

pub fn slt(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    match i256_cmp(&a, &b) {
        Ordering::Less => ctx.stack.push(U256::from(1)),
        Ordering::Greater => ctx.stack.push(U256::from(0)),
        Ordering::Equal => ctx.stack.push(U256::from(0)),
    }
    Ok(())
}

pub fn sgt(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    match i256_cmp(&a, &b) {
        Ordering::Less => ctx.stack.push(U256::from(0)),
        Ordering::Greater => ctx.stack.push(U256::from(1)),
        Ordering::Equal => ctx.stack.push(U256::from(0)),
    }
    Ok(())
}

pub fn eq(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    if a == b {
        ctx.stack.push(U256::from(1));
    } else {
        ctx.stack.push(U256::from(0));
    }
    Ok(())
}

pub fn is_zero(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    if a == U256::from(0) {
        ctx.stack.push(U256::from(1));
    } else {
        ctx.stack.push(U256::from(0));
    }
    Ok(())
}

pub fn and(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a & b);
    Ok(())
}

pub fn or(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a | b);
    Ok(())
}

pub fn xor(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a ^ b);
    Ok(())
}

pub fn not(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    ctx.stack.push(!a);
    Ok(())
}

pub fn byte(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let i = ctx.stack.pop();
    let x = ctx.stack.pop();
    if i >= U256::from(32) {
        ctx.stack.push(U256::from(0));
    } else {
        let limb = x.as_limbs()[i.as_limbs()[0] as usize / 8];
        let byte = (limb >> (i.as_limbs()[0] % 8 * 8)) & 0xff;
        ctx.stack.push(U256::from(byte));
    }
    Ok(())
}

pub fn shl(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a << b);
    Ok(())
}

pub fn shr(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let a = ctx.stack.pop();
    let b = ctx.stack.pop();
    ctx.stack.push(a >> b);
    Ok(())
}

pub fn sar(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let shift = ctx.stack.pop();
    let value = ctx.stack.pop();

    if shift < U256::from(255) {
        ctx.stack
            .push(value.arithmetic_shr(shift.as_limbs()[0] as usize));
    } else if value.bit(255) {
        ctx.stack.push(U256::MAX);
    } else {
        ctx.stack.push(U256::ZERO);
    }
    Ok(())
}

pub fn keccak256(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();
    if size == U256::ZERO {
        ctx.stack.push(U256::ZERO);
        return Ok(());
    }
    let data = ctx.memory.read(u256_to_usize(offset), u256_to_usize(size));
    let hash = alloy_primitives::keccak256(data);
    ctx.stack.push(hash.into());
    Ok(())
}

pub fn address(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(ctx.contract.into_word().into());
    Ok(())
}

pub fn balance(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let address = ctx.stack.pop();
    ctx.stack.push(state.get_balance(u256_to_address(address)));
    Ok(())
}

pub fn origin(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(ctx.origin.into_word().into());
    Ok(())
}

pub fn caller(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(ctx.caller.into_word().into());
    Ok(())
}

pub fn call_value(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(ctx.value);
    Ok(())
}

pub fn call_data_load(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let offset = ctx.stack.pop();
    let mut loaded = [0u8; 32];
    let start_offset = min(u256_to_usize(offset), ctx.call_data.len() - 1);
    let copy_size = min(32usize, ctx.call_data.len() - start_offset);
    for i in 0..copy_size {
        loaded[i] = ctx.call_data[start_offset + i];
    }
    ctx.stack.push(U256::from_be_slice(&loaded));
    Ok(())
}

pub fn call_data_size(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(U256::from(ctx.call_data.len()));
    Ok(())
}

pub fn call_data_copy(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let dst_offset = ctx.stack.pop();
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();

    let copy_size = min(
        u256_to_usize(size),
        ctx.call_data.len() - u256_to_usize(offset),
    );
    let start_offset = u256_to_usize(offset);
    if start_offset >= ctx.call_data.len() {
        ctx.memory
            .fill(u256_to_usize(dst_offset), 0, u256_to_usize(size));
    } else {
        ctx.memory.write(
            u256_to_usize(dst_offset),
            &ctx.call_data[start_offset..start_offset + copy_size],
        );
    }
    Ok(())
}

pub fn code_size(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(U256::from(ctx.code.len()));
    Ok(())
}

pub fn code_copy(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let dst_offset = ctx.stack.pop();
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();
    let copy_size = min(u256_to_usize(size), ctx.code.len() - u256_to_usize(offset));
    let start_offset = u256_to_usize(offset);
    if start_offset >= ctx.code.len() {
        ctx.memory
            .fill(u256_to_usize(dst_offset), 0, u256_to_usize(size));
    } else {
        ctx.memory.write(
            u256_to_usize(dst_offset),
            &ctx.code[start_offset..start_offset + copy_size],
        );
    }
    Ok(())
}

pub fn gas_price(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(blk_ctx.gas_price);
    Ok(())
}

pub fn ext_code_size(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let address = ctx.stack.pop();
    ctx.stack
        .push(U256::from(state.get_code_size(u256_to_address(address))));
    Ok(())
}

pub fn ext_code_copy(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let address = ctx.stack.pop();
    let dst_offset = ctx.stack.pop();
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();

    let code = state.get_code(u256_to_address(address));
    let copy_size = min(u256_to_usize(size), code.len());
    let start_offset = u256_to_usize(offset);
    if start_offset >= code.len() {
        ctx.memory
            .fill(u256_to_usize(dst_offset), 0, u256_to_usize(size));
    } else {
        ctx.memory.write(
            u256_to_usize(dst_offset),
            &code[start_offset..start_offset + copy_size],
        );
    }
    Ok(())
}

pub fn return_data_size(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(U256::from(ctx.return_data.len()));
    Ok(())
}

pub fn return_data_copy(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let dst_offset = ctx.stack.pop();
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();

    let copy_size = min(
        u256_to_usize(size),
        ctx.return_data.len() - u256_to_usize(offset),
    );
    let start_offset = u256_to_usize(offset);
    if start_offset >= ctx.return_data.len() {
        ctx.memory
            .fill(u256_to_usize(dst_offset), 0, u256_to_usize(size));
    } else {
        ctx.memory.write(
            u256_to_usize(dst_offset),
            &ctx.return_data[start_offset..start_offset + copy_size],
        );
    }
    Ok(())
}

pub fn ext_code_hash(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let address = ctx.stack.pop();
    ctx.stack
        .push(state.get_code_hash(u256_to_address(address)).into());
    Ok(())
}

pub fn block_hash(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let block_number = ctx.stack.pop();
    // TODO implement block hash
    ctx.stack.push(U256::ZERO);
    Ok(())
}

pub fn coinbase(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(blk_ctx.block_coinbase);
    Ok(())
}

pub fn timestamp(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(blk_ctx.block_timestamp);
    Ok(())
}

pub fn block_number(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(blk_ctx.block_number);
    Ok(())
}

pub fn difficulty(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(blk_ctx.block_difficulty);
    Ok(())
}

pub fn gas_limit(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(blk_ctx.block_gas_limit);
    Ok(())
}

pub fn chain_id(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(blk_ctx.chain_id);
    Ok(())
}

pub fn self_balance(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(state.get_balance(ctx.contract));
    Ok(())
}

pub fn base_fee(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(blk_ctx.base_fee);
    Ok(())
}

pub fn blob_hash(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    // TODO implement blobhash
    ctx.stack.push(U256::ZERO);
    Ok(())
}

pub fn blob_hash_fee(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    // TODO implement blobhashfee
    ctx.stack.push(U256::ZERO);
    Ok(())
}

pub fn pop(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.pop();
    Ok(())
}

pub fn mload(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let offset = ctx.stack.pop();
    ctx.stack.push(ctx.memory.read32(u256_to_usize(offset)));
    Ok(())
}

pub fn mstore(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let offset = ctx.stack.pop();
    let value = ctx.stack.pop();
    ctx.memory.write32(u256_to_usize(offset), value);
    Ok(())
}

pub fn mstore8(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let offset = ctx.stack.pop();
    let value = ctx.stack.pop();
    ctx.memory
        .write8(u256_to_usize(offset), value.as_limbs()[0] as u8);
    Ok(())
}

pub fn sload(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let key = ctx.stack.pop();
    ctx.stack.push(state.get_state(ctx.contract, key));
    Ok(())
}

pub fn sstore(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let key = ctx.stack.pop();
    let value = ctx.stack.pop();
    state.set_state(ctx.contract, key, value);
    Ok(())
}

pub fn jump(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let counter = ctx.stack.pop();
    ctx.pc = u256_to_usize(counter);
    let next_code = ctx.code[ctx.pc + 1];
    if next_code != JUMPDEST {
        return Err(EVMError::InvalidJumpDestination);
    }
    Ok(())
}

pub fn jumpi(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let counter = ctx.stack.pop();
    let condition = ctx.stack.pop();
    if !condition.is_zero() {
        ctx.pc = u256_to_usize(counter);
    }
    Ok(())
}

pub fn pc(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(U256::from(ctx.pc));
    Ok(())
}

pub fn msize(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    // TODO implement msize
    ctx.stack.push(U256::ZERO);
    Ok(())
}

pub fn gas(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    // TODO implement gas
    ctx.stack.push(U256::ZERO);
    Ok(())
}

pub fn jump_dest(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    // Nothing todo
    Ok(())
}

pub fn tload(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    // TODO implement tload
    let key = ctx.stack.pop();
    ctx.stack.push(U256::ZERO);
    Ok(())
}

pub fn tstore(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    // TODO implement tstore
    let key = ctx.stack.pop();
    let value = ctx.stack.pop();
    ctx.stack.push(U256::ZERO);
    Ok(())
}

pub fn mcopy(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let dst_offset = ctx.stack.pop();
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();
    ctx.memory.copy(
        u256_to_usize(dst_offset),
        u256_to_usize(offset),
        u256_to_usize(size),
    );
    Ok(())
}

pub fn push0(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.push(U256::ZERO);
    Ok(())
}

pub fn push<const N: usize>(
    ctx: &mut Context,
    _state: &mut Box<dyn StateDB>,
    _blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let value = ctx.code[ctx.pc + 1..ctx.pc + N + 1].to_vec();
    ctx.stack.push(U256::from_be_slice(&value));
    Ok(())
}

pub fn dup<const N: usize>(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.dup(N);
    Ok(())
}

pub fn swap<const N: usize>(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    ctx.stack.swap(N);
    Ok(())
}

pub fn log<const N: usize>(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();
    let mut topics = Vec::new();
    for _ in 0..N {
        topics.push(ctx.stack.pop());
    }

    let data = ctx.memory.read(u256_to_usize(offset), u256_to_usize(size));

    // TODO log
    Ok(())
}

pub fn create(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let value = ctx.stack.pop();
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();
    let code = ctx.memory.read(u256_to_usize(offset), u256_to_usize(size));
    let contract_address = state.create_contract(ctx.contract, code);
    if !value.is_zero() {
        state
            .transfer(ctx.contract, contract_address, value)
            .unwrap();
    }
    ctx.stack.push(contract_address.into_word().into());
    Ok(())
}

pub fn call(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    // TODO 往后处理gas
    let gas = ctx.stack.pop();

    let address = ctx.stack.pop();
    let value = ctx.stack.pop();
    let args_offset = ctx.stack.pop();
    let args_size = ctx.stack.pop();
    let ret_offset = ctx.stack.pop();
    let ret_size = ctx.stack.pop();

    // TODO 这里处理失败的情况, 这里默认成功
    ctx.stack.push(U256::from(1));
    Ok(())
}

pub fn delegate_call(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let gas = ctx.stack.pop();
    let address = ctx.stack.pop();
    let args_offset = ctx.stack.pop();
    let args_size = ctx.stack.pop();
    let ret_offset = ctx.stack.pop();
    let ret_size = ctx.stack.pop();

    // TODO 这里处理失败的情况, 这里默认成功
    ctx.stack.push(U256::from(1));
    Ok(())
}

pub fn ret(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();
    ctx.return_data = ctx.memory.read(u256_to_usize(offset), u256_to_usize(size));
    Err(EVMError::Stop)
}

pub fn revert(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    let offset = ctx.stack.pop();
    let size = ctx.stack.pop();
    let err_data = ctx.memory.read(u256_to_usize(offset), u256_to_usize(size));
    ctx.return_data = err_data;
    Err(EVMError::Revert)
}

pub fn invalid(
    ctx: &mut Context,
    state: &mut Box<dyn StateDB>,
    blk_ctx: &BlockContext,
) -> Result<(), EVMError> {
    Err(EVMError::Stop)
}

fn u256_to_usize(value: U256) -> usize {
    let limbs = value.as_limbs();
    if limbs[1] == 0 && limbs[2] == 0 && limbs[3] == 0 {
        limbs[0] as usize
    } else {
        usize::MAX
    }
}

fn u256_to_address(value: U256) -> Address {
    Address::from_word(FixedBytes(value.to_be_bytes()))
}
