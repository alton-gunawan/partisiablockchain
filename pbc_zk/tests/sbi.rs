use pbc_zk::*;
use proptest::prelude::*;

fn assert_from_to(value: Sbi8, bits: [Sbi1; 8]) {
    assert_eq!(value, Sbi8::from_le_bits(bits));
    assert_eq!(value.to_le_bits(), bits);
}

#[test]
fn clone() {
    let x = Sbi8::from(0x33);
    assert_eq!(x, x.clone());
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

proptest! {
    #[test]
    fn add(lhs: i32, rhs: i32) {
        let expected = lhs.wrapping_add(rhs);
        let gotten = Sbi32::from(lhs) + Sbi32::from(rhs);
        assert_eq!(gotten, Sbi32::from(expected));
    }
}

proptest! {
    #[test]
    fn sub(lhs: i32, rhs: i32) {
        let expected = lhs.wrapping_sub(rhs);
        let gotten = Sbi32::from(lhs) - Sbi32::from(rhs);
        assert_eq!(gotten, Sbi32::from(expected));
    }
}

proptest! {
    #[test]
    fn shl(lhs: i32, rhs: usize) {
        let expected = lhs.wrapping_shl(rhs as u32);
        let gotten = Sbi32::from(lhs) << rhs;
        assert_eq!(gotten, Sbi32::from(expected));
    }
}

proptest! {
    #[test]
    fn shr(lhs: i32, rhs: usize) {
        let expected = lhs.wrapping_shr(rhs as u32);
        let gotten = Sbi32::from(lhs) >> rhs;
        assert_eq!(gotten, Sbi32::from(expected));
    }
}

proptest! {
    #[test]
    fn band(lhs: i32, rhs: i32) {
        let expected = lhs & rhs;
        let gotten = Sbi32::from(lhs) & Sbi32::from(rhs);
        assert_eq!(gotten, Sbi32::from(expected));
    }
}

proptest! {
    #[test]
    fn bor(lhs: i32, rhs: i32) {
        let expected = lhs | rhs;
        let gotten = Sbi32::from(lhs) | Sbi32::from(rhs);
        assert_eq!(gotten, Sbi32::from(expected));
    }
}

proptest! {
    #[test]
    fn bxor(lhs: i32, rhs: i32) {
        let expected = lhs ^ rhs;
        let gotten = Sbi32::from(lhs) ^ Sbi32::from(rhs);
        assert_eq!(gotten, Sbi32::from(expected));
    }
}

proptest! {
    #[test]
    fn eq(lhs: i32, rhs: i32) {
        let expected = lhs == rhs;
        let gotten = Sbi32::from(lhs) == Sbi32::from(rhs);
        assert_eq!(gotten, expected);
    }
}

proptest! {
    #[test]
    fn leq(lhs: i32, rhs: i32) {
        let expected = lhs <= rhs;
        let gotten = Sbi32::from(lhs) <= Sbi32::from(rhs);
        assert_eq!(gotten, expected);
    }
}

proptest! {
    #[test]
    fn geq(lhs: i32, rhs: i32) {
        let expected = lhs >= rhs;
        let gotten = Sbi32::from(lhs) >= Sbi32::from(rhs);
        assert_eq!(gotten, expected);
    }
}

proptest! {
    #[test]
    fn lt(lhs: i32, rhs: i32) {
        let expected = lhs < rhs;
        let gotten = Sbi32::from(lhs) < Sbi32::from(rhs);
        assert_eq!(gotten, expected);
    }
}

proptest! {
    #[test]
    fn gt(lhs: i32, rhs: i32) {
        let expected = lhs > rhs;
        let gotten = Sbi32::from(lhs) > Sbi32::from(rhs);
        assert_eq!(gotten, expected);
    }
}

proptest! {
    #[test]
    fn partial_cmp(lhs: i32, rhs: i32) {
        let expected = lhs.partial_cmp(&rhs);
        let gotten = Sbi32::from(lhs).partial_cmp(&Sbi32::from(rhs));
        assert_eq!(gotten, expected);
    }
}
