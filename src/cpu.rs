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
    Add {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sub {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sll {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Slt {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sltu {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Xor {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Srl {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sra {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Or {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    And {
        rd: usize,
        rs1: usize,
        rs2: usize,
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
        let rs2 = op.view_bits::<Lsb0>()[20..=24].load_le();
        let funct7 = op.view_bits::<Lsb0>()[25..=31].load_le();

        match opcode {
            0b0010011 => {
                let imm = op.view_bits::<Lsb0>()[20..=31].load_le::<i32>() as u32;
                Opcode::Addi {
                    rd,
                    rs1,
                    imm,
                }
            },
            0b0110011 => {
                match (funct7, funct3) {
                    (0b00000_00, 0b000) => {
                        Opcode::Add {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b01000_00, 0b000) => {
                        Opcode::Sub {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b00000_00, 0b001) => {
                        Opcode::Sll {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b00000_00, 0b010) => {
                        Opcode::Slt {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b00000_00, 0b011) => {
                        Opcode::Sltu {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b00000_00, 0b100) => {
                        Opcode::Xor {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b00000_00, 0b101) => {
                        Opcode::Srl {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b01000_00, 0b101) => {
                        Opcode::Sra {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b00000_00, 0b110) => {
                        Opcode::Or {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    (0b00000_00, 0b111) => {
                        Opcode::And {
                            rd,
                            rs1,
                            rs2,
                        }
                    },
                    _ => {
                        unimplemented!();
                    }
                }
            }
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
            },
            Opcode::Add { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 + rs2);
            },
            Opcode::Sub { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 - rs2);
            },
            Opcode::Sll { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 << rs2);
            },
            Opcode::Slt { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                if (rs1 as i32) < (rs2 as i32) {
                    self.reg.write(rd, 1u32);
                } else {
                    self.reg.write(rd, 0u32);
                }
            },
            Opcode::Sltu { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                if rs1 < rs2 {
                    self.reg.write(rd, 1u32);
                } else {
                    self.reg.write(rd, 0u32);
                }
            },
            Opcode::Xor { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 ^ rs2);
            },
            Opcode::Srl { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 >> rs2);
            },
            Opcode::Sra { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, (rs1 as i32 >> rs2 as i32) as u32);
            },
            Opcode::Or { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 | rs2);
            },
            Opcode::And { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 & rs2);
            },
        }
    }
}


