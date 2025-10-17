#![no_main]
use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;

fuzz_target!(|data: &[u8]| {
    // try to interpret input as utf8, otherwise skip
    if let Ok(s) = std::str::from_utf8(data) {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), "v".to_string());
        // best-effort: ignore result
        let _ = fill_prompt::fill_template(s, vars);
    }
});