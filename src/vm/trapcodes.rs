use std::io::{self, Read};

use super::{Register, Vm};

pub(crate) enum TrapCodes {
    Getc = 0x20, // gets character from keyboard, does not echo to the terminal
    Out,         // outputs a character
    Puts,        // outputs a word string
    In,          // gets character from keyboard and echoes to terminal
    Putsp,       // outputs a byte string
    Halt,        // halts the program
}

impl TrapCodes {
    pub(crate) fn execute(&self, vm: &mut Vm) {
        match self {
            TrapCodes::Getc => {
                let mut buffer = [0; 1];
                io::stdin().read(&mut buffer).unwrap();
                vm.set_register(Register::R0 as u16, buffer[0] as u16);
                vm.update_flag(Register::R0 as u16);
            }

            TrapCodes::Out => {
                println!(
                    "{}",
                    (vm.get_register(Register::R0 as u16) & 0xFF) as u8 as char
                );
            }

            TrapCodes::Puts => {
                let mut r0 = vm.get_register(Register::R0 as u16);
                while vm.mem_read(r0) != 0 {
                    print!("{}", vm.mem_read(r0) as u8 as char);
                    r0 += 1;
                }
            }

            TrapCodes::In => {
                println!("Please pass in a value!");
                let mut buffer = [0; 1];
                io::stdin().read(&mut buffer).unwrap();
                println!("{}", buffer[0] as u8 as char);
                vm.set_register(Register::R0 as u16, buffer[0] as u16);
                vm.update_flag(Register::R0 as u16);
            }

            TrapCodes::Putsp => {
                let mut r0 = vm.get_register(Register::R0 as u16);

                while vm.mem_read(r0) != 0 {
                    print!(
                        "{}{}",
                        (vm.mem_read(r0) >> 8) as u8 as char,
                        vm.mem_read(r0) as u8 as char
                    );
                    r0 += 1;
                }
            }

            TrapCodes::Halt => {
                vm.running = false;
                println!("Program execution halted")
            }
        }
    }
}

impl Into<TrapCodes> for u16 {
    fn into(self) -> TrapCodes {
        if self == 0x20 {
            TrapCodes::Getc
        } else if self == 0x21 {
            TrapCodes::Out
        } else if self == 0x22 {
            TrapCodes::Puts
        } else if self == 0x23 {
            TrapCodes::In
        } else if self == 0x24 {
            TrapCodes::Putsp
        } else if self == 0x25 {
            TrapCodes::Halt
        } else {
            dbg!(self);
            panic!("Invalid trapcode")
        }
    }
}

// Device Register Assignment
// Memory mapped registers
pub(crate) enum Mmr {
    Kbsr = 0xFE00, // keyboard status register
    Kbdr = 0xFE02, // keyboard data register
    Dsr = 0xFE04,  // display status register
    Ddr = 0xFE06,  // display data register
    Mcr = 0xFFFE,  // machine control register
}

#[cfg(test)]
mod tests {
    use crate::vm::Vm;

    use super::Register;

    fn create_vm() -> Vm {
        Vm::initialize()
    }

    #[test]
    fn test_puts_trapcode() {
        let mut vm = create_vm();
        let r0 = vm.get_register(Register::R0 as u16);
        vm.mem_write(r0, 0x61);
        vm.mem_write(r0 + 1, 0x62);
        vm.mem_write(r0 + 2, 0x63);
        vm.mem_write(r0 + 3, 0x0a);
        vm.mem_write(r0 + 4, 0x0);
        vm.mem_write(r0 + 5, 0x63);

        // instruction
        // F022 -> 1111 0000 00100010
        vm.execute(0xF022);
    }

    #[test]
    fn test_getc_trapcode() {
        let mut vm = create_vm();

        // instruction
        // F020 -> 1111 0000 00100000
        vm.execute(0xF020);
    }

    #[test]
    fn test_out_trapcode() {
        let mut vm = create_vm();
        vm.set_register(Register::R0 as u16, 98);

        // instruction
        // F021 -> 1111 0000 00100001
        vm.execute(0xF021);
    }

    #[test]
    fn test_in_trapcode() {
        let mut vm = create_vm();

        // Instruction
        // FO23 -> 1111 0000 00100011
        vm.execute(0xF023);
        dbg!(vm.get_register(Register::R0 as u16));
    }

    #[test]
    fn test_halt_trapcode() {
        let mut vm = create_vm();

        // Instruction
        // FO25 -> 1111 0000 00100101
        vm.execute(0xF025);
    }

    #[test]
    fn test_putsp_trapcode() {
        let mut vm = create_vm();
        let r0 = vm.get_register(Register::R0 as u16);
        vm.mem_write(r0, 0x6162);
        vm.mem_write(r0 + 1, 0x6364);
        vm.mem_write(r0 + 2, 0x0a);

        // instruction
        // F024 -> 1111 0000 00100100
        vm.execute(0xF024);
    }
}
