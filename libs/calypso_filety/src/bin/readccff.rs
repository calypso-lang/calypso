use calypso_filety::ccff;
use ccff::hl::*;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut args = env::args();
    if let Some(file) = args.nth(1) {
        let mut file = File::open(file).expect("Failed to open file");
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("Failed to read file");
        let compressed =
            ContainerFile::is_compressed(&buf).expect("Failed to check if CCFF file is compressed");
        let container = ContainerFile::from_bytes(buf).expect("Failed to load CCFF file");
        dump_cc(container, compressed);
    } else {
        eprintln!("usage: readccff <FILE>");
        return;
    }
}

use pretty_hex::{config_hex, HexConfig};

fn dump_cc(container: ContainerFile, compressed: bool) {
    println!("=== metadata ===");
    println!("=> compressed:  {}", if compressed { "yes" } else { "no" });
    println!("=> ABI version: {}", container.get_header().get_abi());
    println!("=> File type:   {}", container.get_header().get_filety());
    println!("=== sections ===");
    for (idx, (name, section)) in container.sections_iter().enumerate() {
        let config = HexConfig {
            title: false,
            ascii: true,
            group: 0,
            width: 16,
            ..HexConfig::simple()
        };
        println!(":: idx {}", idx);
        println!(
            "  => name:         {} @ .shstrtab<+{:x}>",
            name,
            section.get_name_offset().unwrap()
        );
        println!("  => type:         {:x}", u64::from(section.get_type()));
        println!("  => flags:        {:x}", section.get_flags());
        println!("  => offset:       {:x}", section.get_offset().unwrap());
        println!("  => size:         {:x}", section.get_data().len());
        println!("  => hexdump:\n{}", config_hex(&section.get_data(), config));
    }
}
