# Fuzzing fill_prompt

Cette arborescence est prête pour `cargo-fuzz` et cible la fonction `fill_prompt::fill_template`.

## Pré-requis

```bash
cargo install cargo-fuzz
```

## Lancer une campagne

```bash
cargo fuzz run fill_prompt
```

Les corpus et artéfacts générés apparaissent dans `fuzz/corpus/` et `fuzz/artifacts/`.

## À propos du harness

Le harness (`fuzz_targets/fill_prompt.rs`) interprète la première ligne de l’entrée fuzzée comme un template et le reste comme une succession de lignes `clé=valeur`. Les valeurs trim sont injectées dans la fonction, en plus d’une variable par défaut `x=default`, afin d’exercer à la fois les cas de succès et d’erreur (variables manquantes, accolades incomplètes, etc.). Il suffit d’ajouter des fichiers au corpus ou d’affiner cette logique pour explorer des cas spécifiques.
