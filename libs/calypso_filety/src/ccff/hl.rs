use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{BufWriter, Cursor};
use std::mem;

use calypso_diagnostic::error::Result as CalResult;

use super::ll::{Cc, CcHdr, CcSectionHdr};
use super::Compression;

#[derive(Debug, Clone)]
pub struct ContainerFile {
    header: ContainerHeader,
    sections: HashMap<String, Section>,
}

#[derive(Debug, Copy, Clone)]
pub struct ContainerHeader {
    abi: u8,
}

#[derive(Debug, Clone)]
pub struct Section {
    r#type: u64,
    flags: u64,
    data: Vec<u8>,
}

impl ContainerFile {
    pub fn new(header: ContainerHeader) -> Self {
        Self {
            header,
            sections: HashMap::new(),
        }
    }

    pub fn add_section(&mut self, name: String, section: Section) -> &mut Self {
        assert!(name.starts_with('.'), "section name must start with `.`");
        self.sections.insert(name, section);
        self
    }

    /// Remove a section from the file. Returns true if it was removed,
    /// otherwise false.
    pub fn remove_section(&mut self, name: &str) -> bool {
        self.sections.remove(name).is_some()
    }

    pub fn get_section(&self, name: &str) -> Option<&Section> {
        self.sections.get(name)
    }

    pub fn get_section_mut(&mut self, name: &str) -> Option<&mut Section> {
        self.sections.get_mut(name)
    }

    pub fn get_header(&self) -> &ContainerHeader {
        &self.header
    }

    pub fn get_header_mut(&mut self) -> &mut ContainerHeader {
        &mut self.header
    }

    pub fn into_ll(self) -> CalResult<(Cc, Vec<u8>)> {
        let mut strtab = Vec::new();
        let mut strtab_indices = HashMap::new();
        strtab_indices.insert(".shstrtab", 0);
        strtab.extend(bincode::serialize(".shstrtab")?);
        for (name, _) in self.sections.iter() {
            strtab_indices.insert(name, strtab.len());
            strtab.extend(bincode::serialize(name)?);
        }
        // 1: section header string table
        let shstrtab = Section::new(1, 0, strtab);

        let mut data = Vec::new();
        let mut data_indices = HashMap::new();
        data_indices.insert(".shstrtab", 0);
        data.extend(shstrtab.get_data());
        for (name, section) in self.sections.iter() {
            data_indices.insert(name, data.len());
            data.extend(section.get_data());
        }

        let num_sections = self.sections.len() + 1;
        let section_hdr_size = mem::size_of::<CcSectionHdr>();
        let hdr_size = mem::size_of::<CcHdr>();
        let data_offset = (section_hdr_size * num_sections) + hdr_size + mem::size_of::<u64>();

        let mut sections = Vec::new();
        sections.push(CcSectionHdr {
            name: *strtab_indices.get(".shstrtab").unwrap() as u64,
            section_type: shstrtab.r#type,
            flags: shstrtab.flags,
            offset: (*data_indices.get(".shstrtab").unwrap() + data_offset) as u64,
            size: shstrtab.get_data().len() as u64,
        });
        for (name, section) in self.sections.iter() {
            let data_index = *data_indices.get(&&**name).unwrap() + data_offset;
            let strtab_index = *strtab_indices.get(&&**name).unwrap();
            sections.push(CcSectionHdr {
                name: strtab_index as u64,
                section_type: section.r#type,
                flags: section.flags,
                offset: data_index as u64,
                size: section.get_data().len() as u64,
            });
        }

        let hdr = CcHdr {
            abi: self.header.abi,
        };

        let ll = Cc {
            header: hdr,
            sections,
        };

        Ok((ll, data))
    }

    pub fn into_bytes(self, compression: Compression) -> CalResult<Vec<u8>> {
        let (ll, data) = self.into_ll()?;
        let buf = Vec::new();
        let cursor = Cursor::new(buf);
        let mut writer = BufWriter::new(cursor);
        ll.write(&mut writer, compression)?;
        writer.write_all(&data)?;

        Ok(writer
            .into_inner()
            .map_err(|_| "failed to flush buffer")?
            .into_inner())
    }
}

impl ContainerHeader {
    pub fn new(abi: u8) -> Self {
        Self { abi }
    }

    pub fn get_abi(&self) -> u8 {
        self.abi
    }

    pub fn set_abi(&mut self, abi: u8) -> &mut Self {
        self.abi = abi;
        self
    }
}

impl Section {
    pub fn new(r#type: u64, flags: u64, data: Vec<u8>) -> Self {
        Self {
            r#type,
            flags,
            data,
        }
    }

    pub fn get_type(&self) -> u64 {
        self.r#type
    }

    pub fn set_type(&mut self, r#type: u64) -> &mut Self {
        self.r#type = r#type;
        self
    }

    pub fn get_flags(&self) -> u64 {
        self.flags
    }

    pub fn set_flags(&mut self, flags: u64) -> &mut Self {
        self.flags = flags;
        self
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}
