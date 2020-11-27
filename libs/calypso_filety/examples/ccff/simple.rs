use calypso_filety::{ccff::hl::*, ccff::*};
fn main() {
    let hdr = ContainerHeader::new(1);
    let mut container = ContainerFile::new(hdr);
    let code_section = Section::new(
        SectionType::Other(2),
        0,
        "some bytecode data here i guess".as_bytes().to_vec(),
    );
    container.add_section(".code".to_string(), code_section);
    let bytes = container.into_bytes(Compression::Uncompressed).unwrap();
    println!("{:02x?}", bytes);
}
