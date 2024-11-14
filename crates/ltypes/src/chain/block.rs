use super::BlockV1;
use super::BlockV2;
use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deserialize, Serialize)]
pub enum Block {
    #[serde(rename = "Version1")]
    V1(BlockV1),
    #[serde(rename = "Version2")]
    V2(BlockV2),
}

// ------------------------------------------------------------------------
// Constructors.
// ------------------------------------------------------------------------

impl Block {
    pub fn new_v1(inner: BlockV1) -> Self {
        Self::V1(inner)
    }

    pub fn new_v2(inner: BlockV2) -> Self {
        Self::V2(inner)
    }
}
