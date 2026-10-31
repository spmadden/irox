// SPDX-License-Identifier: MIT
// Copyright 2026 IROX Contributors
//

use crate::Error;
use std::io::Write;
use std::io::{Stdin, Stdout};

pub fn readline(stdin: &mut Stdin) -> Result<String, Error> {
    let mut s = String::new();
    stdin.read_line(&mut s)?;
    s = s.trim().to_string();
    Ok(s)
}
pub fn prompt<T: AsRef<str>>(prompt: &T, stdout: &mut Stdout) -> Result<(), Error> {
    write!(stdout, "{}", prompt.as_ref())?;
    stdout.flush()?;
    Ok(())
}
pub fn readline_prompted<T: AsRef<str>>(
    prompts: &T,
    stdin: &mut Stdin,
    stdout: &mut Stdout,
) -> Result<String, Error> {
    let mut out = String::new();
    while out.is_empty() {
        prompt(prompts, stdout)?;
        stdin.read_line(&mut out)?;
        out = out.trim().to_string();
    }
    Ok(out)
}
pub fn acknowledge(prompts: &str, stdin: &mut Stdin, stdout: &mut Stdout) -> Result<bool, Error> {
    loop {
        let ok = readline_prompted(&format!("{prompts} Ok? [yN]"), stdin, stdout)?;
        let ok = ok.trim().to_lowercase();
        if ok == "y" {
            return Ok(true);
        } else if ok == "n" {
            return Ok(false);
        }
    }
}
