use super::*;
use crate::utils::*;
use core::str;

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
    pub(crate) fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        // handle module
        let (leb_len, _bytes) = calc_len_and_offset(bytes, offset);
        let module = Cow::Borrowed(&_bytes[..leb_len]);

        // handle name
        let (leb_len, _bytes) = calc_len_and_offset(_bytes, 0);
        let name = Cow::Borrowed(&_bytes[..leb_len]);

        // tag
        let tag = ImportTag::from(_bytes[leb_len]);

        // description
        let desc = ImportDesc::Func(_bytes[leb_len + 1] as u32);

        Self {
            module,
            name,
            tag,
            desc,
        }
    }

    pub fn module(&self) -> &str {
        str::from_utf8(&self.module).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImportTag {
    Func = 0x00,
    Table = 0x01,
    Mem = 0x02,
    Global = 0x03,
}

impl From<u8> for ImportTag {
    fn from(u: u8) -> Self {
        match u {
            0x00 => Self::Func,
            0x01 => Self::Table,
            0x02 => Self::Mem,
            0x03 => Self::Global,
            _ => panic!("not supported tag"),
        }
    }
}

pub type TypeIndex = u32;

#[derive(Clone, Copy, Debug)]
pub enum ImportDesc {
    Func(TypeIndex),
    // Table(),
    // Mem = 0x02,
    // Global = 0x03,
}

impl<'a> ImportSection<'a> {
    pub const ID: u8 = 0x02;

    pub fn id(&self) -> u8 {
        self.data[0]
    }

    pub fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        assert_eq!(_bytes[0], Self::ID);

        // get the length of content for type section
        // let (content_length, leb_offset) = uleb128_decode(&_bytes[1..]);
        // let content_bytes = &_bytes[1 + leb_offset + 1..1 + leb_offset + content_length];

        // let import_count = _bytes[2] as usize;
        // let mut description: Vec<ImportDescription> = Vec::with_capacity(import_count);

        // // module
        // let mod_bytes_len = content_bytes[0] as usize;
        // let mod_bytes = &content_bytes[1..1 + mod_bytes_len];
        // let module_name = Cow::Borrowed(mod_bytes);

        // // name
        // let name_bytes_len = content_bytes[1 + mod_bytes_len..][0] as usize;
        // let name_bytes = &content_bytes[1 + mod_bytes_len..1 + mod_bytes_len + 1 + name_bytes_len];
        // let name = Cow::Borrowed(name_bytes);

        // 1 means we should skip id, because we already visited it.
        let (len, _bytes) = calc_len_and_offset(_bytes, 1);
        // dbg!(len, _bytes);

        // how many items
        let (len, _bytes) = calc_len_and_offset(_bytes, 1);
        // dbg!(len, &_bytes[..len]);
        let name = str::from_utf8(&_bytes[..len]).unwrap();

        let (len, _bytes) = calc_len_and_offset(&_bytes[len..], 0);
        let name = str::from_utf8(&_bytes[..len]).unwrap();

        let tag = _bytes[len];
        let index = _bytes[len + 1];

        // module

        Self {
            data: Cow::Borrowed(_bytes),
            desc: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn deserialoze_import_section_should_work() {
        let import_sec_bytes = [2u8, 11, 1, 3, 102, 111, 111, 3, 98, 97, 114, 0, 0];
        let import_sec = ImportSection::from_bytes(&import_sec_bytes, 0);
    }

    // #[test]
    // fn if_import_desc_is_func_should_work() {
    //     let func_imports_bytes = []
    // }
}
