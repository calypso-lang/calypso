# Calypso Container File Format (CCFF)

This is a semi-normative specification of the Calypso Container File Format (CCFF), used within Calypso and SaturnVM. Assume all multi-byte fields are little-endian, unless otherwise specified. No padding is present between fields.

The beginning of a CCFF file must contain four big-endian bytes: `0x43434646` (`CCFF` in ASCII encoding). This is the magic number of the CCFF file, and must be present for a CCFF file to be valid.

The next part of the file must contain an instance of a [file header (`Hdr`)](#file-header-hdr). The rest of the file after this header is section data.

## File Header (`Hdr`)

The offsets provided in this table are from the start of the `Hdr`. No padding is present and multi-byte fields are little-endian.

| Offset (bytes) | Size (bytes) |      Name       |      Type      | Description                                                  |
| :------------: | :----------: | :-------------: | :------------: | :----------------------------------------------------------- |
|     `0x00`     |    `0x02`    |    `abiver`     |     `u16`      | A user-defined ABI version.                                  |
|     `0x02`     |    `0x01`    |    `filety`     |      `u8`      | A user-defined file type.                                    |
|     `0x03`     |    `0x01`    | `len(sections)` |      `u8`      | The number of sections in this container file.               |
|     `0x04`     |  `dynamic`   |   `sections`    | `[SectionHdr]` | An array (consecutive memory with no padding) of [section headers](#section-header-sectionhdr) `len(sections)` long. Note that because `SectionHdr` contains a dynamic-length field, accessing a section is O(n), so implementations should ideally store sections in a `HashMap` (or some other near-O(1) lookup complexity data structure) from section name (or some other unique identifier) to the section's metadata and/or data. |

## Section Header (`SectionHdr`)

The offsets provided in this table are from the start of the `SectionHdr`. No padding is present and multi-byte fields are little-endian.

| Offset (bytes) |  Size (bytes)  |      Name      | Type  | Description                                                  |
| :------------: | :------------: | :------------: | :---: | :----------------------------------------------------------- |
|     `0x00`     |     `0x01`     |     `type`     | `u8`  | A user-defined section type.                                 |
|     `0x01`     |     `0x04`     |    `flags`     | `u32` | User-defined section bitflags.                               |
|     `0x05`     |     `0x08`     |    `offset`    | `u64` | The byte offset of this section's data from the start of the file, including the magic bytes. |
|     `0x0d`     |     `0x08`     |     `size`     | `u64` | The byte size of this section's data.                        |
|     `0x15`     |     `0x01`     | `sizeof(name)` | `u8`  | The byte size of the section name.                           |
|     `0x16`     | `sizeof(name)` |     `name`     | `str` | ASCII-encoded section name. Must contain only the characters `A-Za-z0-9_` and must be unique. |

