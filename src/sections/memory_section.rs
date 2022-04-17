use super::*;
use crate::utils::*;

#[derive(Clone, Debug)]
pub struct MemorySection<'a> {
    data: Cow<'a, [u8]>,
    limits: Limits,
}

impl<'a> MemorySection<'a> {
    const ID: u8 = 5;

    pub fn id() -> u8 {
        Self::ID
    }

    pub fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        assert_eq!(_bytes[0], Self::ID);

        // 1 means skip the id.
        let (leb_len, _bytes) = calc_len_and_offset(&_bytes, 1);
        let data = Cow::Borrowed(&_bytes[..leb_len]);

        let limits = Limits::from_bytes(&data);
        Self { data, limits }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_memory_section_should_work() {
        let memory_bytes = [5, 3, 1, 0, 17];
        let memory_sec = MemorySection::from_bytes(&memory_bytes, 0);
        assert_eq!(memory_sec.limits.tag, Tag::One);
        assert_eq!(memory_sec.limits.min, Some(memory_bytes[3] as u32));
        assert_eq!(memory_sec.limits.max, Some(memory_bytes[4] as u32));
    }
}
