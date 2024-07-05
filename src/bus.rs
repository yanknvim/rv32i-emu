use crate::memory::MEMORY_END;
use crate::rom::ROM_END;
use crate::{
    memory::{Memory, MEMORY_BASE, MEMORY_SIZE},
    rom::{Rom, RomData, ROM_BASE, ROM_SIZE},
    util::MemorySize,
};

#[derive(Default)]
pub struct Bus {
    mem: Memory,
    rom: Rom,
}

impl Bus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_binary_file(rom: RomData) -> Self {
        Self {
            rom: Rom::from_binary_file(rom),
            ..Default::default()
        }
    }

    pub fn read(&self, address: u32, size: MemorySize) -> u32 {
        match address {
            ROM_BASE..=ROM_END => self.rom.read(address - ROM_BASE, size),
            MEMORY_BASE..=MEMORY_END => self.mem.read(address - MEMORY_BASE, size),
            _ => panic!("Error: Invalid memory address"),
        }
    }

    pub fn write(&mut self, address: u32, size: MemorySize, value: u32) {
        match address {
            MEMORY_BASE..=MEMORY_END => self.mem.write(address - MEMORY_BASE, size, value),
            _ => panic!("Error: Invalid memory address"),
        }
    }
}
