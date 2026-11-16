# Working in IROX

This file guides AI agents working anywhere in this repository. Follow more
specific instructions in a descendant directory when applicable, and follow the
user's current instructions over this guidance. Keep this document useful without
requiring access to a past chat or a private wiki. Update facts when the code changes.

## Keep this guide useful

- After completing work, consider whether you learned something verified,
  durable, and repository-specific that would prevent future mistakes. Add or
  correct that guidance here when warranted; an update is not required for every task.
- Revise the relevant existing guidance before adding another rule. Keep entries
  concise, explain when they apply, and reference authoritative code or documents
  where useful. Distinguish observed facts from recommendations and assumptions.
- Remove obsolete advice and consolidate duplicates. A lesson from one task is
  not automatically a repository-wide requirement; put narrowly applicable
  guidance in the relevant crate's documentation or a scoped `AGENTS.md`.
- Keep task chronology, command transcripts, temporary failures, personal data,
  and unverified guesses out of this file. Do not turn it into a task diary or
  preserve stale check results as permanent facts.
- Carry other knowledge forward in the artifact closest to its use: API contracts
  and invariants in rustdoc or code comments, usage in READMEs/examples, regressions
  in tests/fixtures, design tradeoffs in versioned design notes, and operational
  procedures in troubleshooting documentation. Use issues or PR descriptions for
  unresolved work and task-specific evidence. Include reproduction steps, relevant
  versions, sources, and uncertainty where needed; link rather than duplicate.
- Use existing documentation locations before inventing a new knowledge system.
  Keep private or sensitive material in an appropriate access-controlled location.
  Do not rely on chat history or an agent's private memory as the only record of
  information future contributors need.

## Start with the problem and the existing system

- Understand the user's intended result and exit criteria. Make reasonable,
  reversible implementation decisions and complete the work. Ask about ambiguity
  when the answer materially changes correctness or scope; do useful independent
  work while waiting. Avoid repeated confirmation for work already authorized.
- Read `git status --short` before editing. Preserve existing and concurrent work;
  a dirty file is not automatically yours. Review the final diff and untracked
  files. Never discard another contributor's changes to obtain a clean check.
- Read the relevant crate manifest, neighboring implementations, examples, and
  tests before designing. Search with `rg` and `rg --files`. Check the installed
  dependency source when an API is uncertain instead of guessing from memory.
- Identify the interfaces: types, units, coordinate/reference frames, valid ranges,
  precision, ownership, timing, failure behavior, and platform/feature constraints.
  Record material assumptions in code documentation or the change description.
- Prefer a small working solution built from existing components. Minimize new
  dependencies and interactions, not merely line count. A user's constraint such
  as “no additional external dependencies” is an acceptance criterion, including
  for examples. Check workspace dependencies and internal crates first.
- Explain consequential tradeoffs and challenge an approach when evidence warrants
  it. Correct mistaken assumptions promptly. “Temporary” code should still meet
  the requirements; avoid speculative frameworks and unrelated cleanup.

## Repository map and Rust conventions

- `libraries/`: reusable crates. Useful existing building blocks include
  `bits` (byte/bit streams), `tools` (general utilities), `units` (quantities and
  reference frames), `geometry`, `imagery`, `compression`, `carto`, `threading`,
  and `egui_extras` (IROX's egui extensions).
- `data-formats/`: format readers/writers and codecs.
- `interfaces/`: integrations and external APIs/transports; `eieio/`: I/O crates.
- `tools/`: executable applications. `irox/`: aggregator crate.
- `build/`: the Rust build orchestrator, with its own manifest. The root
  `justfile` builds it in release mode and forwards arguments to it.
- Prefer manifest inheritance (`workspace = true`) for dependencies, package
  metadata, and lints. Check the actual Cargo package name: directory names are
  not always package names (`libraries/network` is `irox-networking`).
- Preserve each crate's `no_std`, `alloc`, and `std` boundaries. Prefer `core`
  and `alloc` where supported; gate filesystem, networking, threading, and GUI
  functionality appropriately. Do not turn on default features globally to make
  a local example compile. Native and wasm have different capabilities.
- Unsafe code is forbidden by the workspace lints. Follow the surrounding SPDX
  and copyright header convention. Follow local error types and `Result`-based
  propagation; avoid panicking on malformed external input or silently claiming
  success after a required operation fails.
- Read `[workspace.lints]` in `Cargo.toml`, `.clippy.toml`, and crate-level lint
  attributes. In particular, named generic parameters are preferred over
  argument-position `impl Trait`; logging is preferred over production
  `println!`/`eprintln!`; non-ASCII string literals use escapes. Avoid blanket
  lint suppression. Existing test allowances are not production allowances.
- Use the pinned `rust-toolchain.toml`; do not infer an MSRV from it or upgrade
  the toolchain as a side effect. Keep public API changes and required feature
  combinations documented. Read `CONTRIBUTING.md` before preparing commits: it
  requires Conventional Commits, DCO sign-off, and GPG signatures. Do not invent
  identity or bypass signing requirements.

## Build and verification

Run commands from the repository root. Treat `build/src/main.rs` as the source
of truth: command help and `build/README.md` can lag the implementation.

| Command | Purpose and important behavior |
| --- | --- |
| `just run qc` | Canonical quick checks: strict Clippy feature matrix, workspace formatting check, then dependency policy checks via cargo-deny. Does not run tests. |
| `just run lints` | Clippy matrix without the quick-check task's explicit `-Dwarnings`. |
| `just run fmt` | Formats the workspace; review formatting changes in already modified files. |
| `just run test` | All-target tests across the feature matrix with the builder's optimized/native CPU `RUSTFLAGS`. |
| `just run build` | All-target builds across the feature matrix. |
| `just run check` | Cleans build artifacts, then checks discovered per-package features, including wasm. May install helper tooling. |
| `just run ci` | Broader checks, tests, strict lints, formatting, license report, docs, upgrade dry-run, and dependency policy checks. Includes cleaning/tool installation through component tasks. |
| `just run deny` | Installs/checks cargo-deny, attempts advisory refresh, then runs offline policy checks. Can need network access. |
| `just run doc` | Builds Rust documentation. |

The lint matrix selects binaries, libraries, and examples separately, each with
`--no-default-features`, `-Falloc`, `-Fstd`, `-Falloc,std`, `-Fdefault`, and
`--all-features`. A default-feature package check alone does not cover it.

Do not use bare `just run` as a harmless check: its default task updates the
Rust toolchain and Cargo lockfile, builds/tests/formats/lints, and checks upgrades.
Do not invoke release/publishing tasks for routine validation.

For development, start with focused commands, for example:

```sh
cargo test -p irox-egui-extras --lib --example slippy_map --features eframe,glow
cargo clippy -p irox-egui-extras --lib --example slippy_map --features eframe,glow -- -D warnings
cargo fmt --all -- --check
```

- Use `just run qc` for full workspace quick checks before handing off code
  changes, and run tests appropriate to the behavior changed. For documentation
  alone, review accuracy, commands, paths, and links; avoid expensive builds that
  cannot provide relevant evidence.
- Add `--offline` to direct Cargo commands when useful and dependencies are
  cached. Tool downloads and advisory refreshes may still need network access in
  builder tasks. Respect sandbox permissions; do not bypass a required approval.
- Avoid concurrent Cargo jobs sharing a target directory: build locks serialize
  them anyway. Wait for long-running commands to finish and inspect exit status.
- Report which stages passed and which failed. A dependency audit failure after
  successful Clippy/formatting is still a failed `qc`; neither call it all green
  nor assume it is a regression in the patch. Re-evaluate current diagnostics,
  identify the dependency chain, and keep unrelated upgrades or policy changes
  separate. Never add an advisory ignore merely to obtain a passing check.

## Test the contract, including the seams

- Choose tests that demonstrate observable requirements, regressions, or boundary
  behavior. A compilation success or an `is_ok()` decoder assertion does not
  establish that the displayed image is correct. Use representative real-format
  fixtures and check dimensions and meaningful pixel values where relevant.
- Test malformed/truncated input and unsupported format variants when changing
  parsers. Do not assume an internal implementation handles every variant of a
  named standard because a simple fixture passes. Document limitations explicitly.
- Test asynchronous pending, ready, and failure states, partial completion,
  out-of-order results, and transitions. Use controlled fake loaders instead of
  live services for deterministic tests. Check behavior, not implementation shape.
- Bound work, memory, caches, and requests with the expected operating conditions
  in mind. A large coordinate or zoom change must not accidentally enumerate an
  entire world grid. Consider stale callbacks and resource lifetime after eviction.
- Distinguish automated checks, a runnable example, and actual manual GUI
  verification. Say when a graphical or live-network path was not exercised.

## UI and integration work

- Make coordinate spaces and units explicit; validate conversions at fractional
  and high-DPI scales as well as the default scale.
- Test visible behavior through asynchronous transitions, including partial
  completion and errors. Preserve useful output while replacements load.
- Supply runnable integration examples with loader setup, feature requirements,
  and operational constraints. Keep implementation-specific details in the crate.
  See [the slippy-map integration notes](libraries/egui_extras/doc/slippy_map.md)
  for the map's contracts, limitations, and regression coverage.

## Engineering judgment and durable handoff

The following is an adaptation of Sean's wiki guidance, not a verbatim policy
import. Its collections contain different authors' heuristics, not universally
applicable instructions. Apply them with judgment and within the user's scope.

- Know the customer and measure success in their terms. Define what “works” means
  before polishing the implementation. Record why a design meets the need, not
  only how it is implemented.
- Expect human and agent errors. Validate input and output, test assumptions,
  correct errors, and revise the model when requirements or evidence change.
- Treat interfaces as the first place to look for failures. Track units, data,
  resource ownership, and failure signals across the entire path.
- Choose understandable, explicit designs. Reuse familiar components when they
  fit; account for the cost of dependencies and integration. Optimize against
  measured constraints rather than unsupported performance claims.
- Work deliberately, then iterate. Make progress with available information,
  identify consequential unknowns, and verify estimates. Avoid both impulsive
  changes and analysis paralysis. Take ownership of resolving routine problems.
- Leave knowledge in versioned, accessible artifacts: decisions, operating
  assumptions, examples, test commands, and limitations. Favor open formats and
  reproducible outputs. Reference sources without copying unrelated private wiki
  material into this repository.
- A handoff should explain the resulting behavior, why the approach fits, evidence
  of correctness, remaining risks, and how to run or maintain it. Keep the final
  user response concise and precise about unverified claims.

Sources consulted for this guidance are publicly available at [spm.wiki](https://spm.wiki/)
without authentication. The original review used content from wiki repository
revision `79f4ca83ce7fa80dda2ad5740510bf32661c3b9e`:

- [SPM's Rules of Engagement](https://spm.wiki/laws/spms-roes)
- [SPM's Laws of Engineering](https://spm.wiki/laws/spms-laws-of-eng)
- [System Requirements](https://spm.wiki/laws/spms-take-on-requirements)
- [Engineering Tool Guidelines](https://spm.wiki/laws/eng-tool-reqs)
- [Review Guidelines](https://spm.wiki/laws/reviews)
- [Laws & Guidelines index](https://spm.wiki/laws), particularly the software/systems engineering, spacecraft design, and infosec collections.
