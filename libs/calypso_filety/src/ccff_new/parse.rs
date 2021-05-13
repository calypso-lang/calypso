use calypso_base::symbol::Symbol;

use nom::{
    bytes::complete::tag,
    character::complete::satisfy,
    multi::fold_many_m_n,
    number::complete::{le_u32, le_u64, le_u8},
    sequence::tuple,
    IResult,
};

use super::Section;

fn magic(b: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(b"CCFF")(b)
}

fn section_header(b: &[u8]) -> IResult<&[u8], (Symbol, Section)> {
    let (rest, (stype, flags, offset, size, name_len)) =
        tuple((le_u8, le_u32, le_u64, le_u64, le_u8))(b)?;

    let section = Section {
        data: Vec::with_capacity(size as usize),
        stype,
        flags,
        offset: Some(offset),
    };

    let (rest, name) = fold_many_m_n(
        name_len as usize,
        name_len as usize,
        satisfy(|c| c.is_ascii_alphanumeric() || c == '_'),
        String::new(),
        |mut acc: String, item| {
            acc.push(item);
            acc
        },
    )(rest)?;

    Ok((rest, (Symbol::intern(&name), section)))
}
