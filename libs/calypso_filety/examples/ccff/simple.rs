use std::{fs::File, io::Write};

use calypso_filety::ccff::hl::*;
fn main() {
    let mut file = File::create("simple.ccff").expect("Failed to open CCFF file");

    let container = ContainerFile::new().abi(1).filety(1).add_section(
        Section::new(".code".to_string())
            .stype(1)
            .flags(0)
            .data("some bytecode data here i guess".as_bytes().to_vec()),
    );

    let (hdr, data) = container.encode();

    hdr.write(&mut file)
        .expect("Failed to write CCFF header to file");

    file.write_all(&data).expect("Failed to write data to file");
}
