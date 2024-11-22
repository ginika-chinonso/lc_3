use std::fmt::Display;

use crate::vm::opcodes::Opcodes;

#[derive(Debug)]
pub(crate) struct Instruction {
    opcode: Opcodes,
    dr: u16,
    sr1: u16,
    sr2: u16,
    imm5: u16,
    imm_or_cond_flag: u16,
    nzp: u16,
    base_r: u16,
    offset_6: u16,
    trap_vect_8: u16,
    pc_offset_9: u16,
    pc_offset_11: u16,
}

impl Instruction {
    fn new(opcode: Opcodes) -> Self {
        Instruction {
            opcode,
            dr: 0,
            sr1: 0,
            sr2: 0,
            imm5: 0,
            imm_or_cond_flag: 0,
            nzp: 0,
            base_r: 0,
            offset_6: 0,
            trap_vect_8: 0,
            pc_offset_9: 0,
            pc_offset_11: 0,
        }
    }

    fn encode(&self) -> u16 {
        match self.opcode {
            Opcodes::Br => ((Opcodes::Br as u16) << 12) | (self.nzp << 9) | self.pc_offset_9,
            Opcodes::Add => {
                let mut res = ((Opcodes::Add as u16) << 12) | (self.dr << 9) | (self.sr1 << 6);
                if self.imm_or_cond_flag == 1 {
                    res |= (1 << 5) | self.imm5;
                    res
                } else {
                    res |= self.sr2;
                    res
                }
            }
            Opcodes::Ld => ((Opcodes::Ld as u16) << 12) | self.dr << 9 | self.pc_offset_9,
            Opcodes::St => ((Opcodes::St as u16) << 12) | self.sr1 << 9 | self.pc_offset_9,
            Opcodes::Jsr => {
                let mut res = (Opcodes::Jsr as u16) << 12;
                if self.imm_or_cond_flag == 1 {
                    res |= 1 << 11 | self.pc_offset_11;
                    res
                } else {
                    res |= self.base_r << 6;
                    res
                }
            }
            Opcodes::And => {
                let mut res = (Opcodes::And as u16) << 12 | self.dr << 9 | self.sr1 << 6;
                if self.imm_or_cond_flag == 1 {
                    res |= 1 << 5 | self.imm5;
                    res
                } else {
                    res |= self.sr2;
                    res
                }
            }
            Opcodes::Ldr => {
                (Opcodes::Ldr as u16) << 12 | self.dr << 9 | self.base_r << 6 | self.offset_6
            }
            Opcodes::Str => {
                (Opcodes::Str as u16) << 12 | self.sr1 << 9 | self.base_r << 6 | self.offset_6
            }
            Opcodes::Rti => unimplemented!(),
            Opcodes::Not => (Opcodes::Not as u16) << 12 | self.dr << 9 | self.sr1 << 6 | 0x1F,
            Opcodes::Ldi => ((Opcodes::Ldi as u16) << 12) | self.dr << 9 | self.pc_offset_9,
            Opcodes::Sti => ((Opcodes::Sti as u16) << 12) | self.sr1 << 9 | self.pc_offset_9,
            Opcodes::Jmp => ((Opcodes::Jmp as u16) << 12) | self.base_r << 6,
            Opcodes::Res => unimplemented!(),
            Opcodes::Lea => ((Opcodes::Lea as u16) << 12) | self.dr << 9 | self.pc_offset_9,
            Opcodes::Trap => ((Opcodes::Trap as u16) << 12) | self.trap_vect_8,
        }
    }
}

fn encode_instruction_string(instruction: String) -> Instruction {
    let assembly_code = instruction.split(" ").collect::<Vec<&str>>();
    if assembly_code[0] == "BR" {
        let mut instr = Instruction::new(Opcodes::Br);
        instr.nzp = assembly_code[1].parse().unwrap();
        instr.pc_offset_9 = assembly_code[2].parse().unwrap();
        instr
    } else if assembly_code[0] == "ADD" {
        let mut instr = Instruction::new(Opcodes::Add);
        instr.dr = assembly_code[1].parse().unwrap();
        instr.sr1 = assembly_code[2].parse().unwrap();
        if assembly_code[3].parse::<u8>().unwrap() == 1 {
            instr.imm_or_cond_flag = 1;
            instr.imm5 = assembly_code[4].parse().unwrap();
        } else {
            instr.sr2 = assembly_code[4].parse().unwrap();
        }
        instr
    } else if assembly_code[0] == "LD" {
        let mut instr = Instruction::new(Opcodes::Ld);
        instr.dr = assembly_code[1].parse().unwrap();
        instr.pc_offset_9 = assembly_code[2].parse().unwrap();
        instr
    } else if assembly_code[0] == "ST" {
        let mut instr = Instruction::new(Opcodes::St);
        instr.sr1 = assembly_code[1].parse().unwrap();
        instr.pc_offset_9 = assembly_code[2].parse().unwrap();
        instr
    } else if assembly_code[0] == "JSR" {
        let mut instr = Instruction::new(Opcodes::Jsr);
        if assembly_code[1].parse::<u8>().unwrap() == 1 {
            instr.imm_or_cond_flag = 1;
            instr.pc_offset_11 = assembly_code[2].parse().unwrap();
        } else {
            instr.base_r = assembly_code[2].parse().unwrap();
        }
        instr
    } else if assembly_code[0] == "AND" {
        let mut instr = Instruction::new(Opcodes::And);
        instr.dr = assembly_code[1].parse().unwrap();
        instr.sr1 = assembly_code[2].parse().unwrap();
        if assembly_code[3].parse::<u8>().unwrap() == 1 {
            instr.imm_or_cond_flag = 1;
            instr.imm5 = assembly_code[4].parse().unwrap();
        } else {
            instr.sr2 = assembly_code[4].parse().unwrap();
        }
        instr
    } else if assembly_code[0] == "LDR" {
        let mut instr = Instruction::new(Opcodes::Ldr);
        instr.dr = assembly_code[1].parse().unwrap();
        instr.base_r = assembly_code[2].parse().unwrap();
        instr.offset_6 = assembly_code[3].parse().unwrap();
        instr
    } else if assembly_code[0] == "STR" {
        let mut instr = Instruction::new(Opcodes::Str);
        instr.sr1 = assembly_code[1].parse().unwrap();
        instr.base_r = assembly_code[2].parse().unwrap();
        instr.offset_6 = assembly_code[3].parse().unwrap();
        instr
    } else if assembly_code[0] == "RTI" {
        Instruction::new(Opcodes::Rti)
    } else if assembly_code[0] == "NOT" {
        let mut instr = Instruction::new(Opcodes::Not);
        instr.dr = assembly_code[1].parse().unwrap();
        instr.sr1 = assembly_code[2].parse().unwrap();
        instr.offset_6 = 0x3F;
        instr
    } else if assembly_code[0] == "LDI" {
        let mut instr = Instruction::new(Opcodes::Ldi);
        instr.dr = assembly_code[1].parse().unwrap();
        instr.pc_offset_9 = assembly_code[2].parse().unwrap();
        instr
    } else if assembly_code[0] == "STI" {
        let mut instr = Instruction::new(Opcodes::Sti);
        instr.sr1 = assembly_code[1].parse().unwrap();
        instr.pc_offset_9 = assembly_code[2].parse().unwrap();
        instr
    } else if assembly_code[0] == "JMP" {
        let mut instr = Instruction::new(Opcodes::Jmp);
        instr.base_r = assembly_code[1].parse().unwrap();
        instr
    } else if assembly_code[0] == "RES" {
        Instruction::new(Opcodes::Res)
    } else if assembly_code[0] == "LEA" {
        let mut instr = Instruction::new(Opcodes::Lea);
        instr.dr = assembly_code[1].parse().unwrap();
        instr.pc_offset_9 = assembly_code[2].parse().unwrap();
        instr
    } else if assembly_code[0] == "TRAP" {
        let mut instr = Instruction::new(Opcodes::Trap);
        instr.trap_vect_8 = assembly_code[1].parse().unwrap();
        instr
    } else {
        panic!("Incorrect instruction")
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode {
            Opcodes::Br => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.nzp, self.pc_offset_9
            )),
            Opcodes::Add => f.write_fmt(format_args!(
                "{} {} {} {} {}",
                self.opcode,
                self.dr,
                self.sr1,
                self.imm_or_cond_flag,
                if self.imm_or_cond_flag == 1 {
                    self.imm5
                } else {
                    self.sr2
                }
            )),
            Opcodes::Ld => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.dr, self.pc_offset_9
            )),
            Opcodes::St => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.sr1, self.pc_offset_9
            )),
            Opcodes::Jsr => {
                let op = self.opcode.to_string();
                f.write_fmt(format_args!(
                    "{} {} {}",
                    if self.imm_or_cond_flag == 1 {
                        op
                    } else {
                        "JSRR".to_string()
                    },
                    self.imm_or_cond_flag,
                    if self.imm_or_cond_flag == 1 {
                        self.pc_offset_11
                    } else {
                        self.base_r
                    }
                ))
            }
            Opcodes::And => f.write_fmt(format_args!(
                "{} {} {} {} {}",
                self.opcode,
                self.dr,
                self.sr1,
                self.imm_or_cond_flag,
                if self.imm_or_cond_flag == 1 {
                    self.imm5
                } else {
                    self.sr2
                }
            )),
            Opcodes::Ldr => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.dr, self.base_r, self.offset_6
            )),
            Opcodes::Str => f.write_fmt(format_args!(
                "{} {} {} {}",
                self.opcode, self.sr1, self.base_r, self.offset_6
            )),
            Opcodes::Rti => f.write_fmt(format_args!("{}", self.opcode)),
            Opcodes::Not => f.write_fmt(format_args!("{} {} {}", self.opcode, self.dr, self.sr1)),
            Opcodes::Ldi => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.dr, self.pc_offset_9
            )),
            Opcodes::Sti => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.sr1, self.pc_offset_9
            )),
            Opcodes::Jmp => {
                let op = self.opcode.to_string();
                f.write_fmt(format_args!(
                    "{} {}",
                    if self.base_r == 0x7 { "RET" } else { &op },
                    self.base_r
                ))
            }
            Opcodes::Res => f.write_fmt(format_args!("{}", self.opcode)),
            Opcodes::Lea => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.dr, self.pc_offset_9
            )),
            Opcodes::Trap => f.write_fmt(format_args!("{} {}", self.opcode, self.trap_vect_8)),
        }
    }
}

pub(crate) fn decode_instruction(instruction: u16) -> Instruction {
    let opcode: Opcodes = (instruction >> 12).into();
    let mut res = Instruction::new(opcode.clone());
    match opcode {
        Opcodes::Br => {
            res.nzp = (instruction >> 9) & 0x7;
            res.pc_offset_9 = instruction & 0x1FF;
            res
        }
        Opcodes::Add => {
            res.dr = (instruction >> 9) & 0x7;
            res.sr1 = (instruction >> 6) & 0x7;
            if (instruction >> 5) & 1 == 1 {
                res.imm5 = instruction & 0x1F;
                res
            } else {
                res.sr2 = instruction & 0x7;
                res
            }
        }
        Opcodes::Ld => {
            res.dr = (instruction >> 9) & 0x7;
            res.pc_offset_9 = instruction & 0x1FF;
            res
        }
        Opcodes::St => {
            res.sr1 = (instruction >> 9) & 0x7;
            res.pc_offset_9 = instruction & 0x1FF;
            res
        }
        Opcodes::Jsr => {
            if (instruction >> 11) & 1 == 1 {
                res.imm_or_cond_flag = 1;
                res.pc_offset_11 = instruction & 0x3FF;
                res
            } else {
                res.base_r = (instruction >> 6) & 0x7;
                res
            }
        }
        Opcodes::And => {
            res.dr = (instruction >> 9) & 0x7;
            res.sr1 = (instruction >> 6) & 0x7;
            if ((instruction >> 5) & 0x1) == 1 {
                res.imm5 = instruction & 0x1F;
                res
            } else {
                res.sr2 = instruction & 0x7;
                res
            }
        }
        Opcodes::Ldr => {
            res.dr = (instruction >> 9) & 0x7;
            res.base_r = (instruction >> 6) & 0x7;
            res.offset_6 = instruction & 0x3F;
            res
        }
        Opcodes::Str => {
            res.sr1 = (instruction >> 9) & 0x7;
            res.base_r = (instruction >> 6) & 0x7;
            res.offset_6 = instruction & 0x3F;
            res
        }
        Opcodes::Rti => unimplemented!(),
        Opcodes::Not => {
            res.dr = (instruction >> 9) & 0x7;
            res.sr1 = (instruction >> 6) & 0x7;
            res.offset_6 = 0x3F;
            res
        }
        Opcodes::Ldi => {
            res.dr = (instruction >> 9) & 0x7;
            res.pc_offset_9 = instruction & 0x1FF;
            res
        }
        Opcodes::Sti => {
            res.sr1 = (instruction >> 9) & 0x7;
            res.pc_offset_9 = instruction & 0x1FF;
            res
        }
        Opcodes::Jmp => {
            res.base_r = (instruction >> 6) & 0x7;
            res
        }
        Opcodes::Res => unimplemented!(),
        Opcodes::Lea => {
            res.dr = (instruction >> 9) & 0x7;
            res.pc_offset_9 = instruction & 0x1FF;
            res
        }
        Opcodes::Trap => {
            res.trap_vect_8 = instruction & 0xFF;
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::encode_instruction_string;

    use super::decode_instruction;

    #[test]
    fn test_decode_instruction() {
        let instruction = decode_instruction(0x475);
        println!("{}", instruction);
    }

    #[test]
    fn test_encode_instruction() {
        let instruction = decode_instruction(0x475);
        println!("{:0x}", instruction.encode());
        println!("{}", instruction);
        dbg!(format!("{:0x}", encode_instruction_string("BR 2 117".to_string()).encode()));
    }
}
