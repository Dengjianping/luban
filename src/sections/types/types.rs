pub type Tag = u8;

/// https://webassembly.github.io/spec/core/syntax/values.html#bytes
pub type Byte = u8;

/// Reference: https://webassembly.github.io/spec/core/syntax/types.html#number-types
/// https://webassembly.github.io/spec/core/binary/types.html#number-types
#[derive(Debug, Clone, Copy)]
pub enum NumberType {
    F64 = 0x7c,
    F32 = 0x7d,
    U64 = 0x7e,
    I32 = 0x7f,
}

impl From<u8> for NumberType {
    fn from(ty: u8) -> Self {
        match ty {
            0x7c => Self::F64,
            0x7d => Self::F32,
            0x7e => Self::U64,
            0x7f => Self::I32,
            _ => panic!("Currently webassembly doesn't suppurt this type"),
        }
    }
}

/// Reference: https://webassembly.github.io/spec/core/syntax/types.html#vector-types
/// https://webassembly.github.io/spec/core/binary/types.html#vector-types
pub const VECTOR_TYPE: Byte = 0x7b; // u128, simd related

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
#[derive(Debug, Clone)]
pub struct Limits {
    pub tag: Tag,
    pub min: u32,
    pub max: u32,
}

pub type MemoryType = Limits;
pub type ElementType = u8;
const FUNCTION_REF: u8 = 0x70;

#[derive(Debug, Clone)]
pub struct TableType {
    pub elem_type: ElementType,
    pub limits: Limits,
}

#[derive(Debug, Clone)]
pub enum Mutable {
    No = 0x00,
    Yes = 0x01,
}

#[derive(Debug, Clone)]
pub struct GlobalType {
    pub val_type: ValueType,
    pub mutable: Mutable,
}
