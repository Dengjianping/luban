/// https://webassembly.github.io/spec/core/syntax/values.html#bytes
pub type Byte = u8;

/// Reference: https://webassembly.github.io/spec/core/syntax/types.html#number-types
/// https://webassembly.github.io/spec/core/binary/types.html#number-types
#[derive(Debug, Clone, Copy)]
pub enum NumberType {
    F64 = 0x7c,
    F32 = 0x7d,
    I64 = 0x7e,
    I32 = 0x7f,
}

impl From<u8> for NumberType {
    fn from(ty: u8) -> Self {
        match ty {
            0x7c => Self::F64,
            0x7d => Self::F32,
            0x7e => Self::I64,
            0x7f => Self::I32,
            _ => panic!("Currently webassembly doesn't suppurt this type"),
        }
    }
}

/// Reference: https://webassembly.github.io/spec/core/syntax/types.html#reference-types
#[derive(Debug, Clone, Copy)]
pub enum ReferenceType {
    FuncRef = 0x70,
    ExternRef = 0x6f,
}

/// https://webassembly.github.io/spec/core/syntax/types.html#value-types
/// https://webassembly.github.io/spec/core/binary/types.html#value-types
#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    NumType(NumberType),
    VectorType,
    RefType(ReferenceType),
}

/// https://webassembly.github.io/spec/core/syntax/types.html#result-types
pub type ResultType = Vec<ValueType>;
pub type ParamsType = ResultType;

/// https://webassembly.github.io/spec/core/syntax/types.html#limits
#[derive(Clone, Debug)]
pub struct Limits {
    pub tag: Tag,
    pub min: Option<u32>,
    pub max: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tag {
    Zero,
    One,
}

#[derive(Debug, Clone)]
pub enum Mutable {
    No = 0x00,
    Yes = 0x01,
}
