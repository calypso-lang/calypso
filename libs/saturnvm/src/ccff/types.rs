//! Types that are used within the CCFF interface, specifically bitflags, file
//! types, and section types.

use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// The type of the file.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u64)]
pub enum FileType {
    /// Unspecified file type
    Null,
    /// Executable files
    Executable,
    /// Object files
    Object,
    /// Shared object files
    DynamicObject,
}

/// The type of the section.
#[derive(Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u64)]
pub enum SectionType {
    /// The atom table.
    AtomTable,
    /// String data. This is simply just strings inlined without any other
    /// padding or metadata.
    StrData,
}

bitflags! {
    /// The flags for a section. These currently just apply to permissions and
    /// lifetime information (i.e. static vs non-static).
    pub struct SectionFlags: u64 {
        /// This section has no flags.
        const NONE   = 0b0000;
        /// This section's data is readable.
        const READ   = 0b0001;
        /// This section's data is writable.
        const WRITE  = 0b0010;
        /// This section's data is executable.
        const EXEC   = 0b0100;
        /// This section's data lives for the entirety of the program.
        const STATIC = 0b1000;
        /// [`Self::READ`] | [`Self::EXEC`]
        const RX = Self::READ.bits | Self::EXEC.bits;
        /// [`Self::READ`] | [`Self::WRITE`]
        const RW = Self::READ.bits | Self::WRITE.bits;
        /// [`Self::READ`] | [`Self::STATIC`]
        const RS = Self::READ.bits | Self::STATIC.bits;
    }
}
