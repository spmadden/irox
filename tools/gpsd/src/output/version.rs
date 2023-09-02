//!
//! Structs around the version message

/// Response to an initial connection with version info
pub struct Version {
    /// Public release level
    pub release: String,

    /// Internal revision-control level
    pub rev: String,

    /// API major revision level
    pub proto_major: u8,

    /// API minor revision level
    pub proto_minor: u8,

    /// URL of the remote daemon reporting this version.  If empty, this is the version
    /// of the local daemon
    pub remote: Option<String>,
}
