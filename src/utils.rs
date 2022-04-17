pub(crate) fn unsigned_leb128_decode(bytes: &[u8]) -> (usize, usize) {
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

pub(crate) fn signed_leb128_decode(bytes: &[u8]) -> (usize, usize) {
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

pub(crate) fn calc_len_and_offset(bytes: &[u8], offset: usize) -> (usize, &[u8]) {
    // If first byte is bigger than 0x80(128, 0x1000_0000)
    if bytes[0] < 0x80 {
        (bytes[offset..][0] as usize, &bytes[1 + offset..])
    } else {
        let (len, leb_offset) = unsigned_leb128_decode(&bytes[offset..]);
        (len, &bytes[offset + leb_offset..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_zero_and_one() {
        let a = [0xfc, 0x06];
        assert_eq!(unsigned_leb128_decode(&a), (892usize, 2));
        assert_eq!(unsigned_leb128_decode(&[0x80, 0x7f]), (16256usize, 2));
        assert_eq!(unsigned_leb128_decode(&[0xe0, 0x98, 0x17]), (380000usize, 3));
        assert_eq!(unsigned_leb128_decode(&[0x0d]), (13usize, 1));
        assert_eq!(unsigned_leb128_decode(&[128, 128, 192, 0]), (1048576usize, 4));
    }

    #[test]
    fn calc_len_and_offset_should_work() {
        let a = [6, 3, 4, 5, 4, 1, 8, 9];
        let (len, _a) = calc_len_and_offset(&a, 0);

        assert_eq!(len, a[0] as usize);
        assert_eq!(_a[..len], a[1..1 + len]);
    }

    #[test]
    fn test_leb1288() {
        use leb128;
        use std::io::*;

        let mut buf = [0; 5];
        let mut writable = &mut buf[..];
        let s = leb128::write::signed(&mut writable, -892);
        dbg!(&buf);
    }
}
