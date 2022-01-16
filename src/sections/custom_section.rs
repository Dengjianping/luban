/// Reference: https://webassembly.github.io/spec/core/binary/modules.html#binary-customsec
#[derive(Clone, Debug)]
pub struct CustomSegment<'a> {
    data: &'a [u8],
}

impl<'a> CustomSegment<'a> {
    const ID: u8 = 0x00;
}
