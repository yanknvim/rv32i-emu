use crate::{
    bus::Bus,
    cpu::Cpu,
};

pub struct Motherboard {
    pub bus: Bus,
    pub cpu: Cpu,
}

impl Motherboard {
    pub fn new() -> Self {
        Motherboard {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.cpu.step(&self.bus);
        }
    }
}
