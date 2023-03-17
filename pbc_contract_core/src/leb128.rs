/// Encode unsigned 32-bit ints as LEB128.
pub(crate) fn to_leb128_bytes(mut value: u32) -> Vec<u8> {
    if value == 0 {
        return vec![0];
    }

    let mut result = Vec::new();
    while value != 0 {
        let lower_seven = value & 0x7f;
        value >>= 7;

        let high_bit = if value != 0 { 0x80 } else { 0 };
        result.push((lower_seven | high_bit) as u8);
    }
    result
}

#[cfg(test)]
mod test {
    use super::to_leb128_bytes;

    #[test]
    fn leb() {
        assert_eq!(to_leb128_bytes(0), vec![0]);
        assert_eq!(to_leb128_bytes(1), vec![1]);
        assert_eq!(to_leb128_bytes(65), vec![65]);
        assert_eq!(to_leb128_bytes(127), vec![127]);
        assert_eq!(to_leb128_bytes(128), vec![128, 1]);
        assert_eq!(to_leb128_bytes(192), vec![192, 1]);
        assert_eq!(to_leb128_bytes(255), vec![255, 1]);
        assert_eq!(to_leb128_bytes(256), vec![128, 2]);
        assert_eq!(to_leb128_bytes(624485), vec![0xE5, 0x8E, 0x26]);
        assert_eq!(
            to_leb128_bytes(0xFFFFFFFF),
            vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F]
        );
    }
}
