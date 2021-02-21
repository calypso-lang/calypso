use std::convert::TryInto;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};

use bincode::ErrorKind;
use serde::{Deserialize, Serialize};

/// The header for a CCFF file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CcffHeader {
    /// The 2 magic bytes
    pub magic: [u8; 2],
    /// A user-defined ABI version.
    pub abi: u64,
    /// A user-defined file type.
    pub filety: u64,
    /// The CCFF section headers
    pub sections: Vec<CcffSectionHeader>,
}

/// The header for a CCFF section. This does not include the associated data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CcffSectionHeader {
    /// The section name as a length-string (i.e. a `(u64, str)` where the `u64` is the length of the `str`)
    pub name: String,
    /// A user-defined section type.
    pub section_type: u64,
    /// User-defined section flags.
    pub flags: u64,
    /// The offset, in bytes, of the section from the begining of the file
    pub offset: u64,
    /// The size, in bytes, of this section
    pub size: u64,
}

impl CcffHeader {
    /// Read the CCFF header and magic bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data could not be read,
    /// could not be deserialized, or if the magic bytes are invalid.
    pub fn read<I: Read>(input: &mut I) -> bincode::Result<Self> {
        let header: Self = bincode::deserialize_from(input)?;
        if &header.magic == b"\xCC\xFF" {
            Ok(header)
        } else {
            Err(Box::new(ErrorKind::Custom(String::from(
                "invalid magic bytes",
            ))))
        }
    }

    /// Write the CCFF header and magic bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data could not be serialized,
    /// could not be written, or if the magic bytes were invalid.
    pub fn write<O: Write>(&self, input: &mut O) -> bincode::Result<()> {
        if &self.magic == b"\xCC\xFF" {
            bincode::serialize_into(input, self)
        } else {
            Err(Box::new(ErrorKind::Custom(String::from(
                "invalid magic bytes",
            ))))
        }
    }

    /// Get the size of the CCFF header, including the magic bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        // 26 = 2*u8 + 3*u64
        //    = magic: [u8; 2]
        //    + abi: u64
        //    + filety: u64
        //    + len(sections): u64
        26 + self
            .sections
            .iter()
            .map(CcffSectionHeader::size)
            .sum::<usize>()
    }
}

impl CcffSectionHeader {
    /// Seek to the data.
    ///
    /// # Errors
    ///
    /// This function will return an error if the section data had a malformed
    /// size.
    pub fn seek_to_data<I: Seek>(&self, input: &mut I) -> IOResult<()> {
        input
            .seek(SeekFrom::Start(self.offset.try_into().map_err(|_| {
                IOError::new(
                    IOErrorKind::InvalidData,
                    "section data had a malformed size",
                )
            })?))
            .map_err(|_| {
                IOError::new(
                    IOErrorKind::InvalidData,
                    "section data had a malformed size",
                )
            })?;
        Ok(())
    }

    /// Read the entirety of the data. This function assumes that you are
    /// already seeked to the beginning of the data. It is not recommended to
    /// use this unless you know you're using a small file as it could
    /// potentially use more memory than reading piece by piece.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data could not be read, if
    /// the section data was too large to read, or if the section header had a
    /// malformed size.
    pub fn read_data<I: Read + Seek>(&self, input: &mut I) -> IOResult<Vec<u8>> {
        let mut buf = Vec::with_capacity(self.size.try_into().map_err(|_| {
            IOError::new(
                IOErrorKind::InvalidData,
                "section data was too large to read or had a malformed size",
            )
        })?);
        input.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Get the size of this header as it would be encoded.
    #[must_use]
    pub fn size(&self) -> usize {
        // 40 = 5*u64
        //    = section_type
        //      + flags: u64
        //      + offset: u64
        //      + size: u64
        //      + len(name): u64
        40 + self.name.len()
    }
}
