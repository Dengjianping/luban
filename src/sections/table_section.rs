use super::*;
use crate::utils::*;

impl Limits {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        match bytes[0] {
            0 => {
                let tag = Tag::Zero;
                let min = Some(bytes[1] as u32);
                let max = None;
                Self { tag, min, max }
            }
            1 => {
                let tag = Tag::One;
                let min = Some(bytes[1] as u32);
                let max = Some(bytes[2] as u32);
                Self { tag, min, max }
            }
            _ => panic!("unsupported tag"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReferenceType {
    FuncRef = 0x70,
    ExternRef = 0x6f,
}

impl From<u8> for ReferenceType {
    fn from(u: u8) -> Self {
        match u {
            0x70 => Self::FuncRef,
            0x6f => Self::ExternRef,
            _ => panic!("Uknown reference type."),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableType {
    ref_type: ReferenceType,
    limits: Limits,
}

#[derive(Clone, Debug)]
pub struct TableSection<'a> {
    data: Cow<'a, [u8]>,
    table_types: [TableType; 1],
}

impl<'a> TableSection<'a> {
    const ID: u8 = 4;

    pub fn id() -> u8 {
        Self::ID
    }

    pub fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        assert_eq!(_bytes[0], Self::ID);

        // 1 means skip the id.
        let (leb_len, _bytes) = calc_len_and_offset(_bytes, 1);

        let data = Cow::Borrowed(&_bytes[..leb_len]);

        let (leb_len, _bytes) = calc_len_and_offset(&data, 0);
        // currently webassembly only supports 1 table.
        assert_eq!(leb_len, 1);

        let ref_type = ReferenceType::from(_bytes[0]);
        let limits = Limits::from_bytes(&_bytes[1..]);
        let table_type = TableType { ref_type, limits };

        Self {
            data,
            table_types: [table_type],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_limits_should_work() {
        let limits_bytes_with_max = [1, 13, 13];
        let limits = Limits::from_bytes(&limits_bytes_with_max);
        let tag = Tag::One;
        assert_eq!(limits.tag, tag);
        assert_eq!(limits.min, Some(limits_bytes_with_max[1] as u32));
        assert_eq!(limits.max, Some(limits_bytes_with_max[2] as u32));
    }

    #[test]
    fn deserialize_table_section_should_work() {
        let table_bytes = [4, 5, 1, 112, 1, 13, 13];
        let table_sec = TableSection::from_bytes(&table_bytes, 0);

        assert_eq!(table_sec.table_types.len(), 1);
        assert_eq!(table_sec.table_types[0].ref_type, ReferenceType::FuncRef);
    }
}
