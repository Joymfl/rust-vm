use bitflags::bitflags;
bitflags! {
    pub enum {
        Pos = 1 << 0,   // Positive
        Zero = 1 << 1,  // Zero
        Neg = 1 << 2,   // Negative
    }
}