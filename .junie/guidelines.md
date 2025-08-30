Project development guidelines (IROX workspace)

Audience: Advanced Rust developers working on this workspace of crates. The notes below focus on project-specific setup, build, testing, linting, and conventions observed in this repo.

Build and configuration
- Toolchain
  - The workspace pins Rust via rust-toolchain.toml: channel 1.85, profile minimal, components rustfmt and clippy, with targets: wasm32-unknown-unknown, x86_64-pc-windows-msvc, x86_64-unknown-linux-gnu.
  - Use rustup to install and respect the pinned toolchain: rustup show (will auto-install on first build) or rustup toolchain install 1.85.
  - Clippy and rustfmt are required (CI expects them). 
- Workspace layout
  - Cargo workspace with many member crates under libraries/, data-formats/, interfaces/, eieio/, tools/, and the irox aggregator crate.
  - Many crates support no_std and/or alloc-gated functionality. Defaults typically include the std feature; some helpers are gated behind explicit alloc.
- Platform/targets
  - Some crates are Windows-specific (e.g., interfaces/win-location-api uses windows crate and Windows.Devices.Geolocation APIs). Build on Windows MSVC target when working there.
  - wasm32-unknown-unknown is listed; UI-related crates (egui) are in workspace dependencies but default features are disabled in workspace dependency declarations and enabled per-crate as needed.
- Building
  - Build entire workspace: cargo build
  - Build a specific crate: cargo build -p <crate-name>
  - Example: cargo build -p irox-tools
  - Release builds: cargo build --release (or -p <crate>)
  - The justfile includes a simple run recipe for a builder binary if you use just: just run -- <params> (optional; not central to most crates).
- Features
  - Many crates use cfg_attr(not(feature = "std"), no_std). Functions that allocate are under a cfg_feature_alloc macro. If you need alloc-only APIs without std, enable the alloc feature explicitly for that crate in your consuming crate.
  - In tests and most binaries, std is enabled by default, which also implies alloc is available, but note: alloc-gated functions in irox-tools are not exported unless the alloc feature is enabled for that crate. Prefer the StrBuf-based APIs when you need to avoid alloc.

Testing
- Running tests
  - All workspace tests: cargo test
  - Per-crate: cargo test -p irox-tools (replace with desired crate)
  - Per binary/integration test file: cargo test -p irox-tools --test <name>
  - Quiet mode (useful in CI): cargo test -q
- Doctests
  - By default doctests are run with cargo test. Disable with --lib --tests if necessary.
- Targeted testing and features
  - Some crates expose alloc- or std-gated APIs. If you need to test those without std, run with appropriate features on the specific crate being tested.
  - Example (if a crate defines an "alloc" feature explicitly): cargo test -p irox-tools --features alloc
- Adding tests
  - Unit tests: place module-level tests under src in a #[cfg(test)] mod tests block.
  - Integration tests: place files under <crate>/tests/. Each file compiles as a separate crate and imports the crate as an external dependency.
  - Use existing internal infrastructure: many utilities expose no_std-friendly traits (MutBits, WriteToBEBits, StrBuf, etc.). Prefer APIs that do not require alloc when working in no_std contexts.
  - Strict lints: unsafe_code is forbidden at workspace level. Tests must compile cleanly under the configured clippy lint set (see below). Address or allow with justification at the narrowest scope.
- Demo: creating and running a simple integration test (verified)
  - We verified the following integration test in libraries/tools/tests/demo_guidelines_test.rs and executed it with: cargo test -p irox-tools --test demo_guidelines_test (it passed). The file was then removed to keep the repo clean as per the task instructions.
    
    use irox_tools::hex::{from_hex_into, to_hex_strbuf_upper};
    use irox_tools::buf::StrBuf;
    
    #[test]
    fn demo_hex_roundtrip() -> Result<(), irox_bits::Error> {
        // Input hex string (spaces permitted by from_hex_into)
        let input = "de ad be ef";
        
        // Convert hex -> bytes into a fixed buffer; &mut [u8] implements MutBits
        let mut buf = [0u8; 4];
        let wrote = from_hex_into(input, &mut &mut buf[..])?;
        assert_eq!(wrote, 4);
        assert_eq!(buf, [0xDE, 0xAD, 0xBE, 0xEF]);
        
        // Convert bytes -> UPPER hex string into a StrBuf
        let mut s: StrBuf<8> = StrBuf::new();
        to_hex_strbuf_upper(&buf, &mut s)?;
        assert_eq!(s.as_str(), Ok("DEADBEEF"));
        Ok(())
    }
    
  - Notes specific to this workspace:
    - to_hex_str_upper allocates and is gated behind an alloc feature in irox-tools; prefer to_hex_strbuf_upper with StrBuf when writing portable tests that work regardless of feature flags.
    - from_hex_into expects a MutBits target; &mut [u8] implements MutBits so you can pass &mut &mut buf[..] directly.

Linting, quality gates, and CI expectations
- Lints
  - Workspace lints configured in Cargo.toml:
    - rust: unsafe_code = "forbid" (no unsafe)
    - rustdoc: warn on broken or invalid doc links/blocks; crate-level docs may be missing (allowed).
    - clippy: extensive warnings enabled (see [workspace.lints.clippy]). Highlights include unwrap_used, panic, indexing_slicing, large_types_passed_by_value, etc.
  - Guidance:
    - Avoid unwrap/expect in library code and tests; propagate errors with Result or use .ok_or_else/.context-style patterns. Where a test intentionally unwraps, prefer irox-tools assert helpers or explicit error messages.
    - Avoid println!/eprintln! in libraries; use logging from irox-log or the provided macros where applicable; clippy warns on print_stdout/print_stderr.
    - Keep allocations explicit; prefer stack-based buffers and StrBuf where available.
- Formatting and edition
  - Edition 2021. Run cargo fmt --all to format. rustfmt component is pinned by the toolchain.
- Dependency policy
  - cargo-deny is configured (deny.toml) with all-features = true for metadata collection. Run cargo deny check to audit advisories, bans, sources, and licenses.
  - One advisory is temporarily ignored (RUSTSEC-2024-0436 for paste) with a rationale; update dependencies when feasible.
- Benchmarks
  - criterion is in workspace dependencies; use per-crate benches/ if benchmarking. Not all crates include benches by default.

Crate-specific notes and patterns
- irox-tools
  - Aggregates a variety of utilities (ansi_colors, arrays, fmt, hex, iterators, options, random, read/map/sync when std, packetio/vec/str when alloc, errors, fs, hash, buf, macros, math, primitives, util).
  - StrBuf and FixedU8Buf provide fixed-capacity, no-alloc buffers useful in no_std contexts and for deterministic behavior in tests.
  - hex APIs: prefer from_hex_into + StrBuf output writers in generic code; use HexDump trait for debugging (hexdump() gated on std).
- irox-bits
  - Provides MutBits and related traits used across the workspace; many encode/decode helpers operate on these traits; leverage them in IO-less tests to avoid std requirements.
- Windows APIs
  - interfaces/win-location-api depends on windows crate features Devices_Geolocation and Foundation; to build/run anything that uses this, use the MSVC toolchain on Windows.

Release and packaging
- The workspace uses version = 0.1.0 at the workspace.package level; individual crates may be published independently to crates.io.
- homepage/repository point to GitHub; docs.rs badges present. Follow semver and conventional commits as indicated by README badges.

Troubleshooting
- If a test fails due to missing alloc-gated functions (e.g., to_hex_str_upper), switch to the StrBuf-based API or enable alloc feature for the crate under test.
- If cargo-deny fails with network or auth issues, you can set advisories.git-fetch-with-cli = true and ensure git is configured, or run with --no-fetch.
- For platform-specific crates, limit builds/tests with -p to avoid cross-target compilation errors on your platform.
