use ::crypto::padding::*;
use sodiumoxide::randombytes::randombytes;
use util::data::SensitiveData;

#[test]
fn test_padding_round_trip() {
    let mut data = SensitiveData::from(randombytes(123));
    let original_data = data.clone();
    data = pad(data);
    assert!(data.len() > original_data.len());
    data = unpad(data).unwrap();
    assert_eq!(original_data, data);
}

#[test]
fn test_unpadding_invalid_size() {
    assert!(unpad(SensitiveData::from(vec![])).is_err());
}
