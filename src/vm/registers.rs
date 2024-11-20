pub(crate) enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    Pc,   // program counter
    Cond, // conditional
}

pub(crate) enum Cond {
    Pos = 1 << 0, // 1 -> 001
    Zro = 1 << 1, // 2 -> 010
    Neg = 1 << 2, // 4 -> 100
}
