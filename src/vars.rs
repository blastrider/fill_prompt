//! Chargement de variables depuis JSON / YAML / TOML.

use std::collections::HashMap;

#[cfg(feature = "serde")]
mod with_serde {
    use super::*;
    use anyhow::{anyhow, Context, Result};
    use std::fs;
    use std::path::Path;

    pub fn parse_vars_arg(arg: &str) -> Result<HashMap<String, String>> {
        let path = Path::new(arg);

        if path.exists() && path.is_file() {
            let content = fs::read_to_string(path)
                .with_context(|| format!("lecture du fichier de variables: {}", arg))?;
            if let Some(ext) = path
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase())
            {
                match ext.as_str() {
                    "json" => parse_json(&content),
                    "yaml" | "yml" => parse_yaml(&content),
                    "toml" => parse_toml(&content),
                    other => Err(anyhow!("extension de fichier non supportée: {}", other)),
                }
            } else {
                parse_inline_try_all(&content)
            }
        } else {
            parse_inline_try_all(arg)
        }
    }

    fn parse_inline_try_all(s: &str) -> Result<HashMap<String, String>> {
        parse_json(s)
            .or_else(|_| parse_toml(s))
            .or_else(|_| parse_yaml(s))
            .map_err(|e| {
                anyhow!(
                    "impossible de parser la valeur inline en JSON/TOML/YAML: {}",
                    e
                )
            })
    }

    fn parse_json(s: &str) -> Result<HashMap<String, String>> {
        let v: serde_json::Value = serde_json::from_str(s).context("parse JSON: invalid JSON")?;
        value_to_map_json(&v)
    }

    fn parse_yaml(s: &str) -> Result<HashMap<String, String>> {
        let v: serde_yaml::Value = serde_yaml::from_str(s).context("parse YAML: invalid YAML")?;
        value_to_map_yaml(&v)
    }

    fn parse_toml(s: &str) -> Result<HashMap<String, String>> {
        let v: toml::Value = s.parse().context("parse TOML: invalid TOML")?;
        value_to_map_toml(&v)
    }

    fn value_to_map_json(v: &serde_json::Value) -> Result<HashMap<String, String>> {
        match v {
            serde_json::Value::Object(map) => {
                let mut out = HashMap::with_capacity(map.len());
                for (k, val) in map {
                    out.insert(k.clone(), json_value_to_string(val)?);
                }
                Ok(out)
            }
            _ => Err(anyhow!("JSON attendu: objet map clé->valeur à la racine")),
        }
    }

    fn json_value_to_string(v: &serde_json::Value) -> Result<String> {
        match v {
            serde_json::Value::String(s) => Ok(s.clone()),
            serde_json::Value::Number(n) => Ok(n.to_string()),
            serde_json::Value::Bool(b) => Ok(b.to_string()),
            serde_json::Value::Null => Ok(String::new()),
            other => Err(anyhow!(
                "valeur non scalaire pour la clé (JSON): {:?}",
                other
            )),
        }
    }

    fn value_to_map_yaml(v: &serde_yaml::Value) -> Result<HashMap<String, String>> {
        match v {
            serde_yaml::Value::Mapping(map) => {
                let mut out = HashMap::with_capacity(map.len());
                for (k, val) in map {
                    let key = match k {
                        serde_yaml::Value::String(s) => s.clone(),
                        _ => return Err(anyhow!("clé YAML non-string")),
                    };
                    out.insert(key, yaml_value_to_string(val)?);
                }
                Ok(out)
            }
            _ => Err(anyhow!("YAML attendu: mapping (clé->valeur) à la racine")),
        }
    }

    fn yaml_value_to_string(v: &serde_yaml::Value) -> Result<String> {
        match v {
            serde_yaml::Value::String(s) => Ok(s.clone()),
            serde_yaml::Value::Bool(b) => Ok(b.to_string()),
            serde_yaml::Value::Number(n) => Ok(n.to_string()),
            serde_yaml::Value::Null => Ok(String::new()),
            other => Err(anyhow!(
                "valeur non scalaire pour la clé (YAML): {:?}",
                other
            )),
        }
    }

    fn value_to_map_toml(v: &toml::Value) -> Result<HashMap<String, String>> {
        match v {
            toml::Value::Table(map) => {
                let mut out = HashMap::with_capacity(map.len());
                for (k, val) in map {
                    out.insert(k.clone(), toml_value_to_string(val)?);
                }
                Ok(out)
            }
            _ => Err(anyhow!("TOML attendu: table (clé = valeur) à la racine")),
        }
    }

    fn toml_value_to_string(v: &toml::Value) -> Result<String> {
        match v {
            toml::Value::String(s) => Ok(s.clone()),
            toml::Value::Integer(i) => Ok(i.to_string()),
            toml::Value::Float(f) => Ok(f.to_string()),
            toml::Value::Boolean(b) => Ok(b.to_string()),
            toml::Value::Datetime(dt) => Ok(dt.to_string()),
            toml::Value::Array(_) | toml::Value::Table(_) => {
                Err(anyhow!("valeur non scalaire pour la clé (TOML)"))
            }
        }
    }
}

#[cfg(not(feature = "serde"))]
mod without_serde {
    use super::*;
    use anyhow::{anyhow, Result};

    pub fn parse_vars_arg(_arg: &str) -> Result<HashMap<String, String>> {
        Err(anyhow!(
            "feature \"serde\" non activée. Recompilez avec `--features serde`."
        ))
    }
}

// re-exports publics (un seul point d’export)
#[cfg(feature = "serde")]
pub use with_serde::parse_vars_arg;
#[cfg(not(feature = "serde"))]
pub use without_serde::parse_vars_arg;
