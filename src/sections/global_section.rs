use super::*;
use crate::instructions::*;
use crate::utils::*;

/// https://webassembly.github.io/spec/core/binary/types.html#global-types
#[derive(Debug, Clone)]
pub struct GlobalType {
    pub val_type: ValueType,
    pub mutable: Mutable,
}

/// https://webassembly.github.io/spec/core/binary/modules.html#global-section
#[derive(Clone, Debug)]
pub struct GlobalSection<'a> {
    data: Cow<'a, [u8]>,
    globals: Vec<Global>,
}

/// https://webassembly.github.io/spec/core/binary/modules.html#binary-global
#[derive(Clone, Debug)]
pub struct Global {
    global_type: GlobalType,
    expressions: Expressions,
}

impl<'a> Parse<'a> for GlobalType {
    type Output = GlobalType;
    fn from_bytes(bytes: &'a [u8], offset: usize) -> Self::Output {
        let val_type = ValueType::NumType(NumberType::from(bytes[0]));
        let mutable = match bytes[1] {
            0 => Mutable::No,
            1 => Mutable::Yes,
            _ => {
                panic!("the variable either mutable or imutable, cannot be another mutable status.")
            }
        };

        Self { val_type, mutable }
    }
}

impl<'a> GlobalSection<'a> {
    const ID: u8 = 0x06;

    pub fn id() -> u8 {
        Self::ID
    }

    pub(crate) fn from_bytes(bytes: &'a [u8], offset: usize) -> Self {
        let _bytes = &bytes[offset..];
        assert_eq!(_bytes[0], Self::ID);

        let (leb_len, _bytes) = calc_len_and_offset(_bytes, 1);
        // first 1 for id, second 1 for content length
        let data = Cow::Borrowed(&bytes[offset..offset + leb_len + 1 + 1]);

        // how many global items
        let (globals_count, _bytes) = calc_len_and_offset(&data, 2);

        let mut globals = Vec::with_capacity(globals_count);
        for expr in _bytes.split_inclusive(|b| *b == 0x0b) {
            let global_type = GlobalType::from_bytes(&expr[..2], 0);
            let len = expr.len();

            let expressions = match expr[2] {
                0x41 => {
                    let op = NumericInstructions::I32Const;
                    // skip value type, tag, operand, they occupy 3 bytes.
                    let (result, offset) = unsigned_leb128_decode(&expr[3..]);
                    assert_eq!(expr[offset + 3..][0], 0x0b);
                    assert_eq!(expr[offset + 3..].len(), 1);
                    let instructions = vec![Instructions::NumericInstructions(
                        op,
                        Primitives::I32(result as i32),
                    )];
                    let expressions = Expressions {
                        end: _bytes[0],
                        instructions,
                    };
                    expressions
                }
                0x42 => {
                    let op = NumericInstructions::I64Const;
                    let (result, offset) = unsigned_leb128_decode(&expr[3..]);
                    assert_eq!(expr[offset + 3..][0], 0x0b);
                    assert_eq!(expr[offset + 3..].len(), 1);
                    let instructions = vec![Instructions::NumericInstructions(
                        op,
                        Primitives::I64(result as i64),
                    )];
                    let expressions = Expressions {
                        end: _bytes[0],
                        instructions,
                    };
                    expressions
                }
                0x43 => {
                    let op = NumericInstructions::F32Const;
                    let (result, offset) = unsigned_leb128_decode(&expr[3..]);
                    assert_eq!(expr[offset + 3..][0], 0x0b);
                    assert_eq!(expr[offset + 3..].len(), 1);
                    let instructions = vec![Instructions::NumericInstructions(
                        op,
                        Primitives::F32(result as f32),
                    )];
                    let expressions = Expressions {
                        end: _bytes[0],
                        instructions,
                    };
                    expressions
                }
                0x44 => {
                    let op = NumericInstructions::F64Const;
                    let (result, offset) = unsigned_leb128_decode(&expr[3..]);
                    assert_eq!(expr[offset + 3..][0], 0x0b);
                    assert_eq!(expr[offset + 3..].len(), 1);
                    let instructions = vec![Instructions::NumericInstructions(
                        op,
                        Primitives::F64(result as f64),
                    )];
                    let expressions = Expressions {
                        end: _bytes[0],
                        instructions,
                    };
                    expressions
                }
                _ => panic!("unsupport instruction"),
            };
            globals.push(Global {
                global_type,
                expressions,
            });
        }

        Self { data, globals }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_global_section_should_work() {
        let global_bytes = [
            6, 25, 3, 127, 1, 65, 128, 128, 192, 0, 11, 127, 0, 65, 160, 132, 192, 0, 11, 127, 0,
            65, 160, 132, 192, 0, 11,
        ];

        let global_section = GlobalSection::from_bytes(&global_bytes, 0);
    }
}
