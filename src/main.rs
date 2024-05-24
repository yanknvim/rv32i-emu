mod bus;
mod cpu;
mod memory;
mod motherboard;
mod register;
mod util;

use crate::{
    motherboard::Motherboard,
    memory::MEMORY_BASE,
    util::MemorySize,
};

fn main() {
    let mut mb = Motherboard::new();
    mb.bus.write(MEMORY_BASE, MemorySize::Word, 0b000000000001_00000_000_00001_00100_11); 
    mb.bus.write(MEMORY_BASE+4, MemorySize::Word, 0b000000000010_00001_000_00010_00100_11); 

    mb.run();
}
