/// Reference: https://webassembly.github.io/spec/core/binary/modules.html#type-section
#[derive(Clone, Debug)]
pub struct TypeSegment<'a> {
    pub data: &'a [u8],
}

impl<'a> TypeSegment<'a> {
    const ID: u8 = 0x01;
}
