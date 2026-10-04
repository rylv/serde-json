// Modified by the rylv-serde-json fork: crate rename and related references.
// Original project: https://github.com/serde-rs/json (MIT OR Apache-2.0).

#![no_main]

use libfuzzer_sys::fuzz_target;
use rylv_serde_json::{from_slice, Value};

fuzz_target!(|data: &[u8]| {
    _ = from_slice::<Value>(data);
});
