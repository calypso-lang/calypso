use calypso_base::symbol::Symbol;

use indexmap::IndexMap;
use nom::{
    bytes::complete::{tag, take},
    combinator::{map_opt, verify},
    multi::fold_many_m_n,
    number::complete::{le_u16, le_u32, le_u8},
    sequence::tuple,
    IResult,
};

use std::{cell::RefCell, str};

use super::{ContainerFile, Section};

fn magic(b: &[u8]) -> IResult<&[u8], &[u8]> {
    tag(b"CCFF")(b)
}

pub fn container_file(b: &[u8]) -> IResult<&[u8], ContainerFile> {
    let (rest, (_, abiver, filety, num_sections)) = tuple((magic, le_u16, le_u8, le_u8))(b)?;

    // this is kinda gross, but it's the best way I can find to do this.
    let sections = RefCell::new(IndexMap::with_capacity(num_sections as usize));

    let (rest, _) = fold_many_m_n(
        num_sections as usize,
        num_sections as usize,
        map_opt(
            verify(section_header, |(sym, _)| {
                !sections.borrow().contains_key(sym)
            }),
            |(sym, mut sec)| {
                let start = sec.offset? as usize;
                let end = start + sec.data.capacity();
                let data = b.get(start..end)?;
                sec.data.extend_from_slice(data);
                Some((sym, sec))
            },
        ),
        || (),
        |_, (sym, sec)| {
            sections.borrow_mut().insert(sym, sec);
        },
    )(rest)?;

    let cf = ContainerFile {
        abiver,
        filety,
        sections: sections.into_inner(),
    };

    Ok((rest, cf))
}

fn section_header(b: &[u8]) -> IResult<&[u8], (Symbol, Section)> {
    let (rest, (stype, flags, offset, size, name_len)) =
        tuple((le_u8, le_u32, le_u32, le_u32, le_u8))(b)?;

    let section = Section {
        data: Vec::with_capacity(size as usize),
        stype,
        flags,
        offset: Some(offset),
    };

    let (rest, name) = verify(
        map_opt(take(name_len as usize), |x: &[u8]| {
            x.is_ascii().then(|| {
                // SAFETY: It is safe to convert to UTF-8 without checking, as we
                // have already confirmed this slice is entirely ASCII using the
                // above check, and ASCII is entirely backwards-compatible with
                // UTF-8 (there is no difference in encoding of ASCII strings in
                // UTF-8). Thus the provided &str will be valid UTF-8 and will not
                // break anything.
                //
                // This is probably not a very important optimization, to be honest.
                // I'll get to profiling this later and see how much of a
                // difference this makes. It probably won't be a lot.
                // But at this moment it doesn't matter, since I'm 99.99999%
                // sure this is safe.
                let s = unsafe { str::from_utf8_unchecked(x) };

                s.to_owned()
            })
        }),
        |s: &str| s.chars().all(|c| c.is_ascii_graphic()),
    )(rest)?;

    Ok((rest, (Symbol::intern(&name), section)))
}

// TODO(@ThePuzzlemaker: filety|test): Add some tests here.
