use std::collections::HashMap;

#[test]
fn integration_fill() {
    let tpl = "A {{x}} B {{y}}";
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), "X".to_string());
    vars.insert("y".to_string(), "Y".to_string());
    let s = fill_prompt::fill_template(tpl, vars).unwrap();
    assert_eq!(s, "A X B Y");
}
