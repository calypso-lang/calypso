/// A high-level interface for CCFF files
pub mod hl;
/// Low-level (binary representation with bincode) for CCFF files
pub mod ll;

pub use flate2::Compression as CompressionLevel;
#[derive(Debug, Copy, Clone)]
pub enum Compression {
    Compressed(CompressionLevel),
    Uncompressed,
}
