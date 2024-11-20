#[derive(Debug)]
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
