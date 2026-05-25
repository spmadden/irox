// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use crate::error::{Error, ErrorKind};
use crate::os::win::env::set_osgeo_envs;
use irox_bits::{BitsWrapper, MutBits};
use irox_log::log::debug;
use irox_log::log::warn;
use std::ffi::OsStr;
use std::fmt::{Arguments, Debug};
use std::fs::OpenOptions;
use std::io::{stdout, BufRead, BufReader, Read, Stdout, Write};
use std::path::{Path, PathBuf};
use std::process::{ChildStderr, ChildStdin, ChildStdout, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct ChildIO<'a, T: MutBits> {
    pub outstream: BitsWrapper<'a, T>,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
}
impl<'a, T: MutBits> ChildIO<'a, T> {
    pub fn run_some(&mut self) -> Result<(), Error> {
        if let Some(so) = &mut self.stdout {
            let mut bw = BitsWrapper::Borrowed(so);
            self.outstream.write_all_into_self_from(&mut bw)?;
        }
        if let Some(so) = &mut self.stderr {
            let mut bw = BitsWrapper::Borrowed(so);
            self.outstream.write_all_into_self_from(&mut bw)?;
        }
        Ok(())
    }
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
    let mut child = std::process::Command::new(cmd);
    set_osgeo_envs(&mut child)?;
    let mut child = child
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
pub fn exec_stdout_lines_env<I, K, V, P>(
    cmd: &str,
    args: &[&str],
    cwd: Option<P>,
    vars: I,
) -> Result<Vec<String>, Error>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr> + Debug,
    V: AsRef<OsStr> + Debug,
    P: AsRef<Path>,
{
    let mut child = std::process::Command::new(cmd);
    set_osgeo_envs(&mut child)?;
    debug!("envs: {:?}", child.get_envs());
    if let Some(cwd) = cwd {
        child.current_dir(cwd);
    }

    let mut child = child
        .args(args)
        .envs(vars)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|_| panic!("Unable to spawn command: {cmd} {}", args.join(" ")));
    let stdout = child.stdout.take().expect("Unable to take stdout");
    let mut stderr = child.stderr.take().expect("Unable to take stderr");
    let run = Arc::new(AtomicBool::new(true));
    let run3 = run.clone();
    let stdout_hnd = std::thread::spawn(move || {
        let buffered = BufReader::new(stdout);
        buffered.lines().map_while(Result::ok).collect()
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
    let lines = stdout_hnd.join().unwrap_or_default();
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

    Ok(lines)
}
pub fn exec_stdout_lines(cmd: &str, args: &[&str]) -> Result<Vec<String>, Error> {
    exec_stdout_lines_env::<_, &str, &str, &str>(cmd, args, None, [])
}

pub fn exec_stdout_file(cmd: &str, args: &[&str], file: &str) -> Result<(), Error> {
    let mut child = std::process::Command::new(cmd);
    set_osgeo_envs(&mut child)?;
    debug!("envs: {:#?}", std::env::vars());
    let mut child = child
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
pub fn exec_passthru<P: AsRef<Path>>(
    cmd: &str,
    cwd: Option<P>,
    args: &[&str],
) -> Result<(), Error> {
    exec_passthru_env::<_, &str, &str, P>(cmd, args, cwd, [])
}
pub fn exec_passthru_osgeo(cmd: &str, args: &[&str]) -> Result<(), Error> {
    exec_passthru_osgeo_env::<_, &str, &str>(cmd, args, [])
}
pub fn exec_passthru_osgeo_env<I, K, V>(cmd: &str, args: &[&str], vars: I) -> Result<(), Error>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr>,
    V: AsRef<OsStr>,
{
    let mut child = std::process::Command::new(cmd);
    set_osgeo_envs(&mut child)?;
    let mut child = child
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
pub fn exec_passthru_env<I, K, V, P>(
    cmd: &str,
    args: &[&str],
    cwd: Option<P>,
    vars: I,
) -> Result<(), Error>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr> + Debug,
    V: AsRef<OsStr> + Debug,
    P: AsRef<Path>,
{
    let vars = vars.into_iter().collect::<Vec<_>>();
    debug!("Executing command: {cmd} with args: {args:?} and vars: {vars:?}");
    let mut child = std::process::Command::new(cmd);
    if let Some(cwd) = cwd {
        child.current_dir(cwd);
    }

    let mut child = child
        .args(args)
        .envs(vars)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("Unable to spawn command: {cmd} {}: {e}", args.join(" ")));
    let mut io = ChildIO {
        outstream: BitsWrapper::Owned(String::new()),
        stdout: child.stdout.take(),
        stderr: child.stderr.take(),
    };
    let status = loop {
        let res = child.try_wait()?;
        if let Some(res) = res {
            break res;
        }
        io.run_some()?;
    };

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

pub fn ignore_errors<'a, T: Into<Arguments<'a>>>(cmd: &str, args: &[&str], msg: T) {
    if let Err(e) = exec(cmd, args) {
        warn!("{}.  Error was: {e:?}", msg.into());
    }
}
pub fn run_powershell_cmd(script: &str) -> Result<(), Error> {
    exec_passthru(
        "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe",
        Option::<&Path>::None,
        &[
            "-Command",
            &format!("\"& {{{script} | Out-String -Width 400 -Stream}}\""),
        ],
    )
}
pub fn check_cyg_present<T: AsRef<Path>>(at: T) -> Result<(), Error> {
    let path = at.as_ref();
    if std::fs::exists(path)? {
        return Ok(());
    };
    run_powershell_cmd(".\\setup-msys2.ps1")?;
    if std::fs::exists(path)? {
        return Ok(());
    };

    Ok(())
}
pub fn winpath(cyg: &str, p: &str) -> String {
    let p = p.replace("/", "\\");
    format!("{cyg}{p}")
}
pub fn run_cyg_command_lines(cmd: &str, args: &[&str]) -> Result<Vec<String>, Error> {
    let cwd: PathBuf = if let Ok(p) = std::env::var("CYG_TOOLBOX_HOME") {
        p.into()
    } else {
        std::env::current_dir()?
    };
    let cygpath = (&cwd).join("msys2");
    check_cyg_present(&cygpath)?;
    let cygpath = cygpath.join("msys64");
    let bash = (&cygpath.clone())
        .join("usr")
        .join("bin")
        .join("bash.exe")
        .to_string_lossy()
        .to_string();
    let args = format!("/ucrt64/bin/{cmd} {}", args.join(" "));
    let mut a = Vec::new();
    a.append(&mut vec!["-lc", &args]);

    exec_stdout_lines_env(
        &bash,
        a.as_slice(),
        Option::<&Path>::None,
        [("MSYSTEM", "UCRT"), ("CHERE_INVOKING", "yes")],
    )
}
pub fn run_cyg_command(cmd: &str, args: &[&str]) -> Result<(), Error> {
    let cwd: PathBuf = if let Ok(p) = std::env::var("CYG_TOOLBOX_HOME") {
        p.into()
    } else {
        std::env::current_dir()?
    };
    let cygpath = (&cwd).join("msys2");
    check_cyg_present(&cygpath)?;
    let cygpath = cygpath.join("msys64");
    let bash = (&cygpath.clone())
        .join("usr")
        .join("bin")
        .join("bash.exe")
        .to_string_lossy()
        .to_string();
    let args = format!("/ucrt64/bin/{cmd} {}", args.join(" "));
    let mut a = Vec::new();
    a.append(&mut vec!["-lc", &args]);

    exec_passthru_env(
        &bash,
        a.as_slice(),
        Option::<&Path>::None,
        [("MSYSTEM", "UCRT"), ("CHERE_INVOKING", "yes")],
    )
}
pub fn run_cyg_command_in<P: AsRef<Path>>(
    cmd: &str,
    c: Option<P>,
    args: &[&str],
) -> Result<(), Error> {
    let cwd: PathBuf = if let Ok(p) = std::env::var("GIS_TOOLBOX_HOME") {
        p.into()
    } else {
        std::env::current_dir()?
    };
    let cygpath = (&cwd).join("msys2");
    check_cyg_present(&cygpath)?;
    let cygpath = cygpath.join("msys64");
    let bash = (&cygpath.clone())
        .join("usr")
        .join("bin")
        .join("bash.exe")
        .to_string_lossy()
        .to_string();
    let args = format!("/ucrt64/bin/{cmd} {}", args.join(" "));
    let mut a = Vec::new();
    a.append(&mut vec!["-lc", &args]);

    exec_passthru_env(
        &bash,
        a.as_slice(),
        c,
        [("MSYSTEM", "UCRT"), ("CHERE_INVOKING", "yes")],
    )
}

pub fn append_extension<T: AsRef<Path>>(orig: T, ext: &str) -> PathBuf {
    let orig = orig.as_ref();
    let parent = orig.parent().unwrap_or_else(|| Path::new("."));
    let stem = orig
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    parent.join(format!("{stem}{ext}"))
}
pub fn replace_extension<T: AsRef<Path>>(orig: T, ext: &str) -> PathBuf {
    let orig = orig.as_ref();
    let parent = orig.parent().unwrap_or_else(|| Path::new("."));
    let stem = orig
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    parent.join(format!("{stem}{ext}"))
}

pub fn run_qgis_processing(
    project_path: &str,
    script: &str,
    extraparams: &[&str],
) -> Result<(), Error> {
    let projpath = format!("--PROJECT_PATH={project_path}");
    let mut params = Vec::new();
    params.push("run");
    params.push(script);
    params.push(projpath.as_str());
    params.push("--");
    params.extend(extraparams);
    exec_passthru_osgeo("qgis_process.exe", &params)?;
    Ok(())
}
