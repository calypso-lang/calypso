use std::collections::HashMap;
use std::mem;
use std::slice::Iter;

use super::ll::{Cc, CcHdr, CcSectionHdr};
use super::Compression;

use bincode::ErrorKind;

#[derive(Debug, Clone)]
pub struct ContainerFile {
    header: ContainerHeader,
    sections: Vec<(String, Section)>,
    compressed: Option<bool>,
}

#[derive(Debug, Copy, Clone)]
pub struct ContainerHeader {
    abi: u64,
    filety: u64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SectionType {
    // Section header string table (`1`)
    ShStrTab,
    Other(u64),
}

impl From<SectionType> for u64 {
    fn from(r#type: SectionType) -> Self {
        match r#type {
            SectionType::ShStrTab => 1,
            SectionType::Other(r#type) => {
                assert!(
                    r#type != 0 && r#type != 1,
                    "SectionType::Other cannot have a section type of ShStrTab (1)"
                );
                r#type
            }
        }
    }
}

impl From<u64> for SectionType {
    fn from(r#type: u64) -> SectionType {
        match r#type {
            1 => SectionType::ShStrTab,
            other => SectionType::Other(other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Section {
    name_offset: Option<u64>,
    r#type: SectionType,
    flags: u64,
    offset: Option<u64>,
    data: Vec<u8>,
}

impl ContainerFile {
    pub fn new(header: ContainerHeader) -> Self {
        Self {
            header,
            sections: Vec::new(),
            compressed: None,
        }
    }

    pub fn add_section(&mut self, name: String, section: Section) -> &mut Self {
        self.sections.push((name, section));
        self
    }

    /// Remove a section from the file. Returns true if it was removed,
    /// otherwise false.
    pub fn remove_section(&mut self, name: &str) -> bool {
        let item = self
            .sections
            .iter()
            .enumerate()
            .find(|elem| elem.1 .0 == name)
            .map(|elem| elem.0);
        if let Some(idx) = item {
            self.sections.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn get_section(&self, name: &str) -> Option<&Section> {
        self.sections
            .iter()
            .find(|&elem| elem.0 == name)
            .map(|elem| &elem.1)
    }

    pub fn get_section_mut(&mut self, name: &str) -> Option<&mut Section> {
        self.sections
            .iter_mut()
            .find(|elem| elem.0 == name)
            .map(|elem| &mut elem.1)
    }

    pub fn get_header(&self) -> &ContainerHeader {
        &self.header
    }

    pub fn get_header_mut(&mut self) -> &mut ContainerHeader {
        &mut self.header
    }

    pub fn sections_iter(&self) -> Iter<'_, (String, Section)> {
        self.sections.iter()
    }

    pub fn into_ll(self) -> bincode::Result<(Cc, Vec<u8>)> {
        let mut strtab = Vec::new();
        let mut strtab_indices = HashMap::new();
        strtab_indices.insert(".shstrtab", 0);
        strtab.extend(bincode::serialize(".shstrtab")?);
        for (name, _) in self.sections.iter() {
            strtab_indices.insert(name, strtab.len());
            strtab.extend(bincode::serialize(name)?);
        }
        let shstrtab = Section::new(SectionType::ShStrTab, 0, strtab);

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
            section_type: shstrtab.r#type.into(),
            flags: shstrtab.flags,
            offset: (*data_indices.get(".shstrtab").unwrap() + data_offset) as u64,
            size: shstrtab.get_data().len() as u64,
        });
        for (name, section) in self.sections.iter() {
            if section.r#type == SectionType::ShStrTab {
                // Don't include the .shstrtab section if present as we calculate that anyway
                continue;
            }
            let data_index = *data_indices.get(&&**name).unwrap() + data_offset;
            let strtab_index = *strtab_indices.get(&&**name).unwrap();
            sections.push(CcSectionHdr {
                name: strtab_index as u64,
                section_type: section.r#type.into(),
                flags: section.flags,
                offset: data_index as u64,
                size: section.get_data().len() as u64,
            });
        }

        let hdr = CcHdr {
            abi: self.header.abi,
            filety: self.header.filety,
        };

        let ll = Cc {
            header: hdr,
            sections,
        };

        Ok((ll, data))
    }

    pub fn into_bytes(self, compression: Compression) -> bincode::Result<Vec<u8>> {
        let (ll, data) = self.into_ll()?;
        let bytes = ll.write(compression, &data)?;

        Ok(bytes)
    }

    pub fn from_ll(ll: Cc, data: Vec<u8>) -> bincode::Result<Self> {
        let header = ContainerHeader {
            abi: ll.header.abi,
            filety: ll.header.filety,
        };
        let mut container = Self::new(header);
        let section_type = ll
            .sections
            .first()
            .ok_or_else(|| {
                Box::new(ErrorKind::Custom(
                    "the section header string table must be present".to_string(),
                ))
            })?
            .section_type;
        if section_type != 1 {
            return Err(Box::new(ErrorKind::Custom(
                "the section header string table must be first".to_string(),
            )));
        }
        for section in &ll.sections {
            let name_offset = section.name;
            let name: String = bincode::deserialize(&data[name_offset as usize..])?;
            let offset = section.offset as usize - ll.size();
            let end = offset + section.size as usize;
            let data = data[offset..end].to_vec();
            let mut section = Section::new(section.section_type.into(), section.flags, data);
            section.offset = Some(offset as u64 + ll.size() as u64);
            section.name_offset = Some(name_offset);
            container.add_section(name, section);
        }
        Ok(container)
    }

    pub fn from_bytes(buf: Vec<u8>) -> bincode::Result<Self> {
        let (ll, data) = Cc::load(buf)?;
        Self::from_ll(ll, data)
    }

    pub fn is_compressed(buf: &[u8]) -> bincode::Result<bool> {
        Cc::is_compressed(buf)
    }
}

impl ContainerHeader {
    pub fn new(abi: u64, filety: u64) -> Self {
        Self { abi, filety }
    }

    pub fn get_abi(&self) -> u64 {
        self.abi
    }

    pub fn set_abi(&mut self, abi: u64) -> &mut Self {
        self.abi = abi;
        self
    }

    pub fn get_filety(&self) -> u64 {
        self.filety
    }

    pub fn set_filety(&mut self, filety: u64) -> &mut Self {
        self.filety = filety;
        self
    }
}

impl Section {
    pub fn new(r#type: SectionType, flags: u64, data: Vec<u8>) -> Self {
        Self {
            name_offset: None,
            offset: None,
            r#type,
            flags,
            data,
        }
    }

    pub fn get_type(&self) -> SectionType {
        self.r#type
    }

    pub fn set_type(&mut self, r#type: SectionType) -> &mut Self {
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

    pub fn get_name_offset(&self) -> Option<u64> {
        self.name_offset
    }

    pub fn get_offset(&self) -> Option<u64> {
        self.offset
    }
}
