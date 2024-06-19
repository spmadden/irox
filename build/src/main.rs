mod error;
mod utils;

pub use utils::*;

use crate::error::Error;
use clap::{Parser, Subcommand};

const FEATURE_ARGS: &[&str] = &["--no-default-features", "-Fdefault", "--all-features"];
const TARGETS: &[&str] = &[
    "x86_64-pc-windows-msvc",
    "wasm32-unknown-unknown",
    "x86_64-unknown-linux-gnu",
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
    Updates,
    /// Iterates through multiple feature sets, default, all, none, etc, calling
    /// `cargo build`
    Build,
    /// Like Build, but runs `cargo test`
    Test,
    /// Runs `rustfmt`
    Format,
    /// Runs `cargo clippy`
    Lints,
    /// Runs `cargo upgrade`
    Upgrade,
    /// Runs `cargo deny`
    Deny,
    /// Runs `cargo about`
    About,
    /// Runs `cargo doc`
    Doc,
    /// Runs `cargo check` for all targets
    Check,
    /// Sets up for a release
    Release,
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
}

#[derive(Debug, Default, Parser)]
struct Args {
    #[command(subcommand)]
    commands: Option<Commands>,
}

fn main() -> Result<(), Error> {
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
        Commands::Release => release()?,
        Commands::New { dest } => new(&dest)?,
        Commands::BuildPerf => buildperf()?,
        Commands::Package => package()?,
        Commands::Unused => unused()?,
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
    test()?;
    about()?;
    doc()?;
    upgrade()?;
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
    logstart("test");
    for feature in FEATURE_ARGS {
        exec_passthru(
            "cargo",
            &["test", "--all-targets", feature, "--color=always"],
        )?;
    }
    logend();
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
    exec(
        "cargo",
        &["install", "--locked", "cargo-edit", "--color=always"],
    )?;
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
    exec_stdout_file("cargo", &["about", "generate", "about.hbs"], "about.html")?;
    logend();
    Ok(())
}

fn doc() -> Result<(), Error> {
    logstart("doc");
    exec("cargo", &["doc", "--color=always"])?;
    logend();
    Ok(())
}

fn check(pkg: &str) -> Result<(), Error> {
    logstart(&format!("check-{pkg}"));
    exec("rustup", &["target", "add", pkg])?;
    cleanlocal()?;
    exec("cargo", &["check", "--target", pkg, "--color=always"])?;
    logend();
    Ok(())
}

fn check_all() -> Result<(), Error> {
    for target in TARGETS {
        check(target)?;
    }
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
    exec(
        "cargo",
        &["install", "--locked", "cargo-deny", "--color=always"],
    )?;
    exec("cargo", &["deny", "check"])?;
    logend();
    Ok(())
}

fn release() -> Result<(), Error> {
    logstart("release");
    exec(
        "cargo",
        &[
            "install",
            "--locked",
            "cargo-smart-release",
            "--color=always",
        ],
    )?;
    exec(
        "cargo",
        &[
            "smart-release",
            "--no-conservative-pre-release-version-handling",
            "--no-isolate-dependencies-from-breaking-changes",
            "-u",
        ],
    )?;
    logend();
    Ok(())
}

fn new(dest: &str) -> Result<(), Error> {
    exec(
        "cargo",
        &["install", "--locked", "cargo-generate", "--color=always"],
    )?;
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

fn cleanlocal() -> Result<(), Error> {
    let lines = exec_stdout_lines("cargo", &["describe", "-oplain", "-fname"])?;
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
