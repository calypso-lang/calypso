use std::io::prelude::*;
use std::mem;

use bincode::ErrorKind;
use flate2::{bufread::ZlibDecoder, write::ZlibEncoder};
use serde::{Deserialize, Serialize};

use super::Compression;

pub const MAGIC_BYTES_COMPRESSED: [u8; 3] = [b'\xCC', b'\xFF', b'Z'];
pub const MAGIC_BYTES_UNCOMPRESSED: [u8; 3] = [b'\xCC', b'\xFF', b'U'];

/// A CCFF file (minus its data)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cc {
    /// The CCFF header
    pub header: CcHdr,
    /// The CCFF section headers
    pub sections: Vec<CcSectionHdr>,
}

/// The header for a CCFF file
#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(C)]
pub struct CcHdr {
    /// A user-defined ABI version.
    pub abi: u64,
    /// A user-defined file type.
    pub filety: u64,
}

/// The header for a single CCFF section
#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(C)]
pub struct CcSectionHdr {
    /// The section name as an offset into the section header string table
    pub name: u64,
    /// A user-defined section type.
    /// Reserved types:
    /// - `1`: section header string table
    pub section_type: u64,
    /// User-defined section flags.
    pub flags: u64,
    /// The offset of the section in the uncompressed data (without the magic bytes)
    pub offset: u64,
    /// The size in bytes of this section
    pub size: u64,
}

impl CcSectionHdr {
    pub fn get<'a>(&self, data: &'a [u8]) -> Option<&'a [u8]> {
        let offset = self.offset as usize;
        let end = offset + self.size as usize;
        data.get(offset..end)
    }
}

impl Cc {
    pub fn load(buf: Vec<u8>) -> bincode::Result<(Self, Vec<u8>)> {
        let rest = &buf[3..];
        let rest = if Self::is_compressed(&buf)? {
            let mut decoder = ZlibDecoder::new(rest);
            let mut vec = Vec::new();
            decoder.read_to_end(&mut vec)?;
            vec
        } else {
            rest.to_vec()
        };

        let cc: Cc = bincode::deserialize(&rest)?;
        let data = rest[cc.size()..].to_vec();

        Ok((cc, data))
    }

    pub fn size(&self) -> usize {
        let num_sections = self.sections.len();
        let section_hdr_size = mem::size_of::<CcSectionHdr>();
        let hdr_size = mem::size_of::<CcHdr>();
        // there are `num_sections` section headers, then the main header, then a `u64` used for
        // the size of the Vec<CcSectionHdr> in bincode
        (section_hdr_size * num_sections) + hdr_size + mem::size_of::<u64>()
    }

    pub fn is_compressed(buf: &[u8]) -> bincode::Result<bool> {
        let magic = &buf[0..3];
        Ok(if magic == MAGIC_BYTES_COMPRESSED {
            true
        } else if magic == MAGIC_BYTES_UNCOMPRESSED {
            false
        } else {
            return Err(Box::new(ErrorKind::Custom(
                "invalid magic bytes for CCFF file".to_string(),
            )));
        })
    }

    pub fn write(&self, compression: Compression, data: &[u8]) -> bincode::Result<Vec<u8>> {
        let vec = if let Compression::Compressed(level) = compression {
            let mut encoded = Vec::new();
            encoded.extend(&bincode::serialize(self)?);
            encoded.extend(data);
            let mut compressor = ZlibEncoder::new(Vec::new(), level);
            compressor.write_all(&encoded)?;
            let mut vec = MAGIC_BYTES_COMPRESSED.to_vec();
            vec.extend(compressor.finish()?);
            vec
        } else {
            let mut vec = MAGIC_BYTES_UNCOMPRESSED.to_vec();
            vec.extend(&bincode::serialize(self)?);
            vec.extend(data);
            vec
        };

        Ok(vec)
    }
}
