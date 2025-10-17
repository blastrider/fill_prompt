use proptest::prelude::*;

proptest! {
    #[test]
    fn no_braces_identity(s in "\\PC*") {
        // if string contains no "{" or "}", result must equal input
        prop_assume!(!s.contains('{') && !s.contains('}'));
        let vars = std::collections::HashMap::<String,String>::new();
        let out = fill_prompt::fill_template(&s, vars).unwrap();
        prop_assert_eq!(out, s);
    }
}
