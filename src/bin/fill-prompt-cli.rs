//! Minimal CLI for fill_prompt with validation of descriptions.
//!
//! Usage:
//!   cargo run --bin fill-prompt-cli -- --template "<TEMPLATE>" --var key=val ...
//! Example:
//!   cargo run --bin fill-prompt-cli -- --file project_request.tpl \
//!     --var crate_type=bin --var crate_name=fill_prompt --var msrv=1.90.0 \
//!     --var short_description="Phrase courte ..." \
//!     --var context_paragraph="Paragraphe de contexte ..."
use std::collections::HashMap;
use std::env;
use std::fs;

use anyhow::{anyhow, Context, Result};

use fill_prompt::validate::{validate_context, validate_short};

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  fill-prompt-cli --template <TEMPLATE> [--var key=val]... ");
    eprintln!("  fill-prompt-cli --file <path> [--var key=val]... ");
    eprintln!("Options:");
    eprintln!("  --template <TEMPLATE>   provide template string");
    eprintln!("  --file <PATH>           read template from file");
    eprintln!("  --var key=value         provide a variable (repeatable)");
    eprintln!("  --help, -h              show this message");
}

fn parse_kv(s: &str) -> Result<(String, String)> {
    let mut parts = s.splitn(2, '=');
    match (parts.next(), parts.next()) {
        (Some(k), Some(v)) if !k.is_empty() => Ok((k.to_string(), v.to_string())),
        _ => Err(anyhow!("--var requires key=value")),
    }
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let mut template: Option<String> = None;
    let mut vars_map: HashMap<String, String> = HashMap::new();

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
            }
            "--var" => {
                let kv = args
                    .next()
                    .ok_or_else(|| anyhow!("--var requires key=value"))?;
                let (k, v) = parse_kv(&kv)?;
                // newest value overrides
                vars_map.insert(k, v);
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

    // Validation step: if the relevant keys are present, validate them.
    if let Some(short) = vars_map.get("short_description") {
        validate_short(short).map_err(|e| anyhow!("short_description validation failed: {}", e))?;
    }

    if let Some(ctx) = vars_map.get("context_paragraph") {
        validate_context(ctx).map_err(|e| anyhow!("context_paragraph validation failed: {}", e))?;
    }

    // All validations passed — fill the template
    let output = fill_prompt::fill_template(&template, vars_map.into_iter())
        .map_err(|e| anyhow!("failed to fill template: {}", e))?;

    println!("{}", output);
    Ok(())
}
