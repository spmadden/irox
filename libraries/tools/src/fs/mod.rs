// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Filesystem utilities

use crate::{cfg_feature_alloc, cfg_feature_std};
use core::fmt::{Display, Formatter};

cfg_feature_std! {
    mod temp;
    pub use temp::*;
}

///
/// A list of characters that are usually prohibited by common filesystems like VFAT and NTFS.
/// See [Wikipedia:Filename](https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words)
pub static USUALLY_PROHIBITED_FS_CHARS: &[char; 9] =
    &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

///
/// A list of characters that are prohibited by FAT12, FAT16, FAT34
/// See [Wikipedia:Filename](https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words)
pub static FATXX_PROHIBITED_FS_CHARS: &[char; 18] = &[
    '"', '*', '/', ':', '<', '>', '?', '\\', '|', '+', ',', '.', ';', '=', '[', ']', '!', '@',
];

///
/// A list of filenames prohibited by windows.  
/// See [Wikipedia:Filename](https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words)
pub static WINDOWS_PROHIBITED_FILE_NAMES: &[&str; 45] = &[
    "CON", "PRN", "AUX", "CLOCK$", "LST", "KEYBD$", "SCREEN$", "$IDLE$", "CONFIG$", "NUL", "COM0",
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "LPT0", "LPT1", "LPT2",
    "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9", "$Mft", "$MftMirr", "$LogFile",
    "$Volume", "$AttrDef", "$Bitmap", "$Boot", "$BadClus", "$Secure", "$Upcase", "$Extend",
    "$Quota", "$ObjId", "$Reparse", "$Extend",
];

///
/// A list of characters that are usually safe for use in filesystem names.  This is essentially the
/// printable characters minus [`USUALLY_PROHIBITED_FS_CHARS`] and [`;`,`$`] (to avoid shell issues)
/// See [Wikipedia:Filename](https://en.wikipedia.org/wiki/Filename#Reserved_characters_and_words)
pub static USUALLY_SAFE_FS_CHARS: &[char; 83] = &[
    ' ', '!', '#', '%', '&', '(', ')', '+', ',', '-', '.', '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', '=', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', ']', '^', '_', '`', 'a', 'b', 'c',
    'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '{', '}', '~',
];

///
/// An error case returned from the filename checker
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FilenameError {
    StartsWithWindowsProhibited(&'static str),
    ContainsUsuallyInvalidChar(char),
    EndsWithInvalidCharacter(char),
}
impl Display for FilenameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            FilenameError::StartsWithWindowsProhibited(pro) => {
                write!(f, "Filename starts with windows prohibited word: {pro}")
            }
            FilenameError::ContainsUsuallyInvalidChar(chr) => {
                write!(
                    f,
                    "Filename contains a usually invalid character: {chr}(0x{:02X})",
                    *chr as u16
                )
            }
            FilenameError::EndsWithInvalidCharacter(chr) => {
                write!(
                    f,
                    "Filename ends with a usually invalid character: {chr}(0x{:02X})",
                    *chr as u16
                )
            }
        }
    }
}
impl core::error::Error for FilenameError {}

cfg_feature_alloc! {
    extern crate alloc;
    ///
    /// Removes any character in the input value that isn't in [`USUALLY_SAFE_FS_CHARS`]
    pub fn clean_filename<T: AsRef<str>>(val: &T) -> alloc::string::String {
        let input = val.as_ref();
        let mut out = alloc::string::String::with_capacity(input.len());
        for v in input.chars() {
            if USUALLY_SAFE_FS_CHARS.binary_search(&v).is_ok() {
                out.push(v);
            }
        }
        out
    }
}

///
/// Checks the provided filename against the set of [`WINDOWS_PROHIBITED_FILE_NAMES`] and
/// [`USUALLY_PROHIBITED_FS_CHARS`], returning an error if either set of checks are found
pub fn is_filename_probably_valid<T: AsRef<str>>(val: &T) -> Result<(), FilenameError> {
    let input = val.as_ref();
    for invalid in WINDOWS_PROHIBITED_FILE_NAMES {
        if input.starts_with(invalid) {
            return Err(FilenameError::StartsWithWindowsProhibited(invalid));
        }
    }
    for v in input.chars() {
        let vi = v as u32;
        if !(0x20..=0x7E).contains(&vi) || USUALLY_PROHIBITED_FS_CHARS.binary_search(&v).is_ok() {
            return Err(FilenameError::ContainsUsuallyInvalidChar(v));
        }
    }
    Ok(())
}

cfg_feature_std! {
    use std::path::{Path, PathBuf};
    use std::fs::{DirEntry};
    use irox_bits::{Error, ErrorKind};

    ///
    /// Recursively finds all files with the associated extension in the starting directory
    pub fn find_all_files_with_extension<T: AsRef<Path>>(start: T, ext: &str) -> Result<Vec<PathBuf>, Error> {
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
        _ext: &str,
    ) -> Result<(), Error> {
        let path = dirent.path();
        if path.is_dir() {
            let ent = std::fs::read_dir(path)?;
            for dirent in ent {
                let dirent = dirent?;
                collect_fileswithext_in_dirent(&dirent, out, _ext)?;
            }
        } else if let Some(ext) = path.extension() {
            if ext.to_string_lossy().to_lowercase().as_str() == ext {
                out.push(path);
            }
        }

        Ok(())
    }

    ///
    /// Finds
    pub fn find_associated_file<T: AsRef<Path>>(file: T, newext: &str) -> Result<PathBuf, Error> {
        let path = file.as_ref();
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        // check concat first.
        let Some(filename) = path.file_name() else {
            return Error::err(ErrorKind::NotFound, "Provided file doesn't have a name");
        };
        let filename = format!("{}.{newext}", filename.to_string_lossy());
        let mut appended = path.to_path_buf();
        appended.set_file_name(filename);

        if appended.exists() && appended.is_file() {
            return Ok(appended);
        }
        // check replacement next.
        let Some(stem) = path.file_stem() else {
            return Error::err(ErrorKind::NotFound, "Provided file doesn't have a stem");
        };
        Ok(dir.join(format!("{}.{}", stem.to_string_lossy(), newext)))
    }
}
