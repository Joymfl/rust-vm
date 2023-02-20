const MEMORY_MAX:usize = u16::MAX as usiz + 1;
pub struct Memory {
    pub cells: [u16;MEMORY_MAX],
}

impl Memory{
    fn new() -> Self {
        Memory{
            cells: [0;MEMORY_MAX],
        }
    }
}