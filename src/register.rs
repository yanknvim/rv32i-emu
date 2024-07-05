#[derive(Debug)]
pub struct Register {
    reg: [u32; 32],
}

impl Register {
    pub fn new() -> Self {
        Register { reg: [0u32; 32] }
    }

    pub fn read(&self, register: usize) -> u32 {
        if register > 31 {
            panic!("Error: unexpected register");
        }
        self.reg[register]
        // x0 always return 0 becase x0 is never overwritten
    }

    pub fn write(&mut self, register: usize, value: u32) {
        if register > 31 {
            panic!("Error: unexpected register");
        }

        if register != 0 {
            self.reg[register] = value;
        }
    }
}
