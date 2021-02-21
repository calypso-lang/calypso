use std::convert::TryInto;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};

use super::ll::{CcffHeader, CcffSectionHeader};
use calypso_error::CalResult;

/// A higher-level interface to a CCFF container file.
#[derive(Debug, Clone, Default)]
pub struct ContainerFile {
    abi: u64,
    filety: u64,
    sections: Vec<Section>,
}

impl ContainerFile {
    /// Create a new container file.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ABI of the container file. This can be any arbitrary value you
    /// choose.
    #[must_use]
    pub fn abi(mut self, abi: u64) -> Self {
        self.abi = abi;
        self
    }

    /// Set the file type of the container file. This can be any arbitrary
    /// value you choose.
    #[must_use]
    pub fn filety(mut self, filety: u64) -> Self {
        self.filety = filety;
        self
    }

    /// Get the ABI of the container file.
    #[must_use]
    pub fn get_abi(&self) -> u64 {
        self.abi
    }

    /// Get the file type of the container file.
    #[must_use]
    pub fn get_filety(&self) -> u64 {
        self.filety
    }

    /// Add a section to the container file.
    #[must_use]
    pub fn add_section(mut self, section: Section) -> Self {
        self.sections.push(section);
        self
    }

    /// Remove a section from the container file. Does nothing if the section
    /// does not exist.
    #[must_use]
    pub fn remove_section(mut self, name: &str) -> Self {
        let idx = self.sections.iter().position(|s| s.name == name);
        if let Some(idx) = idx {
            self.sections.remove(idx);
        }
        self
    }

    /// Get a reference to a section in the container file.
    #[must_use]
    pub fn get_section(&self, name: &str) -> Option<&Section> {
        self.sections.iter().find(|&s| s.name == name)
    }

    /// Get a mutable reference to a section in the container file.
    #[must_use]
    pub fn get_section_mut(&mut self, name: &str) -> Option<&mut Section> {
        self.sections.iter_mut().find(|s| s.name == name)
    }

    /// Iterate over the sections in the container file.
    pub fn sections(&self) -> impl Iterator<Item = &Section> {
        self.sections.iter()
    }

    /// Encode the container file as its low-level counterpart. This function
    /// returns a tuple of the encoded header and the section data.
    #[must_use]
    pub fn encode(self) -> (CcffHeader, Vec<u8>) {
        let (sections, data): (Vec<_>, Vec<Vec<_>>) =
            self.sections.into_iter().map(Section::encode).unzip();
        let data = data.into_iter().flatten().collect();

        let mut header = CcffHeader {
            abi: self.abi,
            filety: self.filety,
            magic: *b"\xCC\xFF",
            sections,
        };

        let mut offset = header.size() as u64;
        for section in &mut header.sections {
            section.offset = offset;
            offset += section.size;
        }

        (header, data)
    }

    /// Decode the container file from its low-level counterpart.
    #[must_use]
    pub fn decode(header: CcffHeader) -> Self {
        Self {
            sections: header
                .sections
                .into_iter()
                .map(|s| Section {
                    data: None,
                    flags: s.flags,
                    stype: s.section_type,
                    name: s.name,
                    offset: Some(s.offset),
                    size: Some(s.size),
                })
                .collect::<Vec<_>>(),
            abi: header.abi,
            filety: header.filety,
        }
    }

    /// Read the entirety of the container file's section data. This function
    /// assumes that you are already seeked to the beginning of the data. It is
    /// not recommended to use this unless you know you're using a small file
    /// as it could potentially use more memory than reading piece by piece.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data could not be read, if
    /// the section data was too large to read, if the section header had a
    /// malformed size, or if the size was not provided (this will not happen
    /// if you load from a file).
    pub fn read_all<I: Read + Seek>(&mut self, input: &mut I) -> CalResult<()> {
        for section in &mut self.sections {
            section.read_data(input)?;
        }
        Ok(())
    }
}

/// A higher-level interface to a CCFF section.
#[derive(Debug, Clone, Default)]
pub struct Section {
    name: String,
    stype: u64,
    flags: u64,
    data: Option<Vec<u8>>,
    offset: Option<u64>,
    size: Option<u64>,
}

impl Section {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            data: Some(Vec::new()),
            ..Self::default()
        }
    }

    /// Get the name of the section.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set the type of the section. This can be any arbitrary value you
    /// choose.
    pub fn stype(mut self, stype: u64) -> Self {
        self.stype = stype;
        self
    }

    /// Get the type of the section.
    #[must_use]
    pub fn get_stype(&self) -> u64 {
        self.stype
    }

    /// Set the flags of the section. This can be any arbitrary value you
    /// choose.
    pub fn flags(mut self, flags: u64) -> Self {
        self.flags = flags;
        self
    }

    /// Get the flags of the section.
    #[must_use]
    pub fn get_flags(&self) -> u64 {
        self.flags
    }

    /// Set the data of the section. This can be any arbitrary data you choose.
    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }

    /// Get a reference to the data of the section. This may not be present if
    /// reading from a file in order to save memory for large files.
    /// To get data from large files, use [`Section::seek_to_data`] or
    /// [`Section::read_data`].
    #[must_use]
    pub fn get_data(&self) -> Option<&[u8]> {
        self.data.as_ref().map(|d| d.as_ref())
    }

    /// Get a mutable reference to the data of the section. This may not be
    /// present if reading from a file in order to save memory for large files.
    /// To get data from large files, use [`Section::seek_to_data`] or
    /// [`Section::read_data`].
    pub fn get_data_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.data.as_mut()
    }

    /// Seek to the location of the data in the reader.
    ///
    /// # Errors
    ///
    /// This function will return an error if the section data had a malformed
    /// size, or if the offset was not set (this will not happen if you load
    /// from a file).
    pub fn seek_to_data<I: Seek>(&self, input: &mut I) -> CalResult<()> {
        input
            .seek(SeekFrom::Start(
                self.offset
                    .ok_or_else(|| {
                        IOError::new(IOErrorKind::InvalidInput, "offset was not provided")
                    })?
                    .try_into()
                    .map_err(|_| {
                        IOError::new(
                            IOErrorKind::InvalidData,
                            "section data had a malformed size",
                        )
                    })?,
            ))
            .map_err(|_| {
                IOError::new(
                    IOErrorKind::InvalidData,
                    "section data had a malformed size",
                )
            })?;
        Ok(())
    }

    /// Read the entirety of the section's data. It is not recommended to use
    /// this unless you know you're using a small file as it could potentially
    /// use more memory than reading piece by piece.
    ///
    /// # Errors
    ///
    /// This function will return an error if the data could not be read, if
    /// the section data was too large to read, if the section header had a
    /// malformed size, or if the size was not provided (this will not happen
    /// if you load from a file).
    pub fn read_data<I: Read + Seek>(&mut self, input: &mut I) -> CalResult<()> {
        self.seek_to_data(input)?;
        let size: usize = self
            .size
            .ok_or_else(|| IOError::new(IOErrorKind::InvalidInput, "size was not provided"))?
            .try_into()
            .map_err(|_| {
                IOError::new(
                    IOErrorKind::InvalidData,
                    "section data was too large to read or had a malformed size",
                )
            })?;
        let mut buf = Vec::with_capacity(size);
        let n_read = input.take(size as u64).read_to_end(&mut buf)?;
        if n_read < size {
            return Err(IOError::new(IOErrorKind::WriteZero, "could not read section data").into());
        }
        self.data = Some(buf);
        Ok(())
    }

    /// Get the offset of the data. This is only present if loading from a file
    /// and cannot be manually set to prevent errors.
    pub fn get_offset(&self) -> Option<u64> {
        self.offset
    }

    /// Encode the section as its low-level counterpart. This function returns
    /// a tuple of the encoded section header and the section data. Note that
    /// this does not set the offset as that requires knowing all of the
    /// sections. To do so, encode an entire container file with
    /// [`ContainerFile::encode`].
    ///
    /// # Panics
    ///
    /// This function will panic if the data was not present. This will only
    /// happen if you are loading from a file.
    #[must_use]
    pub fn encode(self) -> (CcffSectionHeader, Vec<u8>) {
        let data = self.data.unwrap();
        let hdr = CcffSectionHeader {
            name: self.name,
            section_type: self.stype,
            flags: self.flags,
            size: data.len() as u64,
            offset: 0,
        };
        (hdr, data)
    }
}
