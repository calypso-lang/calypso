/// A high-level interface for CCFF files
pub mod hl;
/// Low-level (binary representation with bincode) for CCFF files.
/// There is no guarantee that this system will generate a valid
/// CCFF file if used wrong. It's recommended to use the high-level
/// interface as it should never generate an invalid file.
pub mod ll;

pub use flate2::Compression as CompressionLevel;
#[derive(Debug, Copy, Clone)]
pub enum Compression {
    Compressed(CompressionLevel),
    Uncompressed,
}
