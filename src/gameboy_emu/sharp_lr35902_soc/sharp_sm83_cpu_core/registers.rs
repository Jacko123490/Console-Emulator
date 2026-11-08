use rand;

use crate::hardware_definitions::cpu::register::{Register, SingleRegister, DualRegister};
use crate::dev_utils::{mem_entry_bits, mem_entry_display};

pub(super) struct CPURegisters {
    pub af: DualRegister, // AF (Hi: A, Lo: -) Accumulator, Flags
    pub bc: DualRegister, // BC (Hi: B, Lo: C)
    pub de: DualRegister, // DE (Hi: D, Lo: E)
    pub hl: DualRegister, // HL (Hi: H, Lo: L)
    pub sp: SingleRegister, // Stack Pointer
    pub pc: SingleRegister, // Program Counter
}

impl CPURegisters {
    pub fn new() -> CPURegisters {
        CPURegisters {
            af: rand::random::<u16>().into(),
            bc: rand::random::<u16>().into(),
            de: rand::random::<u16>().into(),
            hl: rand::random::<u16>().into(),
            sp: rand::random::<u16>().into(),
            pc: 0.into(),
        }
    }

    // Flag accessors
    ///Zero Flag
    pub fn flag_z(&self) -> bool { self.get_flag(7) }
    pub fn set_flag_z(&mut self, flag: bool) { self.set_flag(7, flag); }
    ///Subtraction Flag (BCD)
    pub fn flag_n(&self) -> bool { self.get_flag(6) }
    pub fn set_flag_n(&mut self, flag: bool) { self.set_flag(6, flag); }
    ///Half Carry Flag (BCD)
    pub fn flag_h(&self) -> bool { self.get_flag(5) }
    pub fn set_flag_h(&mut self, flag: bool) { self.set_flag(5, flag); }
    ///Carry Flag
    pub fn flag_c(&self) -> bool { self.get_flag(4) }
    pub fn set_flag_c(&mut self, flag: bool) { self.set_flag(4, flag); }

    fn get_flag(&self, index: u8) -> bool { self.af.lo() & (1 << index) != 0 }
    fn set_flag(&mut self, index: u8, value: bool) {
        if value {
            self.af.set_lo(self.af.lo() | (1 << index))
        } else {
            self.af.set_lo(self.af.lo() & !(1 << index))
        }
    }

    /// Initialise the Registers to bootup values
    pub fn init(&mut self) {
        self.af = rand::random::<u16>().into();
        self.bc = rand::random::<u16>().into();
        self.de = rand::random::<u16>().into();
        self.hl = rand::random::<u16>().into();
        self.sp = rand::random::<u16>().into();
        self.pc = 0.into();
    }

    // Debug
    pub fn print_registers(&self) {
        println!("AF | A: {}, F: {}", mem_entry_display(self.af.hi()), mem_entry_bits(self.af.lo()));
        println!("BC | B: {}, C: {}", mem_entry_display(self.bc.hi()), mem_entry_display(self.bc.lo()));
        println!("DE | D: {}, E: {}", mem_entry_display(self.de.hi()), mem_entry_display(self.de.lo()));
        println!("HL | H: {}, L: {}", mem_entry_display(self.hl.hi()), mem_entry_display(self.hl.lo()));
        println!("SP | {}", mem_entry_display(self.sp.value()));
        println!("PC | {}", mem_entry_display(self.pc.value()));
    }
}