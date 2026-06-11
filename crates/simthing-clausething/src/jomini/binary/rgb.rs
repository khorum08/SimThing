// Vendored from github.com/rakaly/jomini @ v0.34.1 (commit fff00d8c7f8f06c084d776d1a2c98b34324e64ed)
// License: MIT - see crates/simthing-clausething/src/jomini/LICENSE
/// Extracted color info
///
/// This is only for the binary format. RGB values that are in plaintext are
/// behind a [TextToken::Header](crate::jomini::TextToken::Header) of `rgb`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    /// Red channel
    pub r: u32,

    /// Green channel
    pub g: u32,

    /// Blue channel
    pub b: u32,

    /// Optional alpha channel
    pub a: Option<u32>,
}
