//!
//! The Solidity compiler version.
//!

///
/// The Solidity compiler version.
///
#[derive(Debug, Clone)]
pub struct Version {
    /// The long version string.
    pub long: String,
    /// The short `semver`.
    pub default: semver::Version,
    /// The L2 revision additional versioning.
    pub l2_revision: Option<semver::Version>,
}

impl Version {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        long: String,
        default: semver::Version,
        l2_revision: Option<semver::Version>,
    ) -> Self {
        Self {
            long,
            default,
            l2_revision,
        }
    }
}
