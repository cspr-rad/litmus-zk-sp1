use crate::misc::SemanticVersion;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProtocolVersion(SemanticVersion);

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl ProtocolVersion {
    pub fn new(semantic_version: SemanticVersion) -> Self {
        // TODO: validate inputs
        Self(semantic_version)
    }
}
