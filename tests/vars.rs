#[cfg(feature = "serde")]
mod with_serde {
    use fill_prompt::vars::parse_vars_arg;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    const VAR_FIXTURES: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/vars");

    fn fixture(path: &str) -> PathBuf {
        PathBuf::from(VAR_FIXTURES).join(path)
    }

    fn value_trimmed<'a>(map: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
        map.get(key)
            .map(|s| s.trim_end_matches(|c| c == '\n' || c == '\r'))
    }

    fn assert_basic_vars(map: &HashMap<String, String>) {
        assert_eq!(value_trimmed(map, "crate_type"), Some("bin"));
        assert_eq!(value_trimmed(map, "crate_name"), Some("fill_prompt"));
        assert_eq!(value_trimmed(map, "msrv"), Some("1.90.0"));
        assert_eq!(
            value_trimmed(map, "short_description"),
            Some("Petit outil pour remplir un template à partir de variables.")
        );
        assert_eq!(
            value_trimmed(map, "context_paragraph"),
            Some("Utile pour générer des prompts cohérents, reproductibles, sans erreurs d’oubli de variables.")
        );
        assert_eq!(value_trimmed(map, "author"), Some("Max"));
        assert_eq!(value_trimmed(map, "public"), Some("développeurs"));
        assert_eq!(value_trimmed(map, "opt_level"), Some("3"));
        assert_eq!(value_trimmed(map, "license"), Some("Apache-2.0"));
    }

    #[test]
    fn parse_inline_json() {
        let arg = r#"{"name":"bob","age":30}"#;
        let vars = parse_vars_arg(arg).expect("parse inline json");
        assert_eq!(vars.get("name").map(String::as_str), Some("bob"));
        assert_eq!(vars.get("age").map(String::as_str), Some("30"));
    }

    #[test]
    fn parse_inline_yaml() {
        let arg = r#"
            name: bob
            active: true
            ratio: 0.75
        "#;
        let vars = parse_vars_arg(arg).expect("parse inline yaml");
        assert_eq!(vars.get("name").map(String::as_str), Some("bob"));
        assert_eq!(vars.get("active").map(String::as_str), Some("true"));
        assert_eq!(vars.get("ratio").map(String::as_str), Some("0.75"));
    }

    #[test]
    fn parse_inline_toml() {
        let arg = r#"
            name = "bob"
            counter = 42
        "#;
        let vars = parse_vars_arg(arg).expect("parse inline toml");
        assert_eq!(vars.get("name").map(String::as_str), Some("bob"));
        assert_eq!(vars.get("counter").map(String::as_str), Some("42"));
    }

    #[test]
    fn parse_files_by_extension() {
        for fixture_name in ["basic.json", "basic.yaml", "basic.toml"] {
            let map = parse_vars_arg(fixture(fixture_name).to_str().unwrap())
                .unwrap_or_else(|e| panic!("parse {}: {}", fixture_name, e));
            assert_basic_vars(&map);
        }
    }

    #[test]
    fn parse_inline_invalid_reports_error() {
        let err = parse_vars_arg("not-valid").expect_err("should fail to parse inline junk");
        assert!(
            err.to_string()
                .contains("impossible de parser la valeur inline"),
            "unexpected error: {err:#}"
        );
    }

    #[test]
    fn parse_file_unknown_extension_reports_error() {
        let dir = TempDir::new().expect("temp dir");
        let path = dir.path().join("vars.txt");
        fs::write(&path, "name=bob").expect("write temp file");
        let err =
            parse_vars_arg(path.to_str().unwrap()).expect_err("expected unsupported extension");
        assert!(
            err.to_string()
                .contains("extension de fichier non supportée"),
            "unexpected error: {err:#}"
        );
    }

    #[test]
    fn parse_directory_path_reports_error() {
        let dir = TempDir::new().expect("temp dir");
        let err =
            parse_vars_arg(dir.path().to_str().unwrap()).expect_err("expected directory error");
        assert!(
            err.to_string()
                .contains("impossible de parser la valeur inline"),
            "unexpected error: {err:#}"
        );
    }
}

#[cfg(not(feature = "serde"))]
mod without_serde {
    use fill_prompt::vars::parse_vars_arg;

    #[test]
    fn serde_feature_disabled_returns_error() {
        let err = parse_vars_arg(r#"{"name":"bob"}"#).expect_err("feature disabled must error");
        assert!(
            err.to_string().contains("feature \"serde\" non activée"),
            "unexpected error: {err:#}"
        );
    }
}
