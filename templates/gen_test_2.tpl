Je veux un projet Rust lib nommé "{{crate_name}}" :

+ DESCRIPTION MINIMALE
- {{short_description}}

- {{context_paragraph}}

CONTRAINTES GÉNÉRALES (obligatoires)
- Utilise la **dernière toolchain stable** ; documente et fixe la `MSRV = "{{msrv}}"` dans README et `[package.metadata]` du Cargo.toml.
- Format : `cargo fmt` obligatoire ; CI doit exécuter `cargo fmt -- --check`.
- Lint : `cargo clippy --all-targets -- -D warnings` en CI.
- Sécurité : code **100% safe par défaut** ; `unsafe` autorisé uniquement si strictement nécessaire, isolé, documenté dans `# Safety` et accompagné de tests validant les invariants. **Aucun unsafe exposé publiquement.**
- Erreurs : les bibliothèques **ne doivent pas panicer** pour des erreurs attendues — retourner `Result<T, E>`. Binaire : `anyhow` pour top-level ; librairie : `thiserror` pour types d'erreur.
- Pas d’`unwrap`/`expect` hors tests ou prototypes clairement identifiés.

CHECKLIST POUR UN CODE RUST IRRÉPROCHABLE (intégrée)
Appliquer systématiquement ces 5 aspects fondamentaux :

1) Bonnes pratiques idiomatiques (API design, lisibilité, ownership)
- Nommage : `snake_case` fonctions/variables, `CamelCase` types/traits.
- API prévisible : itérateurs via `iter`/`iter_mut`/`into_iter`; getters sans `get_` superflu.
- Implémenter traits standards pertinents (`Debug`, `Clone`, `Copy`, `Eq`, `Hash`) seulement quand ils sont sémantiquement corrects.
- Éviter out-parameters ; préférer retours. Préférer `&str`/`&[u8]` dans l’API, `Cow` pour acceptation emprunt/possédé.
- Documentation rustdoc complète pour chaque élément public, avec `# Examples` et description des erreurs.

2) Performance & efficacité mémoire
- Favoriser abstractions zéro-coût (itérateurs, closures).
- Minimiser allocations : réutiliser buffers, préférer slices `&[T]` à `Vec<T>` quand possible.
- Choix judicieux de structures/algorithmes (HashMap vs Vec, bitflags, packing des structs).
- Mesurer avant d’optimiser : `criterion`, `cargo bench`, profiling OS.
- Compiler et mesurer en `--release` ; envisager LTO/incremental build si pertinent.

3) Sécurité & sûreté (gestion de l’unsafe, absence d’UB)
- Écrire du code safe autant que possible ; isoler l’unsafe dans de petites unités avec doc `Safety`.
- Valider invariants manuellement dans unsafe ; utiliser Miri, sanitizers et fuzzing pour détecter UB.
- Pas d’accès hors-borne, pas de transmute non justifié, pas de lecture de mémoire non initialisée.
- Protéger frontières FFI : ne pas laisser panics traverser ; documenter ownership/contract FFI.
- Zeroize si manipulation de secrets ; éviter Drop coûteux/paniquant.

4) Conformité aux guidelines officielles (API Guidelines, Clippy, rustfmt)
- Respecter Rust API Guidelines pour nommage, ergonomie, erreurs.
- CI : rustfmt + clippy (pedantic option si demandé), `cargo fix` manuel avant application.
- Metadata Cargo.toml complète (description, keywords, categories, license, readme).
- Pas de dépendances nightly dans une lib publique (sauf justification forte).

5) Exigences industrielles (politiques entreprise, vetting des dépendances, CI)
- CI robuste : tests matrix (stable, MSRV), fmt, clippy -D, cargo-audit, cargo-deny.
- Vetting dépendances : `cargo outdated`, justification des crates, minimiser l’arbre.
- Revue de code obligatoire pour changements critiques, justification d’`unsafe`, tests Send/Sync si nécessaire.
- Policy : SemVer respecté, CHANGELOG humain, procédure de contribution & support documentés.

DOCS / EXEMPLES / TESTS
- README complet (objectif, MSRV `{{msrv}}`, usage, badges, license `{{license}}`, repo `{{repository}}`).
- Doctests compilables et examples/ utiles.
- Tests : unitaires, d’intégration, property tests (`proptest`/`quickcheck`), harness `cargo-fuzz` placeholder.
- Benchmarks `criterion/benches` avec script reproducible.
- Tests en debug et release ; activer overflow_checks quand utile.

CI / RELEASE / SECURITY
- Fournir `.github/workflows/ci.yml` :
  - matrix: latest-stable, MSRV (`{{msrv}}`).
  - jobs: fmt check, clippy (-D warnings), test (debug + release), doc, cargo-audit, cargo-deny, bench placeholder, fuzz placeholder.
  - build `--all-features`.
- Fournir script `release.md` : tag, changelog, cargo publish checklist.
- Cargo.toml : déclarer features additives (serde optional), pas de features exclusives sans garde.

SORTIE ATTENDUE (format attendu de la réponse)
1. Arborescence du projet (`src/`, `examples/`, `tests/`, `benches/`, `fuzz/`, `docs/`).
2. `Cargo.toml` complet (incluant `[package.metadata] msrv = "{{msrv}}"`).
3. Code minimal compilable (lib.rs ou main.rs) suivant toutes les contraintes (no unsafe public, errors via thiserror/anyhow).
4. Types d’erreur, gestion d’erreur idiomatique (thiserror pour libs, anyhow pour bin).
5. Exemples et doctests compilables.
6. Tests unitaires + un test d’intégration + un property test d’exemple.
7. `.github/workflows/ci.yml` configuré pour les checks listés.
8. README court + checklist de conformité (point-par-point : OK / TODO).
9. CHANGELOG.md initial et LICENSE ({{license}} par défaut, à confirmer).
10. Indications pour fuzzing/benchmarks + commandes pour reproduire mesures.
11. **Description minimale** : phrase ≤30 mots + court paragraphe ≤40 mots.
12. Liste des références citées (Rust API Guidelines, Clippy, ANSSI secure Rust, Rust Performance Book).

CONTRAINTES SUPPLÉMENTAIRES
- Aucune vulnérabilité connue incluse (attention aux regex, allocations non bornées, dépendances non auditées).
- Exemples/doctests doivent compiler avec `cargo test`.
- Pas d’`unsafe` public ; si `unsafe` interne, fournir checklist de revue pour ce bloc.
- Organisation cible : {{org_name}}.

RÉFÉRENCES À INCLURE AUTOMATIQUEMENT
- Rust API Guidelines — https://rust-lang.github.io/api-guidelines/
- Clippy Lints — https://rust-lang.github.io/rust-clippy/master/index.html
- ANSSI Secure Rust Guide — https://anssi-fr.github.io/rust-guide/
- Rust Performance Book — https://nnethercote.github.io/perf-book/
- Miri / Users forum pointers pour revue unsafe.
