// Modified by the rylv-serde-json fork: crate rename and related references.
// Original project: https://github.com/serde-rs/json (MIT OR Apache-2.0).

use rylv_serde_json::Value;

#[test]
fn test() {
    let x1 = rylv_serde_json::from_str::<Value>("18446744073709551615.");
    assert!(x1.is_err());
    let x2 = rylv_serde_json::from_str::<Value>("18446744073709551616.");
    assert!(x2.is_err());
}
