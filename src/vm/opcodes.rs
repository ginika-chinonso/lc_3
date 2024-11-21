use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum Opcodes {
    Br,   // branch
    Add,  // add
    Ld,   // load
    St,   // store
    Jsr,  // jump register
    And,  // and
    Ldr,  // load register
    Str,  // store register
    Rti,  // unused -> Return from interupt
    Not,  // bitwise not
    Ldi,  // load indirect
    Sti,  // store indirect
    Jmp,  // jump
    Res,  // reserved(unused)
    Lea,  // load effective addresses
    Trap, // execute trap
}

impl Display for Opcodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcodes::Br => f.write_str("BR"),
            Opcodes::Add => f.write_str("ADD"),
            Opcodes::Ld => f.write_str("LD"),
            Opcodes::St => f.write_str("ST"),
            Opcodes::Jsr => f.write_str("JSR"),
            Opcodes::And => f.write_str("AND"),
            Opcodes::Ldr => f.write_str("LDR"),
            Opcodes::Str => f.write_str("STR"),
            Opcodes::Rti => f.write_str("RTI"),
            Opcodes::Not => f.write_str("NOT"),
            Opcodes::Ldi => f.write_str("LDI"),
            Opcodes::Sti => f.write_str("STI"),
            Opcodes::Jmp => f.write_str("JMP"),
            Opcodes::Res => f.write_str("RES"),
            Opcodes::Lea => f.write_str("LEA"),
            Opcodes::Trap => f.write_str("TRAP"),
        }
    }
}

impl Into<Opcodes> for u16 {
    fn into(self) -> Opcodes {
        if self == 0 {
            Opcodes::Br
        } else if self == 1 {
            Opcodes::Add
        } else if self == 2 {
            Opcodes::Ld
        } else if self == 3 {
            Opcodes::St
        } else if self == 4 {
            Opcodes::Jsr
        } else if self == 5 {
            Opcodes::And
        } else if self == 6 {
            Opcodes::Ldr
        } else if self == 7 {
            Opcodes::Str
        } else if self == 8 {
            Opcodes::Rti
        } else if self == 9 {
            Opcodes::Not
        } else if self == 10 {
            Opcodes::Ldi
        } else if self == 11 {
            Opcodes::Sti
        } else if self == 12 {
            Opcodes::Jmp
        } else if self == 13 {
            Opcodes::Res
        } else if self == 14 {
            Opcodes::Lea
        } else if self == 15 {
            Opcodes::Trap
        } else {
            panic!("Invalid opcode")
        }
    }
}
