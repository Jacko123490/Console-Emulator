mod instructions;
mod registers;

use registers::CPURegisters;

pub struct SharpSM83Core {
    registers: CPURegisters
}

impl SharpSM83Core {
    pub fn new() -> SharpSM83Core {
        SharpSM83Core {
            registers: CPURegisters::new(),
        }
    }
}
