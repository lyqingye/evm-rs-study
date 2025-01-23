use alloy_primitives::{keccak256, Address, B256, U256};

use crate::opcode::{CALLCODE, CREATE, CREATE2, DELEGATECALL, STATICCALL};
use crate::u256::u256_to_address;
use crate::{
    context::{BlockContext, Context},
    error::EVMError,
    opcode::{get_opcode_size, CALL, PUSH1, PUSH32},
    opcode_table::OPCODE_TABLE,
    state::StateDB,
    u256::u256_to_usize,
};

pub struct Interpreter<'a> {
    state: Box<dyn StateDB>,
    blk_ctx: &'a BlockContext,
}

macro_rules! stack_pop {
    ($ctx:ident, $count:expr) => {{
        // 创建一个向量用于存储弹出的值
        let mut values: Vec<_> = (0..$count).map(|_| $ctx.stack.pop()).collect();
        // 将值转化为固定大小的数组，并返回
        // 注意：这里假设我们有足够的弹出值
        let result: [_; $count] = values
            .try_into()
            .expect("Expected the correct number of elements");
        result
    }};
}

impl<'a> Interpreter<'a> {
    pub fn new(state: Box<dyn StateDB>, blk_ctx: &'a BlockContext) -> Self {
        Self { state, blk_ctx }
    }

    pub fn run_with_ctx(&mut self, ctx: &mut Context) -> Result<(), EVMError> {
        loop {
            let opcode = ctx.code[ctx.pc];
            match OPCODE_TABLE.get(&opcode) {
                Some((opcode_name, _, inst_fn)) => {
                    // print the instruction
                    match opcode {
                        PUSH1..=PUSH32 => {
                            let operand_size = (opcode - PUSH1 + 1) as usize;
                            let operand =
                                ctx.code[(ctx.pc + 1)..(ctx.pc + 1 + operand_size)].to_vec();
                            println!(
                                "{}{} 0x{}",
                                " ".repeat(ctx.depth * 4),
                                opcode_name,
                                hex::encode(operand)
                            );
                        }
                        _ => {
                            println!(
                                "{}{}",
                                " ".repeat(ctx.depth * 4), // 每个深度级别缩进4个空格
                                opcode_name
                            );
                        }
                    };

                    let result = match opcode {
                        CALL => self.call(ctx),
                        CALLCODE => self.call_code(ctx),
                        STATICCALL => self.static_call(ctx),
                        CREATE => self.create(ctx),
                        CREATE2 => self.create2(ctx),
                        DELEGATECALL => self.delegate_call(ctx),
                        _ => {
                            // execute the instruction
                            inst_fn(ctx, &mut self.state, &self.blk_ctx)
                        }
                    };

                    match result {
                        Ok(_) => {
                            ctx.pc += get_opcode_size(opcode);
                        }
                        Err(EVMError::Stop) => break,
                        Err(e) => return Err(e),
                    }
                }
                None => return Err(EVMError::InvalidOpcode(opcode)),
            }
            if ctx.pc >= ctx.code.len() {
                break;
            }
        }
        Ok(())
    }

    pub fn run(
        &mut self,
        origin: Address,
        from: Address,
        to: Address,
        args: Vec<u8>,
        value: U256,
    ) -> Result<(), EVMError> {
        let mut ctx = Context::new();
        ctx.contract = to;
        ctx.code = self.state.get_code(to);
        ctx.call_data = args;
        ctx.value = value;
        ctx.caller = from;
        ctx.origin = origin;

        self.run_with_ctx(&mut ctx)?;
        ctx.stack.print_stack();
        Ok(())
    }

    fn call(&mut self, ctx: &mut Context) -> Result<(), EVMError> {
        // TODO 往后处理gas
        let [gas, to, value, args_offset, args_size, ret_offset, ret_size] = ctx.stack.pop_n::<7>();

        let call_data = ctx
            .memory
            .read(u256_to_usize(args_offset), u256_to_usize(args_size));

        let mut new_ctx = Context::new();
        new_ctx.contract = u256_to_address(to);
        new_ctx.code = self.state.get_code(new_ctx.contract);
        new_ctx.call_data = call_data;
        new_ctx.value = value;
        new_ctx.caller = ctx.origin;
        new_ctx.depth = ctx.depth + 1;

        if !value.is_zero() {
            match self.state.transfer(ctx.caller, new_ctx.contract, value) {
                Ok(_) => {}
                Err(EVMError::InsufficientBalance) => {
                    ctx.stack.push(U256::ZERO);
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // prepare state for transaction
        self.state.prepare();
        match self.run_with_ctx(&mut new_ctx) {
            Ok(_) => {
                ctx.stack.push(U256::from(1));

                // commit state for transaction
                self.state.commit();
            }
            Err(e) => {
                ctx.stack.push(U256::ZERO);
            }
        }

        ctx.memory.write_with_size(
            u256_to_usize(ret_offset),
            u256_to_usize(ret_size),
            new_ctx.return_data.as_slice(),
        );
        ctx.return_data = new_ctx.return_data;

        Ok(())
    }

    fn delegate_call(&mut self, ctx: &mut Context) -> Result<(), EVMError> {
        // TODO 往后处理gas
        let [gas, to, args_offset, args_size, ret_offset, ret_size] = ctx.stack.pop_n::<6>();

        let call_data = ctx
            .memory
            .read(u256_to_usize(args_offset), u256_to_usize(args_size));

        let mut new_ctx = Context::new();

        new_ctx.contract = ctx.contract;
        new_ctx.code = self.state.get_code(u256_to_address(to));
        new_ctx.call_data = call_data;
        new_ctx.caller = ctx.caller;
        new_ctx.depth = ctx.depth + 1;

        // prepare state for transaction
        self.state.prepare();
        match self.run_with_ctx(&mut new_ctx) {
            Ok(_) => {
                ctx.stack.push(U256::from(1));

                // commit state for transaction
                self.state.commit();
            }
            Err(e) => {
                ctx.stack.push(U256::ZERO);
            }
        }

        ctx.memory.write_with_size(
            u256_to_usize(ret_offset),
            u256_to_usize(ret_size),
            new_ctx.return_data.as_slice(),
        );
        ctx.return_data = new_ctx.return_data;

        Ok(())
    }

    fn call_code(&mut self, ctx: &mut Context) -> Result<(), EVMError> {
        // TODO 往后处理gas
        let [gas, to, args_offset, args_size, ret_offset, ret_size] = ctx.stack.pop_n::<6>();

        let call_data = ctx
            .memory
            .read(u256_to_usize(args_offset), u256_to_usize(args_size));

        let mut new_ctx = Context::new();

        new_ctx.contract = ctx.contract;
        new_ctx.caller = u256_to_address(to);
        new_ctx.code = self.state.get_code(new_ctx.caller);
        new_ctx.call_data = call_data;
        new_ctx.depth = ctx.depth + 1;

        // prepare state for transaction
        self.state.prepare();
        match self.run_with_ctx(&mut new_ctx) {
            Ok(_) => {
                ctx.stack.push(U256::from(1));

                // commit state for transaction
                self.state.commit();
            }
            Err(e) => {
                ctx.stack.push(U256::ZERO);
            }
        }

        ctx.memory.write_with_size(
            u256_to_usize(ret_offset),
            u256_to_usize(ret_size),
            new_ctx.return_data.as_slice(),
        );
        ctx.return_data = new_ctx.return_data;

        Ok(())
    }

    fn static_call(&mut self, ctx: &mut Context) -> Result<(), EVMError> {
        let [gas, to, args_offset, args_size, ret_offset, ret_size] = ctx.stack.pop_n::<6>();

        let call_data = ctx
            .memory
            .read(u256_to_usize(args_offset), u256_to_usize(args_size));

        let mut new_ctx = Context::new();
        new_ctx.contract = u256_to_address(to);
        new_ctx.code = self.state.get_code(new_ctx.contract);
        new_ctx.call_data = call_data;
        new_ctx.caller = ctx.origin;
        new_ctx.depth = ctx.depth + 1;

        // prepare state for transaction
        self.state.prepare();
        match self.run_with_ctx(&mut new_ctx) {
            Ok(_) => {
                ctx.stack.push(U256::from(1));
            }
            Err(e) => {
                ctx.stack.push(U256::ZERO);
            }
        }

        ctx.memory.write_with_size(
            u256_to_usize(ret_offset),
            u256_to_usize(ret_size),
            new_ctx.return_data.as_slice(),
        );
        ctx.return_data = new_ctx.return_data;

        Ok(())
    }

    fn create(&mut self, ctx: &mut Context) -> Result<(), EVMError> {
        let [value, offset, size] = ctx.stack.pop_n::<3>();
        let code = ctx.memory.read(u256_to_usize(offset), u256_to_usize(size));

        let contract_address = ctx.caller.create(self.state.get_nonce(ctx.caller));

        if !value.is_zero() {
            self.state.transfer(ctx.contract, contract_address, value)?;
        }

        let contract_code = self.init_contract(ctx, contract_address, code)?;
        self.state.set_code(contract_address, contract_code);
        ctx.stack.push(contract_address.into_word().into());
        Ok(())
    }

    fn create2(&mut self, ctx: &mut Context) -> Result<(), EVMError> {
        let [value, offset, size, salt] = ctx.stack.pop_n::<4>();

        let code = ctx.memory.read(u256_to_usize(offset), u256_to_usize(size));
        let code_hash = keccak256(&code);
        let contract_address = ctx.caller.create2(B256::from(salt), B256::from(code_hash));

        if !value.is_zero() {
            self.state.transfer(ctx.contract, contract_address, value)?;
        }

        let contract_code = self.init_contract(ctx, contract_address, code)?;
        self.state.set_code(contract_address, contract_code);
        ctx.stack.push(contract_address.into_word().into());
        Ok(())
    }

    fn init_contract(
        &mut self,
        ctx: &mut Context,
        contract_address: Address,
        code: Vec<u8>,
    ) -> Result<Vec<u8>, EVMError> {
        let mut new_ctx = Context::new();
        new_ctx.contract = contract_address;
        new_ctx.code = code;
        new_ctx.caller = ctx.caller;
        new_ctx.depth = ctx.depth + 1;

        self.run_with_ctx(&mut new_ctx)?;

        Ok(new_ctx.return_data)
    }
}
