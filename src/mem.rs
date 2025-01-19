use alloy_primitives::U256;

const MEMORY_SIZE: usize = 1024 * 1024 * 16;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        let mut mem = Memory {
            memory: Vec::with_capacity(MEMORY_SIZE),
        };
        mem.memory.resize(MEMORY_SIZE, 0);
        mem
    }

    pub fn write32(&mut self, offset: usize, value: U256) {
        self.memory[offset..offset + 32].copy_from_slice(value.to_be_bytes_vec().as_slice());
    }

    pub fn write8(&mut self, offset: usize, value: u8) {
        self.memory[offset] = value;
    }

    pub fn write(&mut self, offset: usize, value: &[u8]) {
        if value.len() > 0 {
            self.memory[offset..offset + value.len()].copy_from_slice(value);
        }
    }

    pub fn fill(&mut self, offset: usize, value: u8, size: usize) {
        self.memory[offset..offset + size].fill(value);
    }

    pub fn read(&self, offset: usize, size: usize) -> Vec<u8> {
        if size == 0 {
            return vec![];
        }
        self.memory[offset..offset + size].to_vec()
    }

    pub fn read32(&self, offset: usize) -> U256 {
        U256::from_be_slice(&self.memory[offset..offset + 32])
    }

    pub fn copy(&mut self, dst_offset: usize, src_offset: usize, size: usize) {
        let (dst_slice, src_slice) = self.memory.split_at_mut(dst_offset);
        dst_slice[dst_offset..dst_offset + size]
            .copy_from_slice(&src_slice[src_offset..src_offset + size]);
    }
}
