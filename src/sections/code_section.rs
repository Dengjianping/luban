use super::*;
use crate::instructions::*;
use crate::utils::*;

#[derive(Clone, Debug)]
pub struct CodeSection<'a> {
    data: Cow<'a, [u8]>,
    codes: Vec<Code<'a>>,
}

#[derive(Clone, Debug)]
pub struct Code<'a> {
    locals: Locals,
    expr: Vec<Expression<'a>>,
}

#[derive(Clone, Debug)]
pub struct Locals {
    count: u32,
    _type: ValueType,
}

#[derive(Clone, Debug)]
pub struct Expression<'a> {
    expr_bytes: Cow<'a, [u8]>,
}

impl<'a> CodeSection<'a> {
    const ID: u8 = 10;

    pub fn id() -> u8 {
        Self::ID
    }

    pub(crate) fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        assert_eq!(_bytes[0], Self::ID);

        let (len, _bytes) = calc_len_and_offset(&_bytes, 1);
        let data = Cow::Borrowed(&bytes[offset..offset + len + 1 + 1]);
        dbg!(&data);

        let (len, _bytes) = calc_len_and_offset(&_bytes, 0);
        dbg!(len, _bytes);

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_code_section_should_work() {
        let code_bytes = [10, 10, 2, 2, 0, 11, 5, 0, 65, 42, 26, 11];
        let code_section = CodeSection::from_bytes(&code_bytes, 0);
        assert!(true);
    }
}