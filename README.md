
# fill_prompt

Génère des textes à partir de modèles avec des variables `{{...}}`.
Fourni en **bibliothèque** Rust et en **CLI** minimal.

> **MSRV** : `1.90.0` (fixée dans `[package.metadata]`)

![Rust](https://img.shields.io/badge/Rust-stable-blue)
![License](https://img.shields.io/badge/license-Apache--2.0-green)
![CI](https://img.shields.io/badge/CI-rustfmt%2Fclippy%2Ftests-brightgreen)

---

## ✨ Caractéristiques

* Remplacement sûr de `{{clé}}` → valeur (pas de panics attendus, pas d’`unsafe` exposé).
* Entrées variables via `--var key=val` **ou** fichiers **JSON / YAML / TOML** (`--vars`), avec conversion automatique des scalaires.
* Validation intégrée de descriptions :

  * `short_description` ≤ **30 mots**
  * `context_paragraph` ≤ **40 mots**
* Messages d’erreurs clairs (lib: `thiserror`, CLI: `anyhow`).
* Sortie vers **stdout** ou vers un **fichier** via `--out-dir`.

---

## 📦 Installation (dev)

```bash
# cloner
git clone <URL> && cd fill_prompt

# tests rapides (libre de features)
cargo test

# si vous utilisez --vars (JSON/YAML/TOML), activez la feature:
cargo test --features serde

# exécuter le CLI en dev
cargo run --bin fill-prompt-cli --features serde -- --help
```

> La feature `serde` active les parseurs `serde_json`, `serde_yaml`, `toml`.

---

## 🔧 Utilisation – CLI

### Template depuis un fichier + variables depuis TOML

```bash
cargo run --bin fill-prompt-cli --features serde -- \
  --file templates/project_request.tpl \
  --vars vars/basic.toml
```

### Variables inline (JSON) + sortie fichier

```bash
cargo run --bin fill-prompt-cli --features serde -- \
  --template "Bonjour {{who}}!" \
  --vars '{"who":"Monde"}' \
  --out-dir out/
# => écrit out/output-filled.txt
```

### Mélanger plusieurs sources + overrides

```bash
cargo run --bin fill-prompt-cli --features serde -- \
  --file templates/project_request.tpl \
  --vars vars/basic.toml \
  --vars vars/override.yaml \
  --var author="Maxime"
# la dernière occurrence gagne
```

### Détection des variables manquantes

La CLI liste toutes les `{{...}}` absentes avant le remplissage et échoue proprement.

---

## 🧰 Utilisation – Bibliothèque

```rust
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tpl = "Salut {{who}}";
    let mut vars = HashMap::new();
    vars.insert("who".to_string(), "Alice".to_string());

    let out = fill_prompt::fill_template(tpl, vars)?;
    assert_eq!(out, "Salut Alice");
    Ok(())
}
```

---

## 📁 Exemples de fichiers de variables

### JSON

```json
{
  "crate_type": "bin",
  "crate_name": "fill_prompt",
  "msrv": "1.90.0",
  "short_description": "Petit outil pour remplir un template.",
  "context_paragraph": "Idéal pour des prompts cohérents et reproductibles.",
  "author": "Max",
  "public": "développeurs",
  "opt_level": 3,
  "license": "Apache-2.0"
}
```

### YAML

```yaml
crate_type: bin
crate_name: fill_prompt
msrv: "1.90.0"
short_description: >
  Petit outil pour remplir un template.
context_paragraph: >
  Idéal pour des prompts cohérents et reproductibles.
author: Max
public: développeurs
opt_level: 3
license: Apache-2.0
```

### TOML

```toml
crate_type = "bin"
crate_name = "fill_prompt"
msrv = "1.90.0"
short_description = "Petit outil pour remplir un template."
context_paragraph = "Idéal pour des prompts cohérents et reproductibles."
author = "Max"
public = "développeurs"
opt_level = 3
license = "Apache-2.0"
```

> Les valeurs scalaires (string/number/bool/null) sont converties en chaîne.
> Les objets/arrays imbriqués ne sont pas supportés (aplatir si nécessaire).

---

## ⚙️ Options et conventions

* `--var key=value` : ajoute/écrase une variable (répétable).
* `--vars <fichier|inline>` : charge un objet racine clé→valeur (JSON/YAML/TOML).
* `--out-dir <DIR>` : écrit `<DIR>/<basename>-filled.txt` (créé si manquant).

  * Template inline → `output-filled.txt`.

---

## ✅ Qualité & Sécurité

* **Format** : `cargo fmt` (CI: `cargo fmt -- --check`)
* **Lint** : `cargo clippy --all-targets -- -D warnings`
* **Erreurs** : pas de panics pour erreurs attendues ; `Result<T,E>` partout.
* **Sécurité** : 100% safe par défaut ; aucun `unsafe` public.
* **MSRV** : `1.90.0` (voir `[package.metadata]`).

---

## 🧪 Tests & Benchmarks

* Unitaires, intégration, et **property tests** via `proptest`.
* Tests CLI via `assert_cmd` / `predicates`.
* Benchmarks avec `criterion` (feature `bench` à activer côté binaire dédié).

```bash
cargo test
cargo test --features serde
cargo bench --features bench   # si vous avez un binaire/bench configuré
```

---

## 📚 Références utiles

* Rust API Guidelines — [https://rust-lang.github.io/api-guidelines/](https://rust-lang.github.io/api-guidelines/)
* Clippy Lints — [https://rust-lang.github.io/rust-clippy/master/index.html](https://rust-lang.github.io/rust-clippy/master/index.html)
* ANSSI Secure Rust Guide — [https://anssi-fr.github.io/rust-guide/](https://anssi-fr.github.io/rust-guide/)
* Rust Performance Book — [https://nnethercote.github.io/perf-book/](https://nnethercote.github.io/perf-book/)
* Miri — [https://github.com/rust-lang/miri](https://github.com/rust-lang/miri)

---

## 🔒 Licence

Apache-2.0. Voir `LICENSE`.

---

## 📝 Description courte

**Phrase (≤30 mots)**
Génère des textes à partir de modèles `{{…}}`, avec chargement de variables, validations et erreurs claires. Idéal pour des prompts fiables et reproductibles.

**Paragraphe (≤40 mots)**
La bibliothèque et la CLI remplacent les variables dans vos modèles, chargent vos données depuis JSON/YAML/TOML, valident des champs clés et produisent un résultat propre, sur la sortie standard ou dans un fichier dédié.

---

## 🤝 Contribution

* Ouvrez une issue avant changements majeurs.
* Respecter rustfmt/clippy/MSRV.
* Ajouter tests/doc/examples pour toute nouvelle API.

---

**Bon usage et bons prompts !**
