use std::fmt::{Display, Formatter};

use crate::hardware_definitions::memory::{AddressSpace, ROM};

// Boot ROM Types
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

// Boot ROM Generation
// ----------------------------------------------------------------------------
pub fn get_gb_boot_rom(rom_type: GBBootRomType) -> ROM<u16, u8, 256> {
    let (data, filename): (&[u8; 256], &str) = match rom_type {
        GBBootRomType::DMG0 => (include_bytes!("boot_rom_files/GB_dmg0.bin"), "boot_rom_files/GB_dmg0.bin"),
        GBBootRomType::DMG  => (include_bytes!("boot_rom_files/GB_dmg.bin"), "boot_rom_files/GB_dmg.bin"),
        GBBootRomType::MGB  => (include_bytes!("boot_rom_files/GB_mgb.bin"), "boot_rom_files/GB_mgb.bin"),
        GBBootRomType::SGB  => (include_bytes!("boot_rom_files/GB_sgb.bin"), "boot_rom_files/GB_sgb.bin"),
        GBBootRomType::SGB2 => (include_bytes!("boot_rom_files/GB_sgb2.bin"), "boot_rom_files/GB_sgb2.bin"),
    };
    println!("Collected Boot ROM: {filename}");

    ROM::new(
        Box::new(*data),
        AddressSpace::new(vec![[0x0000, 0x00ff]]),  // Hardcoded address space
    ).expect("ROM definition prepared incorrectly.")
}

pub fn get_gbc_boot_rom(rom_type: GBCBootRomType) -> ROM<u16, u8, 2048> {
    let (data, filename): (&[u8; 2304], &str) = match rom_type {
        GBCBootRomType::CGB0 => (include_bytes!("boot_rom_files/GBC_cgb0.bin"), "boot_rom_files/GBC_cgb0.bin"),
        GBCBootRomType::CGB  => (include_bytes!("boot_rom_files/GBC_cgb.bin"), "boot_rom_files/GBC_cgb.bin"),
        GBCBootRomType::AGB0 => (include_bytes!("boot_rom_files/GBC_agb0.bin"), "boot_rom_files/GBC_agb0.bin"),
        GBCBootRomType::AGB  => (include_bytes!("boot_rom_files/GBC_agb.bin"), "boot_rom_files/GBC_agb.bin"),
    };
    println!("Collected Boot ROM: {filename}");

    // GBC Boot ROM files come padded with 0x00 values padding empty AddressSpace: [XX..., 00..., YY...] -> [XX..., YY...]
    // Need to manually cut that out to generate continuous data in local address space
    let mut rom_data: Box<[u8; 2048]> = Box::new([0; 2048]);
    rom_data[..=0x00ff].copy_from_slice(&data[0x0000..=0x00ff]);
    rom_data[0x0100..].copy_from_slice(&data[0x0200..=0x08ff]);
    ROM::new(
        rom_data,
        AddressSpace::new(vec![[0x0000, 0x00ff], [0x0200, 0x08ff]]),  // Hardcoded address space
    ).expect("ROM definition prepared incorrectly.")
}

// Unit Tests
// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gb_boot_rom() {
        get_gb_boot_rom(GBBootRomType::DMG0);
        get_gb_boot_rom(GBBootRomType::DMG);
        get_gb_boot_rom(GBBootRomType::MGB);
        get_gb_boot_rom(GBBootRomType::SGB);
        get_gb_boot_rom(GBBootRomType::SGB2);
    }

    #[test]
    fn test_gbc_boot_rom() {
        get_gbc_boot_rom(GBCBootRomType::CGB0);
        get_gbc_boot_rom(GBCBootRomType::CGB);
        get_gbc_boot_rom(GBCBootRomType::AGB0);
        get_gbc_boot_rom(GBCBootRomType::AGB);
    }
}
