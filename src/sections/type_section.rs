use super::*;

/// Reference: https://webassembly.github.io/spec/core/binary/modules.html#type-section
#[derive(Clone, Debug)]
pub struct TypeSegment<'a> {
    pub data: Cow<'a, [u8]>,
}

impl<'a> TypeSegment<'a> {
    const ID: u8 = 0x01;

    /// Return type segmemnt id
    pub fn id() -> u8 {
        Self::ID
    }

    pub fn byte_count(&self) -> usize {
        todo!();
    }
}
