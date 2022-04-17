use super::*;
use crate::instructions::*;
use crate::utils::*;

// only can export 4 types: function, table, memory, global

#[derive(Clone, Debug)]
pub struct ExportSection<'a> {
    data: Cow<'a, [u8]>,
    exports: Vec<Export>,
}

#[derive(Clone, Debug)]
pub struct Export {
    name: String,
    desc: ExportDesc,
}

#[derive(Clone, Debug)]
pub enum ExportType {
    Func = 0x00,
    Table = 0x01,
    Mem = 0x02,
    Global = 0x03,
}

impl From<u8> for ExportType {
    fn from(u: u8) -> Self {
        match u {
            0x00 => Self::Func,
            0x01 => Self::Table,
            0x02 => Self::Mem,
            0x03 => Self::Global,
            _ => panic!("Unknow tag."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExportDesc {
    index: usize,
    export_type: ExportType,
}

impl<'a> ExportSection<'a> {
    const ID: u8 = 7;

    pub fn id() -> u8 {
        Self::ID
    }

    pub fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        assert_eq!(_bytes[0], Self::ID);

        let (leb_len, _bytes) = calc_len_and_offset(_bytes, 1);
        // first 1 for id
        let data = Cow::Borrowed(&bytes[offset..offset + leb_len + 1 + 1]);

        // skip section id and content length
        let (export_count, mut _bytes) = calc_len_and_offset(&data, 2);

        let mut exports = Vec::with_capacity(export_count);

        loop {
            if _bytes.len() <= 0 {
                break;
            }

            let (leb_len, rest_bytes) = calc_len_and_offset(_bytes, 0);
            // let name = std::str::from_utf8(&rest_bytes[..leb_len]).unwrap();
            let name = unsafe { String::from_utf8_unchecked(rest_bytes[..leb_len].to_vec()) };
            let export_type = ExportType::from(rest_bytes[leb_len]);
            let index = rest_bytes[leb_len + 1] as usize;
            let desc = ExportDesc { index, export_type };
            let export = Export { name, desc };
            exports.push(export);
            // first 1 means tag, second 1 means index
            _bytes = &rest_bytes[leb_len + 1 + 1..];
        }

        Self { data, exports }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_export_section_should_work() {
        let exports_bytes = [7, 9, 1, 5, 102, 117, 110, 99, 49, 0, 1];
        let export_section = ExportSection::from_bytes(&exports_bytes, 0);
        dbg!(export_section);
    }
}
