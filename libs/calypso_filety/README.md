# `calypso_filety`

Binary file type definitions, parsers, high-level interfaces, and more for file types used in [Calypso](https://github.com/calypso-lang/calypso).

File types currently included are
- Calypso Container File Format (CCFF). For more information, see [the "spec"](https://github.com/calypso-lang/calypso/blob/main/libs/calypso_filety/ccff.md). Interfaces are located in the module `ccff`, with a high-level interface in `ccff::hl` and a lower-level (as the binary format with bincode) interface in `ccff::ll`.