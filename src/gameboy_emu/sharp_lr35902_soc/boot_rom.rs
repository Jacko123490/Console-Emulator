use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use crate::hardware_definitions::memory::{AddressSpace, AddressableMemory};

#[derive(Debug)]
pub enum BootRomError {
    LengthError(GBBootRomType, String, usize, usize), // (ROM Type, Binary File, Expected Length, Actual Length)
    AddressLengthError(String, Vec<[u16; 2]>, usize), // (Binary File, Memory Address, Actual Length)
    AddressOverlapError(String, Vec<[u16; 2]>), // (Binary File, Memory Address)
}

impl Display for BootRomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            BootRomError::LengthError(rom_type, filename, expected_length, actual_length) => {
                write!(f, "The boot ROM {filename} is {actual_length}B long, {rom_type} ROMs should be {expected_length}B")
            },
            BootRomError::AddressLengthError(filename, address_ranges, actual_length) => {
                write!(f, "The boot ROM {filename} address ranges {address_ranges:?} do not match the ROM length, total address range should be {actual_length}B.")
            },
            BootRomError::AddressOverlapError(filename, address_ranges) => {
                write!(f, "The boot ROM {filename} address ranges {address_ranges:?} overlap.")
            },
        }
    }
}

impl Error for BootRomError {}

// GB Boot ROM
// ----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug)]
pub enum GBBootRomType {
    DMG0,
    DMG,
    MGB,
    SGB,
    SGB2,
}

impl Display for GBBootRomType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GBBootRomType::DMG0 => write!(f, "dmg0"),
            GBBootRomType::DMG => write!(f, "dmg"),
            GBBootRomType::MGB => write!(f, "mgb"),
            GBBootRomType::SGB => write!(f, "sgb"),
            GBBootRomType::SGB2 => write!(f, "sgb2"),
        }
    }
}

pub struct GBBootRom {
    rom_type: GBBootRomType,
    data: Box<[u8; 256]>,  // Raw memory data
    memory_map: AddressSpace<u16>,  // Addressable memory space
}

impl GBBootRom {
    pub fn new(rom_type: GBBootRomType) -> GBBootRom {
        let (data, filename): (&[u8; 256], &str) = match rom_type {
            GBBootRomType::DMG0 => (include_bytes!("boot_rom_files/GB_dmg0.bin"), "boot_rom_files/GB_dmg0.bin"),
            GBBootRomType::DMG  => (include_bytes!("boot_rom_files/GB_dmg.bin"), "boot_rom_files/GB_dmg.bin"),
            GBBootRomType::MGB  => (include_bytes!("boot_rom_files/GB_mgb.bin"), "boot_rom_files/GB_mgb.bin"),
            GBBootRomType::SGB  => (include_bytes!("boot_rom_files/GB_sgb.bin"), "boot_rom_files/GB_sgb.bin"),
            GBBootRomType::SGB2 => (include_bytes!("boot_rom_files/GB_sgb2.bin"), "boot_rom_files/GB_sgb2.bin"),
        };
        println!("Collected Boot ROM: {filename}");

        GBBootRom {
            rom_type,
            data: Box::new(*data),
            memory_map: AddressSpace::new(vec![[0x0000, 0x00ff]]),  // Hardcoded address space
        }
    }

    pub fn rom_type(&self) -> GBBootRomType { self.rom_type }
}

impl Index<u16> for GBBootRom {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        if self.memory_map.in_range(index) { panic!("Could not address {index:#x} in range {}", self.memory_map) }
        &self.data[index as usize]
    }
}

impl IndexMut<u16> for GBBootRom {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        if self.memory_map.in_range(index) { panic!("Could not address {index:#x} in range {}", self.memory_map) }
        &mut self.data[index as usize]
    }
}

impl AddressableMemory<u16, u8> for GBBootRom {
    fn address_space(&self) -> &AddressSpace<u16> { &self.memory_map }
}

// GBC Boot ROM
// ----------------------------------------------------------------------------
#[derive(Copy, Clone, Debug)]
pub enum GBCBootRomType {
    CGB0,
    CGB,
    AGB0,
    AGB,
}

impl Display for GBCBootRomType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GBCBootRomType::CGB0 => write!(f, "cgb0"),
            GBCBootRomType::CGB => write!(f, "cgb"),
            GBCBootRomType::AGB0 => write!(f, "agb0"),
            GBCBootRomType::AGB => write!(f, "agb"),
        }
    }
}

pub struct GBCBootRom {
    rom_type: GBCBootRomType,
    data: Box<[u8; 2304]>, // Raw memory data
    memory_map: AddressSpace<u16>,  // Addressable memory space
}

impl GBCBootRom {

    pub fn new(rom_type: GBCBootRomType) -> GBCBootRom {
        let (data, filename) = match rom_type {
            GBCBootRomType::CGB0 => (include_bytes!("boot_rom_files/GBC_cgb0.bin"), "boot_rom_files/GBC_cgb0.bin"),
            GBCBootRomType::CGB  => (include_bytes!("boot_rom_files/GBC_cgb.bin"), "boot_rom_files/GBC_cgb.bin"),
            GBCBootRomType::AGB0 => (include_bytes!("boot_rom_files/GBC_agb0.bin"), "boot_rom_files/GBC_agb0.bin"),
            GBCBootRomType::AGB  => (include_bytes!("boot_rom_files/GBC_agb.bin"), "boot_rom_files/GBC_agb.bin"),
        };
        println!("Collected Boot ROM: {filename}");

        GBCBootRom {
            rom_type,
            data: Box::new(*data),
            memory_map: AddressSpace::new(vec![[0x0000, 0x00ff], [0x0200, 0x08ff]]),  // Hardcoded address space
        }
    }

    pub fn rom_type(&self) -> GBCBootRomType { self.rom_type }
}

impl Index<u16> for GBCBootRom {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        if self.memory_map.in_range(index) { panic!("Could not address {index:#x} in range {}", self.memory_map) }
        &self.data[index as usize]
    }
}

impl IndexMut<u16> for GBCBootRom {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        if self.memory_map.in_range(index) { panic!("Could not address {index:#x} in range {}", self.memory_map) }
        &mut self.data[index as usize]
    }
}

impl AddressableMemory<u16, u8> for GBCBootRom {
    fn address_space(&self) -> &AddressSpace<u16> { &self.memory_map }
}
