use pbc_contract_common::shortname::{Shortname, ShortnameCallback, ShortnameZkComputation};

#[test]
pub fn clone_shortname() {
    let shortname: Shortname = Shortname::from_u32(3);
    assert_eq!(shortname.clone(), shortname);
}

#[test]
pub fn clone_shortnamecallback() {
    let shortname: ShortnameCallback = ShortnameCallback::from_u32(3);
    assert_eq!(shortname.clone(), shortname);
}

#[test]
pub fn clone_shortnamezkcomputation() {
    let shortname: ShortnameZkComputation = ShortnameZkComputation::from_u32(3);
    assert_eq!(shortname.clone(), shortname);
}

#[test]
pub fn debug_shortnamecallback() {
    let shortname: ShortnameCallback = ShortnameCallback::new(Shortname::from_u32(3));
    assert_eq!(
        format!("{:?}", shortname),
        "ShortnameCallback { shortname: Shortname { value: 3 } }"
    );
}

#[test]
pub fn debug_shortnamezkcomputation() {
    let shortname: ShortnameZkComputation = ShortnameZkComputation::new(Shortname::from_u32(3));
    assert_eq!(
        format!("{:?}", shortname),
        "ShortnameZkComputation { shortname: Shortname { value: 3 } }"
    );
}

#[test]
pub fn empty_shortname() {
    let shortname = Shortname::from_be_bytes(&[]);
    assert_eq!(shortname, Err("Shortname must not be empty".to_string()));
}

#[test]
pub fn continuation_on_last_byte() {
    let shortname = Shortname::from_be_bytes(&[0x80, 0x81]);
    assert_eq!(
        shortname,
        Err("Shortname's last byte must not have continuation bit set".to_string())
    );
}

#[test]
pub fn normalized_shortname() {
    let shortname = Shortname::from_be_bytes(&[0x70, 0, 0, 0]);
    assert_eq!(
        shortname,
        Err("Shortname must be normalized, with no trailing zeroes".to_string())
    );
}

#[test]
pub fn no_continuation_on_non_last_bytes() {
    let shortname = Shortname::from_be_bytes(&[0x70, 0x01]);
    assert_eq!(
        shortname,
        Err("Shortname's non-last bytes must have their continuation bits set".to_string())
    );
}

#[test]
pub fn too_large() {
    let shortname = Shortname::from_be_bytes(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]);
    assert_eq!(
        shortname,
        Err("Shortname value too large for u32".to_string())
    );
}
