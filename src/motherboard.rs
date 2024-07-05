use crate::{bus::Bus, cpu::Cpu, rom::RomData};

pub struct Motherboard {
    pub bus: Bus,
    pub cpu: Cpu,
}

impl Motherboard {
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn from_binary_file(rom: RomData) -> Self {
        Self {
            bus: Bus::from_binary_file(rom),
            cpu: Cpu::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.cpu.step(&self.bus);
        }
    }
}
