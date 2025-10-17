use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn fills_template_from_inline_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fill-prompt-cli")?;
    cmd.args(["--template", "Bonjour {{name}}", "--var", "name=Codex"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Bonjour Codex"));
    Ok(())
}

#[test]
fn reports_missing_variables_before_fill() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fill-prompt-cli")?;
    cmd.args(["--template", "Salut {{who}}"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Missing variables"))
        .stderr(predicate::str::contains("incomplete variable set"));
    Ok(())
}

#[test]
fn writes_output_file_when_requested() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = tempdir()?;
    let mut cmd = Command::cargo_bin("fill-prompt-cli")?;
    cmd.args([
        "--template",
        "Hello {{name}}",
        "--var",
        "name=world",
        "--out-dir",
        out_dir.path().to_str().unwrap(),
    ]);

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Wrote filled template to"));

    let written = fs::read_to_string(out_dir.path().join("output-filled.txt"))?;
    assert_eq!(written, "Hello world");
    Ok(())
}

#[test]
fn rejects_long_short_description() -> Result<(), Box<dyn std::error::Error>> {
    let words: Vec<String> = (0..31).map(|i| format!("mot{}", i)).collect();
    let long_short = words.join(" ");

    let mut cmd = Command::cargo_bin("fill-prompt-cli")?;
    cmd.args([
        "--template",
        "{{short_description}}",
        "--var",
        &format!("short_description={long_short}"),
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "short_description validation failed",
        ))
        .stderr(predicate::str::contains("trop long:"));
    Ok(())
}

#[cfg(feature = "serde")]
#[test]
fn loads_variables_from_structured_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fill-prompt-cli")?;
    cmd.args([
        "--template",
        "Crate {{crate_name}} par {{author}}",
        "--vars",
        concat!(env!("CARGO_MANIFEST_DIR"), "/vars/basic.toml"),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("fill_prompt par Max"));
    Ok(())
}
