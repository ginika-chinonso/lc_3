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
            Opcodes::Br => todo!(),
            Opcodes::Add => todo!(),
            Opcodes::Ld => todo!(),
            Opcodes::St => todo!(),
            Opcodes::Jsr => todo!(),
            Opcodes::And => todo!(),
            Opcodes::Ldr => todo!(),
            Opcodes::Str => todo!(),
            Opcodes::Rti => todo!(),
            Opcodes::Not => todo!(),
            Opcodes::Ldi => todo!(),
            Opcodes::Sti => todo!(),
            Opcodes::Jmp => todo!(),
            Opcodes::Res => todo!(),
            Opcodes::Lea => todo!(),
            Opcodes::Trap => todo!(),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode {
            Opcodes::Br => f.write_fmt(format_args!("{} {}", self.opcode, self.pc_offset_9)),
            Opcodes::Add => {
                if self.imm_or_cond_flag == 1 {
                    f.write_fmt(format_args!(
                        "{} {} {} {}",
                        self.opcode, self.dr, self.sr1, self.imm5
                    ))
                } else {
                    f.write_fmt(format_args!(
                        "{} {} {} {}",
                        self.opcode, self.dr, self.sr1, self.sr2
                    ))
                }
            }
            Opcodes::Ld => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.dr, self.pc_offset_9
            )),
            Opcodes::St => f.write_fmt(format_args!(
                "{} {} {}",
                self.opcode, self.sr1, self.pc_offset_9
            )),
            Opcodes::Jsr => {
                if self.imm_or_cond_flag == 1 {
                    f.write_fmt(format_args!("{} {}", self.opcode, self.pc_offset_11))
                } else {
                    f.write_fmt(format_args!("{}R {}", self.opcode, self.base_r))
                }
            }
            Opcodes::And => {
                if self.imm_or_cond_flag == 1 {
                    f.write_fmt(format_args!(
                        "{} {} {} {}",
                        self.opcode, self.dr, self.sr1, self.imm5
                    ))
                } else {
                    f.write_fmt(format_args!(
                        "{} {} {} {}",
                        self.opcode, self.dr, self.sr1, self.sr2
                    ))
                }
            }
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
                if self.base_r == 0x7 {
                    f.write_fmt(format_args!("RET {}", self.base_r))
                } else {
                    f.write_fmt(format_args!("{} {}", self.opcode, self.base_r))
                }
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
    use super::decode_instruction;

    #[test]
    fn test_decode_instruction() {
        let instruction = decode_instruction(0x475);
        println!("{}", instruction);
    }
}
