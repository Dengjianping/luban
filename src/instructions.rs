use crate::sections::Parse;

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#control-instructions
#[derive(Clone, Copy, Debug)]
pub enum ControlInstructions {
    Unreachable = 0x00,  // unreachable
    Nop = 0x01,          // nop
    Block = 0x02,        // block, ends with 0x0b
    Loop = 0x03,         // loop, ends with 0x0b
    If = 0x04,           // if, ends with 0x0b
    Else = 0x05,         // else
    Br = 0x0c,           // br
    BrIf = 0x0d,         // br_if
    BrTable = 0x0e,      // br_table
    Return = 0x0f,       // return
    Call = 0x10,         // call
    CallIndirect = 0x11, // call_indirect
    BlockType = 0x40,    // blocktype
}

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#reference-instructions
#[derive(Clone, Copy, Debug)]
pub enum ReferenceInstructions {
    RefNull = 0xd0,   // ref.null
    RefIsNull = 0xd1, // ref.is_null
    RefFunc = 0xd2,   // ref.func
}

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#parametric-instructions
#[derive(Clone, Copy, Debug)]
pub enum ParametricInstructions {
    Drop = 0x1a,    // drop
    Select = 0x1b,  // select
    SelectV = 0x1c, // select t*
}

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#variable-instructions
#[derive(Clone, Copy, Debug)]
pub enum VariableInstructions {
    LocalGet = 0x20,  // local.get
    LocalSet = 0x21,  // local.set
    LocalTee = 0x22,  // local.tee
    GlobalGet = 0x23, // global.get
    GlobalSet = 0x24, // global.set
}

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#table-instructions
#[derive(Clone, Copy, Debug)]
pub enum TableInstructions {
    TableGet = 0x25, // table.get
    TableSet = 0x26, // table.set
                     // Todo, figure out how these instructions work
                     // TableInit = 0xfc, // table.init
                     // ElemDrop  = 0xfc, // elem.drop
                     // TableCopy = 0xfc, // table.copy
                     // TableGrow = 0xfc, // table.grow
                     // TableSize = 0xfc, // table.size
                     // TableFill = 0xfc, // table.fill
}

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#memory-instructions
#[derive(Clone, Copy, Debug)]
pub enum MemoryInstructions {
    // Load Instructions
    I32Load = 0x28,           // i32.load
    I64Load = 0x29,           // i64.load
    F32Load = 0x2a,           // f32.load
    F64Load = 0x2b,           // f64.load
    I32Load8Signed = 0x2c,    // i32.load8_s
    I32Load8Unsigned = 0x2d,  // i32.load8_u
    I32Load16Signed = 0x2e,   // i32.load16_s
    I32Load16Unsigned = 0x2f, // i32.load16_u
    I64Load8Signed = 0x30,    // i64.load8_s
    I64Load8Unsigned = 0x31,  // i64.load8_u
    I64Load16Signed = 0x32,   // i64.load16_s
    I64Load16Unsigned = 0x33, // i64.load16_u
    I64Load32Signed = 0x34,   // i64.load32_s
    I64Load32Unsigned = 0x35, // i64.load32_u
    // Store Instructions
    I32Store = 0x36,   // i32.store
    I64Store = 0x37,   // i64.store
    F32Store = 0x38,   // f32.store
    F64Store = 0x39,   // f64.store
    I32Store8 = 0x3a,  // i32.store8
    I32Store16 = 0x3b, // i32.store16
    I64Store8 = 0x3c,  // i64.store8
    I64Store16 = 0x3d, // i64.store16
    I64Store32 = 0x3e, // i64.store32
    // Memory Operations
    Size = 0x3f, // memory.size
    Grow = 0x40, // memory.grow
                 // Todo, figure out how these instructions work
                 // Init           = 0xfc, // memory.init
                 // Drop           = 0xfc, // data.drop
                 // Copy           = 0xfc, // memory.copy
                 // Fill           = 0xfc, // memory.fill
}

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#numeric-instructions
#[derive(Clone, Copy, Debug)]
pub enum NumericInstructions {
    // Constant Instructions
    I32Const = 0x41, // i32.const
    I64Const = 0x42, // i64.const
    F32Const = 0x43, // f32.const
    F64Const = 0x44, // f64.const
    // i32
    I32Eqz = 0x45,        // i32.eqz
    I32Eq = 0x46,         // i32.eq
    I32Ne = 0x47,         // i32.ne
    I32LtSigned = 0x48,   // i32.lt_s
    I32LtUnsigned = 0x49, // i32.lt_u
    I32GtSigned = 0x4a,   // i32.gt_s
    I32GtUnsigned = 0x4b, // i32.gt_u
    I32LeSigned = 0x4c,   // i32.le_s
    I32LeUnsigned = 0x4d, // i32.le_u
    I32GeSigned = 0x4e,   // i32.ge_s
    I32GeUnsigned = 0x4f, // i32.ge_u
    // i64
    I64Eqz = 0x50,        // i64.eqz
    I64Eq = 0x51,         // i64.eq
    I64Ne = 0x52,         // i64.ne
    I64LtSigned = 0x53,   // i64.lt_s
    I64LtUnsigned = 0x54, // i64.lt_u
    I64GtSigned = 0x55,   // i64.gt_s
    I64GtUnsigned = 0x56, // i64.gt_u
    I64LeSigned = 0x57,   // i64.le_s
    I64LeUnsigned = 0x58, // i64.le_u
    I64GeSigned = 0x59,   // i64.ge_s
    I64GeUnsigned = 0x5a, // i64.ge_u
    // f32
    F32Eq = 0x5b, // f32.eq
    F32Ne = 0x5c, // f32.ne
    F32Lt = 0x5d, // f32.lt
    F32Gt = 0x5e, // f32.gt
    F32Le = 0x5f, // f32.le
    F32Ge = 0x60, // f32.ge
    // f64
    F64Eq = 0x61, // f64.eq
    F64Ne = 0x62, // f64.ne
    F64Lt = 0x63, // f64.lt
    F64Gt = 0x64, // f64.gt
    F64Le = 0x65, // f64.le
    F64Ge = 0x66, // f64.ge
    // i32
    I32Clz = 0x67,         // i32.clz
    I32Ctz = 0x68,         // i32.ctz
    I32PopCnt = 0x69,      // i32.popcnt
    I32Add = 0x6a,         // i32.add
    I32Sub = 0x6b,         // i32.sub
    I32Mul = 0x6c,         // i32.mul
    I32DivSigned = 0x6d,   // i32.div_s
    I32DivUnsigned = 0x6e, // i32.div_u
    I32RemSigned = 0x6f,   // i32.rem_s
    I32RemUnsigned = 0x70, // i32.rem_u
    I32And = 0x71,         // i32.and
    I32Or = 0x72,          // i32.or
    I32Xor = 0x73,         // i32.xor
    I32Shl = 0x74,         // i32.shl
    I32ShrSigned = 0x75,   // i32.shr_s
    I32ShrUnsigned = 0x76, // i32.shr_u
    I32Rotl = 0x77,        // i32.rotl
    I32Rotr = 0x78,        // i32.rotr
    // i64
    I64Clz = 0x79,         // i64.clz
    I64Ctz = 0x7a,         // i64.ctz
    I64PopCnt = 0x7b,      // i64.popcnt
    I64Add = 0x7c,         // i64.add
    I64Sub = 0x7d,         // i64.sub
    I64Mul = 0x7e,         // i64.mul
    I64DivSigned = 0x7f,   // i64.div_s
    I64DivUnsigned = 0x80, // i64.div_u
    I64RemSigned = 0x81,   // i64.rem_s
    I64RemUnsigned = 0x82, // i64.rem_u
    I64And = 0x83,         // i64.and
    I64Or = 0x84,          // i64.or
    I64Xor = 0x85,         // i64.xor
    I64Shl = 0x86,         // i64.shl
    I64ShrSigned = 0x87,   // i64.shr_s
    I64ShrUnsigned = 0x88, // i64.shr_u
    I64Rotl = 0x89,        // i64.rotl
    I64Rotr = 0x8a,        // i64.rotr
    // f32
    F32Abs = 0x8b,      // f32.abs
    F32Neg = 0x8c,      // f32.neg
    F32Ceil = 0x8d,     // f32.ceil
    F32Floor = 0x8e,    // f32.floor
    F32Trunc = 0x8f,    // f32.trunc
    F32Nearest = 0x90,  // f32.nearest
    F32Sqrt = 0x91,     // f32.sqrt
    F32Add = 0x92,      // f32.add
    F32Sub = 0x93,      // f32.sub
    F32Mul = 0x94,      // f32.mul
    F32Div = 0x95,      // f32.div
    F32Min = 0x96,      // f32.min
    F32Max = 0x97,      // f32.max
    F32CopySign = 0x98, // f32.copysign
    //
    F64Abs = 0x99,      // f64.abs
    F64Neg = 0x9a,      // f64.neg
    F64Ceil = 0x9b,     // f64.ceil
    F64Floor = 0x9c,    // f64.floor
    F64Trunc = 0x9d,    // f64.trunc
    F64Nearest = 0x9e,  // f64.nearest
    F64Sqrt = 0x9f,     // f64.sqrt
    F64Add = 0xa0,      // f64.add
    F64Sub = 0xa1,      // f64.sub
    F64Mul = 0xa2,      // f64.mul
    F64Div = 0xa3,      // f64.div
    F64Min = 0xa4,      // f64.min
    F64Mx = 0xa5,       // f64.max
    F64CopySign = 0xa6, // f64.copysign
    //
    I32WrapI64 = 0xa7,            // i32.wrap_i64
    I32TruncF32Signed = 0xa8,     // i32.trunc_f32_s
    I32TruncF32Unsigned = 0xa9,   // i32.trunc_f32_u
    I32TruncF64Signed = 0xaa,     // i32.trunc_f64_s
    I32TruncF64Unsigned = 0xab,   // i32.trunc_f64_u
    I64ExtendI32Signed = 0xac,    // i64.extend_i32_s
    I64ExtendI32Unsigned = 0xad,  // i64.extend_i32_u
    I64TruncF32Signed = 0xae,     // i64.trunc_f32_u
    I64TruncF32Unsigned = 0xaf,   // i64.trunc_f32_u
    I64TruncF64Signed = 0xb0,     // i64.trunc_f64_s
    I64TruncF64Unsigned = 0xb1,   // i64.trunc_f64_u
    F32ConvertI32Signed = 0xb2,   // f32.convert_i32_s
    F32ConvertI32Unsigned = 0xb3, // f32.convert_i32_u
    F32ConvertI64Signed = 0xb4,   // f32.convertI64_s
    F32ConvertI64Unsigned = 0xb5, // f32.convertI64_u
    F32DemoteF64 = 0xb6,          // f32.demote_f64
    F64ConvertI32Signed = 0xb7,   // f64.convert_i32_s
    F64ConvertI32Unsigned = 0xb8, // f64.convert_i32_u
    F64ConvertI64Signed = 0xb9,   // f64.convert_i64_s
    F64ConvertI64Unsigned = 0xba, // f64.convert_i64_u
    F64PromoteF32 = 0xbb,         // f64.promote_f32
    I32ReinterpretF32 = 0xbc,     // i32.reinterpret_f32
    I64ReinterpretF64 = 0xbd,     // i64.reinterpret_f64
    F32ReinterpretI32 = 0xbe,     // f32.reinterpret_i32
    F64ReinterpretI64 = 0xbf,     // f64.reinterpret_i64
    //
    I32Extend8Signed = 0xc0,  // i32.extend8_s
    I32Extend16Signed = 0xc1, // i32.extend16_s
    I64Extend8Signed = 0xc2,  // i64.extend8_s
    I64Extend16Signed = 0xc3, // i32.extend16_s
    I64Extend32Signed = 0xc4, // i32.extend32_s
}

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#vector-instructions
#[derive(Clone, Copy, Debug)]
pub enum VectorInstructions {
    // Todo, this is for simd, define them in the future.
}

/// Reference: https://webassembly.github.io/spec/core/binary/instructions.html#expressions
#[derive(Clone, Debug)]
pub struct Expressions {
    pub end: u8, // the end must be 0x0b
    pub instructions: Vec<Instructions>,
}

/// https://webassembly.github.io/spec/core/binary/instructions.html#expressions
#[derive(Clone, Debug)]
pub enum Instructions {
    ControlInstructions(ControlInstructions),
    ReferenceInstructions(ReferenceInstructions),
    ParametricInstructions(ParametricInstructions),
    VariableInstructions(VariableInstructions),
    TableInstructions(TableInstructions),
    MemoryInstructions(MemoryInstructions),
    NumericInstructions(NumericInstructions, Primitives),
    VectorInstructions(VectorInstructions),
}

#[derive(Clone, Debug)]
pub enum Primitives {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl NumericInstructions {
    pub fn is_i32(&self) -> bool {
        match self {
            NumericInstructions::I32Const => true,
            _ => false,
        }
    }

    pub fn is_f32(&self) -> bool {
        match self {
            NumericInstructions::F32Const => true,
            _ => false,
        }
    }

    pub fn is_i64(&self) -> bool {
        match self {
            NumericInstructions::I64Const => true,
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match self {
            NumericInstructions::F64Const => true,
            _ => false,
        }
    }
}
