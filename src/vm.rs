pub type ByteCodes = Vec<u8>;

#[derive(Clone, Debug)]
pub struct WasmVM {
    pub program: ByteCodes,
    operan_stack: OperandStack,
}

#[derive(Clone, Debug)]
pub struct OperandStack {}
