# AGENTS.md

Guidance for AI coding agents (Copilot, Claude, Cursor, etc.) contributing to
`embedded-usb-pd`. Human contributors should also find this useful, but this
file is written assuming the reader is an automated agent that can run shell
commands and edit files in this repository.

## 1. What this crate is

`embedded-usb-pd` is a `no_std` Rust library that provides shared types for USB
Power Delivery (USB-PD) and UCSI. It is a *types crate*: it defines
bitfields, enums, structured/unstructured VDMs, PDOs, ADO, UCSI command
encodings, and common error/role/port-id types that other crates (PD
controllers, UCSI providers, embedded services) build on top of.

Key shape:

- Single crate, edition 2021, MSRV **1.85.0** (see `.github/workflows/check.yml`).
- `#![no_std]` (`src/lib.rs:1`). Never add `std`-only code or dependencies.
- Public module surface declared in `src/lib.rs`: `ado`, `constants`, `pdinfo`,
  `pdo`, `type_c`, `ucsi`, `usb`, `vdm`.
- Single optional feature: `defmt` (gates `defmt::Format` derives via
  `#[cfg_attr(feature = "defmt", derive(defmt::Format))]`).
- Strict clippy lints denied at the crate level in `Cargo.toml`:
  `correctness`, `expect_used`, `indexing_slicing`, `panic`,
  `panic_in_result_fn`, `perf`, `suspicious`, `style`, `todo`,
  `unimplemented`, `unreachable`, `unwrap_used`.

This means: **no `unwrap()`, no `expect()`, no `panic!`, no `todo!`, no
`unreachable!`, no `unimplemented!`, no `arr[i]` indexing in non-test code.**
Use `get`, `?`, and return `Result<_, PdError>` / `Result<_, Error<BE>>`.

## 2. Repository layout

```
src/
  lib.rs        # crate root: PortId trait, LocalPortId, GlobalPortId,
                # PdError, Error<BE>, PowerRole, DataRole, PlugOrientation
  ado.rs        # Alert Data Object
  constants.rs  # PD spec constants
  pdinfo.rs     # PD info types
  pdo/          # Power Data Objects (source/sink, fixed/variable/battery/PPS)
  type_c.rs     # Type-C state / roles
  ucsi/         # UCSI command/response encodings
  usb.rs        # USB-level helpers
  vdm/          # Vendor Defined Messages (structured + unstructured)
.github/
  copilot-instructions.md  # PR-review-specific guidance for Copilot
  workflows/               # CI: check.yml, nostd.yml, cargo-vet*.yml
docs/                      # design notes / diagrams (aquamarine)
supply-chain/              # cargo-vet audits
deny.toml                  # cargo-deny config
rustfmt.toml               # nightly-only options (see §4)
```

There is **no Cargo workspace**: one `Cargo.toml`, one crate. All paths in
this file are relative to the repository root.

## 3. Setup

Required toolchains:

- **stable** Rust (currently builds on 1.95+, MSRV is 1.85.0).
- **nightly** Rust — *required* for `cargo fmt`. `rustfmt.toml` uses unstable
  options (`group_imports = "StdExternalCrate"`, `imports_granularity = "Module"`)
  that error on stable.
- Target `thumbv8m.main-none-eabihf` for the `no-std` CI job:
  `rustup target add thumbv8m.main-none-eabihf`.

Optional tools used by CI (install on demand):

- `cargo-hack` — feature-powerset checks.
- `cargo-deny` — license/advisory/source checks against `deny.toml`.
- `cargo-vet` — supply-chain audits in `supply-chain/`.

```bash
rustup toolchain install stable nightly
rustup target add thumbv8m.main-none-eabihf
cargo install cargo-hack cargo-deny cargo-vet
```

## 4. The CI commands you must run before pushing

These mirror `.github/workflows/check.yml` and `.github/workflows/nostd.yml`.
All have been verified to pass on a clean checkout of `main`. Run them in
this order and fix anything that fails before opening a PR.

```bash
# 1. Formatting — NIGHTLY ONLY. Will warn-and-skip on stable.
cargo +nightly fmt --check

# 2. Build + clippy across every feature combination, deny warnings.
#    `log` is listed as mutually exclusive even though it isn't currently a
#    feature, to match CI exactly and stay forward-compatible.
cargo hack --feature-powerset --mutually-exclusive-features=log,defmt \
    clippy --locked -- -Dwarnings

# 3. Plain feature-powerset check (no clippy, no -Dwarnings).
cargo hack --feature-powerset check

# 4. Tests across all feature combinations EXCEPT defmt
#    (defmt doesn't link in a host test binary).
cargo hack --feature-powerset --exclude-features defmt \
    test --all-targets --locked

# 5. Test-code clippy with lints capped (tests legitimately panic/unwrap).
cargo clippy --locked --tests -- --cap-lints allow

# 6. Docs build with all features, treating cfg(docsrs) as set.
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features

# 7. no_std build for an embedded target.
cargo check --target thumbv8m.main-none-eabihf --no-default-features

# 8. MSRV check (only needed if you touched dependencies or used new syntax).
rustup toolchain install 1.85.0
cargo +1.85.0 check
```

Additional checks CI runs that you usually don't need locally but should be
aware of:

- `cargo deny --all-features check` — licenses, advisories, bans, sources.
- `cargo vet` — supply-chain review (`supply-chain/audits.toml`).

## 5. Coding conventions

**Error handling.** Every fallible API returns `Result<_, PdError>` or
`Result<_, Error<BE>>` where `BE` is the bus error of the embedded-hal-async
transport. `From<PdError> for Error<BE>` and
`From<PdError> for Result<T, Error<BE>>` are provided in `src/lib.rs`. Prefer
returning `PdError` variants over inventing new error enums; add a new variant
to `PdError` only when no existing one fits the failure mode.

**No panicking constructs.** The crate-level clippy config denies `unwrap`,
`expect`, `panic`, `todo`, `unimplemented`, `unreachable`,
`panic_in_result_fn`, and `indexing_slicing`. Use:

- `slice.get(i).ok_or(PdError::InvalidParams)?` instead of `slice[i]`.
- `?` propagation instead of `.unwrap()`.
- `match`/`if let` exhaustively instead of `unreachable!()`.

Test modules (`#[cfg(test)] mod tests`) may panic and index freely — CI caps
their lints with `--cap-lints allow` (see CI step 5).

**`defmt` gating.** Every type that derives `Debug`, `Copy`, `Clone`, etc.
and is part of the public surface should also conditionally derive
`defmt::Format`:

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Foo(pub u8);
```

For traits with `defmt` bounds, follow the pattern in `src/lib.rs:19-31`:
define two `#[cfg]`-gated copies of the trait, one with the `defmt::Format`
supertrait and one without. **Never** add `defmt` as a non-optional
dependency.

**Imports.** `rustfmt.toml` enforces `group_imports = "StdExternalCrate"` and
`imports_granularity = "Module"`. Always run `cargo +nightly fmt` before
committing.

**Port IDs.** Use `LocalPortId` for per-controller port indices and
`GlobalPortId` for system-wide unique IDs. Both implement the `PortId` trait;
write generic code over `P: PortId` rather than `u8`.

**Bitfields.** This crate uses the `bitfield` crate (0.19) extensively, plus
`bincode` (no_std, no default features) for wire encoding. Follow the
existing patterns in `src/vdm/` and `src/pdo/` when adding new structured
messages — define the bitfield, derive the standard set of traits, add a
`#[cfg_attr(feature = "defmt", derive(defmt::Format))]`, and add unit tests
covering round-trip encode/decode and the boundary values of each field.

## 6. Testing expectations

Unit tests live next to the code (`#[cfg(test)] mod tests { ... }` blocks).
A clean `cargo test --locked` currently passes **169 tests**. When adding a
new public type:

- Round-trip test (`encode → decode → equals original`) if it has a wire form.
- Boundary tests for every bitfield (min, max, invalid).
- A `try_from`/`from_bits` test that exercises the reserved/invalid path.

## 7. Commit & PR workflow

Follow `CONTRIBUTING.md`:

- **Draft PR first.** Open as draft, wait for the `.github` workflows to go
  green, *then* request review.
- **Clean commit history.** Squashing is disabled on merge. Each commit must
  build and pass CI on its own; squash typo/format fixups into the parent
  commit (`git rebase -i`) before pushing the final version.
- **Meaningful messages.** Imperative subject line under ~72 chars, body
  explaining *why*. See <http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html>.
- **Regressions:** if reporting one, include the output of `git bisect`.

When an AI agent makes the change, add a trailer identifying the assistant
(in addition to any human `Co-authored-by` lines), e.g.:

```
Assisted-by: GitHub Copilot:claude-opus-4.7
```

Do **not** force-push to shared branches. Never set git identity globally
inside this repo — pass it per-commit:

```bash
git -c user.name="Your Name" -c user.email="you@example.com" commit -m "..."
```

## 8. Things to *not* do

- Don't add `std` or `alloc` dependencies. This is `no_std` and must stay
  that way (`nostd.yml` enforces it).
- Don't bump MSRV without updating `.github/workflows/check.yml` `msrv` matrix
  and explaining why in the commit message.
- Don't introduce new top-level features without updating the
  `--mutually-exclusive-features` list in CI if they conflict with `defmt`.
- Don't bypass the clippy lints with `#[allow(...)]` on the crate or module
  level. Local `#[allow(...)]` on a single item is acceptable with a comment
  explaining why.
- Don't edit `supply-chain/audits.toml` by hand; use `cargo vet`.
- Don't add files under `target/` or `**/*.rs.bk` (already gitignored).
- Don't open a PR before `cargo +nightly fmt --check`, the
  `cargo hack ... clippy ... -Dwarnings` powerset, and `cargo hack ... test`
  all pass locally.

## 9. Pointers

- Existing Copilot review instructions: `.github/copilot-instructions.md`.
- Contribution policy: `CONTRIBUTING.md`.
- Security policy: `SECURITY.md`.
- Code of Conduct: `CODE_OF_CONDUCT.md`.
- License: `LICENSE` (MIT).
