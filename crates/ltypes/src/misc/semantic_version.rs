// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemanticVersion {
    /// Major version.
    pub major: u32,

    /// Minor version.
    pub minor: u32,

    /// Patch version.
    pub patch: u32,
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl SemanticVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        // TODO: validate inputs
        Self {
            major,
            minor,
            patch,
        }
    }
}
