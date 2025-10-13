// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]
use crate::{BuildEnvironment, BuildVariable, Error, VariableSource};
use std::io::{BufRead, BufReader};

pub static CARGO_ENV_VARIABLES: [&str; 26] = [
    "CARGO_PKG_VERSION",
    "CARGO_PKG_VERSION_MAJOR",
    "CARGO_PKG_VERSION_MINOR",
    "CARGO_PKG_VERSION_PATCH",
    "CARGO_PKG_VERSION_PRE",
    "CARGO_PKG_AUTHORS",
    "CARGO_PKG_NAME",
    "CARGO_PKG_DESCRIPTION",
    "CARGO_PKG_HOMEPAGE",
    "CARGO_PKG_REPOSITORY",
    "CARGO_PKG_LICENSE",
    "CARGO_PKG_LICENSE_FILE",
    "CARGO_PKG_RUST_VERSION",
    "CARGO_PKG_README",
    "CARGO_PKG_CRATE_NAME",
    "CARGO_PKG_BIN_NAME",
    "CARGO_CFG_TARGET_FAMILY",
    "CARGO_CFG_TARGET_OS",
    "CARGO_CFG_TARGET_ARCH",
    "CARGO_CFG_TARGET_VENDOR",
    "CARGO_CFG_TARGET_HAS_ATOMIC",
    "CARGO_CFG_TARGET_ENV",
    "CARGO_CFG_TARGET_POINTER_WIDTH",
    "CARGO_CFG_TARGET_ENDIAN",
    "CARGO_CFG_TARGET_FEATURE",
    "CARGO_PRIMARY_PACKAGE",
];

pub static RUSTC_ENV_VARIABLES: [&str; 10] = [
    "PROCESSOR_ARCHITECTURE",
    "PROCESSOR_IDENTIFIER",
    "PROCESSOR_LEVEL",
    "PROCESSOR_REVISION",
    "TARGET",
    "HOST",
    "PROFILE",
    "RUSTUP_TOOLCHAIN",
    "OPT_LEVEL",
    "DEBUG",
];

pub static BUILD_HOST_VARIABLES: [&str; 7] = [
    "RUSTC_VERSION",
    "CARGO_VERSION",
    "BUILD_HOST_HOSTNAME",
    "BUILD_HOST_KERNELVER",
    "BUILD_HOST_OSNAME",
    "BUILD_HOST_OSVER",
    "BUILD_TIME",
];

static COMMANDS: &[(&str, &[&str])] = &[
    ("RUSTC_VERSION", &["rustc", "--version"]),
    ("CARGO_VERSION", &["cargo", "--version"]),
];
fn run_command(cmd: &[&str]) -> Result<String, Error> {
    let output = std::process::Command::new(cmd[0])
        .args(&cmd[1..])
        .output()?;
    let res = String::from_utf8_lossy(&output.stdout);
    let res = res.trim();
    Ok(res.to_string())
}
pub fn load_buildhost_variables(env: &mut BuildEnvironment) -> Result<(), Error> {
    for (var, cmd) in COMMANDS {
        let res = run_command(cmd)?;
        env.variables.insert(
            var.to_string(),
            BuildVariable::new_str(var, &res, VariableSource::BuildHost),
        );
    }

    let build_time = irox_time::datetime::UTCDateTime::now().format_iso8601_extended();
    env.variables.insert(
        "BUILD_TIME".to_string(),
        BuildVariable::new_str("BUILD_TIME", &build_time, VariableSource::BuildHost),
    );

    #[cfg(target_os = "windows")]
    load_windows_sysinfo(env)?;
    #[cfg(target_os = "linux")]
    load_linux_sysinfo(env)?;

    Ok(())
}
#[cfg(target_os = "windows")]
pub fn load_windows_sysinfo(env: &mut BuildEnvironment) -> Result<(), Error> {
    static SYSINFO_VARS: &[(&str, &str)] = &[
        ("BUILD_HOST_HOSTNAME", "Host Name"),
        ("BUILD_HOST_OSNAME", "OS Name"),
        ("BUILD_HOST_OSVER", "OS Version"),
    ];
    fn parse_output(env: &mut BuildEnvironment, output: Vec<u8>) -> Result<(), Error> {
        let output = BufReader::new(output.as_slice());
        for line in output.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let Some((k, v)) = line.split_once(':') else {
                continue;
            };
            let v = v.trim();
            for (var, key) in SYSINFO_VARS {
                if *key == k {
                    env.variables.insert(
                        var.to_string(),
                        BuildVariable::new_str(var, v, VariableSource::BuildHost),
                    );
                }
            }
        }
        Ok(())
    }
    parse_output(
        env,
        std::process::Command::new("systeminfo").output()?.stdout,
    )?;

    Ok(())
}
#[cfg(target_os = "linux")]
pub fn load_linux_sysinfo(env: &mut BuildEnvironment) -> Result<(), Error> {
    let kernel_ver = run_command(&["uname", "-r"])?;
    env.variables.insert(
        "BUILD_HOST_KERNELVER".to_string(),
        BuildVariable::new_str(
            "BUILD_HOST_KERNELVER",
            &kernel_ver,
            VariableSource::BuildHost,
        ),
    );
    let hostname = run_command(&["uname", "-n"])?;
    env.variables.insert(
        "BUILD_HOST_HOSTNAME".to_string(),
        BuildVariable::new_str("BUILD_HOST_HOSTNAME", &hostname, VariableSource::BuildHost),
    );
    let mut found_prettyname = false;
    let reader = BufReader::new(
        std::fs::OpenOptions::new()
            .read(true)
            .open("/etc/os-release")?,
    );
    for line in reader.lines() {
        let line = line?;
        let Some((key, val)) = line.split_once('=') else {
            continue;
        };
        let val = val.replace('"', "");
        let key = key.trim();
        let mut name = None;
        let mut version = None;
        match key {
            "PRETTY_NAME" => {
                found_prettyname = true;
                let var =
                    BuildVariable::new_str("BUILD_HOST_OSNAME", &val, VariableSource::BuildHost);
                env.variables.insert("BUILD_HOST_OSNAME".to_string(), var);
            }
            "NAME" => {
                name = Some(val.to_string());
            }
            "VERSION" => {
                version = Some(val.to_string());
            }
            _ => continue,
        }
        if let Some(version) = version {
            let var =
                BuildVariable::new_str("BUILD_HOST_OSVER", &version, VariableSource::BuildHost);
            env.variables.insert("BUILD_HOST_OSVER".to_string(), var);
            if !found_prettyname {
                if let Some(name) = name {
                    let osname = format!("{name} {version}");
                    let var = BuildVariable::new_str(
                        "BUILD_HOST_OSNAME",
                        &osname,
                        VariableSource::BuildHost,
                    );
                    env.variables.insert("BUILD_HOST_OSNAME".to_string(), var);
                }
            }
        }
    }
    Ok(())
}
