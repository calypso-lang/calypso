use std::env;
use std::fs;

use pretty_hex::{config_hex, HexConfig};

use calypso_filety::ccff::ContainerFile;

fn main() {
    let mut args = env::args();
    if let Some(file) = args.nth(1) {
        let buf = fs::read(file).expect("Failed to open file");
        let container = ContainerFile::decode(&buf).expect("Failed to load CCFF file");
        dump_cc(container);
    } else {
        eprintln!("usage: readccff <FILE>");
    }
}

fn dump_cc(container: ContainerFile) {
    println!("=== metadata ===");
    println!("=> ABI version: {}", container.get_abiver());
    println!("=> File type:   {}", container.get_filety());
    println!("=== sections ===");
    for (idx, (name, section)) in container.sections().enumerate() {
        let config = HexConfig {
            title: false,
            ascii: true,
            group: 0,
            width: 16,
            ..HexConfig::simple()
        };
        println!(":: idx {}", idx);
        println!("  => name:         {}", name);
        println!("  => type:         0x{:x}", section.get_type());
        println!("  => flags:        0x{:x}", section.get_flags());
        println!("  => offset:       0x{:x}", section.get_offset().unwrap());
        println!("  => size:         0x{:x}", section.get_data().len());
        println!("  => hexdump:\n{}", config_hex(&section.get_data(), config));
    }
}
