# Calypso Container File Format (CCFF)

The beginning of an CCFF file should have two magic bytes: `[0xCC, 0xFF]`. The next part of the file, with no padding, should contain a single instance of the File Header (`CCFF`). The remainder of the data is section data.

All multi-byte fields in any CCFF structure (excluding the magic bytes) are assumed to use little-endian encoding.

### File Header (`CCFF`)

Offsets are from the start of the container file, minus the 3 magic bytes.

| Offset (bytes) |    Size (bytes)     |     Name     |         Type          | Description                                                  |
| :------------: | :-----------------: | :----------: | :-------------------: | :----------------------------------------------------------- |
|     `0x0`      |        `0x8`        |    `abi`     |         `u64`         | A user-defined ABI version                                   |
|     `0x8`      |        `0x8`        |   `filety`   |         `u64`         | A user-defined file type                                     |
|     `0x10`     |        `0x8`        | `n_sections` |         `u64`         | The number of sections in this container file                |
|     `0x18`     | `0x20 * n_sections` |  `sections`  | `[CCFFSectionHeader]` | An array (consecutive memory with no padding) of section headers `n_sections` long |

### Section Header (`CCFFSectionHeader`)

Offsets are from the start of the `CCFFSectionHeader`.

| Offset (bytes) |   Size (bytes)   |      Name      | Type  | Description                                                  |
| :------------: | :--------------: | :------------: | :---: | :----------------------------------------------------------- |
|     `0x0`      |      `0x8`       |   `name_len`   | `u64` | The byte size of the UTF-8-encoded section name              |
|     `0x8`      | `0x1 * name_len` |     `name`     | `str` | The section name as UTF-8 data                               |
|     `0x8`      |      `0x8`       | `section_type` | `u64` | A user-defined section type                                  |
|     `0x10`     |      `0x8`       |    `offset`    | `u64` | The byte offset from the start of the file, including the magic bytes, for this section |
|     `0x18`     |      `0x8`       |     `size`     | `u64` | The byte size of the uncompressed data for this section      |
