//! fill_prompt — small, safe library to fill variableized prompts.
//!
//! # Examples
//!
//! ```
//! use std::collections::HashMap;
//! let tpl = "Create a Rust program named {{name}} that prints {{message}}.";
//! let mut vars = HashMap::new();
//! vars.insert("name".to_string(), "hello_world".to_string());
//! vars.insert("message".to_string(), "Hello, Rust!".to_string());
//! let filled = fill_prompt::fill_template(tpl, vars).unwrap();
//! assert!(filled.contains("hello_world"));
//! assert!(filled.contains("Hello, Rust!"));
//! ```
use std::collections::HashMap;
use thiserror::Error;

/// expose the validation helpers implemented in src/validate.rs
pub mod validate;

/// expose variable loader (JSON/YAML/TOML) — feature-gated on "serde"
pub mod vars;

/// Errors returned when filling templates.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum FillError {
    /// Placeholder opened but never closed.
    #[error("unclosed placeholder starting at byte index {0}")]
    UnclosedPlaceholder(usize),

    /// Placeholder name is empty.
    #[error("empty placeholder at byte index {0}")]
    EmptyPlaceholder(usize),

    /// Variable not provided.
    #[error("missing variable `{0}`")]
    MissingVariable(String),
}

/// Fill `template` by replacing occurrences of `{{key}}` with `vars[key]`.
///
/// - Placeholders are delimited by `{{` and `}}` (double braces).
/// - Returns `Err(FillError::MissingVariable(_))` when a placeholder has no mapping.
/// - No panic, no `unsafe`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// let tpl = "Hi {{who}}";
/// let mut vars = HashMap::new();
/// vars.insert("who".to_string(), "Alice".to_string());
/// let out = fill_prompt::fill_template(tpl, vars).unwrap();
/// assert_eq!(out, "Hi Alice");
/// ```
pub fn fill_template<I, K, V>(template: &str, vars: I) -> Result<String, FillError>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    // Collect variables into owned HashMap<String,String>
    let mut map: HashMap<String, String> = HashMap::new();
    for (k, v) in vars {
        map.insert(k.as_ref().to_string(), v.as_ref().to_string());
    }

    let mut out = String::with_capacity(template.len());
    let bytes = template.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        // look for "{{"
        if i + 1 < bytes.len() && bytes[i] == b'{' && bytes[i + 1] == b'{' {
            let start = i;
            i += 2; // skip "{{"
            let key_start = i;
            // scan until "}}"
            let mut found = false;
            while i + 1 < bytes.len() {
                if bytes[i] == b'}' && bytes[i + 1] == b'}' {
                    found = true;
                    break;
                }
                i += 1;
            }
            if !found {
                return Err(FillError::UnclosedPlaceholder(start));
            }
            // `trim()` returns &str
            let key = template[key_start..i].trim();
            if key.is_empty() {
                return Err(FillError::EmptyPlaceholder(key_start));
            }
            // move i past "}}"
            i += 2;
            match map.get(key) {
                Some(val) => out.push_str(val),
                None => return Err(FillError::MissingVariable(key.to_string())),
            }
        } else {
            // append single char (valid UTF-8 since template is &str)
            // avoid unwrap for clippy
            let mut chars = template[i..].chars();
            if let Some(ch) = chars.next() {
                out.push(ch);
                i += ch.len_utf8();
            } else {
                // should not happen because i < bytes.len(), but guard defensively
                break;
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn success_basic() {
        let tpl = "X {{a}} Y {{b}} Z";
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "1".to_string());
        vars.insert("b".to_string(), "2".to_string());
        let out = fill_template(tpl, vars).unwrap();
        assert_eq!(out, "X 1 Y 2 Z");
    }

    #[test]
    fn missing_variable_error() {
        let tpl = "Hello {{name}}";
        // explicit types so inference works
        let vars: HashMap<String, String> = HashMap::new();
        let err = fill_template(tpl, vars).unwrap_err();
        assert_eq!(err, FillError::MissingVariable("name".to_string()));
    }

    #[test]
    fn unclosed_placeholder_error() {
        let tpl = "Hey {{oops";
        let mut vars = HashMap::new();
        vars.insert("oops".to_string(), "X".to_string());
        let err = fill_template(tpl, vars).unwrap_err();
        match err {
            FillError::UnclosedPlaceholder(_) => {}
            _ => panic!("expected UnclosedPlaceholder"),
        }
    }

    #[test]
    fn empty_placeholder_error() {
        let tpl = "Empty {{  }}";
        // explicit types so inference works
        let vars: HashMap<String, String> = HashMap::new();
        let err = fill_template(tpl, vars).unwrap_err();
        match err {
            FillError::EmptyPlaceholder(_) => {}
            _ => panic!("expected EmptyPlaceholder"),
        }
    }

    #[test]
    fn adjacent_placeholders() {
        let tpl = "{{a}}{{b}}{{c}}";
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "A".to_string());
        vars.insert("b".to_string(), "B".to_string());
        vars.insert("c".to_string(), "C".to_string());
        let out = fill_template(tpl, vars).unwrap();
        assert_eq!(out, "ABC");
    }

    #[test]
    fn utf8_handling() {
        let tpl = "Pré {{a}} cœur";
        let mut vars = HashMap::new();
        vars.insert("a".to_string(), "✓".to_string());
        let out = fill_template(tpl, vars).unwrap();
        assert!(out.contains("✓"));
    }
}
