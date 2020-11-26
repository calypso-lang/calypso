use calypso_filety::ccff;
use ccff::ll::Cc;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

fn main() {
    // fixme(@ThePuzzlemaker: filety): loading
    let mut args = env::args();
    if let Some(file) = args.nth(1) {
        let file = File::open(file).expect("Failed to open file");
        let mut reader = BufReader::new(file);
        let compressed =
            Cc::is_compressed(&mut reader).expect("Failed to check if CCFF file is compressed");
        let cc = Cc::load(&mut reader).expect("Failed to load CCFF file");
        reader.seek(SeekFrom::Start(0)).unwrap();
        dump_cc(cc, compressed, &mut reader);
    } else {
        eprintln!("usage: readccff <FILE>");
        return;
    }
}

use pretty_hex::{config_hex, HexConfig};

fn dump_cc(cc: Cc, compressed: bool, reader: &mut (impl Read + BufRead + Seek)) {
    println!("=== metadata ===");
    println!("=> compressed:  {}", if compressed { "yes" } else { "no" });
    println!("=> ABI version: {}", cc.header.abi);
    println!("=== sections ===");
    for (idx, section) in cc.sections.into_iter().enumerate() {
        let config = HexConfig {
            title: false,
            ascii: true,
            group: 0,
            width: 16,
            ..HexConfig::simple()
        };
        println!(":: idx {}", idx);
        println!("  => name:         TODO @ .shstrtab<+{:x}>", section.name);
        println!("  => type:         {:x}", section.section_type);
        println!("  => flags:        {:x}", section.flags);
        println!("  => offset:       {:x}", section.offset);
        println!("  => size:         {:x}", section.size);
        reader
            .seek(SeekFrom::Start(0))
            .expect("Failed to seek to start of file");
        println!(
            "  => hexdump:\n{}",
            config_hex(
                &section.get(reader).expect("Failed to get section data"),
                config
            )
        );
    }
}
