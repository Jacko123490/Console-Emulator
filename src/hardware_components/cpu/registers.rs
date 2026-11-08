use std::fmt::{Display, Formatter};
use rand;
use crate::dev_utils::{mem_array_display, mem_entry_bits, mem_entry_display};

trait Register<T> {
    fn value(&self) -> T;
    fn set_value(&mut self, value: T);
}

///16-bit register which also supports accessing each byte as an 8-bit register.
///
/// Each 8-bit register can be accessed using the hi,lo methods to retrieve the high or low byte.
#[derive(Copy, Clone, Debug)]
pub struct DualRegister {
    register: [u8; 2],
}

impl DualRegister {
    pub fn new(register: [u8; 2]) -> Self { DualRegister { register } }

    pub fn hi(&self) -> u8 { self.register[1] }
    pub fn set_hi(&mut self, value: u8) { self.register[1] = value; }
    pub fn lo(&self) -> u8 { self.register[0] }
    pub fn set_lo(&mut self, value: u8) { self.register[0] = value; }
}

impl Register<u16> for DualRegister {
    fn value(&self) -> u16 { u16::from_le_bytes(self.register) }
    fn set_value(&mut self, value: u16) { self.register = value.to_le_bytes(); }
}

impl From<u16> for DualRegister {
    fn from(value: u16) -> Self { DualRegister::new(value.to_le_bytes()) }
}

impl Display for DualRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", mem_array_display(&self.register))
    }
}

///Standard 16-bit register
#[derive(Copy, Clone, Debug)]
pub struct SingleRegister {
    register: u16,
}

impl SingleRegister {
    pub fn new(register: u16) -> Self { SingleRegister { register } }
}

impl Register<u16> for SingleRegister {
    fn value(&self) -> u16 { self.register }
    fn set_value(&mut self, value: u16) { self.register = value; }
}

impl From<u16> for SingleRegister {
    fn from(value: u16) -> Self { SingleRegister::new(value) }
}

impl Display for SingleRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", mem_entry_display(&self.register))
    }
}

pub struct CPURegisters {
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