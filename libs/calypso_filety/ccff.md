# Calypso Container File Format (CCFF)

## Uncompressed Data Format

The beginning of an uncompressed CCFF file should have three magic bytes: `[0xCC, 0xFF, 0x55]`. The next part of the file, with no padding, should contain a single instance of the File Header (`CCFF`). The remainder of the data is section data.

All multi-byte fields in any CCFF structure (excluding the magic bytes) are assumed to use little-endian encoding.

### File Header (`CCFF`)

Offsets are from the start of the container file, minus the 3 magic bytes.

| Offset (bytes) |    Size (bytes)     |     Name     |       Type       | Description                                                  |
| :------------: | :-----------------: | :----------: | :--------------: | :----------------------------------------------------------- |
|     `0x0`      |       `0x10`        |   `header`   |    `CCFFHdr`     | File metadata                                                |
|     `0x10`     |        `0x8`        | `n_sections` |      `u64`       | The number of sections in this container file                |
|     `0x18`     | `0x20 * n_sections` |  `sections`  | `CCFFSectionHdr` | An array (consecutive memory with no padding) of section headers `n_sections` long |
### File Metadata (`CCFFHdr`)

Offsets are from the start of the `CCFFHdr`.

| Offset (bytes) | Size (bytes) |   Name   | Type  | Description                |
| :------------: | :----------: | :------: | :---: | :------------------------- |
|     `0x0`      |    `0x8`     |  `abi`   | `u64` | A user-defined ABI version |
|     `0x8`      |    `0x8`     | `filety` | `u64` | A user-defined file type   |

### Section Header (`CCFFSectionHdr`)The

Offsets are from the start of the `CCFFSectionHdr`.

| Offset (bytes) | Size (bytes) |      Name      | Type  | Description                                                  |
| :------------: | :----------: | :------------: | :---: | :----------------------------------------------------------- |
|     `0x0`      |    `0x8`     |     `name`     | `u64` | The section name as a byte offset from the start of the section header string table |
|     `0x8`      |    `0x8`     | `section_type` | `u64` | A user-defined section type. Section type `1` is reserved for the section header string table |
|     `0x10`     |    `0x8`     |    `offset`    | `u64` | The byte offset from the start of the uncompressed data, minus the three magic bytes, for this section |
|     `0x18`     |    `0x8`     |     `size`     | `u64` | The byte size of the uncompressed data for this section      |

### Section Header String Table (`.shstrtab`)

The section header string table is a required section in all CCFF files (even if there are no other sections). It's required to be called `.shstrtab`. It is required to be the first section in a CCFF file, with type `1`. While there may be other sections in the file with type `1` that should be ignored by the parser, it is discouraged to use that as a section type.

It is encoded as a consecutive array of UTF-8 length-strings (their individual encoding is described in the section titled "Section Header String Table String Encoding"). The array should not have a length parameter as the section names are encoded in the section header as an offset into the section header string table.

#### Section Header String Table String Encoding

| Offset (bytes) | Size (bytes) | Name  | Type  | Description                       |
| :------------: | :----------: | :---: | :---: | :-------------------------------- |
|     `0x0`      |    `0x8`     | `len` | `u64` | The length of the string as UTF-8 |
|     `0x8`      | `0x1 * len`  | `str` | `str` | The string's data as UTF-8        |

## Compressed Data Format

The beginning of an compressed CCFF file should have three magic bytes: `[0xCC, 0xFF, 0x5A]`. The remainder of the file should contain a single zlib stream (with a header), containing, when uncompressed, the following (in order):

1. A single instance of a File Header (`CCFF`), encoded as it would be in an uncompressed file
2. Section data

Offsets in a compressed file are assumed to be offsets into the uncompressed data. They still do not include any magic bytes (though there are no magic bytes in the zlib stream).