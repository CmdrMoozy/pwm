use util::serde::*;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
struct TestStruct {
    a: String,
    b: u64,
    c: bool,
}

#[test]
fn test_binary_round_trip() {
    let original = TestStruct {
        a: "this is a test!".to_owned(),
        b: 42,
        c: true,
    };

    let serialized = serialize_binary(&original).unwrap();
    let deserialized = deserialize_binary(serialized.as_slice()).unwrap();
    assert_eq!(original, deserialized);
}
