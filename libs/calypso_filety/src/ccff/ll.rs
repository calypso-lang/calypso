use std::convert::TryInto;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};

use anyhow::{anyhow, Error};
use serde::{Deserialize, Serialize};

use calypso_error::{CalError, CalResult};

/// The header for a CCFF file
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CcffHeader {
    /// The 2 magic bytes
    pub magic: [u8; 2],
    /// A user-defined ABI version.
    pub abi: u64,
    /// A user-defined file type.
    pub filety: u64,
    /// The CCFF section headers, encoded as a length-array (i.e. a
    /// `(u64, [CcffSectionHeader])`, where the `u64` is the length of the
    /// contiguous, unpadded array)
    pub sections: Vec<CcffSectionHeader>,
}

/// The header for a CCFF section. This does not include the associated data.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CcffSectionHeader {
    /// The section name, encoded as a length-string (i.e. a `(u64, str)` where
    /// the `u64` is the length of the `str`)
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
    pub fn read<I: Read>(input: &mut I) -> CalResult<Self> {
        let header: Self = bincode::deserialize_from(input).map_err(Error::from)?;
        if &header.magic == b"\xCC\xFF" {
            Ok(header)
        } else {
            Err(anyhow!("invalid magic bytes `{:x?}`", &header.magic).into())
        }
    }

    /// Write the CCFF header and magic bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data could not be serialized,
    /// could not be written, or if the magic bytes were invalid.
    pub fn write<O: Write>(&self, input: &mut O) -> CalResult<()> {
        if &self.magic == b"\xCC\xFF" {
            bincode::serialize_into(input, self)
                .map_err(Error::from)
                .map_err(CalError::from)
        } else {
            Err(anyhow!("invalid magic bytes `{:x?}`", &self.magic).into())
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
    pub fn seek_to_data<I: Seek>(&self, input: &mut I) -> CalResult<()> {
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
    pub fn read_data<I: Read + Seek>(&self, input: &mut I) -> CalResult<Vec<u8>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static VALID_HEADER_WITHOUT_MAGIC: &[u8] = b"\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00.foo\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x46\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00";

    #[test]
    fn header_read_valid() {
        let mut hdr = b"\xCC\xFF".to_vec();
        hdr.extend(VALID_HEADER_WITHOUT_MAGIC);

        let mut cursor = Cursor::new(&hdr);
        let res = CcffHeader::read(&mut cursor).unwrap();
        assert_eq!(
            res,
            CcffHeader {
                magic: *b"\xCC\xFF",
                abi: 1,
                filety: 1,
                sections: vec![CcffSectionHeader {
                    name: String::from(".foo"),
                    section_type: 1,
                    flags: 0,
                    offset: 70,
                    size: 3
                }]
            }
        );
        assert_eq!(res.size(), hdr.len());
    }

    #[test]
    fn header_read_invalid_magic() {
        let mut hdr = b"\xCC\xFA".to_vec();
        hdr.extend(VALID_HEADER_WITHOUT_MAGIC);
        let mut cursor = Cursor::new(hdr);
        let err = CcffHeader::read(&mut cursor).unwrap_err();
        assert_eq!(
            format!("{:?}", err),
            "Other(invalid magic bytes `[cc, fa]`)"
        )
    }

    #[test]
    fn header_write() {
        let hdr = CcffHeader {
            magic: *b"\xCC\xFF",
            abi: 1,
            filety: 1,
            sections: vec![CcffSectionHeader {
                name: String::from(".foo"),
                section_type: 1,
                flags: 0,
                offset: 70,
                size: 3,
            }],
        };
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        hdr.write(&mut cursor).unwrap();
        let res = cursor.into_inner();
        assert_eq!(&res[..2], b"\xCC\xFF");
        assert_eq!(&res[2..], VALID_HEADER_WITHOUT_MAGIC);
    }

    #[test]
    fn header_write_invalid_magic() {
        let hdr = CcffHeader {
            magic: *b"\xCC\xFA",
            abi: 1,
            filety: 1,
            sections: vec![CcffSectionHeader {
                name: String::from(".foo"),
                section_type: 1,
                flags: 0,
                offset: 70,
                size: 3,
            }],
        };
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let err = hdr.write(&mut cursor).unwrap_err();
        assert_eq!(
            format!("{:?}", err),
            "Other(invalid magic bytes `[cc, fa]`)"
        )
    }
}
