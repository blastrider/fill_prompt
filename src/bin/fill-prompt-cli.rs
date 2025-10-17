//! Minimal CLI for fill_prompt with validation of descriptions and optional file output.
//!
//! Usage:
//!   fill-prompt-cli --template "<TEMPLATE>" [--var key=val]... [--vars file|inline]... [--out-dir DIR]
//!   fill-prompt-cli --file <PATH>           [--var key=val]... [--vars file|inline]... [--out-dir DIR]
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};

use fill_prompt::validate::{validate_context, validate_short};
use fill_prompt::vars::parse_vars_arg;

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  fill-prompt-cli --template <TEMPLATE> [--var key=val]... [--vars file|inline]... [--out-dir DIR]");
    eprintln!("  fill-prompt-cli --file <PATH>         [--var key=val]... [--vars file|inline]... [--out-dir DIR]");
    eprintln!("Options:");
    eprintln!("  --template <TEMPLATE>   provide template string");
    eprintln!("  --file <PATH>           read template from file");
    eprintln!("  --var key=value         provide a variable (repeatable)");
    eprintln!("  --vars <file|inline>    load variables from file (json/yaml/toml) or inline JSON/TOML/YAML");
    eprintln!("  --out-dir <DIR>         write output file into DIR (creates it if missing)");
    eprintln!("  --help, -h              show this message");
}

fn parse_kv(s: &str) -> Result<(String, String)> {
    let mut parts = s.splitn(2, '=');
    match (parts.next(), parts.next()) {
        (Some(k), Some(v)) if !k.is_empty() => Ok((k.to_string(), v.to_string())),
        _ => Err(anyhow!("--var requires key=value")),
    }
}

/// Extract all `{{placeholder}}` keys present in the template (best-effort; errors like unclosed braces are left to the filler).
fn extract_placeholders(template: &str) -> BTreeSet<String> {
    let mut set = BTreeSet::new();
    let bytes = template.as_bytes();
    let mut i = 0usize;
    while i + 1 < bytes.len() {
        if bytes[i] == b'{' && bytes[i + 1] == b'{' {
            i += 2;
            let start = i;
            while i + 1 < bytes.len() && !(bytes[i] == b'}' && bytes[i + 1] == b'}') {
                i += 1;
            }
            if i + 1 >= bytes.len() {
                break; // laisser l'erreur au remplissage
            }
            let key = template[start..i].trim();
            if !key.is_empty() {
                set.insert(key.to_string());
            }
            i += 2; // passer "}}"
        } else {
            // avancer d'un char UTF-8
            if let Some(ch) = template[i..].chars().next() {
                i += ch.len_utf8();
            } else {
                break;
            }
        }
    }
    set
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let mut template: Option<String> = None;
    let mut template_file_path: Option<PathBuf> = None; // pour générer un nom de fichier de sortie
    let mut vars_map: HashMap<String, String> = HashMap::new();
    let mut out_dir: Option<PathBuf> = None;

    while let Some(a) = args.next() {
        match a.as_str() {
            "--template" => {
                let t = args
                    .next()
                    .ok_or_else(|| anyhow!("--template requires an argument"))?;
                template = Some(t);
            }
            "--file" => {
                let p = args
                    .next()
                    .ok_or_else(|| anyhow!("--file requires a path"))?;
                let s = fs::read_to_string(&p)
                    .with_context(|| format!("failed to read file '{}'", p))?;
                template = Some(s);
                template_file_path = Some(PathBuf::from(p));
            }
            "--var" => {
                let kv = args
                    .next()
                    .ok_or_else(|| anyhow!("--var requires key=value"))?;
                let (k, v) = parse_kv(&kv)?;
                vars_map.insert(k, v); // dernière occurrence gagne
            }
            "--vars" => {
                let arg = args
                    .next()
                    .ok_or_else(|| anyhow!("--vars requires a file path or inline value"))?;
                let parsed = parse_vars_arg(&arg)
                    .with_context(|| format!("failed to parse --vars '{}'", arg))?;
                for (k, v) in parsed {
                    vars_map.insert(k, v); // dernière occurrence gagne
                }
            }
            "--out-dir" => {
                let d = args
                    .next()
                    .ok_or_else(|| anyhow!("--out-dir requires a directory path"))?;
                out_dir = Some(PathBuf::from(d));
            }
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            other => {
                eprintln!("Unknown arg: {}", other);
                print_usage();
                return Err(anyhow!("unknown arg {}", other));
            }
        }
    }

    let template = match template {
        Some(t) => t,
        None => {
            print_usage();
            return Err(anyhow!("no template provided"));
        }
    };

    // Pré-check facultatif : lister toutes les variables manquantes d’un coup.
    let needed = extract_placeholders(&template);
    let have: BTreeSet<String> = vars_map.keys().cloned().collect();
    let missing: Vec<_> = needed.difference(&have).cloned().collect();
    if !missing.is_empty() {
        eprintln!(
            "Missing variables ({}): {}",
            missing.len(),
            missing.join(", ")
        );
        anyhow::bail!("incomplete variable set");
    }

    // Validation s'il y a des clés cibles
    if let Some(short) = vars_map.get("short_description") {
        validate_short(short).map_err(|e| anyhow!("short_description validation failed: {}", e))?;
    }
    if let Some(ctx) = vars_map.get("context_paragraph") {
        validate_context(ctx).map_err(|e| anyhow!("context_paragraph validation failed: {}", e))?;
    }

    // Remplissage
    let output = fill_prompt::fill_template(&template, vars_map.into_iter())
        .map_err(|e| anyhow!("failed to fill template: {}", e))?;

    // Écriture conditionnelle
    if let Some(dir) = out_dir {
        // Créer le dossier si nécessaire
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create output directory '{}'", dir.display()))?;

        // Déterminer un nom de fichier
        let base = template_file_path
            .as_deref()
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let filename = format!("{}-filled.txt", base);
        let out_path = dir.join(filename);

        fs::write(&out_path, output.as_bytes())
            .with_context(|| format!("failed to write output file '{}'", out_path.display()))?;
        eprintln!("Wrote filled template to: {}", out_path.display());
    } else {
        // Comportement historique : impression sur stdout
        println!("{}", output);
    }

    Ok(())
}
