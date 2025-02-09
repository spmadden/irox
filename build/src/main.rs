// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

mod error;
mod utils;

pub use utils::*;

use crate::error::Error;
use clap::{Parser, Subcommand};

const FEATURE_ARGS: &[&str] = &[
    "--no-default-features",
    "-Falloc",
    "-Fstd",
    "-Falloc,std",
    "-Fdefault",
    "--all-features",
];
const TARGETS: &[&str] = &[
    "x86_64-pc-windows-msvc",
    // "wasm32-unknown-unknown",
    //"x86_64-unknown-linux-gnu",
];

#[derive(Debug, Clone, Default, Subcommand)]
enum Commands {
    #[default]
    /// Runs: Updates, Build, Test, Format, Lints, Upgrade
    Default,
    /// Runs: Updates, Deny, Build, Test, Format-Check, Lints-Deny, About, Doc, Upgrade
    CI,
    /// Updates Rust with `rustup upadate` and then updates the 'Cargo.toml'
    /// file using `cargo update`
    #[clap(visible_alias("update"))]
    Updates,
    /// Iterates through multiple feature sets, default, all, none, etc, calling
    /// `cargo build`
    Build,
    /// Like Build, but runs `cargo test`
    #[clap(visible_alias("tests"))]
    Test,
    /// Runs `rustfmt`
    #[clap(visible_alias("fmt"))]
    Format,
    /// Runs `cargo clippy`
    #[clap(visible_alias("clippy"))]
    Lints,
    /// Runs `cargo upgrade`
    #[clap(visible_alias("upgrades"))]
    Upgrade,
    /// Runs `cargo deny`
    Deny,
    /// Runs `cargo about`
    About,
    /// Runs `cargo doc`
    #[clap(visible_alias("docs"))]
    Doc,
    /// Runs `cargo check` for all targets
    #[clap(visible_alias("checks"))]
    Check,
    /// Sets up for a release
    Release {
        /// Additional arguments
        args: Vec<String>,
    },
    /// Creates a new module
    New {
        /// Destination path of the new module
        dest: String,
    },
    /// Runs a performance check for how long the build takes
    BuildPerf,
    /// Runs a `cargo package` check
    Package,
    /// Checks for unused modules & deps that could be removed.
    Unused,
    /// Runs quick checks, `ci` without the actual builds/checks
    #[clap(visible_alias("qc"))]
    QuickChecks,
    /// Runs an optimized x86_64 release build
    #[clap(visible_alias("rb"))]
    ReleaseBuild,

    /// Runs cargo-bloat to check what's bloating your binary
    #[clap(visible_alias("bc"))]
    BloatCheck { args: Vec<String> },
}

#[derive(Debug, Default, Parser)]
struct Args {
    #[command(subcommand)]
    commands: Option<Commands>,
}

fn main() -> Result<(), Error> {
    irox_log::init_console_from_env("RUST_LOG");
    let args = Args::parse();

    match args.commands.unwrap_or_default() {
        Commands::Default => default()?,
        Commands::CI => ci()?,
        Commands::Updates => updates()?,
        Commands::Build => build()?,
        Commands::Test => test()?,
        Commands::Format => format()?,
        Commands::Lints => lints()?,
        Commands::Upgrade => upgrade()?,
        Commands::Deny => deny()?,
        Commands::About => about()?,
        Commands::Doc => doc()?,
        Commands::Check => check_all()?,
        Commands::Release { args } => release(
            args.iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .as_slice(),
        )?,
        Commands::New { dest } => new(&dest)?,
        Commands::BuildPerf => buildperf()?,
        Commands::Package => package()?,
        Commands::Unused => unused()?,
        Commands::QuickChecks => quick_checks()?,
        Commands::ReleaseBuild => rbuild()?,
        Commands::BloatCheck { args } => {
            bloat(
                args.iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .as_slice(),
            )?;
        }
    }
    Ok(())
}

fn default() -> Result<(), Error> {
    updates()?;
    build()?;
    test()?;
    format()?;
    lints()?;
    upgrade()?;
    Ok(())
}

fn ci() -> Result<(), Error> {
    deny()?;
    format_check()?;
    lints_deny()?;
    check_all()?;
    test()?;
    about()?;
    doc()?;
    upgrade()?;
    Ok(())
}

fn quick_checks() -> Result<(), Error> {
    lints_deny()?;
    format_check()?;
    deny()?;
    Ok(())
}

fn updates() -> Result<(), Error> {
    logstart("updates");
    exec("rustup", &["update"])?;
    exec("cargo", &["update", "--color=always"])?;
    logend();
    Ok(())
}

fn build() -> Result<(), Error> {
    logstart("build");
    for feature in FEATURE_ARGS {
        exec(
            "cargo",
            &["build", "--all-targets", feature, "--color=always"],
        )?;
    }
    logend();
    Ok(())
}

fn test() -> Result<(), Error> {
    for feature in FEATURE_ARGS {
        logstart(format!("test-{feature}").as_str());
        exec_passthru_env(
            "cargo",
            &["test", "--all-targets", feature, "--color=always"],
            [("RUSTFLAGS", "-Copt-level=1")],
        )?;
        logend();
    }
    Ok(())
}

fn lints() -> Result<(), Error> {
    logstart("lints");
    for feature in FEATURE_ARGS {
        exec(
            "cargo",
            &[
                "clippy",
                "--bins",
                "--lib",
                "--examples",
                feature,
                "--color=always",
            ],
        )?;
    }
    logend();
    Ok(())
}

fn lints_deny() -> Result<(), Error> {
    logstart("lints");
    for feature in FEATURE_ARGS {
        exec(
            "cargo",
            &[
                "clippy",
                "--bins",
                "--lib",
                "--examples",
                feature,
                "--color=always",
                "--",
                "-Dwarnings",
            ],
        )?;
    }
    logend();
    Ok(())
}

fn upgrade() -> Result<(), Error> {
    logstart("upgrade");
    install_update("cargo-edit");
    exec("cargo", &["upgrade", "--dry-run", "--pinned", "-i"])?;
    logend();
    Ok(())
}

fn about() -> Result<(), Error> {
    logstart("about");
    exec(
        "cargo",
        &["install", "--locked", "cargo-about", "--color=always"],
    )?;
    exec_passthru(
        "cargo",
        &["about", "generate", "about.hbs", "-o", "about.html"],
    )?;
    logend();
    Ok(())
}

fn doc() -> Result<(), Error> {
    logstart("doc");
    exec("cargo", &["doc", "--color=always"])?;
    logend();
    Ok(())
}

fn check(target: &str) -> Result<(), Error> {
    logstart(&format!("check-{target}"));
    exec_passthru("rustup", &["target", "add", target])?;
    cleanlocal()?;
    exec_passthru(
        "cargo",
        &[
            "check",
            "--target",
            target,
            "--color=always",
            "--all-targets",
        ],
    )?;
    logend();
    Ok(())
}

fn check_all() -> Result<(), Error> {
    clean()?;
    let mut skip = Some(());
    for feat in exec_stdout_lines(
        "cargo",
        &["describe", "-fname", "-fmodule-features", "-ocsv"],
    )? {
        if skip.take().is_some() {
            // skip header row.
            continue;
        }
        let (module, feats) = feat.split_once(",").unwrap_or_default();
        let feats = feats.split(" ");
        for f in feats {
            logstart(format!("cargo-check-p-{module}-f-{f}").as_str());
            exec_passthru("cargo", &["check", "-p", &module, "-F", f])?;
            logend();
        }
    }
    for target in TARGETS {
        check(target)?;
    }
    logend();
    Ok(())
}

fn format() -> Result<(), Error> {
    logstart("format");
    exec("cargo", &["fmt", "--all"])?;
    logend();
    Ok(())
}

fn format_check() -> Result<(), Error> {
    logstart("format");
    exec("cargo", &["fmt", "--check", "--all"])?;
    logend();
    Ok(())
}

fn deny() -> Result<(), Error> {
    logstart("deny");
    install_update("cargo-deny");
    ignore_errors(
        "cargo",
        &["deny", "fetch"],
        format_args!("Unable to fetch resources from network"),
    );
    exec("cargo", &["deny", "--offline", "check"])?;
    logend();
    Ok(())
}

fn release(in_args: &[&str]) -> Result<(), Error> {
    logstart("release");
    let pkg = in_args.first().unwrap_or(&"irox");
    let mut rgs = Vec::from_iter([
        "smart-release",
        "--no-conservative-pre-release-version-handling",
        "--no-isolate-dependencies-from-breaking-changes",
        "-u",
        "--verbose",
    ]);
    if in_args.is_empty() {
        rgs.push(pkg)
    }
    for v in in_args {
        rgs.push(v);
    }
    install_update("cargo-smart-release");

    exec(
        "cargo",
        &[
            "install",
            "--locked",
            "cargo-smart-release",
            "--color=always",
        ],
    )?;

    exec_passthru("cargo", &["package", "-p", pkg])?;

    exec_passthru("cargo", &rgs)?;
    logend();
    Ok(())
}

fn new(dest: &str) -> Result<(), Error> {
    install_update("cargo-generate");
    std::fs::create_dir_all(dest)?;
    let pwd = std::env::current_dir()?;
    let path = format!("{}/{dest}", pwd.display());
    let src = format!("{}/dev/mod_template", pwd.display());
    exec_passthru(
        "cargo",
        &["generate", "--destination", &path, "--path", &src, "--init"],
    )?;
    Ok(())
}

fn buildperf() -> Result<(), Error> {
    exec("cargo", &["fetch"])?;
    exec("cargo", &["clean"])?;
    let start = std::time::Instant::now();
    exec(
        "cargo",
        &[
            "build",
            "--release",
            "--all-targets",
            "--all-features",
            "--color=always",
        ],
    )?;
    let elapsed = start.elapsed();
    println!("Built in {:.2}s", elapsed.as_secs_f64());
    Ok(())
}
fn clean() -> Result<(), Error> {
    logstart("clean");
    exec_passthru("cargo", &["clean"])?;
    logend();
    Ok(())
}
fn cleanlocal() -> Result<(), Error> {
    let lines = get_modules()?;
    let mut modules = vec!["clean"];
    for line in &lines {
        modules.push("-p");
        modules.push(line);
    }

    exec("cargo", &modules)?;
    Ok(())
}
fn unused() -> Result<(), Error> {
    logstart("unused");
    exec(
        "cargo",
        &[
            "clippy",
            "--bins",
            "--lib",
            "--all-features",
            "--color=always",
            "--",
            "-Wunused_crate_dependencies",
        ],
    )?;
    logend();
    Ok(())
}
fn package() -> Result<(), Error> {
    logstart("package");
    exec(
        "cargo",
        &["package", "-pirox", "--all-features", "--color=always"],
    )?;
    logend();
    Ok(())
}

fn rbuild() -> Result<(), Error> {
    exec("cargo", &["clean"])?;
    let rustc_args = &[
        "-Ctarget-cpu=x86-64-v3",
        "-Cstrip=symbols",
        "-Copt-level=s",
        "-Ccodegen-units=1",
        "-Ccontrol-flow-guard=yes",
        "-Cpanic=abort",
    ]
    .join(" ");
    exec_passthru_env(
        "cargo",
        &["build", "--release", "--examples"],
        [("RUSTFLAGS", rustc_args)],
    )?;
    Ok(())
}

fn bloat(args: &[&str]) -> Result<(), Error> {
    install_update("cargo-bloat");
    let rustc_args = &[
        "-Ctarget-cpu=x86-64-v3",
        "-Cstrip=symbols",
        "-Copt-level=s",
        "-Ccodegen-units=1",
        "-Ccontrol-flow-guard=yes",
        "-Cpanic=abort",
    ]
    .join(" ");

    let args = (&[&["bloat", "--release"], args]).concat();

    exec_env("cargo", &args, [("RUSTFLAGS", rustc_args)])?;
    Ok(())
}
