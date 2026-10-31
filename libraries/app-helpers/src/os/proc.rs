// SPDX-License-Identifier: MIT
// Copyright 2025-2026 IROX Contributors
//

use crate::error::{Error, ErrorKind};
// use crate::os::win::env::set_osgeo_envs;
use crate::subprocess_error;
use core::fmt::{Arguments, Debug};
use core::sync::atomic::{AtomicBool, Ordering};
use irox_bits::{BitsWrapper, MutBits};
use irox_log::log::warn;
use irox_log::log::{debug, error};
use std::ffi::OsStr;
use std::fs::{DirEntry, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Stdio};
extern crate alloc;
use alloc::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

pub struct ChildIO<T: MutBits + 'static> {
    pub child: Child,
    pub outstream: Arc<Mutex<BitsWrapper<'static, T>>>,
    pub stdin: Option<ChildStdin>,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
    pub handles: Vec<JoinHandle<()>>,
}
impl<T: MutBits> Drop for ChildIO<T> {
    fn drop(&mut self) {
        let _ = self.child.kill();
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}
impl<T: MutBits + Send + 'static> ChildIO<T> {
    pub fn start(&mut self) {
        if let Some(mut io) = self.stdout.take() {
            let lock1 = self.outstream.clone();
            let hndl = std::thread::spawn(move || {
                let mut buf = vec![0u8; 4096];
                let lock = lock1;
                loop {
                    match io.read(&mut buf) {
                        Ok(read) => {
                            if read == 0 {
                                break;
                            }
                            if let Some(l) = buf.get(..read) {
                                if let Ok(mut lock) = lock.lock() {
                                    let _ = lock.write_all_bytes(l);
                                }
                            }
                        }
                        Err(e) => {
                            error!("Error reading from stdout: {e}");
                            break;
                        }
                    }
                }
            });
            self.handles.push(hndl);
        }
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
    // set_osgeo_envs(&mut child)?;
    let mut child = match child
        .args(args)
        .envs(vars)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Unable to spawn command: {cmd} {}", args.join(" "));
            return Err(e.into());
        }
    };
    let Some(mut stdout) = child.stdout.take() else {
        return Err(subprocess_error!("Unable to take stdout"));
    };
    let Some(mut stderr) = child.stderr.take() else {
        return Err(subprocess_error!("Unable to take stdout"));
    };
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
                    if let Some(l) = buf.get(..len) {
                        let _ = std::io::stdout().write_all(l);
                    }
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
                    if let Some(l) = buf.get(..len) {
                        let _ = std::io::stderr().write_all(l);
                    }
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
    // set_osgeo_envs(&mut child)?;
    debug!("envs: {:?}", child.get_envs());
    if let Some(cwd) = cwd {
        child.current_dir(cwd);
    }

    let mut child = match child
        .args(args)
        .envs(vars)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Unable to spawn command: {cmd} {}", args.join(" "));
            return Err(e.into());
        }
    };
    let Some(stdout) = child.stdout.take() else {
        return Err(subprocess_error!("Unable to take stdout"));
    };
    let Some(mut stderr) = child.stderr.take() else {
        return Err(subprocess_error!("Unable to take stdout"));
    };
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
                    if let Some(l) = buf.get(..len) {
                        let _ = std::io::stderr().write(l);
                    }
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
    // set_osgeo_envs(&mut child)?;
    debug!("envs: {:#?}", std::env::vars());
    let mut child = match child
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Unable to spawn command: {cmd} {}", args.join(" "));
            return Err(e.into());
        }
    };
    let Some(mut stdout) = child.stdout.take() else {
        return Err(subprocess_error!("Unable to take stdout"));
    };
    let Some(mut stderr) = child.stderr.take() else {
        return Err(subprocess_error!("Unable to take stdout"));
    };
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
                    if let Some(l) = buf.get(..len) {
                        let _ = file.write_all(l);
                    }
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
                    if let Some(l) = buf.get(..len) {
                        let _ = std::io::stderr().write(l);
                    }
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
    // set_osgeo_envs(&mut child)?;
    let mut child = match child
        .args(args)
        .envs(vars)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Unable to spawn command: {cmd} {}", args.join(" "));
            return Err(e.into());
        }
    };
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

    let mut child = match child
        .args(args)
        .envs(vars)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            warn!("Unable to spawn command: {cmd} {}: {e}", args.join(" "));
            return Err(e.into());
        }
    };
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

pub fn exec_passthru_streamed<I, K, V, P, T: MutBits + Send + 'static>(
    cmd: &str,
    args: &[&str],
    cwd: Option<P>,
    vars: I,
    mut streamer: BitsWrapper<'static, T>,
) -> Result<ChildIO<T>, Error>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<OsStr> + Debug,
    V: AsRef<OsStr> + Debug,
    P: AsRef<Path>,
{
    let vars = vars.into_iter().collect::<Vec<_>>();
    let status = format!("Executing command: {cmd} with args: {args:?} and vars: {vars:?}");
    debug!("{status}");
    streamer.write_all_bytes(status.as_bytes())?;
    let mut child = std::process::Command::new(cmd);
    if let Some(cwd) = cwd {
        child.current_dir(cwd);
    }

    let mut child = match child
        .args(args)
        .envs(vars)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            error!("Unable to spawn command: {cmd} {}: {e}", args.join(" "));
            return Err(e.into());
        }
    };
    let mut io = ChildIO {
        outstream: Arc::new(Mutex::new(streamer)),
        stdin: child.stdin.take(),
        stdout: child.stdout.take(),
        stderr: child.stderr.take(),
        child,
        handles: vec![],
    };
    io.start();

    Ok(io)
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
    let cygpath = cwd.join("msys2");
    check_cyg_present(&cygpath)?;
    let cygpath = cygpath.join("msys64");
    let bash = cygpath
        .clone()
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
    let cygpath = cwd.join("msys2");
    check_cyg_present(&cygpath)?;
    let cygpath = cygpath.join("msys64");
    let bash = cygpath
        .clone()
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
    let cygpath = cwd.join("msys2");
    check_cyg_present(&cygpath)?;
    let cygpath = cygpath.join("msys64");
    let bash = cygpath
        .clone()
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
pub fn find_matching_file_in_dir<T: AsRef<Path>>(
    dir: T,
    prefix: &str,
    suffix: &str,
) -> Result<Option<PathBuf>, Error> {
    let dir = std::fs::read_dir(dir)?;
    let mut matches = Vec::new();
    for file in dir {
        let file = file?;
        if !file.metadata()?.is_file() {
            continue;
        }
        let name = file.file_name();
        let name = name.to_string_lossy();
        if name.starts_with(prefix) && name.ends_with(suffix) {
            matches.push(file.path());
        }
    }
    if matches.len() == 1 {
        return Ok(matches.pop());
    }
    warn!("Warning: Multiple world matches found - skipping.");
    for m in matches {
        warn!("  {}", m.display());
    }

    Ok(None)
}

pub fn find_all_files<T: AsRef<Path>>(start: T, ext: &str) -> Result<Vec<PathBuf>, Error> {
    let mut out = Vec::new();
    let path = start.as_ref();
    let ent = std::fs::read_dir(path)?;
    for dirent in ent {
        let dirent = dirent?;
        collect_fileswithext_in_dirent(&dirent, &mut out, ext)?;
    }
    Ok(out)
}

fn collect_fileswithext_in_dirent(
    dirent: &DirEntry,
    out: &mut Vec<PathBuf>,
    ext: &str,
) -> Result<(), Error> {
    let path = dirent.path();
    if path.is_dir() {
        let ent = std::fs::read_dir(path)?;
        for dirent in ent {
            let dirent = dirent?;
            collect_fileswithext_in_dirent(&dirent, out, ext)?;
        }
    } else if let Some(pathext) = path.extension() {
        if pathext.to_string_lossy().to_lowercase().as_str() == ext {
            out.push(path);
        }
    }

    Ok(())
}
