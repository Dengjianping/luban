use super::*;

/// Reference: https://webassembly.github.io/spec/core/binary/modules.html#binary-customsec
#[derive(Clone, Debug)]
pub struct CustomSegment<'a> {
    pub data: Cow<'a, [u8]>,
}

impl<'a> CustomSegment<'a> {
    const ID: u8 = 0x00;

    /// Return sustom segmemnt id
    pub fn id() -> u8 {
        Self::ID
    }

    pub fn byte_count(&self) -> usize {
        todo!();
    }
}
