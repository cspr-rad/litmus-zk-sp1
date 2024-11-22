use crate::primitives::SemanticVersion;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(
    Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize,
)]
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
