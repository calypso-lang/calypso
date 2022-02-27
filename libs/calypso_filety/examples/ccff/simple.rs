use std::{fs::File, io::Write};

use calypso_base::symbol::Symbol;
use calypso_filety::ccff::{ContainerFile, Section};

fn main() {
    let mut file = File::create("simple.ccff").expect("Failed to open CCFF file");

    let mut section = Section::new(1, 0);
    section.set_data(b"some bytecode data here i guess".to_vec());
    let mut container = ContainerFile::new(1, 1);
    container.add_section(Symbol::intern_static("code"), section);

    let data = container.encode();

    file.write_all(&data)
        .expect("Failed to write container to file");
}
