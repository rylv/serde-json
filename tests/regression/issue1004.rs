// Modified by the rylv-serde-json fork: crate rename and related references.
// Original project: https://github.com/serde-rs/json (MIT OR Apache-2.0).

#![cfg(feature = "arbitrary_precision")]

#[test]
fn test() {
    let float = 5.55f32;
    let value = rylv_serde_json::to_value(float).unwrap();
    let json = rylv_serde_json::to_string(&value).unwrap();

    // If the f32 were cast to f64 by Value before serialization, then this
    // would incorrectly serialize as 5.550000190734863.
    assert_eq!(json, "5.55");
}
