pub enum OpCodes {
    Br  = 0,    // Branch
    Add = 1,    // Add
    Ld = 2,     // Load
    St = 3,     // Store
    Jsr = 4,    // Jump Register
    And = 5,    // Bitwise And
    Ldr = 6,    // Load Register
    Str = 7,    // Store Register
    Rti = 8,    // Unused
    Not = 9,    // Bitwise Not
    Ldi = 10,   // Load Indirect
    Sti = 11,   // Store Indirect
    Jmp = 12,   // Jump
    Res = 13,   // Reserved (unused)
    Lea = 14,   // Load effective address
    Trap = 15,  // Execute Trap
}