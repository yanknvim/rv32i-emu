use bitvec::prelude::*;

use crate::{bus::Bus, rom::ROM_BASE, register::Register, util::MemorySize};

pub struct Cpu {
    reg: Register,
    pc: u32,
}

pub enum Opcode {
    Lui { imm: u32, rd: usize },
    Auipc { imm: u32, rd: usize },
    Addi { rd: usize, rs1: usize, imm: i32 },
    Slti { rd: usize, rs1: usize, imm: i32 },
    Sltiu { rd: usize, rs1: usize, imm: u32 },
    Xori { rd: usize, rs1: usize, imm: i32 },
    Ori { rd: usize, rs1: usize, imm: i32 },
    Andi { rd: usize, rs1: usize, imm: i32 },
    Slli { rd: usize, rs1: usize, shamt: u32 },
    Srli { rd: usize, rs1: usize, shamt: u32 },
    Srai { rd: usize, rs1: usize, shamt: i32 },
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Sll { rd: usize, rs1: usize, rs2: usize },
    Slt { rd: usize, rs1: usize, rs2: usize },
    Sltu { rd: usize, rs1: usize, rs2: usize },
    Xor { rd: usize, rs1: usize, rs2: usize },
    Srl { rd: usize, rs1: usize, rs2: usize },
    Sra { rd: usize, rs1: usize, rs2: usize },
    Or { rd: usize, rs1: usize, rs2: usize },
    And { rd: usize, rs1: usize, rs2: usize },
    Fence,
    Jalr { rd: usize, rs1: usize, offset: i32 },
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            reg: Register::new(),
            pc: ROM_BASE,
        }
    }

    pub fn step(&mut self, bus: &Bus) {
        let op = self.fetch(bus);
        self.pc += 4;

        println!("{:#032b} {:#x} {:?}", op, self.pc - 4, self.reg);

        let opcode = Self::decode(op);
        self.exec(opcode);
    }

    fn fetch(&self, bus: &Bus) -> u32 {
        bus.read(self.pc, MemorySize::Word)
    }

    fn decode(op: u32) -> Opcode {
        let opcode = op.view_bits::<Lsb0>()[0..=6].load_le::<usize>();
        let funct3 = op.view_bits::<Lsb0>()[12..=14].load_le::<usize>();
        let rd = op.view_bits::<Lsb0>()[7..=11].load_le::<usize>();
        let rs1 = op.view_bits::<Lsb0>()[15..=19].load_le::<usize>();
        let rs2 = op.view_bits::<Lsb0>()[20..=24].load_le::<usize>();
        let funct7 = op.view_bits::<Lsb0>()[25..=31].load_le::<usize>();

        match opcode {
            0b01101_11 => {
                let imm = op.view_bits::<Lsb0>()[12..=31].load_le::<u32>();
                Opcode::Lui { imm, rd }
            }
            0b00101_11 => {
                let imm = op.view_bits::<Lsb0>()[12..=31].load_le::<u32>();
                Opcode::Auipc { imm, rd }
            }
            0b00100_11 => {
                let imm = op.view_bits::<Lsb0>()[20..=31].load_le::<i32>();
                match funct3 {
                    0b000 => Opcode::Addi { rd, rs1, imm },
                    0b010 => Opcode::Slti { rd, rs1, imm },
                    0b011 => Opcode::Sltiu {
                        rd,
                        rs1,
                        imm: imm as u32,
                    },
                    0b100 => Opcode::Xori { rd, rs1, imm },
                    0b110 => Opcode::Ori { rd, rs1, imm },
                    0b111 => Opcode::Andi { rd, rs1, imm },
                    0b001 => Opcode::Slli {
                        rd,
                        rs1,
                        shamt: rs2 as u32,
                    },
                    0b101 => match funct7 {
                        0b00000_00 => Opcode::Srli {
                            rd,
                            rs1,
                            shamt: rs2 as u32,
                        },
                        0b01000_00 => Opcode::Srai {
                            rd,
                            rs1,
                            shamt: rs2 as i32,
                        },
                        _ => unimplemented!(),
                    },
                    _ => unimplemented!(),
                }
            }
            0b01100_11 => match (funct7, funct3) {
                (0b00000_00, 0b000) => Opcode::Add { rd, rs1, rs2 },
                (0b01000_00, 0b000) => Opcode::Sub { rd, rs1, rs2 },
                (0b00000_00, 0b001) => Opcode::Sll { rd, rs1, rs2 },
                (0b00000_00, 0b010) => Opcode::Slt { rd, rs1, rs2 },
                (0b00000_00, 0b011) => Opcode::Sltu { rd, rs1, rs2 },
                (0b00000_00, 0b100) => Opcode::Xor { rd, rs1, rs2 },
                (0b00000_00, 0b101) => Opcode::Srl { rd, rs1, rs2 },
                (0b01000_00, 0b101) => Opcode::Sra { rd, rs1, rs2 },
                (0b00000_00, 0b110) => Opcode::Or { rd, rs1, rs2 },
                (0b00000_00, 0b111) => Opcode::And { rd, rs1, rs2 },
                _ => {
                    unimplemented!();
                }
            },
            0b11001_11 => {
                let offset = op.view_bits::<Lsb0>()[20..=31].load_le::<i32>();
                Opcode::Jalr { rd, rs1, offset }
            }
            0b00011_11 => Opcode::Fence,
            _ => {
                unimplemented!("Unimplemented instruction: {:#032b}", op);
            }
        }
    }

    fn exec(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Lui { imm, rd } => {
                let imm = imm << 12;
                self.reg.write(rd, imm);
            }
            Opcode::Auipc { imm, rd } => {
                let imm = imm << 12;
                self.reg.write(rd, self.pc.wrapping_add(imm));
            }
            Opcode::Addi { rd, rs1, imm } => {
                let rs1 = self.reg.read(rs1);
                self.reg.write(rd, rs1.wrapping_add(imm as u32));
            }
            Opcode::Slti { rd, rs1, imm } => {
                let rs1 = self.reg.read(rs1);
                if (rs1 as i32) < imm {
                    self.reg.write(rd, 1u32);
                } else {
                    self.reg.write(rd, 0u32);
                }
            }
            Opcode::Sltiu { rd, rs1, imm } => {
                let rs1 = self.reg.read(rs1);
                if rs1 < imm {
                    self.reg.write(rd, 1u32);
                } else {
                    self.reg.write(rd, 0u32);
                }
            }
            Opcode::Xori { rd, rs1, imm } => {
                let rs1 = self.reg.read(rs1);
                self.reg.write(rd, rs1 ^ imm as u32);
            }
            Opcode::Ori { rd, rs1, imm } => {
                let rs1 = self.reg.read(rs1);
                self.reg.write(rd, rs1 | imm as u32);
            }
            Opcode::Andi { rd, rs1, imm } => {
                let rs1 = self.reg.read(rs1);
                self.reg.write(rd, rs1 & imm as u32);
            }
            Opcode::Slli { rd, rs1, shamt } => {
                let rs1 = self.reg.read(rs1);
                self.reg.write(rd, rs1 << shamt);
            }
            Opcode::Srli { rd, rs1, shamt } => {
                let rs1 = self.reg.read(rs1);
                self.reg.write(rd, rs1 >> shamt);
            }
            Opcode::Srai { rd, rs1, shamt } => {
                let rs1 = self.reg.read(rs1);
                self.reg.write(rd, rs1 >> shamt);
            }
            Opcode::Add { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 + rs2);
            }
            Opcode::Sub { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 - rs2);
            }
            Opcode::Sll { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 << rs2);
            }
            Opcode::Slt { rd, rs1, rs2 } => {
                if rs1 < rs2 {
                    self.reg.write(rd, 1u32);
                } else {
                    self.reg.write(rd, 0u32);
                }
            }
            Opcode::Sltu { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                if rs1 < rs2 {
                    self.reg.write(rd, 1u32);
                } else {
                    self.reg.write(rd, 0u32);
                }
            }
            Opcode::Xor { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 ^ rs2);
            }
            Opcode::Srl { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 >> rs2);
            }
            Opcode::Sra { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2) as i32;
                self.reg.write(rd, (rs1 >> rs2) as u32);
            }
            Opcode::Or { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 | rs2);
            }
            Opcode::And { rd, rs1, rs2 } => {
                let rs1 = self.reg.read(rs1);
                let rs2 = self.reg.read(rs2);
                self.reg.write(rd, rs1 & rs2);
            }
            Opcode::Fence => {}
            Opcode::Jalr { rd, rs1, offset } => {
                let rs1 = self.reg.read(rs1);
                let t = self.pc;
                self.pc = rs1.wrapping_add(offset as u32).wrapping_sub(4) &!1;
                self.reg.write(rd, t);
            }
        }
    }
}
