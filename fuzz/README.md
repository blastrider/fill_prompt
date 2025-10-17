# fuzz (placeholder)

To setup fuzzing locally:

1. Install cargo-fuzz:
cargo install cargo-fuzz

java
Copier le code

2. Initialize fuzz (once):
cargo fuzz init

sql
Copier le code

3. Add a harness calling `fill_prompt::fill_template` with arbitrary bytes / strings.
Be careful converting to valid UTF-8: you may skip invalid inputs.

This directory contains a placeholder harness `harness.rs`.