// TODO 汇编，根据opcode 字符串生成Bytecode

use std::collections::HashMap;

use alloy_primitives::U256;
use anyhow::Result;

use crate::error::EVMError;
use crate::opcode::*;
use crate::opcode_table::OPCODE_TABLE;

pub struct Assembler {
    opcode_table: HashMap<String, u8>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            opcode_table: OPCODE_TABLE
                .iter()
                .map(|(k, (name, _, _))| (name.to_string(), k.to_owned()))
                .collect(),
        }
    }

    pub fn asm(&self, opcode: &str) -> Result<Vec<u8>, EVMError> {
        let mut bytes = vec![];
        for line in opcode.lines() {
            if line.is_empty() || line.chars().all(|c| c.is_whitespace()) {
                continue;
            }
            bytes.extend(self.asm_opcode(line)?);
        }
        Ok(bytes)
    }

    fn asm_opcode(&self, opcode: &str) -> Result<Vec<u8>, EVMError> {
        let mut opcode_and_operand = opcode.split_whitespace().filter(|s| !s.is_empty());

        let opcode_token = opcode_and_operand
            .next()
            .ok_or(EVMError::InvalidAsmToken(opcode.to_string()))?;
        let opcode_byte = self
            .opcode_table
            .get(opcode_token)
            .ok_or(EVMError::InvalidAsmToken(opcode.to_string()))?
            .to_owned();

        match opcode_byte {
            PUSH1..=PUSH32 => {
                let operand = opcode_and_operand
                    .next()
                    .ok_or(EVMError::InvalidAsmToken(opcode.to_string()))?;
                let operand_u256 = match operand.strip_prefix("0x") {
                    Some(operand_remove_prefix) => {
                        let operand_u256 = U256::from_str_radix(operand_remove_prefix, 16)
                            .map_err(|_| EVMError::InvalidAsmToken(opcode.to_string()))?;
                        operand_u256
                    }
                    None => {
                        let operand_u256 = U256::from_str_radix(operand, 16)
                            .map_err(|_| EVMError::InvalidAsmToken(opcode.to_string()))?;
                        operand_u256
                    }
                };
                let mut operand_bytes = vec![0];
                if !operand_u256.is_zero() {
                    operand_bytes = operand_u256.to_be_bytes_trimmed_vec()
                }

                let operand_size = (opcode_byte - PUSH1 + 1) as usize;
                if operand_bytes.len() > operand_size {
                    return Err(EVMError::InvalidAsmToken(opcode.to_string()));
                }

                let mut bytes = vec![opcode_byte];
                bytes.extend(&operand_bytes[0..operand_size]);
                Ok(bytes)
            }
            _ => Ok(vec![opcode_byte]),
        }
    }
}
