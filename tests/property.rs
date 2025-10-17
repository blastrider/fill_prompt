use proptest::prelude::*;
use std::collections::HashMap;

proptest! {
    #[test]
    fn no_braces_identity(s in "\\PC*") {
        // if string contains no "{" or "}", result must equal input
        prop_assume!(!s.contains('{') && !s.contains('}'));
        let vars = HashMap::<String,String>::new();
        let out = fill_prompt::fill_template(&s, vars).unwrap();
        prop_assert_eq!(out, s);
    }
}

proptest! {
    #[test]
    fn substitutes_single_placeholder(value in "\\PC*") {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), value.clone());

        let template = "Salut {{ name }} !";
        let out = fill_prompt::fill_template(template, vars).unwrap();
        prop_assert_eq!(out, format!("Salut {} !", value));
    }
}

proptest! {
    #[test]
    fn missing_placeholder_returns_error(key in "[A-Za-z0-9_]{1,8}") {
        let template = format!("{{{{ {key} }}}}");
        let vars = HashMap::<String,String>::new();
        let err = fill_prompt::fill_template(&template, vars).unwrap_err();
        prop_assert_eq!(err, fill_prompt::FillError::MissingVariable(key));
    }
}
