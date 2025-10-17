# Makefile reproduisant .github/workflows/ci.yml
# Usage:
#   make ci                 # full matrix + docs + security checks
#   make fmt-check          # vérifie le format (par défaut toolchain stable)
#   make clippy             # exécute clippy (par défaut toolchain stable)
#   make test               # tests debug (matrix)
#   make test-release       # tests release (matrix)
#   make doc                # docs (stable)
#   make audit              # cargo-audit (stable, installe si absent)
#   make deny               # cargo-deny (stable, installe si absent)
#   make install-tools      # installe cargo-audit et cargo-deny si nécessaire
#
# Pour utiliser une toolchain spécifique pour une commande unique:
#   make fmt-check TOOLCHAIN=1.90.0

# configurable
TOOLCHAINS ?= stable 1.90.0
MSRV ?= 1.90.0
RUSTUP := rustup
CARGO := cargo

.PHONY: help ci fmt-check clippy test test-release doc audit deny install-tools

help:
	@echo "Makefile targets: ci, fmt-check, clippy, test, test-release, doc, audit, deny, install-tools"
	@echo "Default toolchains: $(TOOLCHAINS)"
	@echo "Run with TOOLCHAIN=<toolchain> to override for single-target commands."

# --- per-toolchain check helpers (single toolchain, default = stable) ---
TOOLCHAIN ?= stable

fmt-check:
	@echo ">>> fmt --check (toolchain: $(TOOLCHAIN))"
	@$(RUSTUP) run $(TOOLCHAIN) $(CARGO) fmt -- --check

clippy:
	@echo ">>> clippy -D warnings (toolchain: $(TOOLCHAIN))"
	@$(RUSTUP) run $(TOOLCHAIN) $(CARGO) clippy --all-targets -- -D warnings

test:
	@echo ">>> tests (matrix: $(TOOLCHAINS))"
	@for tc in $(TOOLCHAINS); do \
	  echo "=== running tests on $$tc ==="; \
	  $(RUSTUP) run $$tc $(CARGO) test --all-features --tests || exit 1; \
	done

test-release:
	@echo ">>> tests (release, matrix: $(TOOLCHAINS))"
	@for tc in $(TOOLCHAINS); do \
	  echo "=== running release tests on $$tc ==="; \
	  $(RUSTUP) run $$tc $(CARGO) test --release --all-features --tests || exit 1; \
	done

doc:
	@echo ">>> cargo doc --no-deps (stable)"
	@$(RUSTUP) run stable $(CARGO) doc --no-deps

# --- security / policy checks (stable) ---
install-tools:
	@echo ">>> ensuring cargo-audit and cargo-deny are installed"
	@command -v cargo-audit >/dev/null 2>&1 || (echo "installing cargo-audit..." && $(CARGO) install cargo-audit --locked)
	@command -v cargo-deny >/dev/null 2>&1 || (echo "installing cargo-deny..." && $(CARGO) install cargo-deny --locked)

audit: install-tools
	@echo ">>> cargo-audit (stable) -- non-fatal"
	@$(RUSTUP) run stable $(CARGO) audit || true

deny: install-tools
	@echo ">>> cargo-deny check (stable) -- non-fatal"
	@$(RUSTUP) run stable $(CARGO) deny check || true

# --- full CI flow: run matrix checks, docs, security checks ---
ci: fmt-check-matrix clippy-matrix test test-release doc audit deny
	@echo ">>> CI finished"

fmt-check-matrix:
	@echo ">>> fmt --check on matrix: $(TOOLCHAINS)"
	@for tc in $(TOOLCHAINS); do \
	  echo "=== fmt check $$tc ==="; \
	  $(RUSTUP) run $$tc $(CARGO) fmt -- --check || exit 1; \
	done

clippy-matrix:
	@echo ">>> clippy -D warnings on matrix: $(TOOLCHAINS)"
	@for tc in $(TOOLCHAINS); do \
	  echo "=== clippy $$tc ==="; \
	  $(RUSTUP) run $$tc $(CARGO) clippy --all-targets -- -D warnings || exit 1; \
	done
