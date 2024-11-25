mod registers;
use std::{fs::File, io::Read};

use registers::{Cond, Register};

mod trapcodes;
use trapcodes::{Mmr, TrapCodes};

pub(crate) mod opcodes;
use opcodes::Opcodes;

use crate::assembler::{decode_instruction, Instruction};

// Word size = 16 bits
// Max addressable memory = 2^16 = 1 << 16 = 65536
// Instructions are 16 bits wide and have 4 bits opcodes
// Instruction size = 16 bits; max value = 1 << 16 = 65536 = 0x10000
// Opcode size = 4 bits ; max value = 1 << 4 = 16

const MAX_ADDRESSABLE_MEMORY: usize = 1 << 16;
const TOTAL_REGISTERS: usize = 10;

#[derive(Debug)]
struct Vm {
    running: bool,
    memory: [u16; MAX_ADDRESSABLE_MEMORY],
    registers: [u16; TOTAL_REGISTERS],
}

impl Vm {
    // Initializes the vm
    fn initialize() -> Self {
        // Initialize the vm
        let mut vm = Vm {
            running: false,
            memory: [0; MAX_ADDRESSABLE_MEMORY],
            registers: [0; TOTAL_REGISTERS],
        };

        // sets the conditional register to zero
        vm.set_register(Register::Cond as u16, Cond::Zro as u16);

        // sets the program counter to 0x3000
        vm.set_register(Register::Pc as u16, 0x3000);

        vm
    }

    // Loads a program to memory
    fn load_program(&mut self, program: Vec<u16>) {
        let mut program_start_address = program[0];

        assert!(
            program.len() <= MAX_ADDRESSABLE_MEMORY - program_start_address as usize,
            "Bytecode is too long"
        );

        for i in 1..program.len() {
            self.mem_write(program_start_address.into(), program[i].into());
            program_start_address += 1;
        }
    }

    fn load_program_from_file(&mut self, path: String) {
        let mut file = File::open(path).unwrap();

        let mut prog = vec![];

        file.read_to_end(&mut prog).unwrap();

        let mut program = vec![];

        let mut i = 0;

        while i < prog.len() {
            program.push((prog[i] as u16) << 8 | (prog[i + 1] as u16));
            i += 2;
        }

        self.load_program(program);
    }

    fn run(&mut self) {
        self.running = true;

        while self.running {
            let instruction = self.fetch();
            let instr = decode_instruction(instruction);
            self.update_pc();
            self.execute(instr);
        }
    }

    fn update_pc(&mut self) {
        self.set_register(
            Register::Pc as u16,
            self.get_register(Register::Pc as u16) + 1,
        );
    }

    // Fetches an instruction from memory
    fn fetch(&mut self) -> u16 {
        self.mem_read(self.get_register(Register::Pc as u16))
    }

    // Executes an instruction
    fn execute(&mut self, instruction: Instruction) {
        let opcode: Opcodes = instruction.opcode;

        match opcode {
            Opcodes::Br => {
                let cond = self.get_register(Register::Cond as u16);

                if (instruction.nzp & cond) > 0 {
                    self.set_register(
                        Register::Pc as u16,
                        sign_extend(instruction.pc_offset_9, 9)
                            .wrapping_add(self.get_register(Register::Pc as u16)),
                    );
                }
            }

            Opcodes::Add => {
                let val2 = if instruction.imm_or_cond_flag == 0 {
                    self.get_register(instruction.sr2)
                } else {
                    sign_extend(instruction.imm5, 5)
                };

                self.set_register(
                    instruction.dr,
                    self.get_register(instruction.sr1).wrapping_add(val2),
                );
                self.update_flag(instruction.dr);
            }

            Opcodes::Ld => {
                let memory_address = sign_extend(instruction.pc_offset_9, 9)
                    .wrapping_add(self.get_register(Register::Pc as u16));
                let val = self.mem_read(memory_address);
                self.set_register(instruction.dr, val);
                self.update_flag(instruction.dr);
            }

            Opcodes::St => {
                self.mem_write(
                    sign_extend(instruction.pc_offset_9, 9)
                        .wrapping_add(self.get_register(Register::Pc as u16)),
                    self.get_register(instruction.sr1),
                );
            }

            Opcodes::Jsr => {
                self.set_register(Register::R7 as u16, self.get_register(Register::Pc as u16));

                if instruction.imm_or_cond_flag == 1 {
                    self.set_register(
                        Register::Pc as u16,
                        sign_extend(instruction.pc_offset_11, 11)
                            .wrapping_add(self.get_register(Register::Pc as u16)),
                    );
                } else {
                    self.set_register(Register::Pc as u16, self.get_register(instruction.base_r));
                }
            }

            Opcodes::And => {
                let val2 = if instruction.imm_or_cond_flag == 1 {
                    sign_extend(instruction.imm5, 5)
                } else {
                    self.get_register(instruction.sr2)
                };
                self.set_register(instruction.dr, self.get_register(instruction.sr1) & val2);
                self.update_flag(instruction.dr);
            }

            Opcodes::Ldr => {
                let val = self.mem_read(
                    sign_extend(instruction.offset_6, 6)
                        .wrapping_add(self.get_register(instruction.base_r)),
                );
                self.set_register(instruction.dr, val);
                self.update_flag(instruction.dr);
            }

            Opcodes::Str => {
                self.mem_write(
                    sign_extend(instruction.offset_6, 6)
                        .wrapping_add(self.get_register(instruction.base_r)),
                    self.get_register(instruction.sr1),
                );
            }

            Opcodes::Rti => unimplemented!(),

            Opcodes::Not => {
                self.set_register(instruction.dr, !self.get_register(instruction.sr1));
                self.update_flag(instruction.dr);
            }

            Opcodes::Ldi => {
                let memory_address = sign_extend(instruction.pc_offset_9, 9)
                    .wrapping_add(self.get_register(Register::Pc as u16));
                let value_address = self.mem_read(memory_address);

                let val = self.mem_read(value_address);
                self.set_register(instruction.dr, val);
                self.update_flag(instruction.dr);
            }

            Opcodes::Sti => {
                let addr = self.mem_read(
                    sign_extend(instruction.pc_offset_9, 9)
                        .wrapping_add(self.get_register(Register::Pc as u16)),
                );
                self.mem_write(addr, self.get_register(instruction.sr1));
            }

            Opcodes::Jmp => {
                self.set_register(Register::Pc as u16, self.get_register(instruction.base_r));
            }

            Opcodes::Res => unimplemented!(),

            Opcodes::Lea => {
                self.set_register(
                    instruction.dr,
                    sign_extend(instruction.pc_offset_9, 9)
                        .wrapping_add(self.get_register(Register::Pc as u16)),
                );
                self.update_flag(instruction.dr);
            }

            Opcodes::Trap => {
                let trap_instruction: TrapCodes = instruction.trap_vect_8.into();

                trap_instruction.execute(self);
            }
        }
    }

    fn get_register(&self, register_address: u16) -> u16 {
        self.registers[register_address as usize]
    }

    fn set_register(&mut self, register_address: u16, value: u16) {
        self.registers[register_address as usize] = value;
    }

    fn mem_read(&mut self, memory_address: u16) -> u16 {
        if memory_address == Mmr::Kbsr as u16 {
            self.mem_write(memory_address, 1 << 15);
            let mut val = [0];
            std::io::stdin().read_exact(&mut val).unwrap();
            self.mem_write(Mmr::Kbdr as u16, val[0] as u16);
        } else {
            self.mem_write(Mmr::Kbsr as u16, 0);
        }
        self.memory[memory_address as usize]
    }

    fn mem_write(&mut self, memory_address: u16, value: u16) {
        self.memory[memory_address as usize] = value
    }

    fn update_flag(&mut self, register_address: u16) {
        if self.get_register(register_address) == 0 {
            self.set_register(Register::Cond as u16, Cond::Zro as u16);
        } else if (self.get_register(register_address) >> 15) == 1 {
            self.set_register(Register::Cond as u16, Cond::Neg as u16);
        } else {
            self.set_register(Register::Cond as u16, Cond::Pos as u16);
        }
    }
}

fn sign_extend(value: u16, bit_count: usize) -> u16 {
    if (value >> (bit_count - 1)) & 1 == 1 {
        (0xFFFF << bit_count) | value
    } else {
        value
    }
}

#[cfg(test)]
mod tests {

    // Instructions
    // 1EAA -> 0001 111 010 1 01010
    // FEAA -> 1111 111 010 1 01010
    // EAA ->  0000 111 010 1 01010

    use crate::vm::{decode_instruction, sign_extend, Register, Vm};

    fn create_vm() -> Vm {
        Vm::initialize()
    }

    #[test]
    fn test_add_instruction() {
        let mut vm = create_vm();
        // set register 2 for immediate value 50
        vm.set_register(0x2, 50);
        assert_eq!(vm.get_register(0x2), 50);
        // Run instruction 0x1EAA
        // 1EAA -> 0001 111 010 1 01010
        vm.execute(decode_instruction(0x1EAA));
        assert_eq!(vm.get_register(Register::R7 as u16), 60);
        assert_eq!(vm.get_register(Register::Cond as u16), 1);
    }

    #[test]
    fn test_ldi_instruction() {
        let mut vm = create_vm();

        let pc_offset = 0xBB;
        let memory_address =
            sign_extend(pc_offset, 9).wrapping_add(vm.get_register(Register::Pc as u16));

        vm.mem_write(memory_address, 98);
        assert_eq!(vm.mem_read(memory_address), 98);

        vm.mem_write(0x62, 10);
        assert_eq!(vm.mem_read(0x62), 10);

        // Run instruction
        //  A7BB -> 1010 011 110111011
        //  A6BB -> 1010 011 010111011
        vm.execute(decode_instruction(0xA6BB));

        assert_eq!(vm.get_register(0x3), 10);
        assert_eq!(vm.get_register(Register::Cond as u16), 1);
    }

    #[test]
    fn test_br_instruction() {
        let mut vm = create_vm();

        // Instruction
        // 575 -> 0000 010 101110101
        // 475 -> 0000 010 001110101
        dbg!(vm.get_register(Register::Pc as u16));
        vm.execute(decode_instruction(0x475));
        dbg!(vm.get_register(Register::Pc as u16));
    }

    #[test]
    fn test_load_program() {
        let mut vm = create_vm();

        vm.set_register(0x2, 50);

        let program = vec![0x3000, 0x1EAA];
        vm.load_program(program);
        let instruction = vm.fetch();

        vm.execute(decode_instruction(instruction));

        assert_eq!(vm.get_register(Register::R7 as u16), 60);
        assert_eq!(vm.get_register(Register::Cond as u16), 1);
    }

    #[test]
    fn test_load_program_from_file() {
        let mut vm = create_vm();

        vm.load_program_from_file(String::from("src/examples/2048.obj"));

        dbg!(vm.get_register(Register::Pc as u16));

        let instruction = vm.fetch();
        dbg!(instruction);

        vm.execute(decode_instruction(instruction));

        dbg!(vm.get_register(Register::Pc as u16));
    }

    #[test]
    fn test_run_program() {
        let mut vm = create_vm();

        vm.load_program_from_file(String::from("src/examples/2048.obj"));
        // vm.load_program_from_file(String::from("src/examples/rogue.obj"));
        // vm.load_program_from_file(String::from("src/examples/hello-world.obj"));

        vm.run();
    }
}
