use alloy_primitives::U256;
use std::cmp::max;

const MEMORY_SIZE: usize = 1024;

pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        let mut mem = Memory {
            memory: Vec::with_capacity(MEMORY_SIZE),
        };
        mem
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    fn ensure_capacity(&mut self, offset: usize, size: usize) {
        if offset + size > self.memory.len() {
            self.memory.resize(offset + size, 0);
        }
    }

    pub fn write32(&mut self, offset: usize, value: U256) {
        self.ensure_capacity(offset, 32);
        self.memory[offset..offset + 32].copy_from_slice(value.to_be_bytes_vec().as_slice());
    }

    pub fn write8(&mut self, offset: usize, value: u8) {
        self.ensure_capacity(offset, 1);
        self.memory[offset] = value;
    }

    pub fn write(&mut self, offset: usize, value: &[u8]) {
        if value.len() > 0 {
            self.ensure_capacity(offset, value.len());
            self.memory[offset..offset + value.len()].copy_from_slice(value);
        }
    }

    pub fn write_with_size(&mut self, offset: usize, size: usize, value: &[u8]) {
        self.ensure_capacity(offset, size);
        self.memory[offset..offset + size].copy_from_slice(&value[..size]);

        if size > value.len() {
            for i in offset + value.len()..offset + size {
                self.memory[i] = 0;
            }
        }
    }

    pub fn fill(&mut self, offset: usize, value: u8, size: usize) {
        self.ensure_capacity(offset, size);
        self.memory[offset..offset + size].fill(value);
    }

    pub fn read(&mut self, offset: usize, size: usize) -> Vec<u8> {
        if size == 0 {
            return vec![];
        }
        self.ensure_capacity(offset, size);
        self.memory[offset..offset + size].to_vec()
    }

    pub fn read32(&mut self, offset: usize) -> U256 {
        self.ensure_capacity(offset, 32);
        U256::from_be_slice(&self.memory[offset..offset + 32])
    }

    pub fn copy(&mut self, dst_offset: usize, src_offset: usize, size: usize) {
        self.ensure_capacity(max(src_offset, dst_offset), size);
        let (dst_slice, src_slice) = self.memory.split_at_mut(dst_offset);
        dst_slice[dst_offset..dst_offset + size]
            .copy_from_slice(&src_slice[src_offset..src_offset + size]);
    }

    pub fn print_memory(&self) {
        // 打印表头
        println!("{}", "-".repeat(16 * 3 + 7 + 19));
        print!("Offset  ");
        for i in 0..16 {
            print!("{:02x} ", i);
        }
        print!("  ASCII");
        println!();

        // 打印分隔线
        println!("{}", "-".repeat(16 * 3 + 7 + 19));

        // 按每行 16 字节打印内存内容
        for (i, chunk) in self.memory.chunks(16).enumerate() {
            // 打印左侧的内存地址偏移量
            print!("{:06x}  ", i * 16);

            // 打印每个字节的内容
            for byte in chunk {
                print!("{:02x} ", byte);
            }

            // 如果当前行不足 16 字节，用空格补齐
            if chunk.len() < 16 {
                print!("{}", "   ".repeat(16 - chunk.len()));
            }

            // 打印 ASCII 列
            print!("  ");
            for byte in chunk {
                if byte.is_ascii_graphic() || *byte == b' ' {
                    print!("{}", *byte as char);
                } else {
                    print!(".");
                }
            }

            println!();
        }
        println!("{}", "-".repeat(16 * 3 + 7 + 19));
    }
}
