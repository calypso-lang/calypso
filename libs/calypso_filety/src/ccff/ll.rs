pub const MAGIC_BYTES_COMPRESSED: [u8; 3] = [b'\xCC', b'\xFF', b'Z'];
pub const MAGIC_BYTES_UNCOMPRESSED: [u8; 3] = [b'\xCC', b'\xFF', b'U'];

use flate2::{bufread::ZlibDecoder, write::ZlibEncoder};
use serde::{Deserialize, Serialize};

use std::io::{prelude::*, SeekFrom};

use calypso_diagnostic::error::Result as CalResult;

use super::Compression;

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
    /// A user-defined ABI version. This is used in the bytecode as the VM version.
    pub abi: u8,
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
    pub fn get(&self, reader: &mut (impl Read + BufRead + Seek)) -> CalResult<Vec<u8>> {
        if Cc::is_compressed(reader)? {
            reader.seek(SeekFrom::Start(3))?;
            let mut decoder = ZlibDecoder::new(reader);
            let mut rest = Vec::new();
            decoder.read_to_end(&mut rest)?;
            let offset = self.offset as usize;
            let end = offset + self.size as usize;
            Ok(rest[offset..end].to_vec())
        } else {
            reader.seek(SeekFrom::Start(3 + self.offset))?;
            let mut rest = Vec::new();
            reader.read_to_end(&mut rest)?;
            Ok(rest[0..self.size as usize].to_vec())
        }
    }
}

impl Cc {
    pub fn load(reader: &mut (impl Read + BufRead + Seek)) -> CalResult<Self> {
        let mut magic = [0, 0, 0];
        reader.read_exact(&mut magic)?;
        reader.seek(SeekFrom::Start(3))?;
        let cc = if magic == MAGIC_BYTES_COMPRESSED {
            let decoder = ZlibDecoder::new(reader);
            bincode::deserialize_from(decoder)?
        } else if magic == MAGIC_BYTES_UNCOMPRESSED {
            let mut rest = Vec::new();
            reader.read_to_end(&mut rest)?;
            bincode::deserialize(&rest)?
        } else {
            return Err("invalid magic bytes for CCFF file".into());
        };

        Ok(cc)
    }

    pub fn is_compressed(reader: &mut (impl Read + Seek)) -> CalResult<bool> {
        let mut magic = [0, 0, 0];
        reader.read_exact(&mut magic)?;
        reader.seek(SeekFrom::Start(0))?;
        Ok(if magic == MAGIC_BYTES_COMPRESSED {
            true
        } else if magic == MAGIC_BYTES_UNCOMPRESSED {
            false
        } else {
            return Err("invalid magic bytes for CCFF file".into());
        })
    }

    pub fn write(&self, compression: Compression, data: &[u8]) -> CalResult<Vec<u8>> {
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
