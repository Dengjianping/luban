use super::*;
use crate::instructions::*;
use crate::utils::*;

#[derive(Clone, Debug)]
pub struct DataSection<'a> {
    data: Cow<'a, [u8]>,
    mem_idx: usize,
    offset: Expressions,
    init: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_data_section_should_work() {
        let data_bytes = [11, 8, 1, 0, 65, 0, 11, 2, 104, 105];
    }
}
