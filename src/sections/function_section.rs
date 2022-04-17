use super::*;
use crate::instructions::*;
use crate::utils::*;

#[derive(Clone, Debug)]
pub struct FunctionSection<'a> {
    data: Cow<'a, [u8]>,
    type_idx: Vec<usize>,
}
