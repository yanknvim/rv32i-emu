use crate::util::MemorySize;
use bitvec::prelude::*;

pub const MEMORY_SIZE: u32 = 1024 * 1024;
pub const MEMORY_BASE: u32 = 0x8000_0000;
pub const MEMORY_END: u32 = MEMORY_BASE + MEMORY_SIZE - 1;

pub struct Memory {
    mem: [u8; MEMORY_SIZE as usize],
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            mem: [0u8; MEMORY_SIZE as usize],
        }
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory::default()
    }

    pub fn read(&self, address: u32, size: MemorySize) -> u32 {
        let address = address as usize;
        match size {
            MemorySize::Byte => self.mem[address] as u32,
            MemorySize::HalfWord => self.mem[address..=address + 1]
                .view_bits::<Lsb0>()
                .load_le(),
            MemorySize::Word => self.mem[address..=address + 3]
                .view_bits::<Lsb0>()
                .load_le(),
        }
    }

    pub fn write(&mut self, address: u32, size: MemorySize, value: u32) {
        let address = address as usize;
        match size {
            MemorySize::Byte => {
                self.mem[address] = value as u8;
            }
            MemorySize::HalfWord => self.mem[address..=address + 1]
                .view_bits_mut::<Lsb0>()
                .store_le(value),
            MemorySize::Word => self.mem[address..=address + 3]
                .view_bits_mut::<Lsb0>()
                .store_le(value),
        }
    }
}
