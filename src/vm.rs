use alloy_primitives::{Address, U256};

use crate::{
    context::{BlockContext, Context},
    error::EVMError,
    opcode::{get_opcode_size, PUSH1, PUSH32},
    opcode_table::OPCODE_TABLE,
    state::StateDB,
};

pub struct Interpreter {
    state: Box<dyn StateDB>,
    blk_ctx: BlockContext,
}

impl Interpreter {
    pub fn new(state: Box<dyn StateDB>, blk_ctx: BlockContext) -> Self {
        Self { state, blk_ctx }
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
        ctx.pc = 0;

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
                            println!("{} 0x{}", opcode_name, hex::encode(operand));
                        }
                        _ => {
                            println!("{}", opcode_name);
                        }
                    };

                    // execute the instruction
                    match inst_fn(&mut ctx, &mut self.state, &self.blk_ctx) {
                        Ok(_) => {
                            ctx.pc += get_opcode_size(opcode);
                        }
                        Err(EVMError::Stop) => break,
                        Err(e) => return Err(e),
                    }
                }
                None => return Err(EVMError::InvalidOpcode(opcode)),
            }
        }
        Ok(())
    }
}
