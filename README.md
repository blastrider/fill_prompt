# fill_prompt

**Description (≤30 mots)**  
Lib légère pour remplir des templates de prompt via `{{var}}` → substitution sûre.

**Court paragraphe (≤40 mots)**  
`fill_prompt` fournit une API simple et sûre pour substituer des variables dans des templates texte (format `{{name}}`). Conçue pour être utilisée dans des pipelines qui génèrent des prompts prêts à coller dans ChatGPT.

## MSRV
MSRV = 1.90.0. Utiliser le toolchain stable (rustup default stable) ; CI teste stable et MSRV.

## Quickstart
```rust
use std::collections::HashMap;
let tpl = "Create program {{name}}";
let mut vars = HashMap::new();
vars.insert("name".to_string(), "myprog".to_string());
let out = fill_prompt::fill_template(tpl, vars).unwrap();
println!("{}", out);
