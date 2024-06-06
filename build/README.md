Irox Builder
==============

*The IROX buildsystem*

```
Usage: irox-builder [COMMAND]

Commands:
  default     Runs: Updates, Build, Test, Format, Lints, Upgrade
  ci          Runs: Updates, Deny, Build, Test, Format-Check, Lints-Deny, About, Doc, Upgrade
  updates     Updates Rust with `rustup upadate` and then updates the 'Cargo.toml' file using `cargo update`
  build       Iterates through multiple feature sets, default, all, none, etc, calling `cargo build`
  test        Like Build, but runs `cargo test`
  format      Runs `rustfmt`
  lints       Runs `cargo clippy`
  upgrade     Runs `cargo upgrade`
  deny        Runs `cargo deny`
  about       Runs `cargo about`
  doc         Runs `cargo doc`
  check       Runs `cargo check` for all targets
  release     Sets up for a release
  new         Creates a new module
  build-perf  Runs a performance check for how long the build takes
  package     Runs a `cargo package` check
  unused      Checks for unused modules & deps that could be removed
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

### ...but why?

<img src="./doc/but-why.gif"/>

I originally used [just](https://just.systems) and it was awesome - but trying to make the scripts work reliably on both
msys2/bash, linux/bash, and powershell was just untenable.

