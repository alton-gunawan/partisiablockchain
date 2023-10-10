use pbc_zk::*;
use proptest::prelude::*;

fn assert_from_to(value: Sbi8, bits: [Sbi1; 8]) {
    assert_eq!(value, Sbi8::from_le_bits(bits));
    assert_eq!(value.to_le_bits(), bits);
}

#[test]
fn from_to_i8_specific() {
    assert_from_to(Sbi8::from(0), [false; 8]);
    assert_from_to(
        Sbi8::from(1),
        [true, false, false, false, false, false, false, false],
    );
    assert_from_to(
        Sbi8::from(2),
        [false, true, false, false, false, false, false, false],
    );
    assert_from_to(
        Sbi8::from(9),
        [true, false, false, true, false, false, false, false],
    );
    assert_from_to(
        Sbi8::from(127),
        [true, true, true, true, true, true, true, false],
    );
}

proptest! {
    #[test]
    fn from_to_i8(i: i8) {
        let e = Sbi8::from(i);
        assert_eq!(e, Sbi8::from_le_bits(e.to_le_bits()));
    }

    #[test]
    fn from_to_i16(i: i16) {
        let e = Sbi16::from(i);
        assert_eq!(e, Sbi16::from_le_bits(e.to_le_bits()));
    }

    #[test]
    fn from_to_i32(i: i32) {
        let e = Sbi32::from(i);
        assert_eq!(e, Sbi32::from_le_bits(e.to_le_bits()));
    }

    #[test]
    fn from_to_i64(i: i64) {
        let e = Sbi64::from(i);
        assert_eq!(e, Sbi64::from_le_bits(e.to_le_bits()));
    }

    #[test]
    fn from_to_i128(i: i128) {
        let e = Sbi128::from(i);
        assert_eq!(e, Sbi128::from_le_bits(e.to_le_bits()));
    }
}
