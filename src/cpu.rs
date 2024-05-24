use bitvec::prelude::*;

use crate::{
    bus::Bus,
    memory::MEMORY_BASE,
    register::Register,
    util::MemorySize,
};

pub struct Cpu {
    reg: Register,
    pc: u32,
}

pub enum Opcode {
    Addi {
        rd: usize,
        rs1: usize,
        imm: u32,
    },
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            reg: Register::new(),
            pc: MEMORY_BASE,
        }
    }

    pub fn step(&mut self, bus: &Bus) {
        println!("{:?}", self.reg);
        let op = self.fetch(bus);
        self.pc += 4;
        
        let opcode = Self::decode(op); 
        self.exec(opcode);
    }

    fn fetch(&self, bus: &Bus) -> u32 {
        bus.read(self.pc, MemorySize::Word)
    }

    fn decode(op: u32) -> Opcode {
        let opcode = op.view_bits::<Lsb0>()[0..=6].load_le();
        let funct3 = op.view_bits::<Lsb0>()[12..=14].load_le::<u32>();
        let rd = op.view_bits::<Lsb0>()[7..=11].load_le();
        let rs1 = op.view_bits::<Lsb0>()[15..=19].load_le();
        let rs2 = op.view_bits::<Lsb0>()[20..=24].load_le::<u32>();

        match opcode {
            0b0010011 => {
                let imm = op.view_bits::<Lsb0>()[20..=31].load_le::<i32>() as u32;
                Opcode::Addi {
                    rd,
                    rs1,
                    imm
                }
            },
            _ => {
                unimplemented!();
            },
        }
    }

    fn exec(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Addi { rd, rs1, imm } => {
                let rs1 = self.reg.read(rs1);
                self.reg.write(rd, rs1 + imm);
            }
        }
    }
}


