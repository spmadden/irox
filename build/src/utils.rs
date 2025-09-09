// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

use crate::error::{Error, ErrorKind};
use irox_log::log::warn;
use std::ffi::OsStr;
use std::fmt::Arguments;
use std::fs::OpenOptions;
use std::io::{BufRead, Read, Write};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn get_modules() -> Result<Vec<String>, Error> {
    install_update("cargo-describe");
    exec_stdout_lines("cargo", &["describe", "-oplain", "-fname"])
}

pub fn is_github_action() -> bool {
    let Ok(val) = std::env::var("GITHUB_ACTIONS") else {
        return false;
    };
    val == "true"
}

pub fn logstart(group: &str) {
    if !is_github_action() {
        return;
    }

    println!("::group::{group}");
}

pub fn logend() {
    if !is_github_action() {
        return;
    }
    println!("::endgroup::");
}
pub fn exec(cmd: &str, args: &[&str]) -> Result<(), Error> {
    exec_env::<_, &str, &str>(cmd, args, [])
}
pub fn exec_env<I, K, V>(cmd: &str, args: &[&str], vars: I) -> Result<(), Error>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let mut child = std::process::Command::new(cmd)
        .args(args)
        .envs(vars)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("Unable to spawn command: {cmd} {}", args.join(" ")));
    let mut stdout = child.stdout.take().expect("Unable to take stdout");
    let mut stderr = child.stderr.take().expect("Unable to take stderr");
    let run = Arc::new(AtomicBool::new(true));
    let run2 = run.clone();
    let run3 = run.clone();
    let stdout_hnd = std::thread::spawn(move || {
        while run2.load(Ordering::Relaxed) {
            let mut buf = [0u8; 4096];
            match stdout.read(&mut buf) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    let _ = std::io::stdout().write_all(&buf[..len]);
                }
                Err(_e) => {
                    break;
                }
            }
        }
    });
    let stderr_hnd = std::thread::spawn(move || {
        while run3.load(Ordering::Relaxed) {
            let mut buf = [0u8; 4096];
            match stderr.read(&mut buf) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    let _ = std::io::stderr().write_all(&buf[..len]);
                }
                Err(_e) => {
                    break;
                }
            }
        }
    });

    let status = child.wait()?;
    run.store(false, Ordering::Relaxed);
    let _ = stdout_hnd.join();
    let _ = stderr_hnd.join();

    match status.code() {
        Some(c) => {
            if c != 0 {
                return Err(Error {
                    msg: format!("Command exited with code {c}: {cmd} {}", args.join(" ")),
                    kind: ErrorKind::SubprocessError,
                });
            }
        }
        None => {
            return Err(Error {
                msg: format!("Command exited by signal: {cmd} {}", args.join(" ")),
                kind: ErrorKind::SubprocessError,
            });
        }
    }

    Ok(())
}

pub fn exec_stdout_lines(cmd: &str, args: &[&str]) -> Result<Vec<String>, Error> {
    let output = std::process::Command::new(cmd)
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Unable to spawn command: {cmd} {}", args.join(" ")));
    match output.status.code() {
        Some(c) => {
            if c != 0 {
                return Err(Error {
                    msg: format!("Command exited with code {c}: {cmd} {}", args.join(" ")),
                    kind: ErrorKind::SubprocessError,
                });
            }
        }
        None => {
            return Err(Error {
                msg: format!("Command exited by signal: {cmd} {}", args.join(" ")),
                kind: ErrorKind::SubprocessError,
            });
        }
    }
    let lines: Vec<String> = output.stdout.lines().map_while(Result::ok).collect();
    Ok(lines)
}

pub fn exec_stdout_file(cmd: &str, args: &[&str], file: &str) -> Result<(), Error> {
    let mut child = std::process::Command::new(cmd)
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("Unable to spawn command: {cmd} {}", args.join(" ")));
    let mut stdout = child.stdout.take().expect("Unable to take stdout");
    let mut stderr = child.stderr.take().expect("Unable to take stderr");
    let run = Arc::new(AtomicBool::new(true));
    let run2 = run.clone();
    let run3 = run.clone();
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file)?;
    let stdout_hnd = std::thread::spawn(move || {
        while run2.load(Ordering::Relaxed) {
            let mut buf = [0u8; 4096];
            match stdout.read(&mut buf) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    let _ = file.write_all(&buf[..len]);
                }
                Err(_e) => {
                    break;
                }
            }
        }
    });
    let stderr_hnd = std::thread::spawn(move || {
        while run3.load(Ordering::Relaxed) {
            let mut buf = [0u8; 4096];
            match stderr.read(&mut buf) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    let _ = std::io::stderr().write(&buf[..len]);
                }
                Err(_e) => {
                    break;
                }
            }
        }
    });

    let status = child.wait()?;
    run.store(false, Ordering::Relaxed);
    let _ = stdout_hnd.join();
    let _ = stderr_hnd.join();

    match status.code() {
        Some(c) => {
            if c != 0 {
                return Err(Error {
                    msg: format!("Command exited with code {c}: {cmd} {}", args.join(" ")),
                    kind: ErrorKind::SubprocessError,
                });
            }
        }
        None => {
            return Err(Error {
                msg: format!("Command exited by signal: {cmd} {}", args.join(" ")),
                kind: ErrorKind::SubprocessError,
            });
        }
    }

    Ok(())
}
pub fn exec_passthru(cmd: &str, args: &[&str]) -> Result<(), Error> {
    exec_passthru_env::<_, &str, &str>(cmd, args, [])
}
pub fn exec_passthru_env<I, K, V>(cmd: &str, args: &[&str], vars: I) -> Result<(), Error>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let mut child = std::process::Command::new(cmd)
        .args(args)
        .envs(vars)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap_or_else(|_| panic!("Unable to spawn command: {cmd} {}", args.join(" ")));
    let status = child.wait()?;
    match status.code() {
        Some(c) => {
            if c != 0 {
                return Err(Error {
                    msg: format!("Command exited with code {c}: {cmd} {}", args.join(" ")),
                    kind: ErrorKind::SubprocessError,
                });
            }
        }
        None => {
            return Err(Error {
                msg: format!("Command exited by signal: {cmd} {}", args.join(" ")),
                kind: ErrorKind::SubprocessError,
            });
        }
    }

    Ok(())
}

pub fn install_update(tool: &str) {
    if let Err(e) = exec(
        "cargo",
        &["+stable", "install", "--locked", tool, "--color=always"],
    ) {
        warn!("Unable to install/update {tool} - probably due to a network failure.  The next commands may not work.  Error was: {e:?}");
    };
}

pub fn ignore_errors<'a, T: Into<Arguments<'a>>>(cmd: &str, args: &[&str], msg: T) {
    if let Err(e) = exec(cmd, args) {
        warn!("{}.  Error was: {e:?}", msg.into());
    }
}
