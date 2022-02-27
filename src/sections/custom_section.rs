use super::*;

/// Reference: https://webassembly.github.io/spec/core/binary/modules.html#binary-customsec
#[derive(Clone, Debug)]
pub struct CustomSection<'a> {
    pub data: Cow<'a, [u8]>,
}

impl<'a> CustomSection<'a> {
    const ID: u8 = 0x00;

    /// Return sustom section id
    pub fn id() -> u8 {
        Self::ID
    }

    pub fn byte_count(&self) -> usize {
        todo!();
    }
}

pub trait GetLocation<MultiLocation> {
    fn is_supported_location(&self, localtion: MultiLocation) -> bool;
}
