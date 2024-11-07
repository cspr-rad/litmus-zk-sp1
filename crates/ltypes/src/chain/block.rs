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
