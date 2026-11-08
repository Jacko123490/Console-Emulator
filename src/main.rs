pub mod dev_utils;
pub mod gameboy_emu;
pub mod hardware_definitions;

fn main() {
    let filename = std::path::Path::new("resources/tetris.gb");
    let cartridge = gameboy_emu::cartridge::GBCartridge::load(filename).unwrap();
    println!("Loaded Cartridge:");
    cartridge.print_info();
}
