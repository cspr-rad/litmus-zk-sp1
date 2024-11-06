use super::block_v1::BlockV1;
use super::block_v2::BlockV2;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Block {
    V1(BlockV1),
    V2(BlockV2),
}
