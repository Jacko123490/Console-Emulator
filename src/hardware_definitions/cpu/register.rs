use std::fmt::{Display, Formatter};
use crate::dev_utils::{mem_array_display, mem_entry_display};

pub trait Register<T> {
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