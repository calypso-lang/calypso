use std::convert::TryInto;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{Error as IOError, ErrorKind as IOErrorKind};

use super::ll::{CcffHeader, CcffSectionHeader};
use calypso_error::CalResult;

/// A higher-level interface to a CCFF container file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ContainerFile {
    abi: u64,
    filety: u64,
    sections: Vec<Section>,
}

impl ContainerFile {
    /// Create a new container file. The ABI and file type (`filety`) can be
    /// any arbitrary value you choose.
    #[must_use]
    pub fn new(abi: u64, filety: u64) -> Self {
        Self {
            abi,
            filety,
            ..Self::default()
        }
    }

    /// Set the ABI of the container file. This can be any arbitrary value you
    /// choose.
    pub fn abi(&mut self, abi: u64) {
        self.abi = abi;
    }

    /// Set the file type of the container file. This can be any arbitrary
    /// value you choose.
    pub fn filety(&mut self, filety: u64) {
        self.filety = filety;
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
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    /// Remove a section from the container file. Does nothing if the section
    /// does not exist.
    pub fn remove_section(&mut self, name: &str) {
        let idx = self.sections.iter().position(|s| s.name == name);
        if let Some(idx) = idx {
            self.sections.remove(idx);
        }
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

    /// Iterate mutably over the sections in the container file.
    pub fn sections_mut(&mut self) -> impl Iterator<Item = &mut Section> {
        self.sections.iter_mut()
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
                .map(Section::decode)
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
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Section {
    name: String,
    stype: u64,
    flags: u64,
    data: Option<Vec<u8>>,
    offset: Option<u64>,
    size: Option<u64>,
}

impl Section {
    /// Create a section. The section type (`stype`) or flags can be any
    /// arbitrary value you choose
    #[must_use]
    pub fn new(name: String, stype: u64, flags: u64) -> Self {
        Self {
            name,
            stype,
            flags,
            data: Some(Vec::new()),
            ..Self::default()
        }
    }

    /// Get the name of the section.
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set the type of the section. This can be any arbitrary value you
    /// choose.
    pub fn stype(&mut self, stype: u64) {
        self.stype = stype;
    }

    /// Get the type of the section.
    #[must_use]
    pub fn get_stype(&self) -> u64 {
        self.stype
    }

    /// Set the flags of the section. This can be any arbitrary value you
    /// choose.
    pub fn flags(&mut self, flags: u64) {
        self.flags = flags;
    }

    /// Get the flags of the section.
    #[must_use]
    pub fn get_flags(&self) -> u64 {
        self.flags
    }

    /// Set the data of the section. This can be any arbitrary data you choose.
    #[must_use]
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
            .seek(SeekFrom::Start(self.offset.ok_or_else(|| {
                IOError::new(IOErrorKind::InvalidInput, "offset was not provided")
            })?))
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
    #[must_use]
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

    /// Decode a section from its lower-level counterpart.
    #[must_use]
    pub fn decode(section: CcffSectionHeader) -> Self {
        Section {
            data: None,
            flags: section.flags,
            stype: section.section_type,
            name: section.name,
            offset: Some(section.offset),
            size: Some(section.size),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use pretty_assertions::assert_eq;

    use std::io::Cursor;

    use super::*;

    static VALID_CONTAINER_FILE: &[u8] = b"\xcc\xff\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x05\x00\x00\x00\x00\x00\x00\x00\x2e\x63\x6f\x64\x65\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x47\x00\x00\x00\x00\x00\x00\x00\x1f\x00\x00\x00\x00\x00\x00\x00\x73\x6f\x6d\x65\x20\x62\x79\x74\x65\x63\x6f\x64\x65\x20\x64\x61\x74\x61\x20\x68\x65\x72\x65\x20\x69\x20\x67\x75\x65\x73\x73";

    #[test]
    fn container_create() {
        let mut container = ContainerFile::new(1, 1);
        assert_eq!(
            container,
            ContainerFile {
                abi: 1,
                filety: 1,
                sections: vec![]
            }
        );
        assert_eq!(container.get_abi(), 1);
        assert_eq!(container.get_filety(), 1);

        container.abi(5);
        container.filety(7);

        assert_eq!(container.get_abi(), 5);
        assert_eq!(container.get_filety(), 7);

        assert_eq!(container.clone(), container);
    }

    #[test]
    fn container_section_add() {
        let mut container = ContainerFile::new(1, 1);
        container.add_section(Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec()));
        assert_eq!(
            container,
            ContainerFile {
                abi: 1,
                filety: 1,
                sections: vec![Section {
                    name: ".foo".to_string(),
                    stype: 1,
                    flags: 0,
                    data: Some(b"foo".to_vec()),
                    offset: None,
                    size: None
                }]
            }
        );
    }

    #[test]
    fn container_section_remove() {
        let mut container = ContainerFile::new(1, 1);
        container.add_section(Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec()));
        container.remove_section(".foo");
        assert_eq!(
            container,
            ContainerFile {
                abi: 1,
                filety: 1,
                sections: vec![]
            }
        );
    }

    #[test]
    fn container_section_get() {
        let mut container = ContainerFile::new(1, 1);
        container.add_section(Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec()));
        assert_eq!(
            container.get_section(".foo").unwrap(),
            &Section {
                name: ".foo".to_string(),
                stype: 1,
                flags: 0,
                data: Some(b"foo".to_vec()),
                offset: None,
                size: None
            }
        );
        assert_eq!(
            &container.get_section(".foo").unwrap().clone(),
            container.get_section(".foo").unwrap()
        );
    }

    #[test]
    fn container_section_get_mut() {
        let mut container = ContainerFile::new(1, 1);
        container.add_section(Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec()));
        assert_eq!(
            container.get_section_mut(".foo").unwrap(),
            &mut Section {
                name: ".foo".to_string(),
                stype: 1,
                flags: 0,
                data: Some(b"foo".to_vec()),
                offset: None,
                size: None
            }
        );
    }

    #[test]
    fn container_sections_iter() {
        let mut container = ContainerFile::new(1, 1);
        container.add_section(Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec()));
        assert_eq!(
            container.sections().next().unwrap(),
            &Section {
                name: ".foo".to_string(),
                stype: 1,
                flags: 0,
                data: Some(b"foo".to_vec()),
                offset: None,
                size: None
            }
        );
    }

    #[test]
    fn container_sections_iter_mut() {
        let mut container = ContainerFile::new(1, 1);
        container.add_section(Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec()));
        container.add_section(Section::new(".bar".to_string(), 2, 2).data(b"bar".to_vec()));
        for section in container.sections_mut() {
            section.stype(section.stype + 1);
            section.flags(section.flags | 0b1000);
        }
        let mut sections = container.sections();
        assert_eq!(
            sections.next().unwrap(),
            &Section {
                name: ".foo".to_string(),
                stype: 2,
                flags: 0b1000,
                data: Some(b"foo".to_vec()),
                offset: None,
                size: None
            }
        );
        assert_eq!(
            sections.next().unwrap(),
            &Section {
                name: ".bar".to_string(),
                stype: 3,
                flags: 0b1010,
                data: Some(b"bar".to_vec()),
                offset: None,
                size: None
            }
        );
    }

    #[test]
    fn container_encode() {
        let mut container = ContainerFile::new(1, 1);
        container.add_section(Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec()));
        let (hdr, data) = container.encode();
        assert_eq!(
            hdr,
            CcffHeader {
                magic: *b"\xCC\xFF",
                abi: 1,
                filety: 1,
                sections: vec![CcffSectionHeader {
                    name: ".foo".to_string(),
                    section_type: 1,
                    flags: 0,
                    offset: 70,
                    size: 3
                }]
            }
        );
        assert_eq!(&data, b"foo");
    }

    #[test]
    fn container_decode() {
        let hdr = CcffHeader {
            magic: *b"\xCC\xFF",
            abi: 1,
            filety: 1,
            sections: vec![CcffSectionHeader {
                name: ".foo".to_string(),
                section_type: 1,
                flags: 0,
                offset: 70,
                size: 3,
            }],
        };

        assert_eq!(
            ContainerFile::decode(hdr),
            ContainerFile {
                abi: 1,
                filety: 1,
                sections: vec![Section {
                    name: ".foo".to_string(),
                    stype: 1,
                    flags: 0,
                    offset: Some(70),
                    size: Some(3),
                    data: None
                }]
            }
        );
    }

    #[test]
    fn container_read_all() {
        let mut container = ContainerFile {
            abi: 1,
            filety: 1,
            sections: vec![Section {
                name: ".code".to_string(),
                stype: 1,
                flags: 0,
                offset: Some(0x47),
                size: Some(0x1f),
                data: None,
            }],
        };
        let mut cursor = Cursor::new(VALID_CONTAINER_FILE);
        container.read_all(&mut cursor).unwrap();
        assert_eq!(
            container.get_section(".code").unwrap().get_data().unwrap(),
            b"some bytecode data here i guess"
        );
    }

    #[test]
    fn section_create() {
        let mut section = Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec());
        assert_eq!(
            section,
            Section {
                name: ".foo".to_string(),
                stype: 1,
                flags: 0,
                data: Some(b"foo".to_vec()),
                offset: None,
                size: None,
            }
        );
        assert_eq!(section.get_name(), &section.name);
        assert_eq!(section.get_stype(), section.stype);
        assert_eq!(section.get_flags(), section.flags);
        assert_eq!(section.get_data().unwrap(), b"foo");
        let mut tmp = b"foo".to_vec();
        assert_eq!(section.get_data_mut().unwrap(), &mut tmp);
        assert_eq!(section.get_offset(), None);
    }

    #[test]
    fn section_encode() {
        let section = Section::new(".foo".to_string(), 1, 0).data(b"foo".to_vec());
        let (hdr, data) = section.encode();
        assert_eq!(
            hdr,
            CcffSectionHeader {
                name: ".foo".to_string(),
                section_type: 1,
                flags: 0,
                offset: 0,
                size: 3
            }
        );
        assert_eq!(&data, b"foo");
    }

    #[test]
    fn section_decode() {
        let hdr = CcffSectionHeader {
            name: ".foo".to_string(),
            section_type: 1,
            flags: 0,
            offset: 70,
            size: 3,
        };
        assert_eq!(
            Section::decode(hdr),
            Section {
                name: ".foo".to_string(),
                stype: 1,
                flags: 0,
                offset: Some(70),
                size: Some(3),
                data: None
            }
        );
    }

    #[test]
    fn section_seek_to_data() {
        let section = Section {
            name: ".code".to_string(),
            stype: 1,
            flags: 0,
            offset: Some(0x47),
            size: Some(0x1f),
            data: None,
        };
        let mut cursor = Cursor::new(VALID_CONTAINER_FILE);
        section.seek_to_data(&mut cursor).unwrap();
        let pos = cursor.position();
        assert_eq!(pos, 0x47);
    }

    #[test]
    fn section_read_data() {
        let mut section = Section {
            name: ".code".to_string(),
            stype: 1,
            flags: 0,
            offset: Some(0x47),
            size: Some(0x1f),
            data: None,
        };
        let mut cursor = Cursor::new(VALID_CONTAINER_FILE);
        section.read_data(&mut cursor).unwrap();
        assert_eq!(
            section.get_data().unwrap(),
            b"some bytecode data here i guess"
        );
    }
}
