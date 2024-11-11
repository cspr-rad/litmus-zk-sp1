use super::BlockV1;
use super::BlockV2;

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Block {
    V1(BlockV1),
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
