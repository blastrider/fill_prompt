use std::collections::HashMap;
fn main() {
    let tpl = "Program {{name}} prints {{msg}}";
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), "myprog".to_string());
    vars.insert("msg".to_string(), "Hello".to_string());
    let out = fill_prompt::fill_template(tpl, vars).unwrap();
    println!("{}", out);
}
