use super::*;

/// Reference: https://webassembly.github.io/spec/core/binary/modules.html#type-section
#[derive(Clone, Debug)]
pub struct TypeSection<'a> {
    data: Cow<'a, [u8]>,
    pub func_types: Vec<()>,
}

impl<'a> TypeSection<'a> {
    const ID: u8 = 0x01;

    /// Return type segmemnt id
    pub fn id() -> u8 {
        Self::ID
    }

    pub fn byte_count(&self) -> usize {
        todo!();
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        assert_eq!(bytes[0], Self::ID);

        let content_length = bytes[1];

        todo!()
    }
}
