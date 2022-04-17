
use super::*;
use crate::instructions::*;
use crate::utils::*;

// only can export 4 types: function, table, memory, global

#[derive(Clone, Debug)]
pub struct StartSection<'a> {
    data: Cow<'a, [u8]>,
    func_idx: usize,
}

impl<'a> StartSection<'a> {
    const ID: u8 = 8;

    pub fn id() -> u8 {
        Self::ID
    }

    pub(crate) fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        assert_eq!(_bytes[0], Self::ID);

        let data = Cow::Borrowed(&_bytes[offset..offset + 3]);
        let (len, off) = unsigned_leb128_decode(&data);
        let index = data[2] as usize;
        Self {
            data, func_idx: index
        }
    }
}
