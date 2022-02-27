use super::*;

/// Reference:
#[derive(Clone, Debug)]
pub struct MagicSection<'a> {
    pub(crate) magic_number: Cow<'a, [u8]>,
    pub(crate) version_number: Cow<'a, [u8]>,
}

impl<'a> MagicSection<'a> {
    const LENGTH: usize = 8;

    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            magic_number: Cow::Borrowed(&bytes[..4]),
            version_number: Cow::Borrowed(&bytes[4..8]),
        }
    }

    pub fn magic_number(&self) -> &str {
        core::str::from_utf8(&self.magic_number[..4]).unwrap()
    }

    pub fn version(&self) -> u32 {
        let mut le = [0u8; 4];
        le.copy_from_slice(&*self.version_number);
        u32::from_le_bytes(le)
    }

    pub fn length(&self) -> usize {
        assert_eq!(
            Self::LENGTH,
            self.magic_number.len() + self.version_number.len()
        );
        Self::LENGTH
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_section_should_work() {
        let magic_bytes = vec![0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let magic_sec = MagicSection::from_bytes(&magic_bytes);

        // check magic number
        assert_eq!(magic_sec.magic_number(), "\u{0}asm");

        // check version number
        assert_eq!(magic_sec.version(), 1);

        // check length of magic section
        assert_eq!(magic_sec.length(), 8);
    }
}
