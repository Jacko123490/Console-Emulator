use std::path::Path;
use std::io;

use crate::dev_utils::{mem_array_display, mem_entry_display, mem_array_str};

const MAX_SIZE: usize = 1024 * 1024;

pub struct GBCartridge {
    data: Box<[u8; MAX_SIZE]>,
    loaded: bool,
}

impl GBCartridge {
    const HEADER_START: u16 = 0x0100;

    // Header Getters
    fn entry_point(&self) -> &[u8; 4] { self.data[0x0100..=0x0103].try_into().unwrap() }
    fn nintendo_logo(&self) -> &[u8; 48] { self.data[0x0104..=0x0133].try_into().unwrap() }
    fn title(&self) -> &[u8; 15] { self.data[0x0134..=0x0142].try_into().unwrap() }  // Using newer model with CGP flag, gives 15 bit title
    fn cgp_flag(&self) -> &u8 { &self.data[0x0143] }
    fn new_licensee_code(&self) -> &[u8; 2] { self.data[0x0144..=0x0145].try_into().unwrap() }
    fn sgb_flag(&self) -> &u8 { &self.data[0x0146] }
    fn cartridge_type(&self) -> &u8 { &self.data[0x0147] }
    fn rom_size(&self) -> &u8 { &self.data[0x0148] }
    fn ram_size(&self) -> &u8 { &self.data[0x0149] }
    fn destination_code(&self) -> &u8 { &self.data[0x014a] }
    fn old_licensee_code(&self) -> &u8 { &self.data[0x014b] }
    fn mask_rom_version_number(&self) -> &u8 { &self.data[0x014c] }
    fn header_checksum(&self) -> &u8 { &self.data[0x014d] }
    fn global_checksum(&self) -> &[u8; 2] { self.data[0x014e..=0x014f].try_into().unwrap() }

    pub fn print_info(&self) {
        println!("Entry point: {}", mem_array_display(self.entry_point()));
        println!("Nintendo Logo: {}", mem_array_display(self.nintendo_logo()));
        println!("Title: {}", mem_array_str(self.title()));
        println!("CGP Flag: {}", mem_entry_display(self.cgp_flag()));
        println!("New Licensee Code: {}", mem_array_display(self.new_licensee_code()));
        println!("SGP Flag: {}", mem_entry_display(self.sgb_flag()));
        println!("Cartridge Type: {}", mem_entry_display(self.cartridge_type()));
        println!("ROM Size: {}", mem_entry_display(self.rom_size()));
        println!("RAM Size: {}", mem_entry_display(self.ram_size()));
        println!("Destination Code: {}", mem_entry_display(self.destination_code()));
        println!("Old Licensee Code: {}", mem_entry_display(self.old_licensee_code()));
        println!("Mask ROM Version Number: {}", mem_entry_display(self.mask_rom_version_number()));
        println!("Header Checksum: {}", mem_entry_display(self.header_checksum()));
        println!("Global Checksum: {}", mem_array_display(self.global_checksum()));
    }

    // Init
    pub fn load(filename: &Path) -> io::Result<GBCartridge> {
        // Get File Data
        println!("Loading File: {}", std::path::absolute(filename)?.display());
        let file_data: Vec<u8> = std::fs::read(filename)?;

        // Add file data to cartridge data array
        let mut data: Box<[u8; MAX_SIZE]> = vec![0xff; MAX_SIZE].into_boxed_slice().try_into().unwrap();
        let file_truncate_len = file_data.len().min(MAX_SIZE);
        data[..file_truncate_len].copy_from_slice(&file_data[..file_truncate_len]);
        println!("Loaded ROM: {}, Size: {}",
                 filename.file_name().unwrap().to_str().unwrap(),
                 file_truncate_len.to_string());

        // Build Cartridge
        Ok(GBCartridge { data, loaded: false })
    }
}
