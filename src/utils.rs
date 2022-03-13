pub fn uleb128_decode(bytes: &[u8]) -> (usize, usize) {
    let mut result = 0usize;
    let mut shift = 0usize;
    let mut offset = 0;
    for i in bytes {
        offset += 1;
        let lower_7_bits = *i as usize & 0b0111_1111usize;
        let highest_bit = (i >> 7) & 1;
        result |= lower_7_bits << shift;
        if highest_bit == 0 {
            break;
        }
        shift += 7;
    }
    (result, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_zero_and_one() {
        let a = [0xfc, 0x06];
        assert_eq!(uleb128_decode(&a), (892usize, 2));
        assert_eq!(uleb128_decode(&[0x80, 0x7f]), (16256usize, 2));
        assert_eq!(uleb128_decode(&[0xe0, 0x98, 0x17]), (380000usize, 3));
        assert_eq!(uleb128_decode(&[0x0d]), (13, 1));
    }
}
