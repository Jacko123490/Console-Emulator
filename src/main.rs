pub mod dev_utils;
pub mod hardware_components;

fn main() {
    let filename = std::path::Path::new("resources/tetris.gb");
    let cartridge = hardware_components::cartridge::GBCartridge::load(filename).unwrap();
    println!("Loaded Cartridge:");
    cartridge.print_info();
}
