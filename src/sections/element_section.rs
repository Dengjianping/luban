use super::*;
use crate::instructions::*;
use crate::utils::*;

#[derive(Clone, Debug)]
pub struct ElementSection<'a> {
    data: Cow::<'a, [u8]>,
    elem: Vec<Element>,
}

#[derive(Clone, Debug)]
pub struct Element {
    table_idx: usize,
    offset_expr: Expression,
}