use super::{BlockHash, EraEndV2, EraId, ProtocolVersion, Timestamp};
use crate::crypto::{Digest, PublicKey};

// ------------------------------------------------------------------------
// Declarations.
// ------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BlockV2Header {
    /// The parent block's hash.
    pub(super) parent_hash: BlockHash,
    /// The root hash of global state after the deploys in this block have been executed.
    pub(super) state_root_hash: Digest,
    /// The hash of the block's body.
    pub(super) body_hash: Digest,
    /// A random bit needed for initializing a future era.
    pub(super) random_bit: bool,
    /// A seed needed for initializing a future era.
    pub(super) accumulated_seed: Digest,
    /// The `EraEnd` of a block if it is a switch block.
    pub(super) era_end: Option<EraEndV2>,
    /// The timestamp from when the block was proposed.
    pub(super) timestamp: Timestamp,
    /// The era ID in which this block was created.
    pub(super) era_id: EraId,
    /// The height of this block, i.e. the number of ancestors.
    pub(super) height: u64,
    /// The protocol version of the network from when this block was created.
    pub(super) protocol_version: ProtocolVersion,
    /// The public key of the validator which proposed the block.
    pub(super) proposer: PublicKey,
    /// The gas price of the era
    pub(super) current_gas_price: u8,
    /// The most recent switch block hash.
    pub(super) last_switch_block_hash: Option<BlockHash>,
    #[cfg_attr(any(all(feature = "std", feature = "once_cell"), test), serde(skip))]
    #[cfg_attr(
        all(any(feature = "once_cell", test), feature = "datasize"),
        data_size(skip)
    )]
    #[cfg(any(feature = "once_cell", test))]
    pub(super) block_hash: OnceCell<BlockHash>,
}
