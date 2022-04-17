use super::*;
use crate::utils::unsigned_leb128_decode;

/// Reference: https://webassembly.github.io/spec/core/binary/modules.html#type-section
/// Currently, All types are function.
#[derive(Clone, Debug)]
pub struct TypeSection<'a> {
    data: Cow<'a, [u8]>,
    pub func_types: Vec<FunctionType>,
}

impl<'a> TypeSection<'a> {
    const ID: u8 = 0x01;

    /// Return type segmemnt id
    pub fn id(&self) -> u8 {
        self.data[0]
    }

    pub(crate) fn byte_count(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        // ensure the id is valid.
        assert_eq!(_bytes[0], Self::ID);

        // get the length of content for type section
        let (content_length, leb_offset) = unsigned_leb128_decode(&_bytes[1..]);

        let len = 1 + leb_offset + content_length;
        let data = Cow::Borrowed(&bytes[offset..offset + len]);

        let func_types = {
            // [id, leb_offset, func_count, ...]
            let func_bytes = &data[2 + leb_offset..];
            let func_types_count = func_bytes[0] as usize;

            let mut func_types: Vec<FunctionType> = Vec::with_capacity(func_types_count);
            for _func in func_bytes.split(|byte| *byte as u32 == FunctionType::ID) {
                if _func == [] {
                    continue;
                }
                // Todo, imrpove this line. I don't like concat.
                let func = [&[0x60], _func].concat();
                let func_type = FunctionType::from_bytes(&func);
                func_types.push(func_type)
            }
            func_types
        };

        Self { data, func_types }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_type_scetion_should_work() {
        // three functions in the type section
        let type_sec_bytes = [1, 13, 3, 96, 1, 125, 0, 96, 1, 127, 1, 127, 96, 0, 0];
        let type_sec = TypeSection::from_bytes(&type_sec_bytes, 0);

        assert_eq!(type_sec.id(), type_sec_bytes[0]);
        assert_eq!(type_sec.byte_count(), type_sec_bytes.len());
        assert_eq!(type_sec.func_types.len(), 3);
    }
}
