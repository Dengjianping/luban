use super::*;
use crate::utils::*;

/// https://webassembly.github.io/spec/core/binary/modules.html#import-section
#[derive(Clone, Debug)]
pub struct ImportSection<'a> {
    data: Cow<'a, [u8]>,
    desc: Vec<ImportDescription<'a>>,
}

#[derive(Clone, Debug)]
pub struct ImportDescription<'a> {
    module: Cow<'a, [u8]>,
    name: Cow<'a, [u8]>,
    tag: ImportTag,
    desc: ImportDesc,
}

impl<'a> ImportDescription<'a> {
    pub(crate) fn from_bytes(bytes: &'a [u8]) -> Self {
        todo!();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImportTag {
    Func = 0x00,
    Table = 0x01,
    Mem = 0x02,
    Global = 0x03,
}

pub type TypeIndex = u32;

#[derive(Clone, Copy, Debug)]
pub enum ImportDesc {
    Func(TypeIndex),
    // Table(),
    // Mem = 0x02,
    // Global = 0x03,
}

// #[derive(Clone, Debug)]
// pub struct Tab

impl<'a> ImportSection<'a> {
    pub const ID: u8 = 0x02;

    pub fn id(&self) -> u8 {
        self.data[0]
    }

    pub fn from_bytes(bytes: &[u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        assert_eq!(_bytes[0], Self::ID);

        // get the length of content for type section
        let (content_length, leb_offset) = uleb128_decode(&_bytes[1..]);
        let content_bytes = &_bytes[1 + leb_offset + 1..1 + leb_offset + content_length];

        let import_count = _bytes[2] as usize;
        let mut description: Vec<ImportDescription> = Vec::with_capacity(import_count);

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn deserialoze_import_section_should_work() {
        let import_sec_bytes = [2u8, 11, 1, 3, 102, 111, 111, 3, 98, 97, 114, 0, 0];
        let import_sec = ImportSection::from_bytes(&import_sec_bytes, 0);
    }
}
