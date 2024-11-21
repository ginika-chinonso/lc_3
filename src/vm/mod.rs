mod registers;
use std::{fs::File, io::Read};

use registers::{Cond, Register};

mod trapcodes;
use trapcodes::TrapCodes;

pub(crate) mod opcodes;
use opcodes::Opcodes;

use crate::assembler::decode_instruction;

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
            dbg!(instruction);
            println!("{}", decode_instruction(instruction));

            self.execute(instruction);

            self.set_register(
                Register::Pc as u16,
                self.get_register(Register::Pc as u16) + 1,
            );
        }
    }

    // Fetches an instruction from memory
    fn fetch(&self) -> u16 {
        self.mem_read(self.get_register(Register::Pc as u16))
    }

    // Executes an instruction
    fn execute(&mut self, instruction: u16) {
        let opcode: Opcodes = (instruction >> 12).into();

        match opcode {
            Opcodes::Br => {
                let nzp = (instruction >> 9) & 0x7;
                let pc_offset_9 = instruction & 0x1FF;

                let cond = self.get_register(Register::Cond as u16);

                if (nzp & cond) > 0 {
                    self.set_register(
                        Register::Pc as u16,
                        sign_extend(pc_offset_9, 9)
                            .wrapping_add(self.get_register(Register::Pc as u16)),
                    );
                }
            }

            Opcodes::Add => {
                let dr = (instruction >> 9) & 0x7; // destination register
                let sr = self.get_register((instruction >> 6) & 0x7); // source register
                let imm = (instruction >> 5) & 0x1; // immediate mode
                let val2 = if imm == 0 {
                    self.get_register(instruction & 0x7)
                } else {
                    sign_extend(instruction & 0x1F, 5)
                };

                self.set_register(dr, sr.wrapping_add(val2));
                self.update_flag(dr);
            }

            Opcodes::Ld => {
                let dr = (instruction >> 9) & 0x7;
                let pc_offset_9 = instruction & 0x1FF;
                let memory_address = sign_extend(pc_offset_9, 9)
                    .wrapping_add(self.get_register(Register::Pc as u16));
                self.set_register(dr, self.mem_read(memory_address));
                self.update_flag(dr);
            }

            Opcodes::St => {
                let sr = (instruction >> 9) & 0x7;
                let pc_offset_9 = instruction & 0x1FF;
                self.mem_write(
                    sign_extend(pc_offset_9, 9)
                        .wrapping_add(self.get_register(Register::Pc as u16)),
                    self.get_register(sr),
                );
            }

            Opcodes::Jsr => {
                self.set_register(Register::R7 as u16, self.get_register(Register::Pc as u16));

                if ((instruction >> 11) & 1) == 1 {
                    let pc_offset_11 = instruction & 0x7FF;
                    self.set_register(
                        Register::Pc as u16,
                        sign_extend(pc_offset_11, 11)
                            .wrapping_add(self.get_register(Register::Pc as u16)),
                    );
                } else {
                    let base_r = (instruction >> 6) & 0x7;
                    self.set_register(Register::Pc as u16, self.get_register(base_r));
                }
            }

            Opcodes::And => {
                let dr = (instruction >> 9) & 0x7;
                let sr1 = (instruction >> 6) & 0x7;
                let val2 = if ((instruction >> 5) & 1) == 1 {
                    sign_extend(instruction & 0x1F, 5)
                } else {
                    self.get_register(instruction & 0x7)
                };
                self.set_register(dr, self.get_register(sr1) & val2);
                self.update_flag(dr);
            }

            Opcodes::Ldr => {
                let dr = (instruction >> 9) & 0x7;
                let base_r = (instruction >> 6) & 0x7;
                let offset_6 = instruction & 0x3F;
                self.set_register(
                    dr,
                    self.mem_read(sign_extend(offset_6, 6).wrapping_add(self.get_register(base_r))),
                );
                self.update_flag(dr);
            }

            Opcodes::Str => {
                let sr = (instruction >> 9) & 0x7;
                let base_r = (instruction >> 6) & 0x7;
                let offset_6 = instruction & 0x3F;
                self.mem_write(
                    sign_extend(offset_6, 6).wrapping_add(self.get_register(base_r)),
                    self.get_register(sr),
                );
            }

            Opcodes::Rti => unimplemented!(),

            Opcodes::Not => {
                let dr = (instruction >> 9) & 0x7;
                let sr = (instruction >> 6) & 0x7;
                self.set_register(dr, !self.get_register(sr));
                self.update_flag(dr);
            }

            Opcodes::Ldi => {
                let dr = (instruction >> 9) & 0x7;
                let pc_offset_9 = instruction & 0x1FF;
                let memory_address = sign_extend(pc_offset_9, 9)
                    .wrapping_add(self.get_register(Register::Pc as u16));
                let value_address = self.mem_read(memory_address);

                self.set_register(dr, self.mem_read(value_address));
                self.update_flag(dr);
            }

            Opcodes::Sti => {
                let sr = (instruction >> 9) & 0x7;
                let pc_offset_9 = instruction & 0x1FF;
                self.mem_write(
                    self.mem_read(
                        sign_extend(pc_offset_9, 9)
                            .wrapping_add(self.get_register(Register::Pc as u16)),
                    ),
                    self.get_register(sr),
                );
            }

            Opcodes::Jmp => {
                let base_r = (instruction >> 6) & 0x7;
                self.set_register(Register::Pc as u16, self.get_register(base_r));
            }

            Opcodes::Res => unimplemented!(),

            Opcodes::Lea => {
                let dr = (instruction >> 9) & 0x7;
                let pc_offset_9 = instruction & 0x1FF;
                self.set_register(
                    dr,
                    sign_extend(pc_offset_9, 9)
                        .wrapping_add(self.get_register(Register::Pc as u16)),
                );
                self.update_flag(dr);
            }

            Opcodes::Trap => {
                let trapvect_8 = instruction & 0xFF;

                let trap_instruction: TrapCodes = trapvect_8.into();

                trap_instruction.execute(self);

                // self.set_register(Register::R7 as u16, self.get_register(Register::Pc as u16));
                // self.set_register(Register::Pc as u16, self.mem_read(trapvect_8));
                // self.set_register(Register::Pc as u16, self.get_register(Register::R7 as u16));
                // self.update_flag(Register::Pc as u16);
            }
        }
    }

    fn get_register(&self, register_address: u16) -> u16 {
        self.registers[register_address as usize]
    }

    fn set_register(&mut self, register_address: u16, value: u16) {
        self.registers[register_address as usize] = value;
    }

    fn mem_read(&self, memory_address: u16) -> u16 {
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

    use crate::vm::{sign_extend, Register, Vm};

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
        vm.execute(0x1EAA);
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
        //  A7BB -> 1010 011 110111011 -> throwing overflow error
        //  A6BB -> 1010 011 010111011
        vm.execute(0xA6BB);

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
        vm.execute(0x475);
        dbg!(vm.get_register(Register::Pc as u16));
    }

    #[test]
    fn test_load_program() {
        let mut vm = create_vm();

        vm.set_register(0x2, 50);

        let program = vec![0x3000, 0x1EAA];
        vm.load_program(program);
        let instruction = vm.fetch();

        vm.execute(instruction);

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

        vm.execute(instruction);

        dbg!(vm.get_register(Register::Pc as u16));
    }

    #[test]
    fn test_run_program() {
        let mut vm = create_vm();

        // vm.load_program_from_file(String::from("src/examples/2048.obj"));
        // vm.load_program_from_file(String::from("src/examples/rogue.obj"));
        vm.load_program_from_file(String::from("src/examples/hello-world.obj"));

        vm.run();
    }
}
