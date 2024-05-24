use crate::{
    memory::{Memory, MEMORY_BASE},
    util::MemorySize,
};

pub struct Bus {
    mem: Memory,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            mem: Memory::new()
        }
    }

    pub fn read(&self, address: u32, size: MemorySize) -> u32 {
       if MEMORY_BASE <= address {
            self.mem.read(address, size)
       } else {
            panic!("Error: Invalid memory address");
       }
    }

    pub fn write(&mut self, address: u32, size: MemorySize, value: u32) {
       if MEMORY_BASE <= address {
           self.mem.write(address, size, value)
       } else {
            panic!("Error: Invalid memory address");
       }
    }
}
