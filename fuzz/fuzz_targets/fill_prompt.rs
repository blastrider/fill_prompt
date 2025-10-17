#![no_main]

use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;

fn build_vars(lines: &str) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    for line in lines.lines().take(16) {
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim();
            if key.is_empty() {
                continue;
            }
            vars.insert(key.to_string(), v.trim().to_string());
        }
    }

    // Provide a stable placeholder to exercise successful substitutions even when input lacks key/value pairs.
    vars.entry("x".to_string()).or_insert_with(|| "default".to_string());
    vars
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let (template, kv_section) = s.split_once('\n').unwrap_or((s, ""));
        let vars = build_vars(kv_section);

        // Drive multiple code paths: raw template, trimmed template, and full payload.
        let _ = fill_prompt::fill_template(template, vars.clone());
        let trimmed = template.trim();
        if trimmed != template {
            let _ = fill_prompt::fill_template(trimmed, vars.clone());
        }
        let _ = fill_prompt::fill_template(s, vars);
    }
});
