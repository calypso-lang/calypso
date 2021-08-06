use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

use pretty_hex::{config_hex, HexConfig};

use calypso_filety::ccff;
use ccff::{hl::*, ll::CcffHeader};

fn main() {
    let mut args = env::args();
    if let Some(file) = args.nth(1) {
        let mut file = File::open(file).expect("Failed to open file");
        let mut container =
            ContainerFile::decode(CcffHeader::read(&mut file).expect("Failed to load CCFF file"));
        file.seek(SeekFrom::Start(0)).expect("Failed to seek file");
        container
            .read_all(&mut file)
            .expect("Failed to read CCFF section data");
        dump_cc(container);
    } else {
        eprintln!("usage: readccff <FILE>");
    }
}

fn dump_cc(container: ContainerFile) {
    println!("=== metadata ===");
    println!("=> ABI version: {}", container.get_abi());
    println!("=> File type:   {}", container.get_filety());
    println!("=== sections ===");
    for (idx, section) in container.sections().enumerate() {
        let config = HexConfig {
            title: false,
            ascii: true,
            group: 0,
            width: 16,
            ..HexConfig::simple()
        };
        println!(":: idx {}", idx);
        println!("  => name:         {}", section.get_name());
        println!("  => type:         0x{:x}", section.get_stype());
        println!("  => flags:        0x{:x}", section.get_flags());
        println!("  => offset:       0x{:x}", section.get_offset().unwrap());
        println!(
            "  => size:         0x{:x}",
            section.get_data().unwrap().len()
        );
        println!(
            "  => hexdump:\n{}",
            config_hex(&section.get_data().unwrap(), config)
        );
    }
}
