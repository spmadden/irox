// SPDX-License-Identifier: MIT
// Copyright 2023 IROX Contributors
//

use clap::ValueEnum;
use irox_enums::{EnumIterItem, EnumName};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ValueEnum, EnumName, EnumIterItem)]
pub enum Fields {
    /// Name of the package
    Name,

    /// Version of the package from the manifest
    Version,

    /// 'Git Describe' output of the specific package
    GitVersion,

    /// Path to the module's dir, relative to the root dir
    ModuleRelativePath,

    /// Absolute path on disk to the module dir
    ModuleAbsolutePath,

    /// Path to the module's Cargo.toml, relative to the root dir
    ModuleRelativeManifestPath,

    /// Absolute path on disk to the module's Cargo.toml
    ModuleAbsoluteManifestPath,

    /// A list of all the features a module has.
    ModuleFeatures,

    /// Prints everything
    All,
}

impl Display for Fields {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Fields::Name => "Name",
                Fields::Version => "Version",
                Fields::GitVersion => "Git Version",
                Fields::ModuleRelativePath => "Module Relative Path",
                Fields::ModuleAbsolutePath => "Module Absolute Path",
                Fields::ModuleRelativeManifestPath => "Manifest Relative Path",
                Fields::ModuleAbsoluteManifestPath => "Manifest Absolute Path",
                Fields::ModuleFeatures => "Module Features",
                Fields::All => "All", // but it's not likely to be used!
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub field: Fields,
    pub value: Option<String>,
}
