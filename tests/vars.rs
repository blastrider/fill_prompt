// tests/vars.rs
#[cfg(feature = "serde")]
#[test]
fn parse_inline_json() {
    let arg = r#"{"name":"bob","age":30}"#;
    let vars = fill_prompt::vars::parse_vars_arg(arg).expect("parse inline json");
    assert_eq!(vars.get("name").map(|s| s.as_str()), Some("bob"));
    assert_eq!(vars.get("age").map(|s| s.as_str()), Some("30"));
}
