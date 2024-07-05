use crate::util::MemorySize;
use bitvec::prelude::*;

pub const ROM_SIZE: u32 = 4096;
pub const ROM_BASE: u32 = 0x0000_1000;
pub const ROM_END: u32 = ROM_BASE + ROM_SIZE - 1;

pub type RomData = [u8; ROM_SIZE as usize];

pub struct Rom {
    rom: RomData,
}

impl Default for Rom {
    fn default() -> Self {
        Rom {
            rom: [0u8; ROM_SIZE as usize],
        }
    }
}

impl Rom {
    pub fn new() -> Self {
        Rom::default()
    }

    pub fn from_binary_file(rom: RomData) -> Self {
        Rom { rom }
    }

    pub fn read(&self, address: u32, size: MemorySize) -> u32 {
        let address = address as usize;
        match size {
            MemorySize::Byte => self.rom[address] as u32,
            MemorySize::HalfWord => self.rom[address..=address + 1]
                .view_bits::<Lsb0>()
                .load_le(),
            MemorySize::Word => self.rom[address..=address + 3]
                .view_bits::<Lsb0>()
                .load_le(),
        }
    }
}
