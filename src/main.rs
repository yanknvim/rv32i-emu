mod bus;
mod cpu;
mod memory;
mod motherboard;
mod register;
mod rom;
mod util;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::{
    memory::MEMORY_BASE,
    motherboard::Motherboard,
    rom::{RomData, ROM_SIZE},
    util::MemorySize,
};

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Error: Invalid number of command line args");
    }

    let path = Path::new(&args[1]);

    let bin = load_binary(&path)?;
    let mut mb = Motherboard::from_binary_file(bin);
    mb.run();
}

fn load_binary(path: &Path) -> Result<RomData, std::io::Error> {
    let mut f = File::open(path)?;
    let mut buffer = [0u8; ROM_SIZE as usize];
    f.read(&mut buffer)?;

    Ok(buffer)
}
